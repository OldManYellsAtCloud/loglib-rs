//! Helper library for the log library
//!
//! This is intended to be the same as the C++ version of loglib,
//! however since C/C++ preprocessor macros are not supported in
//! Rust through FFI, it is rewritten in Rust itself.


use std::env;
use std::io::Write;
use std::os::unix::net::UnixStream;
use crate::enums::{LoggerType, LogLevel, RequestType};
use crate::structs::RegisterLogger::RegisterLogger;
use crate::structs::LogMessage::LogMessage;

mod enums;
mod structs;

struct Loglib {
    log_socket: Option<UnixStream>,
    default_name: String,
    min_log_level: LogLevel
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
        let mut register_logger = RegisterLogger::new (
            name.as_bytes().to_vec(),
            logger_type as i32
        );

        register_logger.send_message(&self.log_socket);
    }

    pub fn send_log(&mut self, msg: &str, level: LogLevel, name: &str){
        let mut log_message = LogMessage::new(
            name.as_bytes().to_vec(),
            msg.as_bytes().to_vec(),
            level as i32,
        );

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
        let mut ll = Loglib::new(String::from("msgtest99xx"));
        ll.register_logger(LoggerType::FILE, &"msgtest99xx");
        ll.send_log("important message", LogLevel::ERROR, "msgtest99xx");
    }
}