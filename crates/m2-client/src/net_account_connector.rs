//! Modul `net_account_connector` — Konektor serwera autoryzacji CAccountConnector (Port 11000)
//! Wzorzec referencyjny C++: `source/UserInterface/AccountConnector.cpp`
//! Opis: Odwzorowanie CAccountConnector: maszyna stanow autoryzacji (STATE_OFFLINE, STATE_HANDSHAKE, STATE_AUTH), wysylanie loginu i hasla, odbior klucza sesji dwLoginKey.

pub const HEADER_CG_LOGIN3: u8 = 111;
pub const HEADER_CG_PONG: u8 = 0xfe;
pub const HEADER_GC_PHASE: u8 = 0xfd;
pub const HEADER_GC_HANDSHAKE: u8 = 0xff;
pub const HEADER_GC_AUTH_SUCCESS: u8 = 150;
pub const HEADER_GC_LOGIN_FAILURE: u8 = 7;
pub const HEADER_GC_PING: u8 = 44;

pub const ID_MAX_NUM: usize = 30;
pub const PASS_MAX_NUM: usize = 16;
pub const LOGIN_STATUS_MAX_LEN: usize = 8;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Phase {
    PHASE_CLOSE = 0,
    PHASE_HANDSHAKE = 1,
    PHASE_LOGIN = 2,
    PHASE_SELECT = 3,
    PHASE_LOADING = 4,
    PHASE_GAME = 5,
    PHASE_DEAD = 6,
    PHASE_DBCLIENT_CONNECTING = 7,
    PHASE_DBCLIENT = 8,
    PHASE_P2P = 9,
    PHASE_AUTH = 10,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C, packed)]
pub struct TPacketCGLogin3 {
    pub header: u8,
    pub name: [u8; ID_MAX_NUM + 1],
    pub pwd: [u8; PASS_MAX_NUM + 1],
    pub adwClientKey: [u32; 4],
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C, packed)]
pub struct TPacketGCPhase {
    pub header: u8,
    pub phase: u8,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C, packed)]
pub struct TPacketGCHandshake {
    pub header: u8,
    pub dwHandshake: u32,
    pub dwTime: u32,
    pub lDelta: i32,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C, packed)]
pub struct TPacketGCAuthSuccess {
    pub bHeader: u8,
    pub dwLoginKey: u32,
    pub bResult: u8,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C, packed)]
pub struct TPacketGCLoginFailure {
    pub header: u8,
    pub szStatus: [u8; LOGIN_STATUS_MAX_LEN + 1],
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C, packed)]
pub struct TPacketGCPing {
    pub header: u8,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C, packed)]
pub struct TPacketCGPong {
    pub bHeader: u8,
}


#[derive(Debug, PartialEq, Clone, Copy)]
pub enum State {
    Offline,
    Handshake,
    Auth,
}

impl Default for State {
    fn default() -> Self {
        State::Offline
    }
}

#[derive(Debug, Clone, Default)]
pub struct NetAccountConnector {
    pub state: State,
    pub id: String,
    pub password: String,
    pub addr: String,
    pub port: i32,
    pub client_key: [u32; 4],
    pub login_key: u32,
    pub initialized: bool,
}

impl NetAccountConnector {
    pub fn new() -> Self {
        Self {
            initialized: true,
            state: State::Offline,
            id: String::new(),
            password: String::new(),
            addr: String::new(),
            port: 0,
            client_key: [0; 4],
            login_key: 0,
        }
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }

    pub fn set_login_info(&mut self, id: &str, pwd: &str) {
        self.id = id.to_string();
        self.password = pwd.to_string();
    }

    pub fn clear_login_info(&mut self) {
        self.password = String::new();
    }

    pub fn connect(&mut self, addr: &str, port: i32, _account_addr: &str, _account_port: i32) {
        self.addr = addr.to_string();
        self.port = port;
        self.state = State::Offline;
    }

    pub fn disconnect(&mut self) {
        self.state = State::Offline;
    }

    pub fn recv_phase(&mut self, phase: u8) -> Option<TPacketCGLogin3> {
        if phase == Phase::PHASE_HANDSHAKE as u8 {
            self.state = State::Handshake;
            None
        } else if phase == Phase::PHASE_AUTH as u8 {
            let mut packet = TPacketCGLogin3 {
                header: HEADER_CG_LOGIN3,
                name: [0; ID_MAX_NUM + 1],
                pwd: [0; PASS_MAX_NUM + 1],
                adwClientKey: self.client_key,
            };

            let id_bytes = self.id.as_bytes();
            let copy_id_len = id_bytes.len().min(ID_MAX_NUM);
            packet.name[..copy_id_len].copy_from_slice(&id_bytes[..copy_id_len]);
            packet.name[ID_MAX_NUM] = 0; // null terminate explicitly

            let pwd_bytes = self.password.as_bytes();
            let copy_pwd_len = pwd_bytes.len().min(PASS_MAX_NUM);
            packet.pwd[..copy_pwd_len].copy_from_slice(&pwd_bytes[..copy_pwd_len]);
            packet.pwd[PASS_MAX_NUM] = 0; // null terminate explicitly

            self.clear_login_info();
            self.state = State::Auth;

            Some(packet)
        } else {
            None
        }
    }

    pub fn recv_handshake(&self, packet: TPacketGCHandshake) -> TPacketGCHandshake {
        let mut response = packet;
        response.dwTime = packet.dwTime.wrapping_add(packet.lDelta as u32).wrapping_add(packet.lDelta as u32);
        response.lDelta = 0;
        response
    }

    pub fn recv_auth_success(&mut self, packet: TPacketGCAuthSuccess) {
        if packet.bResult != 0 {
            self.login_key = packet.dwLoginKey;
            self.disconnect();
        } else {
            self.disconnect();
        }
    }

    pub fn recv_auth_failure(&mut self, _packet: TPacketGCLoginFailure) -> bool {
        // Matches C++: PyCallClassMemberFunc("OnLoginFailure", ...);
        // leaving state unaffected (since __OfflineState_Set is commented out)
        // returns true indicating packet processed.
        true
    }

    pub fn recv_ping(&self) -> TPacketCGPong {
        TPacketCGPong {
            bHeader: HEADER_CG_PONG,
        }
    }
}
