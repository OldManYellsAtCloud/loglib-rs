//! Helper library for the log library
//!
//! This is intended to be the same as the C++ version of loglib,
//! however since C/C++ preprocessor macros are not supported in
//! Rust through FFI, it is rewritten in Rust itself.

use std::env;
use std::io::Write;
use std::os::unix::net::UnixStream;
use crate::enums::enums::{LoggerType, LogLevel, RequestType};
mod enums;

struct Loglib {
    log_socket: Option<UnixStream>,
    default_name: String,
    min_log_level: LogLevel
}

#[repr(C)]
struct RegisterLogger {
    request_type: i32,
    name_length: i32,
    name: Vec<u8>,
    logger_type: i32
}

impl RegisterLogger {
    pub fn send_message(&self, socket: &Option<UnixStream>){
        socket.as_ref().unwrap().write_all(&self.request_type.to_ne_bytes()).unwrap();
        socket.as_ref().unwrap().write_all(&self.name_length.to_ne_bytes()).unwrap();
        socket.as_ref().unwrap().write_all(&self.name).unwrap();
        socket.as_ref().unwrap().write_all(&self.logger_type.to_ne_bytes()).unwrap();
    }
}

struct LogMessage {
    request_type: i32,
    name_length: i32,
    name: Vec<u8>,
    msg_length: i32,
    msg: Vec<u8>,
    log_level: i32
}

impl LogMessage {
    pub fn send_message(&self, socket: &Option<UnixStream>){
        socket.as_ref().unwrap().write_all(&self.request_type.to_ne_bytes()).unwrap();
        socket.as_ref().unwrap().write_all(&self.name_length.to_ne_bytes()).unwrap();
        socket.as_ref().unwrap().write_all(&self.name).unwrap();
        socket.as_ref().unwrap().write_all(&self.msg_length.to_ne_bytes()).unwrap();
        socket.as_ref().unwrap().write_all(&self.msg).unwrap();
        socket.as_ref().unwrap().write_all(&self.log_level.to_ne_bytes()).unwrap();
    }
}

impl Loglib {
    pub fn new(name: String) -> Loglib {
        let stream_path = Self::get_socket_path();

        Loglib {
            log_socket: Self::try_connection(),
            default_name: name,
            min_log_level: LogLevel::INFO
        }
    }

    fn get_socket_path() -> String {
        let ret = env::var("LOG_SERVER_SOCKET");
        if let Ok(path) = ret {
            path
        } else {
            String::from("/tmp/log_sock")
        }
    }

    fn try_connection() -> Option<UnixStream>{
        let stream = UnixStream::connect("/tmp/log_sock");
        match stream {
            Ok(connection) => Some(connection),
            Err(_) => {
                eprintln!("Couldn't connect to log socket.");
                None
            }
        }
    }

    pub fn register_logger(&mut self, logger_type: LoggerType, name: &str){
        let mut register_logger = RegisterLogger {
            request_type: RequestType::NewLogger as i32,
            name: name.as_bytes().to_vec(),
            name_length: name.len() as i32,
            logger_type: logger_type as i32
        };
        // add a null-terminator
        register_logger.name.push(0);
        register_logger.name_length += 1;
        register_logger.send_message(&self.log_socket);
    }

    pub fn send_log(&mut self, msg: &str, level: LogLevel, name: &str){
        let mut log_message = LogMessage{
            log_level: level as i32,
            msg: msg.as_bytes().to_vec(),
            request_type: RequestType::LogMessagw as i32,
            name: name.as_bytes().to_vec(),
            msg_length: msg.len() as i32,
            name_length: name.len() as i32
        };

        // null-terminators
        log_message.msg.push(0);
        log_message.msg_length += 1;
        log_message.name.push(0);
        log_message.name_length += 1;

        log_message.send_message(&self.log_socket);
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_loglib() {
        let ll = Loglib::new(String::from("defaultname"));
        assert_eq!(ll.min_log_level as i32, LogLevel::INFO as i32);
        assert_eq!(ll.default_name, "defaultname");
    }

    #[test]
    fn register_logger_test(){
        let mut ll = Loglib::new(String::from("testerino"));
        ll.register_logger(LoggerType::FILE, &"testerino1986");
        ll.send_log("important message", LogLevel::ERROR, "testerino1986");
    }
}