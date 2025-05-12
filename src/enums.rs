pub mod enums {
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
        LogMessagw
    }
}

#[cfg(test)]
mod tests {
    use crate::enums::enums::RequestType;
    use super::*;

    #[test]
    fn level_value_test() {
        let debug = enums::LogLevel::DEBUG as i32;
        assert_eq!(debug, 0);

        let info = enums::LogLevel::INFO as i32;
        assert_eq!(info, debug + 1);

        let warning = enums::LogLevel::WARNING as i32;
        assert_eq!(warning, info + 1);

        let error = enums::LogLevel::ERROR as i32;
        assert_eq!(error, warning + 1);

        let fatal = enums::LogLevel::FATAL as i32;
        assert_eq!(fatal, error + 1);
    }

    #[test]
    fn type_value_test(){
        let console = enums::LoggerType::CONSOLE as i32;
        assert_eq!(console, 0);

        let file = enums::LoggerType::FILE as i32;
        assert_eq!(file, console + 1);
    }

    #[test]
    fn request_type_value_test(){
        let new_logger = RequestType::NEW_LOGGER as i32;
        assert_eq!(new_logger, 0);

        let log_message = RequestType::LOG_MESSAGE as i32;
        assert_eq!(log_message, new_logger + 1);
    }
}