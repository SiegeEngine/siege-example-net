
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub struct LoginPacket {
    pub login: String,
    pub password: String,
}
impl LoginPacket {
    pub fn new(login: String, password: String) -> LoginPacket
    {
        LoginPacket {
            login: login,
            password: password,
        }
    }
}

#[test]
fn test() {
    use std::str::FromStr;
    use std::net::SocketAddr;
    use std::sync::Arc;
    use siege_net::Remote;
    use ring::rand::SystemRandom;
    use bincode::deserialize;
    use super::GamePacket;
    use ::{MAGIC, VERSION};

    let remote_addr: SocketAddr = FromStr::from_str("0.0.0.0:0").unwrap();
    let mut remote = Remote::new(remote_addr, Arc::new(SystemRandom::new())).unwrap();

    let login_packet = LoginPacket::new("mike".to_string(), "password".to_string());
    let packet = GamePacket::Login(login_packet.clone());
    let mut bytes: Vec<u8> = remote.serialize_packet(&packet, MAGIC, VERSION).unwrap();
    let (packet2_bytes,_,stale) = remote.deserialize_packet_header::<GamePacket>(
        &mut bytes[..]
    ).unwrap();
    assert_eq!(stale,false);
    let packet2: GamePacket = deserialize(packet2_bytes).unwrap();
    match packet2 {
        GamePacket::Login(login_packet2) => {
            assert_eq!(login_packet, login_packet2);
        }
        ,
        _ => panic!("Ser/De failed for LoginPacket"),
    }
}
