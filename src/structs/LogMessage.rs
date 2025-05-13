use std::io::Write;
use std::os::unix::net::UnixStream;
use crate::enums::RequestType;
use crate::send_log_trait::send_log;

pub struct LogMessage {
    request_type: i32,
    name_length: i32,
    name: Vec<u8>,
    msg_length: i32,
    msg: Vec<u8>,
    log_level: i32
}

impl LogMessage {
    pub fn new(name: Vec<u8>, msg: Vec<u8>, log_level: i32) -> LogMessage {
        LogMessage {
            request_type: RequestType::LogMessage as i32,
            name_length: name.len() as i32,
            name,
            msg_length: msg.len() as i32,
            msg,
            log_level
        }
    }
}

impl send_log for LogMessage {
    fn send_message(&self, socket: &Option<UnixStream>) {
        socket.as_ref().unwrap().write_all(&self.request_type.to_ne_bytes()).unwrap();
        socket.as_ref().unwrap().write_all(&self.name_length.to_ne_bytes()).unwrap();
        socket.as_ref().unwrap().write_all(&self.name).unwrap();
        socket.as_ref().unwrap().write_all(&self.msg_length.to_ne_bytes()).unwrap();
        socket.as_ref().unwrap().write_all(&self.msg).unwrap();
        socket.as_ref().unwrap().write_all(&self.log_level.to_ne_bytes()).unwrap();
    }
}