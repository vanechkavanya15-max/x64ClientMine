pub const EXCHANGE_ITEM_MAX_NUM: usize = 12;
pub const ITEM_SOCKET_SLOT_MAX_NUM: usize = 3;
pub const ITEM_ATTRIBUTE_SLOT_MAX_NUM: usize = 7;
pub const CHARACTER_NAME_MAX_LEN: usize = 64;

#[derive(Clone, Default, Debug, PartialEq)]
pub struct PlayerItemAttribute {
    pub b_type: u8,
    pub s_value: i16,
}

#[derive(Clone)]
pub struct ExchangeData {
    pub name: String,
    pub item_vnum: [u32; EXCHANGE_ITEM_MAX_NUM],
    pub item_count: [u8; EXCHANGE_ITEM_MAX_NUM],
    pub item_metin: [[u32; ITEM_SOCKET_SLOT_MAX_NUM]; EXCHANGE_ITEM_MAX_NUM],
    pub item_attr: [[PlayerItemAttribute; ITEM_ATTRIBUTE_SLOT_MAX_NUM]; EXCHANGE_ITEM_MAX_NUM],
    pub accept: bool,
    pub elk: i64,
}

impl Default for ExchangeData {
    fn default() -> Self {
        ExchangeData {
            name: String::new(),
            item_vnum: [0; EXCHANGE_ITEM_MAX_NUM],
            item_count: [0; EXCHANGE_ITEM_MAX_NUM],
            item_metin: [[0; ITEM_SOCKET_SLOT_MAX_NUM]; EXCHANGE_ITEM_MAX_NUM],
            item_attr: core::array::from_fn(|_| core::array::from_fn(|_| PlayerItemAttribute::default())),
            accept: false,
            elk: 0,
        }
    }
}

pub struct ExchangeSystem {
    is_trading: bool,
    elk_mode: bool,
    self_data: ExchangeData,
    victim_data: ExchangeData,
    // Store backup to restore on cancel. In real server, items would be returned to inventory.
    // Here we manage the internal exchange state as requested "anulowanie przywraca przedmioty".
    // For simplicity, we just keep the previous state or just handle clear, 
    // but the task says "przywraca przedmioty" so we should be able to track what was placed.
    // Actually, in the client, canceling just clears the window and the server puts items back in inventory.
    // But to satisfy "Zaimplementuj pelna, rzeczywista logike biznesowa ... Anulowanie handlu przyciskiem Cancel lub zamknieciem okna przywraca przedmioty" 
    // we need a mock inventory or just returning items. Since we only have ExchangeSystem, we'll simulate it by returning a structure of items on cancel, or we just clear and consider it canceled.
    // Let's implement actual finalize and cancel logic that returns the state.
}

#[derive(Debug, PartialEq)]
pub enum ExchangeResult {
    Pending,
    Success {
        self_received_items: [u32; EXCHANGE_ITEM_MAX_NUM],
        self_received_counts: [u8; EXCHANGE_ITEM_MAX_NUM],
        self_received_elk: i64,
        target_received_items: [u32; EXCHANGE_ITEM_MAX_NUM],
        target_received_counts: [u8; EXCHANGE_ITEM_MAX_NUM],
        target_received_elk: i64,
    },
    Canceled {
        self_returned_items: [u32; EXCHANGE_ITEM_MAX_NUM],
        self_returned_counts: [u8; EXCHANGE_ITEM_MAX_NUM],
        target_returned_items: [u32; EXCHANGE_ITEM_MAX_NUM],
        target_returned_counts: [u8; EXCHANGE_ITEM_MAX_NUM],
    },
    Error(&'static str),
}

impl ExchangeSystem {
    pub fn new() -> Self {
        let mut sys = ExchangeSystem {
            is_trading: false,
            elk_mode: false,
            self_data: ExchangeData::default(),
            victim_data: ExchangeData::default(),
        };
        sys.clear();
        sys
    }

    pub fn clear(&mut self) {
        self.self_data = ExchangeData::default();
        self.victim_data = ExchangeData::default();
        self.is_trading = false;
    }

    pub fn start(&mut self) {
        self.clear();
        self.is_trading = true;
    }

    pub fn cancel(&mut self) -> ExchangeResult {
        if !self.is_trading {
            return ExchangeResult::Error("Not trading");
        }
        
        let result = ExchangeResult::Canceled {
            self_returned_items: self.self_data.item_vnum,
            self_returned_counts: self.self_data.item_count,
            target_returned_items: self.victim_data.item_vnum,
            target_returned_counts: self.victim_data.item_count,
        };
        
        self.clear();
        result
    }

    pub fn finalize(&mut self) -> ExchangeResult {
        if !self.is_trading {
            return ExchangeResult::Error("Not trading");
        }
        if self.self_data.accept && self.victim_data.accept {
            let result = ExchangeResult::Success {
                self_received_items: self.victim_data.item_vnum,
                self_received_counts: self.victim_data.item_count,
                self_received_elk: self.victim_data.elk,
                target_received_items: self.self_data.item_vnum,
                target_received_counts: self.self_data.item_count,
                target_received_elk: self.self_data.elk,
            };
            self.clear();
            result
        } else {
            ExchangeResult::Pending
        }
    }

    pub fn is_trading(&self) -> bool {
        self.is_trading
    }

    pub fn get_elk_mode(&self) -> bool {
        self.elk_mode
    }

    pub fn set_elk_mode(&mut self, value: bool) {
        self.elk_mode = value;
    }

    pub fn set_self_name(&mut self, name: &str) {
        self.self_data.name = name.chars().take(CHARACTER_NAME_MAX_LEN).collect();
    }

    pub fn set_target_name(&mut self, name: &str) {
        self.victim_data.name = name.chars().take(CHARACTER_NAME_MAX_LEN).collect();
    }

    pub fn get_name_from_self(&self) -> &str {
        &self.self_data.name
    }

    pub fn get_name_from_target(&self) -> &str {
        &self.victim_data.name
    }

    pub fn set_elk_to_target(&mut self, elk: i64) -> bool {
        if !self.is_trading || self.victim_data.accept {
            return false;
        }
        self.victim_data.elk = elk;
        true
    }

    pub fn set_elk_to_self(&mut self, elk: i64) -> bool {
        if !self.is_trading || self.self_data.accept {
            return false;
        }
        self.self_data.elk = elk;
        true
    }

    pub fn get_elk_from_target(&self) -> i64 {
        self.victim_data.elk
    }

    pub fn get_elk_from_self(&self) -> i64 {
        self.self_data.elk
    }

    pub fn set_item_to_target(&mut self, pos: usize, vnum: u32, count: u8) -> bool {
        if !self.is_trading || self.victim_data.accept {
            return false;
        }
        if pos < EXCHANGE_ITEM_MAX_NUM {
            self.victim_data.item_vnum[pos] = vnum;
            self.victim_data.item_count[pos] = count;
            true
        } else {
            false
        }
    }

    pub fn set_item_to_self(&mut self, pos: usize, vnum: u32, count: u8) -> bool {
        if !self.is_trading || self.self_data.accept {
            return false;
        }
        if pos < EXCHANGE_ITEM_MAX_NUM {
            self.self_data.item_vnum[pos] = vnum;
            self.self_data.item_count[pos] = count;
            true
        } else {
            false
        }
    }

    pub fn set_item_metin_socket_to_target(&mut self, pos: usize, imetinpos: usize, vnum: u32) -> bool {
        if !self.is_trading || self.victim_data.accept {
            return false;
        }
        if pos < EXCHANGE_ITEM_MAX_NUM && imetinpos < ITEM_SOCKET_SLOT_MAX_NUM {
            self.victim_data.item_metin[pos][imetinpos] = vnum;
            true
        } else {
            false
        }
    }

    pub fn set_item_metin_socket_to_self(&mut self, pos: usize, imetinpos: usize, vnum: u32) -> bool {
        if !self.is_trading || self.self_data.accept {
            return false;
        }
        if pos < EXCHANGE_ITEM_MAX_NUM && imetinpos < ITEM_SOCKET_SLOT_MAX_NUM {
            self.self_data.item_metin[pos][imetinpos] = vnum;
            true
        } else {
            false
        }
    }

    pub fn set_item_attribute_to_target(&mut self, pos: usize, iattrpos: usize, by_type: u8, s_value: i16) -> bool {
        if !self.is_trading || self.victim_data.accept {
            return false;
        }
        if pos < EXCHANGE_ITEM_MAX_NUM && iattrpos < ITEM_ATTRIBUTE_SLOT_MAX_NUM {
            self.victim_data.item_attr[pos][iattrpos].b_type = by_type;
            self.victim_data.item_attr[pos][iattrpos].s_value = s_value;
            true
        } else {
            false
        }
    }

    pub fn set_item_attribute_to_self(&mut self, pos: usize, iattrpos: usize, by_type: u8, s_value: i16) -> bool {
        if !self.is_trading || self.self_data.accept {
            return false;
        }
        if pos < EXCHANGE_ITEM_MAX_NUM && iattrpos < ITEM_ATTRIBUTE_SLOT_MAX_NUM {
            self.self_data.item_attr[pos][iattrpos].b_type = by_type;
            self.self_data.item_attr[pos][iattrpos].s_value = s_value;
            true
        } else {
            false
        }
    }

    pub fn del_item_of_target(&mut self, pos: usize) -> bool {
        if !self.is_trading || self.victim_data.accept {
            return false;
        }
        if pos < EXCHANGE_ITEM_MAX_NUM {
            self.victim_data.item_vnum[pos] = 0;
            self.victim_data.item_count[pos] = 0;
            true
        } else {
            false
        }
    }

    pub fn del_item_of_self(&mut self, pos: usize) -> bool {
        if !self.is_trading || self.self_data.accept {
            return false;
        }
        if pos < EXCHANGE_ITEM_MAX_NUM {
            self.self_data.item_vnum[pos] = 0;
            self.self_data.item_count[pos] = 0;
            true
        } else {
            false
        }
    }

    pub fn get_item_vnum_from_target(&self, pos: usize) -> u32 {
        if pos < EXCHANGE_ITEM_MAX_NUM {
            self.victim_data.item_vnum[pos]
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
            self.victim_data.item_count[pos]
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

    pub fn get_item_metin_socket_from_target(&self, pos: usize, imetin_socket_pos: usize) -> u32 {
        if pos < EXCHANGE_ITEM_MAX_NUM && imetin_socket_pos < ITEM_SOCKET_SLOT_MAX_NUM {
            self.victim_data.item_metin[pos][imetin_socket_pos]
        } else {
            0
        }
    }

    pub fn get_item_metin_socket_from_self(&self, pos: usize, imetin_socket_pos: usize) -> u32 {
        if pos < EXCHANGE_ITEM_MAX_NUM && imetin_socket_pos < ITEM_SOCKET_SLOT_MAX_NUM {
            self.self_data.item_metin[pos][imetin_socket_pos]
        } else {
            0
        }
    }

    pub fn get_item_attribute_from_target(&self, pos: usize, i_attr_pos: usize) -> (u8, i16) {
        if pos < EXCHANGE_ITEM_MAX_NUM && i_attr_pos < ITEM_ATTRIBUTE_SLOT_MAX_NUM {
            (self.victim_data.item_attr[pos][i_attr_pos].b_type, self.victim_data.item_attr[pos][i_attr_pos].s_value)
        } else {
            (0, 0)
        }
    }

    pub fn get_item_attribute_from_self(&self, pos: usize, i_attr_pos: usize) -> (u8, i16) {
        if pos < EXCHANGE_ITEM_MAX_NUM && i_attr_pos < ITEM_ATTRIBUTE_SLOT_MAX_NUM {
            (self.self_data.item_attr[pos][i_attr_pos].b_type, self.self_data.item_attr[pos][i_attr_pos].s_value)
        } else {
            (0, 0)
        }
    }

    pub fn set_accept_to_target(&mut self, accept: bool) {
        if self.is_trading {
            self.victim_data.accept = accept;
        }
    }

    pub fn set_accept_to_self(&mut self, accept: bool) {
        if self.is_trading {
            self.self_data.accept = accept;
        }
    }

    pub fn get_accept_from_target(&self) -> bool {
        self.victim_data.accept
    }

    pub fn get_accept_from_self(&self) -> bool {
        self.self_data.accept
    }
}
