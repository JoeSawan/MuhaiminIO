use byteorder::{BigEndian, WriteBytesExt};
use eframe::egui;
use serialport::{DataBits, FlowControl, SerialPort}; //use serialport::{self, SerialPort};
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::time::Duration;
// ثوابت البروتوكول
const STX: u8 = 0x02;
const ETX: u8 = 0x03;

// أكواد الأوامر
const CMD_ANALOG_READ: u8 = 0xAE;
const CMD_PORT_WRITE: u8 = 0xBF;
const CMD_DDR_SET: u8 = 0xDD;
const CMD_PWM_WRITE: u8 = 0xE4;
const CMD_PIN_READ: u8 = 0xFE;

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
        if let Some(thread) = self.reader_thread.take() {
            thread.join().unwrap(); // انتظار انتهاء الثانوية
        }
        self.connected = false;
    }

    fn connect(&mut self) {
        if let Some(port_name) = &self.selected_port {
            match serialport::new(port_name, 115_200)
                .timeout(Duration::from_millis(100))
                .open()
            {
                Ok(port) => {
                    *self.serial_port.lock().unwrap() = Some(port);
                    self.connected = true;
                    self.start_reader_thread(); // بدء ثانوية القراءة هنا
                }
                Err(e) => eprintln!("Connection error: {}", e),
            }
        }
    }
    fn send_packet(&self, command: u8, params: &[u8]) -> Result<(), std::io::Error> {
        let mut packet = vec![STX, command];
        packet.extend_from_slice(params);
        packet.push(ETX);
        if let Some(port) = &mut *self.serial_port.lock().unwrap() {
            port.write_all(&packet)?;
        }
        Ok(())
    }

    fn start_reader_thread(&mut self) {
        let ctx = self.ctx.clone();
        let serial_port = Arc::clone(&self.serial_port);
        let data_tx = self.data_tx.clone();
        let mcu = Arc::clone(&self.mcu);

        self.reader_thread = Some(std::thread::spawn(move || {
            let mut buffer = Vec::new();
            loop {
                // 1. قفل الـ Mutex وحفظ الـ Guard
                let mut guard = match serial_port.lock() {
                    Ok(g) => g,
                    Err(_) => break,
                };

                // 2. استخراج المنفذ مع إبقاء الـ Guard نشطًا
                let port = match guard.as_mut() {
                    Some(p) => p,
                    None => break,
                };

                // 3. قراءة البيانات
                let mut byte = [0u8];
                match port.read(&mut byte) {
                    Ok(0) => continue,
                    Ok(_) => {
                        buffer.push(byte[0]);

                        // معالجة الباكت...
                        if let Some(etx_pos) = buffer.iter().position(|&b| b == ETX) {
                            if let Some(stx_pos) = buffer[..etx_pos].iter().position(|&b| b == STX)
                            {
                                let packet = &buffer[stx_pos..=etx_pos];

                                if packet.len() >= 3 {
                                    let command = packet[1];
                                    let params = &packet[2..packet.len() - 1];

                                    let _ = data_tx
                                        .send(format!("CMD: {:X} PARAMS: {:?}", command, params));

                                    match command {
                                        CMD_ANALOG_READ => {
                                            if params.len() >= 2 {
                                                let mut mcu_guard = mcu.lock().unwrap();
                                                let pin = params[0] as usize;
                                                let value =
                                                    (params[1] as u16) << 8 | params[2] as u16;
                                                mcu_guard.analog_pins[pin] = value;
                                            }
                                        }
                                        CMD_PIN_READ => {
                                            // معالجة قراءة المنفذ الرقمي
                                        }
                                        _ => {}
                                    }
                                }

                                buffer.drain(..=etx_pos);
                            }
                        }
                    }
                    Err(_) => break,
                }

                // 4. تحرير الـ Guard قبل الانتظار
                drop(guard); // مهم لإطلاق القفل
                ctx.request_repaint();
            }
        }));
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
                    self.send_packet(CMD_ANALOG_READ, &[0x09]);
                    self.start_reader_thread();
                }
            });
            if let Ok(data) = self.data_rx.try_recv() {
                *self.received_data.lock().unwrap() = data;
            }

            ui.label(&*self.received_data.lock().unwrap());
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
                        ui.label(if pin.pin > 0 { "1" } else { "0" });
                        match pin.mode {
                            PinMode::Input | PinMode::InputPullUp => {
                                ui.label(if pin.value > 0 { "1" } else { "0" });
                            }
                            PinMode::Pwm => {
                                ui.add(egui::Slider::new(&mut pin.value, 0..=255).text("PWM"));
                            }
                            PinMode::AnalogInput => {
                                ui.add(
                                    egui::ProgressBar::new(pin.value as f32 / 1023.0)
                                        .desired_width(200.0),
                                );
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
