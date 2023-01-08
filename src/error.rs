use instant::SystemTime;

#[derive(Clone, Debug)]
pub enum ClientError {
    Disconnected(SystemTime),
}
