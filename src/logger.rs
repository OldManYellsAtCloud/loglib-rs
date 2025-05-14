use std::env;
use std::os::unix::net::UnixStream;
use crate::enums::{LoggerType, LogLevel};
use crate::structs::LogMessage::LogMessage;
use crate::structs::RegisterLogger::RegisterLogger;
use crate::send_log_trait::send_log;

#[macro_export]
macro_rules! log {
    ($log_lib:expr, $log_level:expr, $pattern:expr) => {
        match $log_level {
            LogLevel::DEBUG => $log_lib.debug($pattern),
            LogLevel::INFO => $log_lib.info($pattern),
            LogLevel::WARNING => $log_lib.warning($pattern),
            LogLevel::ERROR => $log_lib.error($pattern),
            LogLevel::FATAL => $log_lib.fatal($pattern),
        }
    };

    ($log_lib:expr, $log_level:expr, $pattern:expr, $($args:tt),*) => {
        let mut v = Vec::new();

        $(
            v.push(stringify!($args));
        )*

        match $log_level {
            LogLevel::DEBUG => $log_lib.debug_f($pattern, v.as_ref()),
            LogLevel::INFO => $log_lib.info_f($pattern, v.as_ref()),
            LogLevel::WARNING => $log_lib.warning_f($pattern, v.as_ref()),
            LogLevel::ERROR => $log_lib.error_f($pattern, v.as_ref()),
            LogLevel::FATAL => $log_lib.fatal_f($pattern, v.as_ref()),
        }
    };
}

pub struct Loglib {
    log_socket: Option<UnixStream>,
    default_name: String,
    min_log_level: LogLevel,
    message_buffer: Vec<Box<dyn send_log>>
}

impl Loglib {
    pub fn new(name: String) -> Loglib {
        Loglib {
            log_socket: Self::try_connection(),
            default_name: name,
            min_log_level: LogLevel::INFO,
            message_buffer: Vec::new()
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
            Err(e) => {
                eprintln!("Couldn't connect to log socket: {e}");
                None
            }
        }
    }

    pub fn register_logger(&mut self, logger_type: LoggerType, name: Option<&str>){
        let name = name.unwrap_or(&self.default_name);

        let mut register_logger = RegisterLogger::new (
            name.as_bytes().to_vec(),
            logger_type as i32
        );

        if self.log_socket.is_none() {
            self.log_socket = Self::try_connection();
        }

        if self.log_socket.is_none() {
            self.message_buffer.push(Box::new(register_logger));
        } else {
            self.send_late_messages();
            register_logger.send_message(&self.log_socket);
        }
    }

    pub fn send_log(&mut self, msg: &str, level: LogLevel, name: Option<&str>){
        let name = name.unwrap_or(&self.default_name);

        let mut log_message = LogMessage::new(
            name.as_bytes().to_vec(),
            msg.as_bytes().to_vec(),
            level as i32,
        );

        if self.log_socket.is_none() {
            self.log_socket = Self::try_connection();
        }

        if self.log_socket.is_none() {
            self.message_buffer.push(Box::new(log_message));
        } else {
            self.send_late_messages();
            log_message.send_message(&self.log_socket);
        }
    }

    pub fn set_min_log_level(&mut self, log_level: LogLevel){
        self.min_log_level = log_level;
    }

    fn send_late_messages(&mut self){
        for msg in &self.message_buffer {
            msg.send_message(&self.log_socket);
        }
        &self.message_buffer.clear();
    }

    fn format_and_log(&mut self, pattern: &str, args: &[&str], loglevel: LogLevel, name: Option<&str>){
        if loglevel < self.min_log_level {
            return;
        }
        let  final_msg = formatter(pattern, args);
        self.send_log(&final_msg, loglevel, name);
    }

    pub fn debug_f(&mut self, pattern: &str, args: &[&str]){
        self.format_and_log(pattern, args, LogLevel::DEBUG, None);
    }

    pub fn info_f(&mut self, pattern: &str, args: &[&str]){
        self.format_and_log(pattern, args, LogLevel::INFO, None);
    }

    pub fn warning_f(&mut self, pattern: &str, args: &[&str]){
        self.format_and_log(pattern, args, LogLevel::WARNING, None);
    }

    pub fn error_f(&mut self, pattern: &str, args: &[&str]){
        self.format_and_log(pattern, args, LogLevel::ERROR, None);
    }

    pub fn fatal_f(&mut self, pattern: &str, args: &[&str]){
        self.format_and_log(pattern, args, LogLevel::FATAL, None);
    }

    pub fn debug(&mut self, msg: &str){
        self.format_and_log(msg, &[], LogLevel::DEBUG, None);
    }

    pub fn info(&mut self, msg: &str){
        self.format_and_log(msg, &[], LogLevel::INFO, None);
    }

    pub fn warning(&mut self, msg: &str){
        self.format_and_log(msg, &[], LogLevel::WARNING, None);
    }

    pub fn error(&mut self, msg: &str){
        self.format_and_log(msg, &[], LogLevel::ERROR, None);
    }

    pub fn fatal(&mut self, msg: &str){
        self.format_and_log(msg, &[], LogLevel::FATAL, None);
    }

    pub fn debug_fn(&mut self, pattern: &str, args: &[&str], name: &str){
        self.format_and_log(pattern, args, LogLevel::DEBUG, Some(name));
    }

    pub fn info_fn(&mut self, pattern: &str, args: &[&str], name: &str){
        self.format_and_log(pattern, args, LogLevel::INFO, Some(name));
    }

    pub fn warning_fn(&mut self, pattern: &str, args: &[&str], name: &str){
        self.format_and_log(pattern, args, LogLevel::WARNING, Some(name));
    }

    pub fn error_fn(&mut self, pattern: &str, args: &[&str], name: &str){
        self.format_and_log(pattern, args, LogLevel::ERROR, Some(name));
    }

    pub fn fatal_fn(&mut self, pattern: &str, args: &[&str], name: &str){
        self.format_and_log(pattern, args, LogLevel::FATAL, Some(name));
    }

    pub fn debug_n(&mut self, msg: &str, name: &str){
        self.format_and_log(msg, &[], LogLevel::DEBUG, Some(name));
    }

    pub fn info_n(&mut self, msg: &str, name: &str){
        self.format_and_log(msg, &[], LogLevel::INFO, Some(name));
    }

    pub fn warning_n(&mut self, msg: &str, name: &str){
        self.format_and_log(msg, &[], LogLevel::WARNING, Some(name));
    }

    pub fn error_n(&mut self, msg: &str, name: &str){
        self.format_and_log(msg, &[], LogLevel::ERROR, Some(name));
    }

    pub fn fatal_n(&mut self, msg: &str, name: &str){
        self.format_and_log(msg, &[], LogLevel::FATAL, Some(name));
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
        ll.register_logger(LoggerType::FILE, Some(&"msgtest99xx"));
        ll.send_log("important message", LogLevel::ERROR, None);
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
        ll.register_logger(LoggerType::FILE, Some(&"formatter_test5"));
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