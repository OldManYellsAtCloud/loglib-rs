
pub enum LogLevel {
    DEBUG = 0,
    INFO,
    WARNING,
    ERROR,
    FATAL
}

pub enum LoggerType {
    CONSOLE = 0,
    FILE
}

pub enum RequestType {
    NewLogger = 0,
    LogMessage
}


#[cfg(test)]
mod tests {
    use crate::enums::RequestType;
    use super::*;

    #[test]
    fn level_value_test() {
        let debug = LogLevel::DEBUG as i32;
        assert_eq!(debug, 0);

        let info = LogLevel::INFO as i32;
        assert_eq!(info, debug + 1);

        let warning = LogLevel::WARNING as i32;
        assert_eq!(warning, info + 1);

        let error = LogLevel::ERROR as i32;
        assert_eq!(error, warning + 1);

        let fatal = LogLevel::FATAL as i32;
        assert_eq!(fatal, error + 1);
    }

    #[test]
    fn type_value_test(){
        let console = LoggerType::CONSOLE as i32;
        assert_eq!(console, 0);

        let file = LoggerType::FILE as i32;
        assert_eq!(file, console + 1);
    }

    #[test]
    fn request_type_value_test(){
        let new_logger = RequestType::NewLogger as i32;
        assert_eq!(new_logger, 0);

        let log_message = RequestType::LogMessage as i32;
        assert_eq!(log_message, new_logger + 1);
    }
}