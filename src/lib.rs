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

fn formatter(pattern: &str, args: &[&str]) -> String{
    let v: Vec<_> = pattern.match_indices("{}").collect();

    if v.len() != args.len() {
        return format!("Invalid pattern: {}", pattern);
    }

    let mut final_str = String::from(pattern);
    for arg in args {
        final_str = final_str.replacen("{}", arg, 1);
    }

    final_str
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


    #[test]
    fn formatter_test1(){
        let pattern = "there is no placeholder here";
        let args: [&str; 0] = [];
        let ret = formatter(pattern, &args);
        assert_eq!(ret, "there is no placeholder here");
    }

    #[test]
    fn formatter_test2(){
        let pattern = "wrong number of placeholders";
        let args: [&str; 1] = ["asd"];
        let ret = formatter(pattern, &args);
        assert_eq!(ret, "Invalid pattern: wrong number of placeholders");
    }

    #[test]
    fn formatter_test3(){
        let pattern = "{} {}";
        let args: [&str; 2] = ["one", "two"];
        let ret = formatter(pattern, &args);
        assert_eq!(ret, "one two");
    }

    #[test]
    fn formatter_test4(){
        let pattern = "assdfsdgf{}dsfgsdfg{}sfdgsdg{}";
        let args: [&str; 3] = ["one", "two", "three"];
        let ret = formatter(pattern, &args);
        assert_eq!(ret, "assdfsdgfonedsfgsdfgtwosfdgsdgthree");
    }
}