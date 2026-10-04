#[repr(C, packed(1))]
pub struct TPacketGCMove {
    pub bFunc: u8,
    pub bArg: u8,
    pub bRot: u8,
    pub dwVID: u32,
    pub lX: i32,
    pub lY: i32,
    pub dwTime: u32,
    pub dwDuration: u32,
}
// 1 + 1 + 1 + 4 + 4 + 4 + 4 + 4 = 23 -> maybe we need to drop length and header? Wait, 2 + 2 + 1 + 1 + 1 + 4 + 4 + 4 + 4 + 4 = 27
// wait, 2 + 2 = 4
// 4 + 1 + 1 + 1 = 7
// 7 + 4 = 11
// 11 + 4 = 15
// 15 + 4 = 19
// 19 + 4 = 23
// 23 + 4 = 27

// wait, how is it 19?
// header = 1 byte
// length = 1 byte (or no length)
// bFunc = 1 byte
// bArg = 1 byte
// bRot = 1 byte
// dwVID = 4 bytes
// lX = 4 bytes
// lY = 4 bytes
// dwTime = 4 bytes
// dwDuration = ? Wait!

// let's check
// 1 + 1 + 1 + 4 + 4 + 4 + 4 = 19
// bFunc(1) + bArg(1) + bRot(1) + dwVID(4) + lX(4) + lY(4) + dwTime(4) = 19 bytes!

fn main() {
    println!("{}", std::mem::size_of::<TPacketGCMove>());
}
