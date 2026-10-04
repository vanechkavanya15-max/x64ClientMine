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
}
