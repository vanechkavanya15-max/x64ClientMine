use std::mem;
use std::sync::{RwLock, OnceLock};

pub const ITEM_SOCKET_SLOT_MAX_NUM: usize = 3;
pub const ITEM_ATTRIBUTE_SLOT_MAX_NUM: usize = 7;
pub const EXCHANGE_ITEM_MAX_NUM: usize = 12;

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TItemPos {
    pub window_type: u8,
    pub cell: u16,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TPlayerItemAttribute {
    pub b_type: u8,
    pub s_value: i16,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketGCExchange {
    pub header: u8,
    pub length: u16,
    pub subheader: u8,
    pub is_me: bool,
    pub arg1: u32,
    pub arg2: TItemPos,
    pub arg3: u32,
    pub al_values: [i32; ITEM_SOCKET_SLOT_MAX_NUM],
    pub a_attr: [TPlayerItemAttribute; ITEM_ATTRIBUTE_SLOT_MAX_NUM],
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExchangeSubHeader {
    Start = 0,
    ItemAdd = 1,
    ItemDel = 2,
    ElkAdd = 3,
    Accept = 4,
    End = 5,
    Already = 6,
    LessElk = 7,
}

impl TryFrom<u8> for ExchangeSubHeader {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ExchangeSubHeader::Start),
            1 => Ok(ExchangeSubHeader::ItemAdd),
            2 => Ok(ExchangeSubHeader::ItemDel),
            3 => Ok(ExchangeSubHeader::ElkAdd),
            4 => Ok(ExchangeSubHeader::Accept),
            5 => Ok(ExchangeSubHeader::End),
            6 => Ok(ExchangeSubHeader::Already),
            7 => Ok(ExchangeSubHeader::LessElk),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExchangeState {
    Idle,
    Open,
    Locked,
    Completed,
}

#[derive(Debug, Clone, Copy)]
pub struct ExchangeData {
    pub item_vnum: [u32; EXCHANGE_ITEM_MAX_NUM],
    pub item_count: [u8; EXCHANGE_ITEM_MAX_NUM],
    pub item_metin: [[u32; ITEM_SOCKET_SLOT_MAX_NUM]; EXCHANGE_ITEM_MAX_NUM],
    pub item_attr: [[TPlayerItemAttribute; ITEM_ATTRIBUTE_SLOT_MAX_NUM]; EXCHANGE_ITEM_MAX_NUM],
    pub accept: u8,
    pub elk: u32,
}

impl Default for ExchangeData {
    fn default() -> Self {
        Self {
            item_vnum: [0; EXCHANGE_ITEM_MAX_NUM],
            item_count: [0; EXCHANGE_ITEM_MAX_NUM],
            item_metin: [[0; ITEM_SOCKET_SLOT_MAX_NUM]; EXCHANGE_ITEM_MAX_NUM],
            item_attr: [[TPlayerItemAttribute::default(); ITEM_ATTRIBUTE_SLOT_MAX_NUM]; EXCHANGE_ITEM_MAX_NUM],
            accept: 0,
            elk: 0,
        }
    }
}

pub struct ExchangeLogic {
    pub state: ExchangeState,
    pub target_vid: u32,
    pub self_data: ExchangeData,
    pub target_data: ExchangeData,
}

impl ExchangeLogic {
    pub const fn new() -> Self {
        Self {
            state: ExchangeState::Idle,
            target_vid: 0,
            // Using default implementation of array init because we can't use Default::default() in const fn easily
            self_data: ExchangeData {
                item_vnum: [0; EXCHANGE_ITEM_MAX_NUM],
                item_count: [0; EXCHANGE_ITEM_MAX_NUM],
                item_metin: [[0; ITEM_SOCKET_SLOT_MAX_NUM]; EXCHANGE_ITEM_MAX_NUM],
                item_attr: [[TPlayerItemAttribute { b_type: 0, s_value: 0 }; ITEM_ATTRIBUTE_SLOT_MAX_NUM]; EXCHANGE_ITEM_MAX_NUM],
                accept: 0,
                elk: 0,
            },
            target_data: ExchangeData {
                item_vnum: [0; EXCHANGE_ITEM_MAX_NUM],
                item_count: [0; EXCHANGE_ITEM_MAX_NUM],
                item_metin: [[0; ITEM_SOCKET_SLOT_MAX_NUM]; EXCHANGE_ITEM_MAX_NUM],
                item_attr: [[TPlayerItemAttribute { b_type: 0, s_value: 0 }; ITEM_ATTRIBUTE_SLOT_MAX_NUM]; EXCHANGE_ITEM_MAX_NUM],
                accept: 0,
                elk: 0,
            },
        }
    }

    pub fn handle_packet(&mut self, packet: &TPacketGCExchange) -> bool {
        let subheader = match ExchangeSubHeader::try_from(packet.subheader) {
            Ok(sh) => sh,
            Err(_) => return true,
        };

        match subheader {
            ExchangeSubHeader::Start => {
                self.state = ExchangeState::Open;
                self.self_data = ExchangeData::default();
                self.target_data = ExchangeData::default();
                self.target_vid = packet.arg1;
                true
            }
            ExchangeSubHeader::ItemAdd => {
                let cell = packet.arg2.cell as usize;
                if cell < EXCHANGE_ITEM_MAX_NUM {
                    let data = if packet.is_me { &mut self.self_data } else { &mut self.target_data };
                    data.item_vnum[cell] = packet.arg1;
                    data.item_count[cell] = packet.arg3 as u8;
                    
                    for i in 0..ITEM_SOCKET_SLOT_MAX_NUM {
                        data.item_metin[cell][i] = packet.al_values[i] as u32;
                    }
                    for i in 0..ITEM_ATTRIBUTE_SLOT_MAX_NUM {
                        data.item_attr[cell][i] = packet.a_attr[i];
                    }
                }
                true
            }
            ExchangeSubHeader::ItemDel => {
                let cell = packet.arg1 as usize;
                if cell < EXCHANGE_ITEM_MAX_NUM {
                    let data = if packet.is_me { &mut self.self_data } else { &mut self.target_data };
                    data.item_vnum[cell] = 0;
                    data.item_count[cell] = 0;
                    data.item_metin[cell] = [0; ITEM_SOCKET_SLOT_MAX_NUM];
                    data.item_attr[cell] = [TPlayerItemAttribute::default(); ITEM_ATTRIBUTE_SLOT_MAX_NUM];
                }
                true
            }
            ExchangeSubHeader::ElkAdd => {
                let data = if packet.is_me { &mut self.self_data } else { &mut self.target_data };
                data.elk = packet.arg1;
                true
            }
            ExchangeSubHeader::Accept => {
                let data = if packet.is_me { &mut self.self_data } else { &mut self.target_data };
                data.accept = packet.arg1 as u8;
                self.state = ExchangeState::Locked;
                true
            }
            ExchangeSubHeader::End => {
                self.state = ExchangeState::Completed;
                true
            }
            ExchangeSubHeader::Already | ExchangeSubHeader::LessElk => {
                true
            }
        }
    }
}

pub fn get_exchange_logic() -> &'static RwLock<ExchangeLogic> {
    static EXCHANGE_LOGIC: OnceLock<RwLock<ExchangeLogic>> = OnceLock::new();
    EXCHANGE_LOGIC.get_or_init(|| RwLock::new(ExchangeLogic::new()))
}

#[no_mangle]
pub extern "C" fn m2_net_recv_exchange(buf: *const u8, len: usize) -> bool {
    if buf.is_null() || len < mem::size_of::<TPacketGCExchange>() {
        return false;
    }

    let packet = unsafe { &*(buf as *const TPacketGCExchange) };
    
    if let Ok(mut logic) = get_exchange_logic().write() {
        logic.handle_packet(packet)
    } else {
        false
    }
}
