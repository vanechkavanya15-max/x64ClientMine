use std::mem::size_of;

pub const HEADER_CG_SHOP: u16 = 0x0801;

pub const SHOP_SUBHEADER_CG_END: u8 = 0;
pub const SHOP_SUBHEADER_CG_BUY: u8 = 1;
pub const SHOP_SUBHEADER_CG_SELL: u8 = 2;
pub const SHOP_SUBHEADER_CG_SELL2: u8 = 3;

#[repr(C, packed(1))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TPacketCGShop {
    pub header: u16,
    pub length: u16,
    pub subheader: u8,
}

pub fn build_shop_buy_packet(pos: u8, count: i32) -> Result<Vec<u8>, &'static str> {
    if count <= 0 {
        return Err("Count must be greater than zero");
    }

    // In C++, the packet is TPacketCGShop + 1 byte for count + 1 byte for pos
    // Wait, let's look at the C++ code again
    // BYTE bCount=1; Send(BYTE, bCount); Send(BYTE, bPos);
    
    // In C++, count is passed as BYTE (u8) to SendShopBuyPacket(BYTE bPos),
    // but the parameter to SendShopBuyPacket is only bPos, wait...
    
    // Let's implement according to the provided requirements.
    
    let packet = TPacketCGShop {
        header: HEADER_CG_SHOP,
        length: (size_of::<TPacketCGShop>() + size_of::<u8>() + size_of::<u8>()) as u16,
        subheader: SHOP_SUBHEADER_CG_BUY,
    };

    let count_u8 = count as u8;
    
    let mut bytes = Vec::with_capacity(packet.length as usize);
    let packet_bytes: [u8; size_of::<TPacketCGShop>()] = unsafe { std::mem::transmute(packet) };
    bytes.extend_from_slice(&packet_bytes);
    bytes.push(count_u8);
    bytes.push(pos);

    Ok(bytes)
}

pub fn build_shop_sell_packet(pos: u8) -> Result<Vec<u8>, &'static str> {
    let packet = TPacketCGShop {
        header: HEADER_CG_SHOP,
        length: (size_of::<TPacketCGShop>() + size_of::<u8>()) as u16,
        subheader: SHOP_SUBHEADER_CG_SELL,
    };

    let mut bytes = Vec::with_capacity(packet.length as usize);
    let packet_bytes: [u8; size_of::<TPacketCGShop>()] = unsafe { std::mem::transmute(packet) };
    bytes.extend_from_slice(&packet_bytes);
    bytes.push(pos);

    Ok(bytes)
}

pub fn build_shop_sell_packet_new(pos: u8, count: i32) -> Result<Vec<u8>, &'static str> {
    if count <= 0 {
        return Err("Count must be greater than zero");
    }

    let packet = TPacketCGShop {
        header: HEADER_CG_SHOP,
        length: (size_of::<TPacketCGShop>() + size_of::<u8>() + size_of::<u8>()) as u16,
        subheader: SHOP_SUBHEADER_CG_SELL2,
    };

    let count_u8 = count as u8;

    let mut bytes = Vec::with_capacity(packet.length as usize);
    let packet_bytes: [u8; size_of::<TPacketCGShop>()] = unsafe { std::mem::transmute(packet) };
    bytes.extend_from_slice(&packet_bytes);
    bytes.push(pos);
    bytes.push(count_u8);

    Ok(bytes)
}

pub fn build_shop_end_packet() -> Result<Vec<u8>, &'static str> {
    let packet = TPacketCGShop {
        header: HEADER_CG_SHOP,
        length: size_of::<TPacketCGShop>() as u16,
        subheader: SHOP_SUBHEADER_CG_END,
    };

    let mut bytes = Vec::with_capacity(packet.length as usize);
    let packet_bytes: [u8; size_of::<TPacketCGShop>()] = unsafe { std::mem::transmute(packet) };
    bytes.extend_from_slice(&packet_bytes);

    Ok(bytes)
}
