use crate::models::{IncomingEvent, OutgoingRequest};
use crate::protocol::{decode_response, encode_request, FrameParser};
use serialport::SerialPort;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub struct SerialManager {
    port: Arc<Mutex<Option<Box<dyn SerialPort>>>>,
    tx: Sender<IncomingEvent>,
}

impl SerialManager {
    pub fn new() -> (Self, Receiver<IncomingEvent>) {
        let (tx, rx) = channel();
        let tx_clone = tx.clone();

        (
            SerialManager {
                port: Arc::new(Mutex::new(None)),
                tx: tx_clone,
            },
            rx,
        )
    }

    pub fn open(&self, port_name: &str, baud_rate: u32) -> Result<(), String> {
        let port = serialport::new(port_name, baud_rate)
            .timeout(Duration::from_millis(100))
            .open()
            .map_err(|e| format!("فشل الاتصال: {}", e))?;

        let mut port_lock = self.port.lock().unwrap();
        *port_lock = Some(port);

        // بدء حلقة القراءة
        let port_clone = Arc::clone(&self.port);
        let tx_clone = self.tx.clone();

        thread::spawn(move || {
            Self::read_loop(port_clone, tx_clone);
        });

        self.tx.send(IncomingEvent::Connected).ok();
        Ok(())
    }

    pub fn close(&self) -> Result<(), String> {
        let mut port_lock = self.port.lock().unwrap();
        *port_lock = None;
        self.tx.send(IncomingEvent::Disconnected).ok();
        Ok(())
    }

    pub fn send(&self, req: &OutgoingRequest) -> Result<(), String> {
        let packet = encode_request(req);

        let port_lock = self.port.lock().unwrap();
        if port_lock.is_some() {
            println!("إرسال: {:?}", packet);
            Ok(())
        } else {
            Err("المنفذ غير متصل".to_string())
        }
    }

    fn read_loop(port: Arc<Mutex<Option<Box<dyn SerialPort>>>>, tx: Sender<IncomingEvent>) {
        let mut parser = FrameParser::new();
        let mut buffer = [0; 256];

        loop {
            let mut port_lock = port.lock().unwrap();
            if let Some(p) = port_lock.as_mut() {
                match p.read(&mut buffer) {
                    Ok(n) => {
                        for &byte in &buffer[0..n] {
                            parser.push_byte(byte);

                            while let Some(frame) = parser.extract_frame() {
                                if let Some((cmd, payload)) = decode_response(&frame) {
                                    // معالجة الاستجابة
                                    match cmd {
                                        0xAE if payload.len() >= 3 => {
                                            let pin = payload[0];
                                            let value =
                                                ((payload[1] as u16) << 8) | (payload[2] as u16);
                                            tx.send(IncomingEvent::AnalogValue { pin, value }).ok();
                                        }
                                        0xE4 if payload.len() >= 1 && payload[0] == 0x01 => {
                                            let pin = payload.get(1).copied().unwrap_or(0);
                                            tx.send(IncomingEvent::PwmAck { pin }).ok();
                                        }
                                        _ => {}
                                    }
                                }
                            }
                        }
                    }
                    Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => {
                        // timeout طبيعي
                    }
                    Err(e) => {
                        tx.send(IncomingEvent::Error(format!("خطأ في القراءة: {}", e)))
                            .ok();
                        break;
                    }
                }
            } else {
                drop(port_lock);
                thread::sleep(Duration::from_millis(100));
                continue;
            }
            drop(port_lock);
        }
    }

    pub fn list_ports() -> Vec<String> {
        serialport::available_ports()
            .unwrap_or_default()
            .iter()
            .map(|p| p.port_name.clone())
            .collect()
    }
}
