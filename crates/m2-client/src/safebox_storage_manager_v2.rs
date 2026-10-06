#![allow(non_snake_case)]
#![allow(dead_code)]

pub const SAFEBOX_SLOT_X_COUNT: usize = 5;
pub const SAFEBOX_SLOT_Y_COUNT: usize = 9;
pub const SAFEBOX_PAGE_SIZE: usize = SAFEBOX_SLOT_X_COUNT * SAFEBOX_SLOT_Y_COUNT;

pub const ITEM_SOCKET_SLOT_MAX_NUM: usize = 3;
pub const ITEM_ATTRIBUTE_SLOT_MAX_NUM: usize = 7;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub struct TPlayerItemAttribute {
    pub bType: u8,
    pub sValue: i16,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TItemData {
    pub vnum: u32,
    pub count: u8,
    pub flags: u32,
    pub anti_flags: u32,
    pub alSockets: [i32; ITEM_SOCKET_SLOT_MAX_NUM],
    pub aAttr: [TPlayerItemAttribute; ITEM_ATTRIBUTE_SLOT_MAX_NUM],
}

impl Default for TItemData {
    fn default() -> Self {
        Self {
            vnum: 0,
            count: 0,
            flags: 0,
            anti_flags: 0,
            alSockets: [0; ITEM_SOCKET_SLOT_MAX_NUM],
            aAttr: [TPlayerItemAttribute::default(); ITEM_ATTRIBUTE_SLOT_MAX_NUM],
        }
    }
}

pub struct CPythonSafeBox {
    m_ItemInstanceVector: Vec<TItemData>,
    m_MallItemInstanceVector: Vec<TItemData>,
    m_dwMoney: u32,
    expected_password_hash: String,
    is_open: bool,
}

impl Default for CPythonSafeBox {
    fn default() -> Self {
        Self::new()
    }
}

impl CPythonSafeBox {
    pub fn new() -> Self {
        Self {
            m_ItemInstanceVector: Vec::new(),
            m_MallItemInstanceVector: Vec::new(),
            m_dwMoney: 0,
            expected_password_hash: String::from("000000"), // Default dummy password
            is_open: false,
        }
    }

    pub fn set_expected_password(&mut self, hash: &str) {
        self.expected_password_hash = hash.to_string();
    }

    pub fn verify_password(&mut self, password_hash: &str) -> bool {
        if password_hash == self.expected_password_hash {
            self.is_open = true;
            true
        } else {
            false
        }
    }

    pub fn deposit_yang(&mut self, amount: u32) -> bool {
        if amount == 0 {
            return false;
        }
        if let Some(new_money) = self.m_dwMoney.checked_add(amount) {
            self.m_dwMoney = new_money;
            true
        } else {
            false
        }
    }

    pub fn withdraw_yang(&mut self, amount: u32) -> bool {
        if amount == 0 {
            return false;
        }
        if self.m_dwMoney >= amount {
            self.m_dwMoney -= amount;
            true
        } else {
            false
        }
    }

    pub fn OpenSafeBox(&mut self, iSize: usize) {
        self.m_dwMoney = 0;
        self.m_ItemInstanceVector.clear();
        let actual_size = SAFEBOX_SLOT_X_COUNT * SAFEBOX_SLOT_Y_COUNT * iSize;
        self.m_ItemInstanceVector.resize(actual_size, TItemData::default());
    }

    pub fn SetItemData(&mut self, dwSlotIndex: usize, rItemInstance: TItemData) {
        if dwSlotIndex >= self.m_ItemInstanceVector.len() {
            return;
        }
        self.m_ItemInstanceVector[dwSlotIndex] = rItemInstance;
    }

    pub fn DelItemData(&mut self, dwSlotIndex: usize) {
        if dwSlotIndex >= self.m_ItemInstanceVector.len() {
            return;
        }
        self.m_ItemInstanceVector[dwSlotIndex] = TItemData::default();
    }

    pub fn SetMoney(&mut self, dwMoney: u32) {
        self.m_dwMoney = dwMoney;
    }

    pub fn GetMoney(&self) -> u32 {
        self.m_dwMoney
    }

    pub fn GetCurrentSafeBoxSize(&self) -> usize {
        self.m_ItemInstanceVector.len()
    }

    pub fn GetSlotItemID(&self, dwSlotIndex: usize, pdwItemID: &mut u32) -> bool {
        if dwSlotIndex >= self.m_ItemInstanceVector.len() {
            return false;
        }
        *pdwItemID = self.m_ItemInstanceVector[dwSlotIndex].vnum;
        true
    }

    pub fn GetItemDataPtr(&self, dwSlotIndex: usize) -> Option<&TItemData> {
        self.m_ItemInstanceVector.get(dwSlotIndex)
    }

    // MALL
    pub fn OpenMall(&mut self, iSize: usize) {
        self.m_MallItemInstanceVector.clear();
        let actual_size = SAFEBOX_SLOT_X_COUNT * SAFEBOX_SLOT_Y_COUNT * iSize;
        self.m_MallItemInstanceVector.resize(actual_size, TItemData::default());
    }

    pub fn SetMallItemData(&mut self, dwSlotIndex: usize, rItemData: TItemData) {
        if dwSlotIndex >= self.m_MallItemInstanceVector.len() {
            return;
        }
        self.m_MallItemInstanceVector[dwSlotIndex] = rItemData;
    }

    pub fn DelMallItemData(&mut self, dwSlotIndex: usize) {
        if dwSlotIndex >= self.m_MallItemInstanceVector.len() {
            return;
        }
        self.m_MallItemInstanceVector[dwSlotIndex] = TItemData::default();
    }

    pub fn GetMallItemDataPtr(&self, dwSlotIndex: usize) -> Option<&TItemData> {
        self.m_MallItemInstanceVector.get(dwSlotIndex)
    }

    pub fn GetSlotMallItemID(&self, dwSlotIndex: usize, pdwItemID: &mut u32) -> bool {
        if dwSlotIndex >= self.m_MallItemInstanceVector.len() {
            return false;
        }
        *pdwItemID = self.m_MallItemInstanceVector[dwSlotIndex].vnum;
        true
    }

    pub fn GetMallSize(&self) -> usize {
        self.m_MallItemInstanceVector.len()
    }
}
