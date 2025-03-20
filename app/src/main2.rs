use eframe::egui;

#[derive(Default)]
struct MCU {
    pin_b: u8,
    pin_c: u8,
    pin_d: u8,
    port_b: u8,
    port_d: u8,
    ddr_b: u8,
    ddr_c: u8,
    ddr_d: u8,
    analog_pins: [u16; 8],
}
#[derive(Clone, PartialEq)]
enum PinMode {
    INPUT,
    INPUT_PULL_UP,
    OUTPUT,
    PWM,
    AnalogInput,
}

#[derive(Clone)]
struct Pin {
    id: String,
    mode: PinMode,
    value: u16,
    pin: u8,
    modes: Vec<PinMode>,
}
impl Pin {
    fn new(id: &str, modes: Vec<PinMode>) -> Self {
        Self {
            id: id.to_string(),
            mode: modes[0].clone(),
            value: 0,
            pin: 0,
            modes,
        }
    }
}
struct App {
    pins: Vec<Pin>,
    mcu: Arc<Mutex<MCU>>,
    ctx: egui::Context,
}
impl App {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut pins = Vec::new();
        for i in 0..14 {
            let id = format!("D{}", i);
            let modes = if [3, 5, 6, 9, 10, 11].contains(&i) {
                vec![
                    PinMode::INPUT,
                    PinMode::INPUT_PULL_UP,
                    PinMode::OUTPUT,
                    PinMode::PWM,
                ]
            } else {
                vec![PinMode::INPUT, PinMode::INPUT_PULL_UP, PinMode::OUTPUT]
            };
            pins.push(Pin::new(&id, modes));
        }
        for i in 0..8 {
            let id = format!("A{}", i);
            let modes = vec![
                PinMode::AnalogInput,
                PinMode::INPUT,
                PinMode::INPUT_PULL_UP,
                PinMode::OUTPUT,
            ];
            pins.push(Pin::new(&id, modes));
        }
        Self {
            pins,
            mcu: Arc::new(Mutex::new(MCU::default())),
            ctx: cc.egui_ctx.clone(),
        }
    }
}
impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.running {
            self.update_pins_from_registers();
        }
        while let Ok(data) = self.data_rx.try_recv() {
            let mut received = self.received_data.lock().unwrap();
            received.push_str(&data);
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Arduino Pin Controller");

            ui.horizontal(|ui| {
                if ui
                    .button(if self.running {
                        "⏹ Stop"
                    } else {
                        "▶ Start"
                    })
                    .clicked()
                {
                    self.running = !self.running;
                    if self.running {
                        self.start_i_loop();
                    }
                }
            });
            ui.horizontal(|ui| {
                let combo_response = egui::ComboBox::from_label("Ports")
                    .selected_text(self.selected_port.as_deref().unwrap_or("Select port"))
                    .show_ui(ui, |ui| {
                        for port in &self.ports {
                            ui.selectable_value(
                                &mut self.selected_port,
                                Some(port.port_name.clone()),
                                &port.port_name,
                            );
                        }
                    });
                if combo_response.response.clicked() {
                    self.refresh_ports();
                }
                if ui
                    .button(if self.connected {
                        "Disconnect"
                    } else {
                        "Connect"
                    })
                    .clicked()
                {
                    if self.connected {
                        self.disconnect();
                    } else {
                        self.connect();
                    }
                }
                if !self.connected {
                    if let Some(port_name) = &self.selected_port {
                        match serialport::new(port_name, 115_200)
                            .timeout(Duration::from_millis(1))
                            .data_bits(DataBits::Eight)
                            .flow_control(FlowControl::None)
                            .open()
                        {
                            Ok(port) => {
                                *self.serial_port.lock().unwrap() = Some(port);
                                self.connected = true;
                                self.start_read_loop(ctx.clone());
                            }
                            Err(e) => {
                                let mut data = self.received_data.lock().unwrap();
                                data.push_str(&format!("Connection error: {}\n", e));
                            }
                        }
                    }
                } else {
                    // Cleanup resources
                    *self.serial_port.lock().unwrap() = None;
                    if let Some(handle) = self.reader_thread.take() {
                        let _ = handle.join();
                    }
                    self.connected = false;
                }
            });

            ui.separator();

            // 6. عرض صفوف الدبابيس
            egui::ScrollArea::vertical().show(ui, |ui| {
                for pin in &mut self.pins {
                    ui.horizontal(|ui| {
                        // عرض معرف الدبوس
                        ui.label(&pin.id);

                        // عرض عناصر التحكم حسب الوضع
                        match pin.mode {
                            PinMode::INPUT => {
                                ui.add(egui::Label::new(
                                    egui::RichText::new(if pin.value > 0 { "1" } else { "0" })
                                        .size(18.0),
                                ));
                            }
                            PinMode::INPUTPULLUP => {
                                ui.add(egui::Label::new(
                                    egui::RichText::new(if pin.value > 0 { "1" } else { "0" })
                                        .size(18.0),
                                ));
                            }
                            PinMode::PWM => {
                                ui.add(egui::Slider::new(&mut pin.value, 0..=255).text("PWM"));
                            }
                            PinMode::AnalogInput => {
                                ui.add(
                                    egui::ProgressBar::new(pin.value as f32).desired_width(200.0),
                                );
                                ui.add(egui::Label::new(
                                    egui::RichText::new(pin.value.to_string()).size(18.0),
                                ));
                            }
                            PinMode::OUTPUT => {
                                let mut state = pin.value > 0;
                                if ui.checkbox(&mut state, "").changed() {
                                    pin.value = state as u16;
                                }
                            }
                            _ => {}
                        }

                        // عرض مؤشر الحالة
                        ui.add(egui::Label::new(
                            egui::RichText::new(if pin.value > 0 { "1" } else { "0" }).size(18.0),
                        ));

                        // زر تغيير الوضع
                        if ui.button(Self::mode_symbol(&pin.mode)).clicked() {
                            let current_idx =
                                pin.modes.iter().position(|m| m == &pin.mode).unwrap();
                            pin.mode = pin.modes[(current_idx + 1) % pin.modes.len()].clone();
                        }
                    });
                }
            });
        });
    }
}
fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Muasitir-IO",
        options,
        Box::new(|cc| Ok(Box::new(App::new(cc)))), // تمرير cc هنا
    );
}
