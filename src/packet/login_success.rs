
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub struct LoginSuccessPacket;

impl LoginSuccessPacket {
    pub fn new() -> LoginSuccessPacket
    {
        LoginSuccessPacket
    }
}
