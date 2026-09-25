//! Modul `phase_login` — Obsluga fazy logowania do serwera gry LoginPhase
//! Wzorzec referencyjny C++: `source/UserInterface/PythonNetworkStreamPhaseLogin.cpp`
//! Opis: Odwzorowanie LoginPhase: wysylanie HEADER_CG_LOGIN2/LOGIN3 z kluczem autoryzacji, obsluga HEADER_GC_LOGIN_SUCCESS i kodow bledow HEADER_GC_LOGIN_FAILURE (WRONGPWD, NOID, ALREADY, BLOCK).

pub const ID_MAX_NUM: usize = 30;
pub const PASS_MAX_NUM: usize = 16;
pub const PLAYER_PER_ACCOUNT3: usize = 3;
pub const PLAYER_PER_ACCOUNT4: usize = 4;
pub const GUILD_NAME_MAX_LEN: usize = 12;
pub const CHARACTER_NAME_MAX_LEN: usize = 24;
pub const LOGIN_STATUS_MAX_LEN: usize = 8;

pub const HEADER_CG_LOGIN: u8 = 1;
pub const HEADER_CG_DIRECT_ENTER: u8 = 9;
pub const HEADER_CG_LOGIN2: u8 = 109;
pub const HEADER_CG_LOGIN3: u8 = 111;

pub const HEADER_GC_LOGIN_SUCCESS3: u8 = 6;
pub const HEADER_GC_LOGIN_FAILURE: u8 = 7;
pub const HEADER_GC_LOGIN_SUCCESS4: u8 = 32;
pub const HEADER_GC_PING: u8 = 44;
pub const HEADER_GC_EMPIRE: u8 = 90;
pub const HEADER_GC_LOGIN_KEY: u8 = 118;
pub const HEADER_GC_HYBRIDCRYPT_KEYS: u8 = 152;
pub const HEADER_GC_HYBRIDCRYPT_SDB: u8 = 153;
pub const HEADER_GC_PHASE: u8 = 253;

#[allow(non_snake_case)]
#[repr(C, packed)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct TPacketCGLogin {
    pub header: u8,
    pub name: [u8; ID_MAX_NUM + 1],
    pub pwd: [u8; PASS_MAX_NUM + 1],
}

#[allow(non_snake_case)]
#[repr(C, packed)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct TPacketCGLogin2 {
    pub header: u8,
    pub name: [u8; ID_MAX_NUM + 1],
    pub login_key: u32,
    pub adwClientKey: [u32; 4],
}

#[allow(non_snake_case)]
#[repr(C, packed)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct TPacketCGLogin3 {
    pub header: u8,
    pub name: [u8; ID_MAX_NUM + 1],
    pub pwd: [u8; PASS_MAX_NUM + 1],
    pub adwClientKey: [u32; 4],
}

#[allow(non_snake_case)]
#[repr(C, packed)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct TPacketCGDirectEnter {
    pub bHeader: u8,
    pub login: [u8; ID_MAX_NUM + 1],
    pub passwd: [u8; PASS_MAX_NUM + 1],
    pub index: u8,
}

#[allow(non_snake_case)]
#[repr(C, packed)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct TSimplePlayerInformation {
    pub dwID: u32,
    pub szName: [u8; CHARACTER_NAME_MAX_LEN + 1],
    pub byJob: u8,
    pub byLevel: u8,
    pub dwPlayMinutes: u32,
    pub byST: u8,
    pub byHT: u8,
    pub byDX: u8,
    pub byIQ: u8,
    pub wMainPart: u16,
    pub bChangeName: u8,
    pub wHairPart: u16,
    pub bDummy: [u8; 4],
    pub x: i32,
    pub y: i32,
    pub lAddr: i32,
    pub wPort: u16,
    pub bySkillGroup: u8,
}

#[allow(non_snake_case)]
#[repr(C, packed)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct TPacketGCLoginSuccess3 {
    pub header: u8,
    pub akSimplePlayerInformation: [TSimplePlayerInformation; PLAYER_PER_ACCOUNT3],
    pub guild_id: [u32; PLAYER_PER_ACCOUNT3],
    pub guild_name: [[u8; GUILD_NAME_MAX_LEN + 1]; PLAYER_PER_ACCOUNT3],
    pub handle: u32,
    pub random_key: u32,
}

#[allow(non_snake_case)]
#[repr(C, packed)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct TPacketGCLoginSuccess4 {
    pub header: u8,
    pub akSimplePlayerInformation: [TSimplePlayerInformation; PLAYER_PER_ACCOUNT4],
    pub guild_id: [u32; PLAYER_PER_ACCOUNT4],
    pub guild_name: [[u8; GUILD_NAME_MAX_LEN + 1]; PLAYER_PER_ACCOUNT4],
    pub handle: u32,
    pub random_key: u32,
}

#[allow(non_snake_case)]
#[repr(C, packed)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct TPacketGCLoginFailure {
    pub header: u8,
    pub szStatus: [u8; LOGIN_STATUS_MAX_LEN + 1],
}

#[allow(non_snake_case)]
#[repr(C, packed)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct TPacketGCEmpire {
    pub bHeader: u8,
    pub bEmpire: u8,
}

#[allow(non_snake_case)]
#[repr(C, packed)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct TPacketGCLoginKey {
    pub bHeader: u8,
    pub dwLoginKey: u32,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct SMarkAuth {
    pub m_dwHandle: u32,
    pub m_dwRandomKey: u32,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone)]
pub struct PhaseLogin {
    pub initialized: bool,
    pub m_dwLoginKey: u32,
    pub m_isWaitLoginKey: bool,
    pub m_dwEmpireID: u8,
    pub m_akSimplePlayerInfo: [TSimplePlayerInformation; PLAYER_PER_ACCOUNT4],
    pub m_adwGuildID: [u32; PLAYER_PER_ACCOUNT4],
    pub m_astrGuildName: [[u8; GUILD_NAME_MAX_LEN + 1]; PLAYER_PER_ACCOUNT4],
    pub m_kMarkAuth: SMarkAuth,
    pub m_stID: String,
    pub m_stPassword: String,
    pub m_strPhase: String,
}

impl PhaseLogin {
    pub fn new() -> Self {
        Self {
            initialized: true,
            m_dwLoginKey: 0,
            m_isWaitLoginKey: false,
            m_dwEmpireID: 0,
            m_akSimplePlayerInfo: [TSimplePlayerInformation {
                dwID: 0,
                szName: [0; CHARACTER_NAME_MAX_LEN + 1],
                byJob: 0,
                byLevel: 0,
                dwPlayMinutes: 0,
                byST: 0,
                byHT: 0,
                byDX: 0,
                byIQ: 0,
                wMainPart: 0,
                bChangeName: 0,
                wHairPart: 0,
                bDummy: [0; 4],
                x: 0,
                y: 0,
                lAddr: 0,
                wPort: 0,
                bySkillGroup: 0,
            }; PLAYER_PER_ACCOUNT4],
            m_adwGuildID: [0; PLAYER_PER_ACCOUNT4],
            m_astrGuildName: [[0; GUILD_NAME_MAX_LEN + 1]; PLAYER_PER_ACCOUNT4],
            m_kMarkAuth: SMarkAuth::default(),
            m_stID: String::new(),
            m_stPassword: String::new(),
            m_strPhase: String::new(),
        }
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }

    pub fn set_login_phase(&mut self) {
        self.m_strPhase = "Login".to_string();
        self.clear_login_info();
    }

    pub fn send_login_packet(&mut self, name: &str, password: &str) -> TPacketCGLogin {
        let mut packet = TPacketCGLogin {
            header: HEADER_CG_LOGIN,
            name: [0; ID_MAX_NUM + 1],
            pwd: [0; PASS_MAX_NUM + 1],
        };
        Self::copy_str_to_buffer(name, &mut packet.name);
        Self::copy_str_to_buffer(password, &mut packet.pwd);
        packet
    }

    pub fn send_login_packet_new(&mut self, name: &str, _password: &str) -> TPacketCGLogin2 {
        let mut packet = TPacketCGLogin2 {
            header: HEADER_CG_LOGIN2,
            name: [0; ID_MAX_NUM + 1],
            login_key: self.m_dwLoginKey,
            adwClientKey: [0; 4],
        };
        Self::copy_str_to_buffer(name, &mut packet.name);
        // We simulate adwClientKey by zero filling here; game client dynamically injects keys
        packet
    }

    #[allow(non_snake_case)]
    pub fn send_login_packet3(&mut self, name: &str, password: &str) -> TPacketCGLogin3 {
        let mut packet = TPacketCGLogin3 {
            header: HEADER_CG_LOGIN3,
            name: [0; ID_MAX_NUM + 1],
            pwd: [0; PASS_MAX_NUM + 1],
            adwClientKey: [0; 4],
        };
        Self::copy_str_to_buffer(name, &mut packet.name);
        Self::copy_str_to_buffer(password, &mut packet.pwd);
        packet
    }

    pub fn send_direct_enter_packet(
        &mut self,
        name: &str,
        password: &str,
        index: u8,
    ) -> TPacketCGDirectEnter {
        let mut packet = TPacketCGDirectEnter {
            bHeader: HEADER_CG_DIRECT_ENTER,
            login: [0; ID_MAX_NUM + 1],
            passwd: [0; PASS_MAX_NUM + 1],
            index,
        };
        Self::copy_str_to_buffer(name, &mut packet.login);
        Self::copy_str_to_buffer(password, &mut packet.passwd);
        packet
    }

    pub fn clear_login_info(&mut self) {
        self.m_stID.clear();
        self.m_stPassword.clear();
    }

    pub fn recv_login_success_packet3(&mut self, packet: &TPacketGCLoginSuccess3) {
        for i in 0..PLAYER_PER_ACCOUNT3 {
            self.m_akSimplePlayerInfo[i] = packet.akSimplePlayerInformation[i];
            self.m_adwGuildID[i] = packet.guild_id[i];
            self.m_astrGuildName[i] = packet.guild_name[i];
        }
        self.m_kMarkAuth.m_dwHandle = packet.handle;
        self.m_kMarkAuth.m_dwRandomKey = packet.random_key;
    }

    pub fn recv_login_success_packet4(&mut self, packet: &TPacketGCLoginSuccess4) {
        for i in 0..PLAYER_PER_ACCOUNT4 {
            self.m_akSimplePlayerInfo[i] = packet.akSimplePlayerInformation[i];
            self.m_adwGuildID[i] = packet.guild_id[i];
            self.m_astrGuildName[i] = packet.guild_name[i];
        }
        self.m_kMarkAuth.m_dwHandle = packet.handle;
        self.m_kMarkAuth.m_dwRandomKey = packet.random_key;
    }

    pub fn recv_empire_packet(&mut self, packet: &TPacketGCEmpire) {
        self.m_dwEmpireID = packet.bEmpire;
    }

    pub fn recv_login_key_packet(&mut self, packet: &TPacketGCLoginKey) {
        self.m_dwLoginKey = packet.dwLoginKey;
        self.m_isWaitLoginKey = false;
    }

    pub fn recv_login_failure_packet(&mut self, packet: &TPacketGCLoginFailure) -> String {
        let mut status = String::new();
        for &b in packet.szStatus.iter() {
            if b == 0 {
                break;
            }
            status.push(b as char);
        }
        status
    }

    fn copy_str_to_buffer(s: &str, buf: &mut [u8]) {
        let bytes = s.as_bytes();
        let max_len = buf.len() - 1;
        let copy_len = bytes.len().min(max_len);
        buf[..copy_len].copy_from_slice(&bytes[..copy_len]);
        // Rest of the buffer remains null (0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skeleton_phase_login() {
        let instance = PhaseLogin::new();
        assert!(instance.is_ready());
    }
}
