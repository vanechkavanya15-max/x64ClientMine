pub const EXCHANGE_ITEM_MAX_NUM: usize = 12;
pub const ITEM_SOCKET_SLOT_MAX_NUM: usize = 3;
pub const ITEM_ATTRIBUTE_SLOT_MAX_NUM: usize = 7;
pub const CHARACTER_NAME_MAX_LEN: usize = 64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlayerItemAttribute {
    pub b_type: u8,
    pub s_value: i16,
}

impl Default for PlayerItemAttribute {
    fn default() -> Self {
        Self { b_type: 0, s_value: 0 }
    }
}

#[derive(Debug, Clone)]
pub struct ExchangeData {
    pub name: String,
    pub item_vnum: [u32; EXCHANGE_ITEM_MAX_NUM],
    pub item_count: [u8; EXCHANGE_ITEM_MAX_NUM],
    pub item_metin: [[u32; ITEM_SOCKET_SLOT_MAX_NUM]; EXCHANGE_ITEM_MAX_NUM],
    pub item_attr: [[PlayerItemAttribute; ITEM_ATTRIBUTE_SLOT_MAX_NUM]; EXCHANGE_ITEM_MAX_NUM],
    pub accept: bool,
    pub elk: u64,
}

impl Default for ExchangeData {
    fn default() -> Self {
        Self {
            name: String::new(),
            item_vnum: [0; EXCHANGE_ITEM_MAX_NUM],
            item_count: [0; EXCHANGE_ITEM_MAX_NUM],
            item_metin: [[0; ITEM_SOCKET_SLOT_MAX_NUM]; EXCHANGE_ITEM_MAX_NUM],
            item_attr: [[PlayerItemAttribute::default(); ITEM_ATTRIBUTE_SLOT_MAX_NUM];
                EXCHANGE_ITEM_MAX_NUM],
            accept: false,
            elk: 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TradeState {
    Closed,
    Open,
    ItemsAdded,
    Accepted,
    Finalized,
}

#[derive(Debug, Clone)]
pub struct ExchangeStateMachine {
    pub self_data: ExchangeData,
    pub target_data: ExchangeData,
    pub current_state: TradeState,
    pub is_trading: bool,
    pub elk_mode: bool,
}

impl Default for ExchangeStateMachine {
    fn default() -> Self {
        Self::new()
    }
}

impl ExchangeStateMachine {
    pub fn new() -> Self {
        Self {
            self_data: ExchangeData::default(),
            target_data: ExchangeData::default(),
            current_state: TradeState::Closed,
            is_trading: false,
            elk_mode: false,
        }
    }

    pub fn clear(&mut self) {
        self.self_data = ExchangeData::default();
        self.target_data = ExchangeData::default();
        // CPythonExchange::Clear does not reset m_isTrading and m_elk_mode
        self.current_state = TradeState::Closed; // Reset state on clear
    }

    pub fn start(&mut self) {
        self.is_trading = true;
        self.current_state = TradeState::Open;
    }

    pub fn end(&mut self) {
        self.is_trading = false;
        self.current_state = TradeState::Closed;
    }

    pub fn is_trading(&self) -> bool {
        self.is_trading
    }

    pub fn set_self_name(&mut self, name: &str) {
        self.self_data.name = name.chars().take(CHARACTER_NAME_MAX_LEN).collect();
    }

    pub fn set_target_name(&mut self, name: &str) {
        self.target_data.name = name.chars().take(CHARACTER_NAME_MAX_LEN).collect();
    }

    pub fn get_name_from_self(&self) -> &str {
        &self.self_data.name
    }

    pub fn get_name_from_target(&self) -> &str {
        &self.target_data.name
    }

    fn reset_accept_flags_if_needed(&mut self) {
        if self.current_state == TradeState::Accepted || self.self_data.accept || self.target_data.accept {
            self.self_data.accept = false;
            self.target_data.accept = false;
            self.current_state = TradeState::ItemsAdded;
        } else if self.current_state == TradeState::Open {
             self.current_state = TradeState::ItemsAdded;
        }
    }

    pub fn set_elk_to_target(&mut self, elk: u64) {
        self.target_data.elk = elk;
        self.reset_accept_flags_if_needed();
    }

    pub fn set_elk_to_self(&mut self, elk: u64) {
        self.self_data.elk = elk;
        self.reset_accept_flags_if_needed();
    }

    pub fn add_elk_to_target(&mut self, amount: u64) -> bool {
        if let Some(new_elk) = self.target_data.elk.checked_add(amount) {
            self.set_elk_to_target(new_elk);
            true
        } else {
            false
        }
    }

    pub fn add_elk_to_self(&mut self, amount: u64) -> bool {
        if let Some(new_elk) = self.self_data.elk.checked_add(amount) {
            self.set_elk_to_self(new_elk);
            true
        } else {
            false
        }
    }

    pub fn get_elk_from_target(&self) -> u64 {
        self.target_data.elk
    }

    pub fn get_elk_from_self(&self) -> u64 {
        self.self_data.elk
    }

    pub fn set_item_to_target(&mut self, pos: usize, vnum: u32, count: u8) {
        if pos < EXCHANGE_ITEM_MAX_NUM {
            self.target_data.item_vnum[pos] = vnum;
            self.target_data.item_count[pos] = count;
            self.reset_accept_flags_if_needed();
        }
    }

    pub fn set_item_to_self(&mut self, pos: usize, vnum: u32, count: u8) {
        if pos < EXCHANGE_ITEM_MAX_NUM {
            self.self_data.item_vnum[pos] = vnum;
            self.self_data.item_count[pos] = count;
            self.reset_accept_flags_if_needed();
        }
    }

    pub fn set_item_metin_socket_to_target(&mut self, pos: usize, imetinpos: usize, vnum: u32) {
        if pos < EXCHANGE_ITEM_MAX_NUM && imetinpos < ITEM_SOCKET_SLOT_MAX_NUM {
            self.target_data.item_metin[pos][imetinpos] = vnum;
            self.reset_accept_flags_if_needed();
        }
    }

    pub fn set_item_metin_socket_to_self(&mut self, pos: usize, imetinpos: usize, vnum: u32) {
        if pos < EXCHANGE_ITEM_MAX_NUM && imetinpos < ITEM_SOCKET_SLOT_MAX_NUM {
            self.self_data.item_metin[pos][imetinpos] = vnum;
            self.reset_accept_flags_if_needed();
        }
    }

    pub fn set_item_attribute_to_target(
        &mut self,
        pos: usize,
        iattrpos: usize,
        by_type: u8,
        s_value: i16,
    ) {
        if pos < EXCHANGE_ITEM_MAX_NUM && iattrpos < ITEM_ATTRIBUTE_SLOT_MAX_NUM {
            self.target_data.item_attr[pos][iattrpos].b_type = by_type;
            self.target_data.item_attr[pos][iattrpos].s_value = s_value;
            self.reset_accept_flags_if_needed();
        }
    }

    pub fn set_item_attribute_to_self(
        &mut self,
        pos: usize,
        iattrpos: usize,
        by_type: u8,
        s_value: i16,
    ) {
        if pos < EXCHANGE_ITEM_MAX_NUM && iattrpos < ITEM_ATTRIBUTE_SLOT_MAX_NUM {
            self.self_data.item_attr[pos][iattrpos].b_type = by_type;
            self.self_data.item_attr[pos][iattrpos].s_value = s_value;
            self.reset_accept_flags_if_needed();
        }
    }

    pub fn del_item_of_target(&mut self, pos: usize) {
        if pos < EXCHANGE_ITEM_MAX_NUM {
            self.target_data.item_vnum[pos] = 0;
            self.target_data.item_count[pos] = 0;
            self.reset_accept_flags_if_needed();
        }
    }

    pub fn del_item_of_self(&mut self, pos: usize) {
        if pos < EXCHANGE_ITEM_MAX_NUM {
            self.self_data.item_vnum[pos] = 0;
            self.self_data.item_count[pos] = 0;
            self.reset_accept_flags_if_needed();
        }
    }

    pub fn get_item_vnum_from_target(&self, pos: usize) -> u32 {
        if pos < EXCHANGE_ITEM_MAX_NUM {
            self.target_data.item_vnum[pos]
        } else {
            0
        }
    }

    pub fn get_item_vnum_from_self(&self, pos: usize) -> u32 {
        if pos < EXCHANGE_ITEM_MAX_NUM {
            self.self_data.item_vnum[pos]
        } else {
            0
        }
    }

    pub fn get_item_count_from_target(&self, pos: usize) -> u8 {
        if pos < EXCHANGE_ITEM_MAX_NUM {
            self.target_data.item_count[pos]
        } else {
            0
        }
    }

    pub fn get_item_count_from_self(&self, pos: usize) -> u8 {
        if pos < EXCHANGE_ITEM_MAX_NUM {
            self.self_data.item_count[pos]
        } else {
            0
        }
    }

    pub fn get_item_metin_socket_from_target(&self, pos: usize, imetinpos: usize) -> u32 {
        if pos < EXCHANGE_ITEM_MAX_NUM && imetinpos < ITEM_SOCKET_SLOT_MAX_NUM {
            self.target_data.item_metin[pos][imetinpos]
        } else {
            0
        }
    }

    pub fn get_item_metin_socket_from_self(&self, pos: usize, imetinpos: usize) -> u32 {
        if pos < EXCHANGE_ITEM_MAX_NUM && imetinpos < ITEM_SOCKET_SLOT_MAX_NUM {
            self.self_data.item_metin[pos][imetinpos]
        } else {
            0
        }
    }

    pub fn get_item_attribute_from_target(
        &self,
        pos: usize,
        iattrpos: usize,
    ) -> Option<(u8, i16)> {
        if pos < EXCHANGE_ITEM_MAX_NUM && iattrpos < ITEM_ATTRIBUTE_SLOT_MAX_NUM {
            let attr = &self.target_data.item_attr[pos][iattrpos];
            Some((attr.b_type, attr.s_value))
        } else {
            None
        }
    }

    pub fn get_item_attribute_from_self(&self, pos: usize, iattrpos: usize) -> Option<(u8, i16)> {
        if pos < EXCHANGE_ITEM_MAX_NUM && iattrpos < ITEM_ATTRIBUTE_SLOT_MAX_NUM {
            let attr = &self.self_data.item_attr[pos][iattrpos];
            Some((attr.b_type, attr.s_value))
        } else {
            None
        }
    }

    pub fn set_accept_to_target(&mut self, accept: bool) {
        self.target_data.accept = accept;
        self.update_state_on_accept();
    }

    pub fn set_accept_to_self(&mut self, accept: bool) {
        self.self_data.accept = accept;
        self.update_state_on_accept();
    }
    
    fn update_state_on_accept(&mut self) {
        if self.self_data.accept && self.target_data.accept {
            self.current_state = TradeState::Accepted;
        } else if self.current_state == TradeState::Accepted {
            self.current_state = TradeState::ItemsAdded;
        }
    }

    pub fn get_accept_from_target(&self) -> bool {
        self.target_data.accept
    }

    pub fn get_accept_from_self(&self) -> bool {
        self.self_data.accept
    }

    pub fn get_elk_mode(&self) -> bool {
        self.elk_mode
    }

    pub fn set_elk_mode(&mut self, value: bool) {
        self.elk_mode = value;
    }
    
    pub fn finalize(&mut self) {
        if self.current_state == TradeState::Accepted {
            self.current_state = TradeState::Finalized;
            self.is_trading = false;
        }
    }
}
