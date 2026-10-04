#[repr(C, packed)]
pub struct TPacketGCMove {
    pub header: u8, // 1
    pub length: u8, // 1
    pub bFunc: u8, // 1
    pub bArg: u8, // 1
    pub bRot: u8, // 1
    pub dwVID: u32, // 4
    pub lX: i32, // 4
    pub lY: i32, // 4
    pub dwTime: u32, // 4
    pub dwDuration: u8, // 1 -> 1+1+1+1+1+4+4+4+4+1 = 21 -> doesn't equal 19
}

fn main() {
    println!("{}", std::mem::size_of::<TPacketGCMove>());
}
