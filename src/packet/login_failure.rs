
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub struct LoginFailurePacket;

impl LoginFailurePacket {
    pub fn new() -> LoginFailurePacket {
        LoginFailurePacket
    }
}
