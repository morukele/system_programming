use rand::RngCore;
use smoltcp::wire;
use std::fmt::{Display, Formatter};

#[derive(Debug, Default)]
pub struct MacAddress([u8; 6]);

impl Display for MacAddress {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let octlet = &self.0;
        write!(
            f,
            "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
            octlet[0], octlet[1], octlet[2], octlet[3], octlet[4], octlet[5]
        )
    }
}

impl MacAddress {
    pub fn new() -> MacAddress {
        let mut octlets: [u8; 6] = [0; 6];
        rand::thread_rng().fill_bytes(&mut octlets);
        octlets[0] |= 0b_0000_0010;
        octlets[0] &= 0b_1111_1110;
        MacAddress { 0: octlets }
    }
}

impl Into<wire::EthernetAddress> for MacAddress {
    fn into(self) -> wire::EthernetAddress {
        wire::EthernetAddress { 0: self.0 }
    }
}
