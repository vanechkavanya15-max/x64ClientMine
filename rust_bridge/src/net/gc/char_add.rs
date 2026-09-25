#[derive(Debug, PartialEq, Clone)]
pub struct CharAddPacket {
    pub header: u8,
    pub vid: u32,
    pub angle: f32,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub b_type: u8,
    pub race_num: u16,
    pub moving_speed: u8,
    pub attack_speed: u8,
    pub state_flag: u8,
    pub affect_flag: [u32; 2],
}

impl CharAddPacket {
    pub const PACKET_SIZE: usize = 35;

    pub fn from_bytes(buf: &[u8]) -> Result<Self, String> {
        if buf.len() < Self::PACKET_SIZE {
            return Err("Bufor jest zbyt krotki".to_string());
        }

        let header = buf[0];
        let vid = u32::from_le_bytes(buf[1..5].try_into().unwrap());
        let angle = f32::from_le_bytes(buf[5..9].try_into().unwrap());
        let x = i32::from_le_bytes(buf[9..13].try_into().unwrap());
        let y = i32::from_le_bytes(buf[13..17].try_into().unwrap());
        let z = i32::from_le_bytes(buf[17..21].try_into().unwrap());
        let b_type = buf[21];
        let race_num = u16::from_le_bytes(buf[22..24].try_into().unwrap());
        let moving_speed = buf[24];
        let attack_speed = buf[25];
        let state_flag = buf[26];
        let affect_flag_0 = u32::from_le_bytes(buf[27..31].try_into().unwrap());
        let affect_flag_1 = u32::from_le_bytes(buf[31..35].try_into().unwrap());

        Ok(CharAddPacket {
            header,
            vid,
            angle,
            x,
            y,
            z,
            b_type,
            race_num,
            moving_speed,
            attack_speed,
            state_flag,
            affect_flag: [affect_flag_0, affect_flag_1],
        })
    }
}
