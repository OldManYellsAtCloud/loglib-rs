use std::os::unix::net::UnixStream;

pub trait send_log {
    fn send_message(&self, socket: &Option<UnixStream>);
}