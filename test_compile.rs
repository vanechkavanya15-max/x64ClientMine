#[repr(C, packed(1))]
pub struct TPacketGCMove {
    pub header: u16,
    pub length: u16,
    pub b_func: u8,
    pub b_arg: u8,
    pub b_rot: u8,
    pub dw_vid: u32,
    pub l_x: i32,
    pub l_y: i32,
    pub dw_time: u32,
    pub dw_duration: u32,
}

fn main() {
    println!("{}", std::mem::size_of::<TPacketGCMove>());
}
