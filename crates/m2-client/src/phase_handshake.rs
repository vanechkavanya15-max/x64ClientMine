//! Modul `phase_handshake` — Obsluga fazy uzgadniania czasu i kluczy HandShake
//! Wzorzec referencyjny C++: `source/UserInterface/PythonNetworkStreamPhaseHandShake.cpp`
//! Opis: Odwzorowanie HandShakePhase: wymiana pakietu HEADER_GC_HANDSHAKE (40), obliczanie delty czasu serwera i synchronizacja zegara klienta.

#![allow(non_snake_case)]

pub const HEADER_GC_HANDSHAKE: u8 = 40;
pub const HEADER_CG_TIME_SYNC: u8 = 41;

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct TPacketGCHandshake {
    pub header: u8,
    pub dwTime: u32,
    pub lDelta: i32,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct TPacketGCBlank {
    pub header: u8,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ServerTimeSync {
    pub m_dwChangeServerTime: u32,
    pub m_dwChangeClientTime: u32,
}

#[derive(Debug, Clone, Default)]
pub struct PhaseHandshake {
    pub initialized: bool,
    pub m_HandshakeData: TPacketGCHandshake,
    pub m_kServerTimeSync: ServerTimeSync,
    pub m_strPhase: String,
    pub m_dwChangingPhaseTime: u32,
    pub server_msec: u32,
}

impl PhaseHandshake {
    pub fn new() -> Self {
        Self {
            initialized: true,
            m_HandshakeData: TPacketGCHandshake::default(),
            m_kServerTimeSync: ServerTimeSync::default(),
            m_strPhase: String::new(),
            m_dwChangingPhaseTime: 0,
            server_msec: 0,
        }
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }

    pub fn set_handshake_phase(&mut self, current_time: u32) {
        self.m_strPhase = String::from("HandShake");
        self.m_dwChangingPhaseTime = current_time;
    }

    pub fn process_handshake_packet(&mut self, packet: TPacketGCHandshake) -> Option<TPacketGCHandshake> {
        self.m_HandshakeData = packet;
        
        // ELTimer_SetServerMSec(m_HandshakeData.dwTime + m_HandshakeData.lDelta);
        // Kopiujemy pola lokalnie, aby uniknac bledu E0793 (unaligned reference of packed struct field)
        let dw_time = self.m_HandshakeData.dwTime;
        let l_delta = self.m_HandshakeData.lDelta;

        self.server_msec = dw_time.wrapping_add(l_delta as u32);

        // m_HandshakeData.dwTime = m_HandshakeData.dwTime + m_HandshakeData.lDelta + m_HandshakeData.lDelta;
        let new_time = dw_time.wrapping_add(l_delta as u32).wrapping_add(l_delta as u32);
        self.m_HandshakeData.dwTime = new_time;
        self.m_HandshakeData.lDelta = 0;

        Some(self.m_HandshakeData)
    }

    pub fn recv_handshake_packet(&mut self, packet: TPacketGCHandshake, current_client_time: u32) -> Option<TPacketGCHandshake> {
        let mut k_handshake_data = packet;
        
        let dw_time = k_handshake_data.dwTime;
        let l_delta = k_handshake_data.lDelta;

        self.m_kServerTimeSync.m_dwChangeServerTime = dw_time.wrapping_add(l_delta as u32);
        self.m_kServerTimeSync.m_dwChangeClientTime = current_client_time;
        
        k_handshake_data.dwTime = dw_time.wrapping_add(l_delta as u32).wrapping_add(l_delta as u32);
        k_handshake_data.lDelta = 0;
        k_handshake_data.header = HEADER_CG_TIME_SYNC;
        
        Some(k_handshake_data)
    }

    pub fn recv_handshake_ok_packet(&mut self, current_client_time: u32) -> u32 {
        let dw_delta = current_client_time.wrapping_sub(self.m_kServerTimeSync.m_dwChangeClientTime);
        self.server_msec = self.m_kServerTimeSync.m_dwChangeServerTime.wrapping_add(dw_delta);
        self.server_msec
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skeleton_phase_handshake() {
        let instance = PhaseHandshake::new();
        assert!(instance.is_ready());
    }
}
