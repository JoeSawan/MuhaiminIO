use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Pin {
    pub id: i32,
    pub name: String,
    pub mode: i32, // 1=PWM, 2=Analog, 3=Output, 4=Input, 5=InputPullup
    pub value: i32,
    pub supports_pwm: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SerialPortInfo {
    pub name: String,
    pub description: String,
}

#[derive(Clone, Debug)]
pub enum OutgoingRequest {
    AnalogRead(u8),
    PwmWrite { pin: u8, value: u16 },
    DigitalWrite { pin: u8, value: bool },
    PinRead(u8),
}

#[derive(Clone, Debug)]
pub enum IncomingEvent {
    AnalogValue { pin: u8, value: u16 },
    PwmAck { pin: u8 },
    DigitalValue { pin: u8, value: bool },
    Connected,
    Disconnected,
    Error(String),
}
