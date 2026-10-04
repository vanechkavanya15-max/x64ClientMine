#[repr(C, packed)]
pub struct TPacketGCMove {
    pub header: u16,
    pub length: u16,
    pub bFunc: u8,
    pub bArg: u8,
    pub bRot: u8,
    pub dwVID: u32,
    pub lX: i32,
    pub lY: i32,
    pub dwTime: u32,
    pub dwDuration: u32,
}

fn main() {
    println!("{}", std::mem::size_of::<TPacketGCMove>());
    println!("header: {}", std::mem::offset_of!(TPacketGCMove, header));
    println!("length: {}", std::mem::offset_of!(TPacketGCMove, length));
    println!("bFunc: {}", std::mem::offset_of!(TPacketGCMove, bFunc));
    println!("bArg: {}", std::mem::offset_of!(TPacketGCMove, bArg));
    println!("bRot: {}", std::mem::offset_of!(TPacketGCMove, bRot));
    println!("dwVID: {}", std::mem::offset_of!(TPacketGCMove, dwVID));
    println!("lX: {}", std::mem::offset_of!(TPacketGCMove, lX));
    println!("lY: {}", std::mem::offset_of!(TPacketGCMove, lY));
    println!("dwTime: {}", std::mem::offset_of!(TPacketGCMove, dwTime));
    println!("dwDuration: {}", std::mem::offset_of!(TPacketGCMove, dwDuration));
}
