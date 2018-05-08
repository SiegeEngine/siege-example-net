
pub use siege_net::packets::{Packet,
                             InitPacket, InitAckPacket, UpgradeRequiredPacket,
                             HeartbeatPacket, HeartbeatAckPacket, ShutdownPacket,
                             ShutdownCompletePacket};

mod login;
pub use self::login::LoginPacket;
mod login_success;
pub use self::login_success::LoginSuccessPacket;
mod login_failure;
pub use self::login_failure::LoginFailurePacket;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[repr(u8)]
pub enum GamePacket {
    Init(InitPacket),
    InitAck(InitAckPacket),
    UpgradeRequired(UpgradeRequiredPacket),
    Heartbeat(HeartbeatPacket),
    HeartbeatAck(HeartbeatAckPacket),
    Shutdown(ShutdownPacket),
    ShutdownComplete(ShutdownCompletePacket),
    Login(LoginPacket),
    LoginSuccess(LoginSuccessPacket),
    LoginFailure(LoginFailurePacket),
}

impl GamePacket {
    pub fn name(&self) -> &'static str
    {
        match *self {
            GamePacket::Init(_) => "Init",
            GamePacket::InitAck(_) => "InitAck",
            GamePacket::UpgradeRequired(_) => "UpgradeRequired",
            GamePacket::Heartbeat(_) => "Heartbeat",
            GamePacket::HeartbeatAck(_) => "HeartbeatAck",
            GamePacket::Shutdown(_) => "Shutdown",
            GamePacket::ShutdownComplete(_) => "ShutdownComplete",
            GamePacket::Login(_) => "Login",
            GamePacket::LoginSuccess(_) => "LoginSuccess",
            GamePacket::LoginFailure(_) => "LoginFailure",
        }
    }
}

impl Packet for GamePacket {
    fn reply_expected(&self) -> bool {
        match *self {
            GamePacket::Init(_) => true,
            GamePacket::Heartbeat(_) => true,
            GamePacket::Login(_) => true,
            _ => false
        }
    }
}
