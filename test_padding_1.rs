#[repr(C, packed)]
pub struct TPacketGCMove {
    pub header: u16, // 2 bytes
    pub length: u16, // 2 bytes
    pub bFunc: u8, // 1 byte
    pub bArg: u8, // 1 byte
    pub bRot: u8, // 1 byte
    pub dwVID: u32, // 4 bytes
    pub lX: i32, // 4 bytes
    pub lY: i32, // 4 bytes
    pub dwTime: u32, // 4 bytes
    pub dwDuration: u32, // 4 bytes
}

fn main() {
    println!("Size: {}", std::mem::size_of::<TPacketGCMove>());
}
