use instant::SystemTime;
use ron::error::SpannedError;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Error {
    Unspecified(String),
    Disconnected(SystemTime),
    Ron(SpannedError),
    //anywhere where the error can't satisfy the derives, we will just use a string
    IO(String),
}
