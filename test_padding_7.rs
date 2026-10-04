#[repr(C, packed(1))]
pub struct TPacketGCMove {
    pub header: u8,
    pub bFunc: u8,
    pub bArg: u8,
    pub bRot: u8,
    pub dwVID: u32,
    pub lX: i32,
    pub lY: i32,
    pub dwTime: u32,
    pub dwDuration: u8, // what gives 19 bytes?
}

fn main() {
    println!("{}", std::mem::size_of::<TPacketGCMove>());
}
