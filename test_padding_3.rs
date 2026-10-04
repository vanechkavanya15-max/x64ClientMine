#[repr(C, packed(1))]
pub struct TPacketGCMove {
    pub header: u16, // 2
    pub length: u16, // 2
    pub bFunc: u8, // 1
    pub bArg: u8, // 1
    pub bRot: u8, // 1
    pub dwVID: u32, // 4
    pub lX: i32, // 4
    pub lY: i32, // 4
    pub dwTime: u32, // 4
    pub dwDuration: u32, // 4
}
// 2 + 2 + 1 + 1 + 1 + 4 + 4 + 4 + 4 + 4 = 27

fn main() {
    println!("{}", std::mem::size_of::<TPacketGCMove>());
}
