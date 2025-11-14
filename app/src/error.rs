use std::fmt;

#[derive(Debug)]
pub enum QuinkError {
    SerialError(String),
    ProtocolError(String),
    TimeoutError,
}

impl fmt::Display for QuinkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            QuinkError::SerialError(msg) => write!(f, "خطأ في المنفذ: {}", msg),
            QuinkError::ProtocolError(msg) => write!(f, "خطأ في البروتوكول: {}", msg),
            QuinkError::TimeoutError => write!(f, "انتهت المهلة الزمنية"),
        }
    }
}

impl std::error::Error for QuinkError {}
