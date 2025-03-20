use eframe::egui;
use serialport::{DataBits, FlowControl, SerialPort};
use std::sync::{Arc, Mutex};
use std::time::Duration;

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
    Input,
    InputPullUp,
    Output,
    Pwm,
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
    running: bool,
    selected_port: Option<String>,
    ports: Vec<serialport::SerialPortInfo>,
    connected: bool,
    serial_port: Arc<Mutex<Option<Box<dyn SerialPort>>>>,
    received_data: Arc<Mutex<String>>,
    data_rx: crossbeam_channel::Receiver<String>,
    data_tx: crossbeam_channel::Sender<String>,
    reader_thread: Option<std::thread::JoinHandle<()>>,
}

impl App {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut pins = Vec::new();
        for i in 0..14 {
            let id = format!("D{}", i);
            let modes = if [3, 5, 6, 9, 10, 11].contains(&i) {
                vec![
                    PinMode::Input,
                    PinMode::InputPullUp,
                    PinMode::Output,
                    PinMode::Pwm,
                ]
            } else {
                vec![PinMode::Input, PinMode::InputPullUp, PinMode::Output]
            };
            pins.push(Pin::new(&id, modes));
        }
        for i in 0..8 {
            let id = format!("A{}", i);
            let modes = vec![
                PinMode::AnalogInput,
                PinMode::Input,
                PinMode::InputPullUp,
                PinMode::Output,
            ];
            pins.push(Pin::new(&id, modes));
        }

        let (data_tx, data_rx) = crossbeam_channel::bounded(1024);

        Self {
            pins,
            mcu: Arc::new(Mutex::new(MCU::default())),
            ctx: cc.egui_ctx.clone(),
            running: false,
            selected_port: None,
            ports: serialport::available_ports().unwrap_or_default(),
            connected: false,
            serial_port: Arc::new(Mutex::new(None)),
            received_data: Arc::new(Mutex::new(String::new())),
            data_rx,
            data_tx,
            reader_thread: None,
        }
    }

    fn mode_symbol(mode: &PinMode) -> &'static str {
        match mode {
            PinMode::Input => "IN",
            PinMode::InputPullUp => "IN↑",
            PinMode::Output => "OUT",
            PinMode::Pwm => "PWM",
            PinMode::AnalogInput => "A",
        }
    }

    fn refresh_ports(&mut self) {
        self.ports = serialport::available_ports().unwrap_or_default();
    }

    fn disconnect(&mut self) {
        *self.serial_port.lock().unwrap() = None;
        self.connected = false;
    }

    fn connect(&mut self) {
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
                }
                Err(e) => {
                    let mut data = self.received_data.lock().unwrap();
                    data.push_str(&format!("Connection error: {}\n", e));
                }
            }
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.running {
            // Logic for running state
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
            });

            ui.separator();

            egui::ScrollArea::vertical().show(ui, |ui| {
                for pin in &mut self.pins {
                    ui.horizontal(|ui| {
                        ui.label(&pin.id);
                        match pin.mode {
                            PinMode::Input | PinMode::InputPullUp => {
                                ui.label(if pin.value > 0 { "1" } else { "0" });
                            }
                            PinMode::Pwm => {
                                ui.add(egui::Slider::new(&mut pin.value, 0..=255).text("PWM"));
                            }
                            PinMode::AnalogInput => {
                                ui.add(egui::ProgressBar::new(pin.value as f32 / 1023.0));
                                ui.label(pin.value.to_string());
                            }
                            PinMode::Output => {
                                let mut state = pin.value > 0;
                                if ui.checkbox(&mut state, "").changed() {
                                    pin.value = state as u16;
                                }
                            }
                        }

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
        Box::new(|cc| Ok(Box::new(App::new(cc)))), // تم إضافة Ok هنا
    );
}
