#[derive(Debug, PartialEq)]
pub struct ItemPos {
    pub window_type: u8,
    pub cell: u16,
}

#[derive(Debug, PartialEq, Clone)]
pub struct PlayerItemAttribute {
    pub b_type: u8,
    pub s_value: i16,
}

#[derive(Debug, PartialEq)]
pub struct PacketGcItemSet {
    pub header: u8,
    pub cell: ItemPos,
    pub vnum: u32,
    pub count: u8,
    pub al_sockets: [i32; 3],
    pub a_attr: [PlayerItemAttribute; 7],
}

impl PacketGcItemSet {
    /// Deserializuje pakiet z bufora bajtow
    pub fn from_bytes(buf: &[u8]) -> Result<Self, String> {
        let expected_size = 42; // Calkowity rozmiar pakietu w bajtach

        if buf.len() < expected_size {
            return Err("Bufor jest za krotki".to_string());
        }

        let header = buf[0];
        let window_type = buf[1];
        let cell = u16::from_le_bytes([buf[2], buf[3]]);
        
        let vnum = u32::from_le_bytes([buf[4], buf[5], buf[6], buf[7]]);
        let count = buf[8];
        
        let mut al_sockets = [0; 3];
        for i in 0..3 {
            let offset = 9 + i * 4;
            al_sockets[i] = i32::from_le_bytes([buf[offset], buf[offset+1], buf[offset+2], buf[offset+3]]);
        }
        
        let mut a_attr = vec![];
        for i in 0..7 {
            let offset = 21 + i * 3;
            let b_type = buf[offset];
            let s_value = i16::from_le_bytes([buf[offset+1], buf[offset+2]]);
            a_attr.push(PlayerItemAttribute { b_type, s_value });
        }
        
        Ok(Self {
            header,
            cell: ItemPos { window_type, cell },
            vnum,
            count,
            al_sockets,
            a_attr: a_attr.try_into().unwrap(),
        })
    }
}
