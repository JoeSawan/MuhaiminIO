use slint::{ModelRc, SharedString, VecModel};
use std::rc::Rc;
use std::sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}};
use std::thread;
use std::time::Duration;
use serialport::SerialPort;

slint::include_modules!();

// دالة مساعدة لتحويل Vec<String> إلى نموذج يفهمه Slint
fn vec_to_model(items: Vec<String>) -> ModelRc<SharedString> {
    let shared_items: Vec<SharedString> = items.into_iter().map(|s| s.into()).collect();
    ModelRc::new(VecModel::from(shared_items))
}

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    
    // متغيرات مشتركة للتحكم في الاتصال
    // نستخدم connected_flag لإيقاف خيط القراءة عند قطع الاتصال
    let port_connection: Arc<Mutex<Option<Box<dyn SerialPort>>>> = Arc::new(Mutex::new(None));
    let connected_flag = Arc::new(AtomicBool::new(false));

    // --- منطق تحديث المنافذ (Refresh Ports) ---
    let ui_handle = ui.as_weak();
    ui.on_refresh_ports(move || {
        let ui = ui_handle.unwrap();
        // فحص المنافذ المتوفرة باستخدام مكتبة serialport
        let ports = serialport::available_ports().expect("No ports found!");
        let port_names: Vec<String> = ports.iter().map(|p| p.port_name.clone()).collect();
        
        // تحديث القائمة في الواجهة
        ui.set_available_ports(vec_to_model(port_names.clone()));
        
        // اختيار أول منفذ تلقائياً إذا وجد
        if let Some(first) = port_names.first() {
            ui.set_selected_port(first.clone().into());
        }
    });

    // استدعاء التحديث مرة واحدة عند بدء التشغيل
    // ملاحظة: لا يمكن استدعاء الـ callback مباشرة من Rust، لذا نكرر المنطق أو نترك المستخدم يضغط
    // سنقوم بتشغيل الفحص يدوياً عند البدء:
    let ports = serialport::available_ports().unwrap_or_default();
    let port_names: Vec<String> = ports.iter().map(|p| p.port_name.clone()).collect();
    ui.set_available_ports(vec_to_model(port_names.clone()));
    if let Some(first) = port_names.first() {
        ui.set_selected_port(first.clone().into());
    }

    // --- منطق الاتصال (Connect) ---
    let ui_handle = ui.as_weak();
    let port_conn_connect = port_connection.clone();
    let flag_connect = connected_flag.clone();

    ui.on_connect_clicked(move || {
        let ui = ui_handle.unwrap();
        let port_name = ui.get_selected_port();
        let baud_rate_str = ui.get_selected_baud();
        
        // تحويل النص إلى رقم
        let baud_rate = baud_rate_str.parse::<u32>().unwrap_or(9600);

        if port_name.is_empty() {
            ui.set_logs(ui.get_logs() + "Error: No port selected.\n");
            return;
        }

        match serialport::new(port_name.as_str(), baud_rate)
            .timeout(Duration::from_millis(10))
            .open() 
        {
            Ok(port) => {
                let mut port_reader = port.try_clone().expect("Failed to clone");
                
                // تخزين الاتصال
                let mut conn = port_conn_connect.lock().unwrap();
                *conn = Some(port);
                flag_connect.store(true, Ordering::SeqCst);

                // تحديث الواجهة
                ui.set_connection_status("Connected".into());
                ui.set_logs(ui.get_logs() + &SharedString::from(format!("Connected to {} at {}\n", port_name, baud_rate)));

                // تشغيل خيط القراءة
                let ui_weak_for_thread = ui_handle.clone();
                let flag_for_thread = flag_connect.clone();

                thread::spawn(move || {
                    let mut serial_buf: Vec<u8> = vec![0; 1000];
                    // يستمر الخيط في العمل طالما العلم true
                    while flag_for_thread.load(Ordering::SeqCst) {
                        match port_reader.read(serial_buf.as_mut_slice()) {
                            Ok(t) if t > 0 => {
                                let received_data = String::from_utf8_lossy(&serial_buf[..t]);
                                let _ = slint::invoke_from_event_loop(move || {
                                    if let Some(ui) = ui_weak_for_thread.upgrade() {
                                        ui.set_logs(ui.get_logs() + &SharedString::from(format!("RX: {}\n", received_data)));
                                    }
                                });
                            }
                            Ok(_) => {},
                            Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => (),
                            Err(_) => {
                                // قد يحدث خطأ عند فصل الكابل أو إغلاق المنفذ
                                break; 
                            }, 
                        }
                        thread::sleep(Duration::from_millis(10));
                    }
                });
            }
            Err(e) => {
                ui.set_logs(ui.get_logs() + &SharedString::from(format!("Connection Error: {}\n", e)));
            }
        }
    });

    // --- منطق قطع الاتصال (Disconnect) ---
    let ui_handle = ui.as_weak();
    let port_conn_disconnect = port_connection.clone();
    let flag_disconnect = connected_flag.clone();

    ui.on_disconnect_clicked(move || {
        let ui = ui_handle.unwrap();
        
        // إشارة للخيط بالتوقف
        flag_disconnect.store(false, Ordering::SeqCst);
        
        // تحرير المنفذ (إغلاقه)
        let mut conn = port_conn_disconnect.lock().unwrap();
        *conn = None; // Drop the port -> closes it

        ui.set_connection_status("Disconnected".into());
        ui.set_logs(ui.get_logs() + "Disconnected.\n");
    });

    // --- منطق الإرسال (Send) ---
    let port_conn_send = port_connection.clone();
    let ui_handle_send = ui.as_weak();
    
    ui.on_send_data(move |text| {
        let ui = ui_handle_send.unwrap();
        let mut conn = port_conn_send.lock().unwrap();
        
        if let Some(port) = conn.as_mut() {
            let data_to_send = format!("{}\n", text); // إضافة سطر جديد
            match port.write(data_to_send.as_bytes()) {
                Ok(_) => {
                    ui.set_logs(ui.get_logs() + &SharedString::from(format!("TX: {}", data_to_send)));
                },
                Err(e) => ui.set_logs(ui.get_logs() + &SharedString::from(format!("Send Error: {}\n", e))),
            }
        } else {
            ui.set_logs(ui.get_logs() + "Error: Not connected.\n");
        }
    });

    ui.run()
}