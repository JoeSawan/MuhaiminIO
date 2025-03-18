use eframe::egui;
use serialport::{DataBits, FlowControl, SerialPortInfo, available_ports};
use std::io::Write;
use std::sync::{
    Arc, Mutex,
    mpsc::{self, Receiver, Sender},
};
use std::thread::JoinHandle;
use std::time::Duration;

const MAX_DISPLAY_LINES: usize = 1000;
const READ_BUFFER_SIZE: usize = 4096;

struct SerialMonitor {
    ports: Vec<SerialPortInfo>,
    selected_port: Option<String>,
    baud_rate: u32,
    connected: bool,
    serial_port: Arc<Mutex<Option<Box<dyn serialport::SerialPort>>>>,
    input_buffer: String,
    received_data: Arc<Mutex<String>>,
    data_rx: Receiver<String>,
    data_tx: Sender<String>,
    reader_thread: Option<JoinHandle<()>>,
}

impl SerialMonitor {
    fn new() -> Self {
        let (data_tx, data_rx) = mpsc::channel();
        Self {
            ports: vec![],
            selected_port: None,
            baud_rate: 115_200,
            connected: false,
            serial_port: Arc::new(Mutex::new(None)),
            input_buffer: String::new(),
            received_data: Arc::new(Mutex::new(String::new())),
            data_rx,
            data_tx,
            reader_thread: None,
        }
    }

    fn refresh_ports(&mut self) {
        self.ports = available_ports().unwrap_or_default();
    }

    fn send_command(&self, command: &str) {
        let cmd = format!("{}\n", command.trim());
        let port = self.serial_port.clone();
        std::thread::spawn(move || {
            if let Some(p) = &mut *port.lock().unwrap() {
                let _ = p.write_all(cmd.as_bytes());
            }
        });
    }

    fn start_read_loop(&mut self, ctx: egui::Context) {
        let port_clone = self.serial_port.clone();
        let tx = self.data_tx.clone();

        self.reader_thread = Some(std::thread::spawn(move || {
            let mut buffer = [0; READ_BUFFER_SIZE];
            loop {
                if let Some(port) = &mut *port_clone.lock().unwrap() {
                    match port.read(&mut buffer) {
                        Ok(bytes_read) if bytes_read > 0 => {
                            let data = String::from_utf8_lossy(&buffer[..bytes_read]);
                            let _ = tx.send(data.to_string());
                        }
                        Err(_) => break,
                        _ => {}
                    }
                } else {
                    break;
                }
            }
        }));
    }

    fn clear_received_data(&mut self) {
        let mut data = self.received_data.lock().unwrap();
        data.clear();
    }

    fn save_received_data(&self) {
        let data = self.received_data.lock().unwrap();
        if let Err(e) = std::fs::write("serial_log.txt", &*data) {
            eprintln!("Failed to save data: {}", e);
        }
    }

    fn update_received_data(&mut self) {
        while let Ok(new_data) = self.data_rx.try_recv() {
            let mut data = self.received_data.lock().unwrap();
            data.push_str(&new_data);
            eprintln!("Received {} bytes", new_data.len());
            // Trim لحفظ الذاكرة مع الاحتفاظ بأحدث البيانات
            if data.len() > 100_000 {
                let start = data.len() - 50_000;
                *data = data.split_off(start);
            }
        }
    }
}

impl eframe::App for SerialMonitor {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.update_received_data();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🚀 Advanced Serial Monitor");

            // Port selection section
            ui.horizontal(|ui| {
                if ui.button("🔄 Refresh Ports").clicked() {
                    self.refresh_ports();
                }

                egui::ComboBox::from_label("📌 Port")
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

                egui::ComboBox::from_label("📶 Baud Rate")
                    .selected_text(format!("{}", self.baud_rate))
                    .show_ui(ui, |ui| {
                        for &rate in &[9600, 19200, 38400, 57600, 115200] {
                            ui.selectable_value(&mut self.baud_rate, rate, format!("{}", rate));
                        }
                    });
            });

            // Connection controls
            ui.horizontal(|ui| {
                if ui
                    .button(if self.connected {
                        "🔌 Disconnect"
                    } else {
                        "🔗 Connect"
                    })
                    .clicked()
                {
                    if !self.connected {
                        if let Some(port_name) = &self.selected_port {
                            match serialport::new(port_name, self.baud_rate)
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
                }

                ui.add_enabled(self.connected, egui::Button::new("🧹 Clear Data"))
                    .clicked()
                    .then(|| self.clear_received_data());

                if ui.button("💾 Save Data").clicked() {
                    self.save_received_data();
                }
            });

            ui.separator();

            // Received data display
            ui.label("📥 Received Data:");
            egui::ScrollArea::vertical()
                .max_height(300.0)
                .show(ui, |ui| {
                    let data = self.received_data.lock().unwrap().clone(); // انسخ البيانات بسرعة
                    ui.add(
                        egui::TextEdit::multiline(&mut data.as_str()).desired_width(f32::INFINITY),
                    );
                });

            ui.separator();

            // Command input section
            ui.vertical(|ui| {
                ui.label("📤 Send Command:");
                let response = ui.horizontal(|ui| {
                    let text_edit = egui::TextEdit::singleline(&mut self.input_buffer)
                        .desired_width(400.0)
                        .hint_text("Enter command...")
                        .id_source("command_input");

                    let output = ui.add(text_edit);

                    if (output.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)))
                        || ui
                            .add_enabled(
                                !self.input_buffer.is_empty(),
                                egui::Button::new("🚀 Send"),
                            )
                            .clicked()
                    {
                        if self.connected && !self.input_buffer.is_empty() {
                            self.send_command(&self.input_buffer);
                            self.input_buffer.clear();
                        }
                    }
                });
            });

            if !self.connected {
                ui.colored_label(egui::Color32::RED, "⚠️ Not connected to any port!");
            }
        });
    }
}

fn main() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Serial Monitor Pro",
        options,
        Box::new(|_cc| Ok(Box::new(SerialMonitor::new()))),
    );
}
