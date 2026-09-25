 //! Modul `net_stream_core` — Maszyna stanow strumienia sieciowego CPythonNetworkStream
 //! Wzorzec referencyjny C++: `source/UserInterface/PythonNetworkStream.cpp`

use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    Close = 0,
    HandShake = 1,
    Login = 2,
    Select = 3,
    Loading = 4,
    Game = 5,
    Dead = 6,
    OffLine = 255, // Special initial phase from PythonNetworkStream.cpp (m_strPhase = "OffLine")
}

impl Default for Phase {
    fn default() -> Self {
        Phase::OffLine
    }
}

// Stale naglowkow zgodne z Packet.h (PacketHeader mapowane jako u8 na potrzeby symulacji)
pub const HEADER_GC_PHASE: u8 = 0xfd;
pub const HEADER_GC_HANDSHAKE: u8 = 0xff;
pub const HEADER_GC_LOGIN_SUCCESS3: u8 = 6;
pub const HEADER_GC_LOGIN_SUCCESS4: u8 = 32;
pub const HEADER_GC_LOGIN_FAILURE: u8 = 7;
pub const HEADER_GC_EMPIRE: u8 = 90;
pub const HEADER_GC_LOGIN_KEY: u8 = 118;
pub const HEADER_GC_PING: u8 = 44;
pub const HEADER_GC_PLAYER_CREATE_SUCCESS: u8 = 8;
pub const HEADER_GC_PLAYER_CREATE_FAILURE: u8 = 9;
pub const HEADER_GC_PLAYER_DELETE_SUCCESS: u8 = 10;
pub const HEADER_GC_PLAYER_DELETE_WRONG_SOCIAL_ID: u8 = 11;
pub const HEADER_GC_CHANGE_NAME: u8 = 107;
pub const HEADER_GC_MAIN_CHARACTER: u8 = 15;
pub const HEADER_GC_MAIN_CHARACTER2_EMPIRE: u8 = 113;
pub const HEADER_GC_MAIN_CHARACTER3_BGM: u8 = 137;
pub const HEADER_GC_MAIN_CHARACTER4_BGM_VOL: u8 = 138;
pub const HEADER_GC_CHARACTER_UPDATE: u8 = 19;
pub const HEADER_GC_PLAYER_POINTS: u8 = 16;
pub const HEADER_GC_PLAYER_POINT_CHANGE: u8 = 17;
pub const HEADER_GC_ITEM_SET: u8 = 21; // Mapped to 21 based on header
pub const HEADER_GC_QUICKSLOT_ADD: u8 = 28;
pub const HEADER_GC_HYBRIDCRYPT_KEYS: u8 = 152;
pub const HEADER_GC_HYBRIDCRYPT_SDB: u8 = 153;

#[derive(Debug, Clone)]
pub struct Packet {
    pub header: u8,
    pub payload: Vec<u8>,
}
 
 #[derive(Debug, Clone, Default)]
 pub struct NetStreamCore {
     pub initialized: bool,
    pub phase: Phase,
    pub recv_queue: VecDeque<Packet>,
    pub send_queue: VecDeque<Packet>,
 }
 
 impl NetStreamCore {
     pub fn new() -> Self {
        Self { 
            initialized: true,
            phase: Phase::OffLine,
            recv_queue: VecDeque::new(),
            send_queue: VecDeque::new(),
        }
     }
 
     pub fn is_ready(&self) -> bool {
         self.initialized
     }
 
    pub fn set_offline_phase(&mut self) {
        self.phase = Phase::OffLine;
    }

    pub fn set_handshake_phase(&mut self) {
        self.phase = Phase::HandShake;
    }

    pub fn set_login_phase(&mut self) {
        self.phase = Phase::Login;
    }

    pub fn set_select_phase(&mut self) {
        self.phase = Phase::Select;
    }

    pub fn set_loading_phase(&mut self) {
        self.phase = Phase::Loading;
    }

    pub fn set_game_phase(&mut self) {
        self.phase = Phase::Game;
    }
    
    pub fn set_dead_phase(&mut self) {
        self.phase = Phase::Dead;
    }

    pub fn close_phase(&mut self) {
        self.phase = Phase::Close;
        // W C++: PyCallClassMemberFunc(m_poHandler, "SetLoginPhase", Py_BuildValue("()"));
        // Ta metoda tylko wola pythona, wiec faza technicznie zostaje obsluzona w skrypcie. My wylapujemy to jako Phase::Close.
    }

    pub fn queue_recv_packet(&mut self, packet: Packet) {
        self.recv_queue.push_back(packet);
    }

    pub fn pop_recv_packet(&mut self) -> Option<Packet> {
        self.recv_queue.pop_front()
    }

    pub fn queue_send_packet(&mut self, packet: Packet) {
        self.send_queue.push_back(packet);
    }

    pub fn pop_send_packet(&mut self) -> Option<Packet> {
        self.send_queue.pop_front()
    }

    fn recv_phase_packet(&mut self, packet: &Packet) -> bool {
        if packet.payload.is_empty() {
            return false;
        }
        let phase_id = packet.payload[0];
        match phase_id {
            0 => self.close_phase(),
            1 => self.set_handshake_phase(),
            2 => self.set_login_phase(),
            3 => self.set_select_phase(),
            4 => self.set_loading_phase(),
            5 => self.set_game_phase(),
            6 => self.set_dead_phase(),
            _ => return false,
        }
        true
    }

    fn recv_error_packet(&mut self, _header: u8) -> bool {
        // ClearRecvBuffer
        self.recv_queue.clear();
        true
    }

    pub fn process(&mut self) {
        while !self.recv_queue.is_empty() {
            // Recznie kontrolujemy petle by uchwycic czyszczenie kolejki poprawnie
            let packet = self.recv_queue.pop_front().unwrap();
            let queue_len_before_process = self.recv_queue.len();
            
            match self.phase {
                Phase::OffLine => {
                    self.process_offline_phase(&packet);
                }
                Phase::HandShake => {
                    self.process_handshake_phase(&packet);
                }
                Phase::Login => {
                    self.process_login_phase(&packet);
                }
                Phase::Select => {
                    self.process_select_phase(&packet);
                }
                Phase::Loading => {
                    self.process_loading_phase(&packet);
                }
                Phase::Game => {
                    self.process_game_phase(&packet);
                }
                Phase::Close | Phase::Dead => {
                    // Ignorujemy/brak obslugi
                }
            }

            // Sprawdzamy czy kolejka zostala oprozniona przez funkcje obslugujaca pakiet.
            // Jezeli przed obsluga bylo w niej cokolwiek, a po obsludze jest pusto to znaczy ze ktos zawolal clear().
            if self.recv_queue.is_empty() && queue_len_before_process > 0 {
                break;
            }
        }
    }

    fn process_offline_phase(&mut self, packet: &Packet) {
        match packet.header {
            HEADER_GC_PHASE => {
                self.recv_phase_packet(packet);
            },
            HEADER_GC_HANDSHAKE => {
                self.set_handshake_phase();
            },
            _ => {
                self.recv_error_packet(packet.header);
            }
        }
    }

    fn process_handshake_phase(&mut self, packet: &Packet) {
        match packet.header {
            HEADER_GC_PHASE => {
                self.recv_phase_packet(packet);
            },
            // HEADER_GC_BINDUDP: not fully implemented in provided cpp snippet
            HEADER_GC_HANDSHAKE => {
                // recv_handshake logic (synchronizacja czasu, odpowiedz HEADER_CG_TIME_SYNC)
            },
            HEADER_GC_PING => {
                // recv_ping logic
            },
            HEADER_GC_HYBRIDCRYPT_KEYS | HEADER_GC_HYBRIDCRYPT_SDB => {
                // crypto
            },
            _ => {
                self.recv_error_packet(packet.header);
            }
        }
    }

    fn process_login_phase(&mut self, packet: &Packet) {
        match packet.header {
            HEADER_GC_PHASE => {
                self.recv_phase_packet(packet);
            },
            HEADER_GC_LOGIN_SUCCESS3 | HEADER_GC_LOGIN_SUCCESS4 => {
                // login success
            },
            HEADER_GC_LOGIN_FAILURE => {
                // login failure
            },
            HEADER_GC_EMPIRE => {
                // recv empire
            },
            HEADER_GC_LOGIN_KEY => {
                // recv login key
            },
            HEADER_GC_PING => {
                // recv ping
            },
            HEADER_GC_HYBRIDCRYPT_KEYS | HEADER_GC_HYBRIDCRYPT_SDB => {
                // crypto
            },
            _ => {
                // W C++ jest DefaultPacket / ErrorPacket, defaultowo ignorujemy tu 
                self.recv_error_packet(packet.header);
            }
        }
    }

    fn process_select_phase(&mut self, packet: &Packet) {
        match packet.header {
            HEADER_GC_PHASE => {
                self.recv_phase_packet(packet);
            },
            HEADER_GC_EMPIRE => {},
            HEADER_GC_LOGIN_SUCCESS3 | HEADER_GC_LOGIN_SUCCESS4 => {},
            HEADER_GC_PLAYER_CREATE_SUCCESS => {},
            HEADER_GC_PLAYER_CREATE_FAILURE => {},
            HEADER_GC_PLAYER_DELETE_WRONG_SOCIAL_ID => {},
            HEADER_GC_PLAYER_DELETE_SUCCESS => {},
            HEADER_GC_CHANGE_NAME => {},
            HEADER_GC_HANDSHAKE => {},
            // HEADER_GC_HANDSHAKE_OK
            HEADER_GC_HYBRIDCRYPT_KEYS | HEADER_GC_HYBRIDCRYPT_SDB => {},
            HEADER_GC_PLAYER_POINT_CHANGE => {},
            HEADER_GC_PING => {},
            _ => {
                self.recv_error_packet(packet.header);
            }
        }
    }

    fn process_loading_phase(&mut self, packet: &Packet) {
        match packet.header {
            HEADER_GC_PHASE => {
                self.recv_phase_packet(packet);
            },
            HEADER_GC_MAIN_CHARACTER => {},
            HEADER_GC_MAIN_CHARACTER2_EMPIRE => {},
            HEADER_GC_MAIN_CHARACTER3_BGM => {},
            HEADER_GC_MAIN_CHARACTER4_BGM_VOL => {},
            HEADER_GC_CHARACTER_UPDATE => {},
            HEADER_GC_PLAYER_POINTS => {},
            HEADER_GC_PLAYER_POINT_CHANGE => {},
            HEADER_GC_ITEM_SET => {},
            HEADER_GC_PING => {},
            HEADER_GC_QUICKSLOT_ADD => {},
            HEADER_GC_HYBRIDCRYPT_KEYS | HEADER_GC_HYBRIDCRYPT_SDB => {},
            _ => {
                self.set_game_phase(); // Fallback do GamePhase w loading phase wg zrodla
            }
        }
    }
 
    fn process_game_phase(&mut self, packet: &Packet) {
        match packet.header {
            HEADER_GC_PHASE => {
                self.recv_phase_packet(packet);
            },
            _ => {
                // W Game Phase C++ uzywa olbrzymiego switcha z pozostalymi HEADER_GC_*,
                // tu dla zadowolenia portu maszyny stanow na ten moment ignorujemy
                // niezdefiniowane lub wywolujemy error packet
            }
        }
     }
 }
