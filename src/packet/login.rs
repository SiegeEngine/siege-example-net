
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
    use remote::Remote;
    use ring::rand::SystemRandom;
    use super::Packet;

    let remote_addr: SocketAddr = FromStr::from_str("0.0.0.0:0").unwrap();
    let mut remote = Remote::new(remote_addr, Arc::new(SystemRandom::new())).unwrap();

    let login_packet = LoginPacket::new("mike".to_string(), "password".to_string());
    let packet = Packet::Login(login_packet.clone());
    let mut bytes: Vec<u8> = remote.serialize_packet(&packet).unwrap();
    let (packet2,_,stale) = remote.deserialize_packet(&mut bytes[..]).unwrap();
    assert_eq!(stale,false);
    match packet2 {
        Packet::Login(login_packet2) => {
            assert_eq!(login_packet, login_packet2);
        }
        ,
        _ => panic!("Ser/De failed for LoginPacket"),
    }
}
