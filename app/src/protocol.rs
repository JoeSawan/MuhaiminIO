use crate::models::OutgoingRequest;

const STX: u8 = 0xEB;
const ETX: u8 = 0xEE;

const CMD_ANALOG_READ: u8 = 0xAE;
const CMD_PWM_WRITE: u8 = 0xE4;
const CMD_PIN_READ: u8 = 0xFE;
const CMD_DIGITAL_WRITE: u8 = 0xBF;

pub struct FrameParser {
    buffer: Vec<u8>,
}

impl FrameParser {
    pub fn new() -> Self {
        FrameParser {
            buffer: Vec::with_capacity(256),
        }
    }

    pub fn push_byte(&mut self, byte: u8) {
        self.buffer.push(byte);

        // حذف البيانات القديمة إذا تجاوزت الحد
        if self.buffer.len() > 256 {
            self.buffer.remove(0);
        }
    }

    /// استخراج إطار واحد من المخزن المؤقت
    pub fn extract_frame(&mut self) -> Option<Vec<u8>> {
        // ابحث عن STX
        if let Some(start) = self.buffer.iter().position(|&b| b == STX) {
            // احذف كل شيء قبل STX
            self.buffer.drain(0..start);

            // ابحث عن ETX بعد STX
            if let Some(end) = self.buffer[1..].iter().position(|&b| b == ETX) {
                let end = end + 1; // تعديل الموضع

                // استخرج الإطار (STX...ETX)
                let frame = self.buffer.drain(0..=end).collect::<Vec<u8>>();
                return Some(frame);
            }
        }
        None
    }
}

pub fn encode_request(req: &OutgoingRequest) -> Vec<u8> {
    let mut packet = vec![STX];

    match req {
        OutgoingRequest::AnalogRead(pin) => {
            packet.push(CMD_ANALOG_READ);
            packet.push(*pin);
        }
        OutgoingRequest::PwmWrite { pin, value } => {
            packet.push(CMD_PWM_WRITE);
            packet.push(*pin);
            packet.push((value & 0xFF) as u8);
        }
        OutgoingRequest::DigitalWrite { pin, value } => {
            packet.push(CMD_DIGITAL_WRITE);
            packet.push(*pin);
            packet.push(*value as u8);
        }
        OutgoingRequest::PinRead(pin) => {
            packet.push(CMD_PIN_READ);
            packet.push(*pin);
        }
    }

    packet.push(ETX);
    packet
}

pub fn decode_response(frame: &[u8]) -> Option<(u8, Vec<u8>)> {
    if frame.len() < 3 {
        return None;
    }

    if frame[0] != STX || frame[frame.len() - 1] != ETX {
        return None;
    }

    let cmd = frame[1];
    let payload = frame[2..frame.len() - 1].to_vec();

    Some((cmd, payload))
}
