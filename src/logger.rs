use std::env;
use std::os::unix::net::UnixStream;
use crate::enums::{LoggerType, LogLevel};
use crate::structs::LogMessage::LogMessage;
use crate::structs::RegisterLogger::RegisterLogger;

pub struct Loglib {
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
        let stream = UnixStream::connect(Self::get_socket_path());
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

    pub fn send_log(&mut self, msg: &str, level: LogLevel){
        let mut log_message = LogMessage::new(
            self.default_name.as_bytes().to_vec(),
            msg.as_bytes().to_vec(),
            level as i32,
        );

        log_message.send_message(&self.log_socket);
    }

    fn format_and_log(&mut self, pattern: &str, args: &[&str], loglevel: LogLevel){
        let  final_msg = formatter(pattern, args);
        self.send_log(&final_msg, loglevel);
    }

    pub fn debug_f(&mut self, pattern: &str, args: &[&str]){
        self.format_and_log(pattern, args, LogLevel::DEBUG);
    }

    pub fn info_f(&mut self, pattern: &str, args: &[&str]){
        self.format_and_log(pattern, args, LogLevel::INFO);
    }

    pub fn warning_f(&mut self, pattern: &str, args: &[&str]){
        self.format_and_log(pattern, args, LogLevel::WARNING);
    }

    pub fn error_f(&mut self, pattern: &str, args: &[&str]){
        self.format_and_log(pattern, args, LogLevel::ERROR);
    }

    pub fn fatal_f(&mut self, pattern: &str, args: &[&str]){
        self.format_and_log(pattern, args, LogLevel::FATAL);
    }

    pub fn debug(&mut self, msg: &str){
        self.format_and_log(msg, &[], LogLevel::DEBUG);
    }

    pub fn info(&mut self, msg: &str){
        self.format_and_log(msg, &[], LogLevel::INFO);
    }

    pub fn warning(&mut self, msg: &str){
        self.format_and_log(msg, &[], LogLevel::WARNING);
    }

    pub fn error(&mut self, msg: &str){
        self.format_and_log(msg, &[], LogLevel::ERROR);
    }

    pub fn fatal(&mut self, msg: &str){
        self.format_and_log(msg, &[], LogLevel::FATAL);
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
        ll.send_log("important message", LogLevel::ERROR);
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

    #[test]
    fn formatter_test5(){
        let mut ll = Loglib::new(String::from("formatter_test5"));
        ll.register_logger(LoggerType::FILE, &"formatter_test5");
        ll.debug("debug message without args");
        ll.debug_f("debug message arg1: {}, arg2: {}", &["1111", "2222"]);

        ll.info("info message without args");
        ll.info_f("info message arg1: {}, arg2: {}", &["1111", "2222"]);

        ll.warning("warning message without args");
        ll.warning_f("warning message arg1: {}, arg2: {}", &["1111", "2222"]);

        ll.error("error message without args");
        ll.error_f("error message arg1: {}, arg2: {}", &["1111", "2222"]);

        ll.fatal("fatal message without args");
        ll.fatal_f("fatal message arg1: {}, arg2: {}", &["1111", "2222"]);
    }
}