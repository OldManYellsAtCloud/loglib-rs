use std::io::Write;
use std::os::unix::net::UnixStream;
use crate::enums::RequestType;
use crate::send_log_trait::send_log;

pub struct RegisterLogger {
    request_type: i32,
    name_length: i32,
    name: Vec<u8>,
    logger_type: i32
}

impl RegisterLogger {
    pub fn new(name: Vec<u8>, logger_type: i32) -> RegisterLogger {
        RegisterLogger {
            request_type: RequestType::NewLogger as i32,
            name_length: name.len() as i32,
            name,
            logger_type
        }
    }
}

impl send_log for RegisterLogger{
    fn send_message(&self, socket: &Option<UnixStream>){
        socket.as_ref().unwrap().write_all(&self.request_type.to_ne_bytes()).unwrap();
        socket.as_ref().unwrap().write_all(&self.name_length.to_ne_bytes()).unwrap();
        socket.as_ref().unwrap().write_all(&self.name).unwrap();
        socket.as_ref().unwrap().write_all(&self.logger_type.to_ne_bytes()).unwrap();
    }
}