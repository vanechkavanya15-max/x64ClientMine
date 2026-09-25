//! Modul `phase_select` — Obsluga fazy wyboru, tworzenia i usuwania postaci SelectPhase
//! Wzorzec referencyjny C++: `source/UserInterface/PythonNetworkStreamPhaseSelect.cpp`
//! Opis: Odwzorowanie SelectPhase: przechowywanie 4 slotow TSimplePlayerInformation (nazwa, rasa, poziom, statystyki STR/HT/DX/IQ, zbroja, bron), wysylanie HEADER_CG_CHARACTER_SELECT/CREATE/DELETE.

pub const CHARACTER_NAME_MAX_LEN: usize = 24;
pub const PLAYER_PER_ACCOUNT4: usize = 4;
pub const GUILD_NAME_MAX_LEN: usize = 12;
pub const PRIVATE_CODE_LENGTH: usize = 8;

pub const HEADER_CG_PLAYER_CREATE: u8 = 4;
pub const HEADER_CG_PLAYER_DESTROY: u8 = 5;
pub const HEADER_CG_PLAYER_SELECT: u8 = 6;
pub const HEADER_CG_EMPIRE: u8 = 90;
pub const HEADER_CG_CHANGE_NAME: u8 = 106;

pub const HEADER_GC_PLAYER_CREATE_SUCCESS: u8 = 8;
pub const HEADER_GC_PLAYER_CREATE_FAILURE: u8 = 9;
pub const HEADER_GC_PLAYER_DELETE_SUCCESS: u8 = 10;
pub const HEADER_GC_PLAYER_DELETE_WRONG_SOCIAL_ID: u8 = 11;
pub const HEADER_GC_EMPIRE: u8 = 90;
pub const HEADER_GC_CHANGE_NAME: u8 = 107;

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TSimplePlayerInformation {
    pub dw_id: u32,
    pub sz_name: [u8; CHARACTER_NAME_MAX_LEN + 1],
    pub by_job: u8,
    pub by_level: u8,
    pub dw_play_minutes: u32,
    pub by_st: u8,
    pub by_ht: u8,
    pub by_dx: u8,
    pub by_iq: u8,
    pub w_main_part: u16,
    pub b_change_name: u8,
    pub w_hair_part: u16,
    pub b_dummy: [u8; 4],
    pub x: i32,
    pub y: i32,
    pub l_addr: i32,
    pub w_port: u16,
    pub by_skill_group: u8,
}

impl Default for TSimplePlayerInformation {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

#[derive(Debug, Clone)]
pub struct PhaseSelect {
    pub initialized: bool,
    pub simple_player_info: [TSimplePlayerInformation; PLAYER_PER_ACCOUNT4],
    pub guild_id: [u32; PLAYER_PER_ACCOUNT4],
    pub guild_name: [String; PLAYER_PER_ACCOUNT4],
    pub empire_id: u32,
    pub sent_packets: Vec<Vec<u8>>,
    pub on_create_success_called: bool,
    pub on_create_failure_called: Option<u8>,
    pub on_delete_success_called: Option<u8>,
    pub on_delete_failure_called: bool,
    pub on_change_name_called: Option<(u8, String)>,
}

impl Default for PhaseSelect {
    fn default() -> Self {
        Self {
            initialized: true,
            simple_player_info: [TSimplePlayerInformation::default(); PLAYER_PER_ACCOUNT4],
            guild_id: [0; PLAYER_PER_ACCOUNT4],
            guild_name: [String::new(), String::new(), String::new(), String::new()],
            empire_id: 0,
            sent_packets: Vec::new(),
            on_create_success_called: false,
            on_create_failure_called: None,
            on_delete_success_called: None,
            on_delete_failure_called: false,
            on_change_name_called: None,
        }
    }
}

impl PhaseSelect {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }

    pub fn set_empire_id(&mut self, empire_id: u32) {
        self.empire_id = empire_id;
    }

    pub fn get_empire_id(&self) -> u32 {
        self.empire_id
    }

    pub fn send_select_empire_packet(&mut self, dw_empire_id: u32) -> bool {
        let mut packet = vec![HEADER_CG_EMPIRE];
        packet.push(dw_empire_id as u8);
        self.sent_packets.push(packet);
        self.set_empire_id(dw_empire_id);
        true
    }

    pub fn send_select_character_packet(&mut self, index: u8) -> bool {
        let mut packet = vec![HEADER_CG_PLAYER_SELECT];
        packet.push(index);
        self.sent_packets.push(packet);
        true
    }

    pub fn send_destroy_character_packet(&mut self, index: u8, sz_private_code: &str) -> bool {
        let mut packet = vec![HEADER_CG_PLAYER_DESTROY];
        packet.push(index);
        
        let mut code_bytes = [0u8; PRIVATE_CODE_LENGTH];
        let bytes = sz_private_code.as_bytes();
        let len = bytes.len().min(PRIVATE_CODE_LENGTH - 1);
        code_bytes[..len].copy_from_slice(&bytes[..len]);
        packet.extend_from_slice(&code_bytes);

        self.sent_packets.push(packet);
        true
    }

    pub fn send_create_character_packet(
        &mut self,
        index: u8,
        name: &str,
        job: u16,
        shape: u8,
        by_con: u8,
        by_int: u8,
        by_str: u8,
        by_dex: u8,
    ) -> bool {
        let mut packet = vec![HEADER_CG_PLAYER_CREATE];
        packet.push(index);

        let mut name_bytes = [0u8; CHARACTER_NAME_MAX_LEN + 1];
        let bytes = name.as_bytes();
        let len = bytes.len().min(CHARACTER_NAME_MAX_LEN);
        name_bytes[..len].copy_from_slice(&bytes[..len]);
        packet.extend_from_slice(&name_bytes);

        packet.extend_from_slice(&job.to_le_bytes());
        packet.push(shape);
        packet.push(by_con);
        packet.push(by_int);
        packet.push(by_str);
        packet.push(by_dex);

        self.sent_packets.push(packet);
        true
    }

    pub fn send_change_name_packet(&mut self, index: u8, name: &str) -> bool {
        let mut packet = vec![HEADER_CG_CHANGE_NAME];
        packet.push(index);

        let mut name_bytes = [0u8; CHARACTER_NAME_MAX_LEN + 1];
        let bytes = name.as_bytes();
        let len = bytes.len().min(CHARACTER_NAME_MAX_LEN);
        name_bytes[..len].copy_from_slice(&bytes[..len]);
        packet.extend_from_slice(&name_bytes);

        self.sent_packets.push(packet);
        true
    }

    pub fn recv_player_create_success_packet(&mut self, b_account_character_slot: u8, k_simple_player_information: TSimplePlayerInformation) -> bool {
        if b_account_character_slot as usize >= PLAYER_PER_ACCOUNT4 {
            // Out of range slot error in C++, but packet processed correctly.
            return true;
        }

        self.simple_player_info[b_account_character_slot as usize] = k_simple_player_information;
        self.on_create_success_called = true;
        true
    }

    pub fn recv_player_create_failure_packet(&mut self, b_type: u8) -> bool {
        self.on_create_failure_called = Some(b_type);
        true
    }

    pub fn recv_player_destroy_success_packet(&mut self, account_index: u8) -> bool {
        if account_index as usize >= PLAYER_PER_ACCOUNT4 {
             return false;
        }
        self.simple_player_info[account_index as usize] = TSimplePlayerInformation::default();
        self.guild_id[account_index as usize] = 0;
        self.guild_name[account_index as usize] = String::new();
        self.on_delete_success_called = Some(account_index);
        true
    }

    pub fn recv_player_destroy_failure_packet(&mut self) -> bool {
        self.on_delete_failure_called = true;
        true
    }

    pub fn recv_change_name(&mut self, pid: u32, name: &str) -> bool {
        for i in 0..PLAYER_PER_ACCOUNT4 {
            // Note: need to read unaligned using addr_of! because of packed struct
            let dw_id = unsafe { std::ptr::addr_of!(self.simple_player_info[i].dw_id).read_unaligned() };
            if pid == dw_id {
                self.simple_player_info[i].b_change_name = 0;
                
                let mut name_bytes = [0u8; CHARACTER_NAME_MAX_LEN + 1];
                let bytes = name.as_bytes();
                let len = bytes.len().min(CHARACTER_NAME_MAX_LEN);
                name_bytes[..len].copy_from_slice(&bytes[..len]);
                self.simple_player_info[i].sz_name = name_bytes;

                self.on_change_name_called = Some((i as u8, name.to_string()));
                return true;
            }
        }
        
        self.on_create_failure_called = Some(100);
        true
    }
}
