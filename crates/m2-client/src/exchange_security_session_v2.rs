use std::ffi::CStr;

pub const EXCHANGE_ITEM_MAX_NUM: usize = 12;
pub const ITEM_SOCKET_SLOT_MAX_NUM: usize = 3;
pub const ITEM_ATTRIBUTE_SLOT_MAX_NUM: usize = 7;
pub const CHARACTER_NAME_MAX_LEN: usize = 64; // Fallback to 64 as standard in Packet.h

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TPlayerItemAttribute {
    pub b_type: u8,
    pub s_value: i16,
}

#[derive(Debug, Clone, Copy)]
pub struct TExchangeData {
    pub name: [u8; CHARACTER_NAME_MAX_LEN + 1],
    pub item_vnum: [u32; EXCHANGE_ITEM_MAX_NUM],
    pub item_count: [u8; EXCHANGE_ITEM_MAX_NUM],
    pub item_metin: [[u32; ITEM_SOCKET_SLOT_MAX_NUM]; EXCHANGE_ITEM_MAX_NUM],
    pub item_attr: [[TPlayerItemAttribute; ITEM_ATTRIBUTE_SLOT_MAX_NUM]; EXCHANGE_ITEM_MAX_NUM],
    pub accept: u8,
    pub elk: u32,
}

impl Default for TExchangeData {
    fn default() -> Self {
        Self {
            name: [0; CHARACTER_NAME_MAX_LEN + 1],
            item_vnum: [0; EXCHANGE_ITEM_MAX_NUM],
            item_count: [0; EXCHANGE_ITEM_MAX_NUM],
            item_metin: [[0; ITEM_SOCKET_SLOT_MAX_NUM]; EXCHANGE_ITEM_MAX_NUM],
            item_attr: [[TPlayerItemAttribute::default(); ITEM_ATTRIBUTE_SLOT_MAX_NUM]; EXCHANGE_ITEM_MAX_NUM],
            accept: 0,
            elk: 0,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ExchangeError {
    ExchangeIsLocked,
    InvalidSlot,
    InvalidSocket,
    InvalidAttribute,
    NotTrading,
}

pub struct CPythonExchange {
    m_is_trading: bool,
    m_elk_mode: bool,
    m_self: TExchangeData,
    m_victim: TExchangeData,
}

impl CPythonExchange {
    pub fn new() -> Self {
        Self {
            m_is_trading: false,
            m_elk_mode: false,
            m_self: TExchangeData::default(),
            m_victim: TExchangeData::default(),
        }
    }

    pub fn clear(&mut self) {
        self.m_self = TExchangeData::default();
        self.m_victim = TExchangeData::default();
    }

    pub fn start(&mut self) {
        self.m_is_trading = true;
    }

    pub fn end(&mut self) {
        self.m_is_trading = false;
    }

    pub fn is_trading(&self) -> bool {
        self.m_is_trading
    }

    pub fn get_elk_mode(&self) -> bool {
        self.m_elk_mode
    }

    pub fn set_elk_mode(&mut self, value: bool) {
        self.m_elk_mode = value;
    }

    pub fn can_modify_exchange(&self) -> bool {
        if self.m_self.accept != 0 || self.m_victim.accept != 0 {
            return false;
        }
        true
    }

    pub fn set_self_name(&mut self, name: &str) {
        let bytes = name.as_bytes();
        let copy_len = bytes.len().min(CHARACTER_NAME_MAX_LEN);
        self.m_self.name[..copy_len].copy_from_slice(&bytes[..copy_len]);
        self.m_self.name[copy_len] = 0;
    }

    pub fn set_target_name(&mut self, name: &str) {
        let bytes = name.as_bytes();
        let copy_len = bytes.len().min(CHARACTER_NAME_MAX_LEN);
        self.m_victim.name[..copy_len].copy_from_slice(&bytes[..copy_len]);
        self.m_victim.name[copy_len] = 0;
    }

    pub fn get_name_from_self(&self) -> String {
        let c_str = unsafe { CStr::from_ptr(self.m_self.name.as_ptr() as *const i8) };
        c_str.to_string_lossy().into_owned()
    }

    pub fn get_name_from_target(&self) -> String {
        let c_str = unsafe { CStr::from_ptr(self.m_victim.name.as_ptr() as *const i8) };
        c_str.to_string_lossy().into_owned()
    }

    pub fn set_elk_to_target(&mut self, elk: u32) -> Result<(), ExchangeError> {
        if !self.can_modify_exchange() {
            return Err(ExchangeError::ExchangeIsLocked);
        }
        self.m_victim.elk = elk;
        Ok(())
    }

    pub fn set_elk_to_self(&mut self, elk: u32) -> Result<(), ExchangeError> {
        if !self.can_modify_exchange() {
            return Err(ExchangeError::ExchangeIsLocked);
        }
        self.m_self.elk = elk;
        Ok(())
    }

    pub fn get_elk_from_target(&self) -> u32 {
        self.m_victim.elk
    }

    pub fn get_elk_from_self(&self) -> u32 {
        self.m_self.elk
    }

    pub fn set_item_to_target(&mut self, pos: u8, vnum: u32, count: u8) -> Result<(), ExchangeError> {
        if !self.can_modify_exchange() {
            return Err(ExchangeError::ExchangeIsLocked);
        }
        let pos = pos as usize;
        if pos >= EXCHANGE_ITEM_MAX_NUM {
            return Err(ExchangeError::InvalidSlot);
        }
        self.m_victim.item_vnum[pos] = vnum;
        self.m_victim.item_count[pos] = count;
        Ok(())
    }

    pub fn set_item_to_self(&mut self, pos: u8, vnum: u32, count: u8) -> Result<(), ExchangeError> {
        if !self.can_modify_exchange() {
            return Err(ExchangeError::ExchangeIsLocked);
        }
        let pos = pos as usize;
        if pos >= EXCHANGE_ITEM_MAX_NUM {
            return Err(ExchangeError::InvalidSlot);
        }
        self.m_self.item_vnum[pos] = vnum;
        self.m_self.item_count[pos] = count;
        Ok(())
    }

    pub fn set_item_metin_socket_to_target(&mut self, pos: u8, imetinpos: u8, vnum: u32) -> Result<(), ExchangeError> {
        if !self.can_modify_exchange() {
            return Err(ExchangeError::ExchangeIsLocked);
        }
        let pos = pos as usize;
        let imetinpos = imetinpos as usize;
        if pos >= EXCHANGE_ITEM_MAX_NUM {
            return Err(ExchangeError::InvalidSlot);
        }
        if imetinpos >= ITEM_SOCKET_SLOT_MAX_NUM {
            return Err(ExchangeError::InvalidSocket);
        }
        self.m_victim.item_metin[pos][imetinpos] = vnum;
        Ok(())
    }

    pub fn set_item_metin_socket_to_self(&mut self, pos: u8, imetinpos: u8, vnum: u32) -> Result<(), ExchangeError> {
        if !self.can_modify_exchange() {
            return Err(ExchangeError::ExchangeIsLocked);
        }
        let pos = pos as usize;
        let imetinpos = imetinpos as usize;
        if pos >= EXCHANGE_ITEM_MAX_NUM {
            return Err(ExchangeError::InvalidSlot);
        }
        if imetinpos >= ITEM_SOCKET_SLOT_MAX_NUM {
            return Err(ExchangeError::InvalidSocket);
        }
        self.m_self.item_metin[pos][imetinpos] = vnum;
        Ok(())
    }

    pub fn set_item_attribute_to_target(&mut self, pos: u8, iattrpos: u8, by_type: u8, s_value: i16) -> Result<(), ExchangeError> {
        if !self.can_modify_exchange() {
            return Err(ExchangeError::ExchangeIsLocked);
        }
        let pos = pos as usize;
        let iattrpos = iattrpos as usize;
        if pos >= EXCHANGE_ITEM_MAX_NUM {
            return Err(ExchangeError::InvalidSlot);
        }
        if iattrpos >= ITEM_ATTRIBUTE_SLOT_MAX_NUM {
            return Err(ExchangeError::InvalidAttribute);
        }
        self.m_victim.item_attr[pos][iattrpos].b_type = by_type;
        self.m_victim.item_attr[pos][iattrpos].s_value = s_value;
        Ok(())
    }

    pub fn set_item_attribute_to_self(&mut self, pos: u8, iattrpos: u8, by_type: u8, s_value: i16) -> Result<(), ExchangeError> {
        if !self.can_modify_exchange() {
            return Err(ExchangeError::ExchangeIsLocked);
        }
        let pos = pos as usize;
        let iattrpos = iattrpos as usize;
        if pos >= EXCHANGE_ITEM_MAX_NUM {
            return Err(ExchangeError::InvalidSlot);
        }
        if iattrpos >= ITEM_ATTRIBUTE_SLOT_MAX_NUM {
            return Err(ExchangeError::InvalidAttribute);
        }
        self.m_self.item_attr[pos][iattrpos].b_type = by_type;
        self.m_self.item_attr[pos][iattrpos].s_value = s_value;
        Ok(())
    }

    pub fn del_item_of_target(&mut self, pos: u8) -> Result<(), ExchangeError> {
        if !self.can_modify_exchange() {
            return Err(ExchangeError::ExchangeIsLocked);
        }
        let pos = pos as usize;
        if pos >= EXCHANGE_ITEM_MAX_NUM {
            return Err(ExchangeError::InvalidSlot);
        }
        self.m_victim.item_vnum[pos] = 0;
        self.m_victim.item_count[pos] = 0;
        Ok(())
    }

    pub fn del_item_of_self(&mut self, pos: u8) -> Result<(), ExchangeError> {
        if !self.can_modify_exchange() {
            return Err(ExchangeError::ExchangeIsLocked);
        }
        let pos = pos as usize;
        if pos >= EXCHANGE_ITEM_MAX_NUM {
            return Err(ExchangeError::InvalidSlot);
        }
        self.m_self.item_vnum[pos] = 0;
        self.m_self.item_count[pos] = 0;
        Ok(())
    }

    pub fn get_item_vnum_from_target(&self, pos: u8) -> u32 {
        let pos = pos as usize;
        if pos >= EXCHANGE_ITEM_MAX_NUM {
            return 0;
        }
        self.m_victim.item_vnum[pos]
    }

    pub fn get_item_vnum_from_self(&self, pos: u8) -> u32 {
        let pos = pos as usize;
        if pos >= EXCHANGE_ITEM_MAX_NUM {
            return 0;
        }
        self.m_self.item_vnum[pos]
    }

    pub fn get_item_count_from_target(&self, pos: u8) -> u8 {
        let pos = pos as usize;
        if pos >= EXCHANGE_ITEM_MAX_NUM {
            return 0;
        }
        self.m_victim.item_count[pos]
    }

    pub fn get_item_count_from_self(&self, pos: u8) -> u8 {
        let pos = pos as usize;
        if pos >= EXCHANGE_ITEM_MAX_NUM {
            return 0;
        }
        self.m_self.item_count[pos]
    }

    pub fn get_item_metin_socket_from_target(&self, pos: u8, i_metin_socket_pos: u8) -> u32 {
        let pos = pos as usize;
        let i_metin_socket_pos = i_metin_socket_pos as usize;
        if pos >= EXCHANGE_ITEM_MAX_NUM || i_metin_socket_pos >= ITEM_SOCKET_SLOT_MAX_NUM {
            return 0;
        }
        self.m_victim.item_metin[pos][i_metin_socket_pos]
    }

    pub fn get_item_metin_socket_from_self(&self, pos: u8, i_metin_socket_pos: u8) -> u32 {
        let pos = pos as usize;
        let i_metin_socket_pos = i_metin_socket_pos as usize;
        if pos >= EXCHANGE_ITEM_MAX_NUM || i_metin_socket_pos >= ITEM_SOCKET_SLOT_MAX_NUM {
            return 0;
        }
        self.m_self.item_metin[pos][i_metin_socket_pos]
    }

    pub fn get_item_attribute_from_target(&self, pos: u8, i_attr_pos: u8, pby_type: &mut u8, ps_value: &mut i16) {
        let pos = pos as usize;
        let i_attr_pos = i_attr_pos as usize;
        if pos >= EXCHANGE_ITEM_MAX_NUM || i_attr_pos >= ITEM_ATTRIBUTE_SLOT_MAX_NUM {
            return;
        }
        *pby_type = self.m_victim.item_attr[pos][i_attr_pos].b_type;
        *ps_value = self.m_victim.item_attr[pos][i_attr_pos].s_value;
    }

    pub fn get_item_attribute_from_self(&self, pos: u8, i_attr_pos: u8, pby_type: &mut u8, ps_value: &mut i16) {
        let pos = pos as usize;
        let i_attr_pos = i_attr_pos as usize;
        if pos >= EXCHANGE_ITEM_MAX_NUM || i_attr_pos >= ITEM_ATTRIBUTE_SLOT_MAX_NUM {
            return;
        }
        *pby_type = self.m_self.item_attr[pos][i_attr_pos].b_type;
        *ps_value = self.m_self.item_attr[pos][i_attr_pos].s_value;
    }

    pub fn set_accept_to_target(&mut self, accept: u8) {
        self.m_victim.accept = if accept != 0 { 1 } else { 0 };
    }

    pub fn set_accept_to_self(&mut self, accept: u8) {
        self.m_self.accept = if accept != 0 { 1 } else { 0 };
    }

    pub fn get_accept_from_target(&self) -> bool {
        self.m_victim.accept != 0
    }

    pub fn get_accept_from_self(&self) -> bool {
        self.m_self.accept != 0
    }
}
