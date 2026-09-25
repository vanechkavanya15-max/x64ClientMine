 //! Wzorzec referencyjny C++: `source/UserInterface/PythonNetworkStreamPhaseGame.cpp`
 //! Opis: Odwzorowanie GamePhase: obsluga HEADER_GC_POINTS, HEADER_GC_POINT_CHANGE (HP, SP, EXP, Gold, Level), HEADER_GC_PING/PONG, HEADER_GC_STUN, HEADER_GC_DEAD.
 
pub const POINT_MAX_NUM: usize = 255;

pub const POINT_LEVEL: u8 = 1;
pub const POINT_GOLD: u8 = 11;
pub const POINT_ST: u8 = 12;
pub const POINT_HT: u8 = 13;
pub const POINT_DX: u8 = 14;
pub const POINT_IQ: u8 = 15;
pub const POINT_SUB_SKILL: u8 = 27;
pub const POINT_SKILL: u8 = 28;
pub const POINT_STAT_RESET_COUNT: u8 = 112;
pub const POINT_HORSE_SKILL: u8 = 113;
pub const POINT_ENERGY: u8 = 128;
pub const POINT_ENERGY_END_TIME: u8 = 129;

#[repr(C, packed)]
#[derive(Debug, Clone, PartialEq)]
pub struct TPacketGCPoints {
    pub header: u8,
    pub points: [i32; POINT_MAX_NUM],
}

#[repr(C, packed)]
#[derive(Debug, Clone, PartialEq)]
#[allow(non_snake_case)]
pub struct TPacketGCPointChange {
    pub header: i32,
    pub dwVID: u32,
    pub type_: u8,
    pub amount: i32,
    pub value: i32,
}

#[repr(C, packed)]
#[derive(Debug, Clone, PartialEq)]
pub struct TPacketGCPing {
    pub header: u8,
}

#[repr(C, packed)]
#[derive(Debug, Clone, PartialEq)]
pub struct TPacketCGPong {
    pub header: u8,
}

#[repr(C, packed)]
#[derive(Debug, Clone, PartialEq)]
pub struct TPacketGCStun {
    pub header: u8,
    pub vid: u32,
}

#[repr(C, packed)]
#[derive(Debug, Clone, PartialEq)]
pub struct TPacketGCDead {
    pub header: u8,
    pub vid: u32,
}

#[derive(Debug, Clone)]
 pub struct PhaseGameMain {
     pub initialized: bool,
    pub main_character_vid: u32,
    pub player_points: [i32; POINT_MAX_NUM],
    pub last_ping_time: u32,
    pub is_game_over: bool,
    pub is_stunned: bool,
    pub gold_picked_up: i32,
    pub skill_window_refreshed: bool,
    pub status_refreshed: bool,
}

impl Default for PhaseGameMain {
    fn default() -> Self {
        Self::new()
    }
 }
 
 impl PhaseGameMain {
     pub fn new() -> Self {
        Self {
            initialized: true,
            main_character_vid: 0,
            player_points: [0; POINT_MAX_NUM],
            last_ping_time: 0,
            is_game_over: false,
            is_stunned: false,
            gold_picked_up: 0,
            skill_window_refreshed: false,
            status_refreshed: false,
        }
     }
 
     pub fn is_ready(&self) -> bool {
         self.initialized
     }

    pub fn recv_player_points(&mut self, packet: &TPacketGCPoints) -> bool {
        for i in 0..POINT_MAX_NUM {
            self.player_points[i] = packet.points[i];
        }
        self.status_refreshed = true;
        true
    }

    pub fn recv_point_change(&mut self, packet: &TPacketGCPointChange) -> bool {
        // Logika z C++ dla swojej postaci (glownej)
        if packet.dwVID == self.main_character_vid {
            let p_type = packet.type_ as usize;
            if p_type < POINT_MAX_NUM {
                self.player_points[p_type] = packet.value;
            }

            match packet.type_ {
                POINT_STAT_RESET_COUNT => {
                    self.status_refreshed = true;
                }
                POINT_LEVEL | POINT_ST | POINT_DX | POINT_HT | POINT_IQ => {
                    self.status_refreshed = true;
                    self.skill_window_refreshed = true;
                }
                POINT_SKILL | POINT_SUB_SKILL | POINT_HORSE_SKILL => {
                    self.skill_window_refreshed = true;
                }
                POINT_ENERGY => {
                    if packet.value == 0 {
                        self.player_points[POINT_ENERGY_END_TIME as usize] = 0;
                    }
                    self.status_refreshed = true;
                }
                _ => {
                    self.status_refreshed = true;
                }
            }

            if packet.type_ == POINT_GOLD && packet.amount > 0 {
                self.gold_picked_up += packet.amount;
            }
        } else {
            // Dla innych postaci tylko poziom jest aktualizowany (w mock'u mozemy to zignorowac dla player_points)
            // C++ log:
            // if (PointChange.Type == POINT_LEVEL) { pInstance->SetLevel(...) }
        }

        true
    }

    pub fn recv_ping_packet(&mut self, _packet: &TPacketGCPing) -> Option<TPacketCGPong> {
        self.last_ping_time = 12345; // Mock for ELTimer_GetMSec()
        Some(TPacketCGPong { header: 0xfe })
    }

    pub fn recv_stun_packet(&mut self, packet: &TPacketGCStun) -> bool {
        if packet.vid == self.main_character_vid {
            self.is_game_over = true; // w oryginalnym C++ wywoluje pkInstSel->Die(); dla glownej, ale my ustawimy flage testowa. Wlasciwie to nie game_over bezposrednio,
            // ale ponizej jest: pkInstSel->Die(); if main_inst, a dla innych Stun(); 
            // My uzyjemy `is_game_over` dla Die glownego bohatera, ale w C++ `StunPacket` na `main_instance` wywoluje `Die()`, a na innych `Stun()`.
            // Zatem:
            self.is_game_over = true; // Simulate Die() for main instance.
        } else {
            self.is_stunned = true; // Simulate Stun() for other instances.
        }
        true
    }

    pub fn recv_dead_packet(&mut self, packet: &TPacketGCDead) -> bool {
        if packet.vid == self.main_character_vid {
            self.is_game_over = true;
        }
        true
    }
}

