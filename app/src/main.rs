mod error;
mod models;
mod protocol;
mod serial;
mod serial_manager;

// Include the Slint-generated Rust file from the build script output.
include!(concat!(env!("OUT_DIR"), "/mine_window.rs"));
use slint::{ComponentHandle, SharedString};
use std::sync::{Arc, Mutex};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ui = r#MainWindow::new()?;

    // إنشاء SerialManager
    let serial_mgr: Arc<
        Mutex<
            Option<(
                serial_manager::SerialManager,
                std::sync::mpsc::Receiver<models::IncomingEvent>,
            )>,
        >,
    > = Arc::new(Mutex::new(None));

    // ربط callback: connect
    let serial_mgr_clone = Arc::clone(&serial_mgr);
    let ui_weak = ui.as_weak();
    ui.on_connect(move |port: SharedString| {
        let port_str = port.to_string();
        let (mgr, rx) = serial_manager::SerialManager::new();
        match mgr.open(&port_str, 115200) {
            Ok(_) => {
                println!("تم الاتصال بـ: {}", port_str);

                // تحديث الـ UI
                if let Some(ui) = ui_weak.upgrade() {
                    ui.set_connected(true);
                    ui.set_connected_port(SharedString::from(port_str.clone()));
                }

                // تخزين المدير
                *serial_mgr_clone.lock().unwrap() = Some((mgr, rx));
            }
            Err(e) => {
                eprintln!("فشل الاتصال: {}", e);
                if let Some(ui) = ui_weak.upgrade() {
                    ui.set_connected(false);
                }
            }
        }
    });

    // ربط callback: disconnect
    let serial_mgr_clone = Arc::clone(&serial_mgr);
    let ui_weak = ui.as_weak();
    ui.on_disconnect(move || {
        if let Some((mgr, _)) = serial_mgr_clone.lock().unwrap().take() {
            mgr.close().ok();
            println!("تم قطع الاتصال");

            // تحديث الـ UI
            if let Some(ui) = ui_weak.upgrade() {
                ui.set_connected(false);
                ui.set_connected_port(SharedString::from(""));
            }
        }
    });

    // ربط callback: list_ports
    let ui_weak = ui.as_weak();
    ui.on_list_ports(move || {
        let ports = serial_manager::SerialManager::list_ports();
        println!("الأطراف المتاحة: {:?}", ports);

        // تحديث الـ UI بقائمة الأطراف
        if let Some(ui) = ui_weak.upgrade() {
            let ports_slint: Vec<SharedString> = ports
                .iter()
                .map(|p| SharedString::from(p.clone()))
                .collect();
            ui.set_available_ports(slint::ModelRc::from(ports_slint.as_slice()));
        }
    });

    // ربط callback: send_pwm
    let serial_mgr_clone = Arc::clone(&serial_mgr);
    ui.on_send_pwm(move |pin: i32, value: i32| {
        if let Ok(mgr_lock) = serial_mgr_clone.lock() {
            if let Some((mgr, _)) = mgr_lock.as_ref() {
                let req = models::OutgoingRequest::PwmWrite {
                    pin: pin as u8,
                    value: value as u16,
                };
                mgr.send(&req).ok();
                println!("إرسال PWM: Pin {} = {}", pin, value);
            }
        }
    });

    // ربط callback: request_analog
    let serial_mgr_clone = Arc::clone(&serial_mgr);
    ui.on_request_analog(move |pin: i32| {
        if let Ok(mgr_lock) = serial_mgr_clone.lock() {
            if let Some((mgr, _)) = mgr_lock.as_ref() {
                let req = models::OutgoingRequest::AnalogRead(pin as u8);
                mgr.send(&req).ok();
                println!("طلب قراءة تماثلية: Pin {}", pin);
            }
        }
    });

    // (No toggle_pin callback on MainWindow currently.)

    let _ = ui.run();
    Ok(())
}
