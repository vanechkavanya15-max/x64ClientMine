// Modul obslugi Quickslotow i Questow/Skryptow (Faza B) w Rust.
// Komentarze wylacznie po polsku bez polskich znakow diakrytycznych.

use m2_client_bridge::ffi_guard::safe_ffi_call;
use m2_client_bridge::net::send_queue::enqueue_outgoing_packet;

// ============================================================================
// Stałe przestrzeni pakietów
// ============================================================================

/// Naglowek pakietu CG dla dodania elementu do paska szybkiego dostepu (Quickslot).
pub const HEADER_CG_QUICKSLOT_ADD: u16 = 0x0509;

/// Naglowek pakietu CG dla odpowiedzi skryptowej (Quest).
pub const HEADER_CG_SCRIPT_ANSWER: u16 = 0x0901;

// ============================================================================
// Struktury danych
// ============================================================================

/// Struktura pojedynczego elementu w pasku szybkiego dostepu (Quickslot).
/// Odpowiada SQuickSlot w C++.
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TQuickSlot {
    pub type_: u8,
    pub position: u8,
}

impl TQuickSlot {
    pub fn new(type_: u8, position: u8) -> Self {
        Self { type_, position }
    }
}

/// Struktura polecenia CG: Dodanie do Quickslota.
/// Odpowiada command_quickslot_add w C++.
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketCGQuickSlotAdd {
    pub header: u16,
    pub length: u16,
    pub pos: u8,
    pub slot: TQuickSlot,
}

impl TPacketCGQuickSlotAdd {
    pub const HEADER: u16 = HEADER_CG_QUICKSLOT_ADD;
    pub const SIZE: u16 = std::mem::size_of::<Self>() as u16;

    /// Tworzy nowy pakiet dodania przedmiotu do Quickslota.
    pub fn new(pos: u8, type_: u8, slot_position: u8) -> Self {
        Self {
            header: Self::HEADER,
            length: Self::SIZE,
            pos,
            slot: TQuickSlot::new(type_, slot_position),
        }
    }

    /// Serializuje pakiet do wektora bajtow w kolejnosci Little-Endian.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(Self::SIZE as usize);
        buf.extend_from_slice(&self.header.to_le_bytes());
        buf.extend_from_slice(&self.length.to_le_bytes());
        buf.push(self.pos);
        buf.push(self.slot.type_);
        buf.push(self.slot.position);
        buf
    }
}

/// Struktura polecenia CG: Odpowiedz do skryptu questa.
/// Odpowiada command_script_answer w C++.
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketCGScriptAnswer {
    pub header: u16,
    pub length: u16,
    pub answer: u8,
}

impl TPacketCGScriptAnswer {
    pub const HEADER: u16 = HEADER_CG_SCRIPT_ANSWER;
    pub const SIZE: u16 = std::mem::size_of::<Self>() as u16;

    /// Tworzy nowy pakiet odpowiedzi questa.
    pub fn new(answer: u8) -> Self {
        Self {
            header: Self::HEADER,
            length: Self::SIZE,
            answer,
        }
    }

    /// Serializuje pakiet do wektora bajtow w kolejnosci Little-Endian.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(Self::SIZE as usize);
        buf.extend_from_slice(&self.header.to_le_bytes());
        buf.extend_from_slice(&self.length.to_le_bytes());
        buf.push(self.answer);
        buf
    }
}

// ============================================================================
// Interfejs C FFI
// ============================================================================

/// Zastepuje CPythonNetworkStream::SendQuickSlotAddPacket.
/// Kolejkuje gotowy pakiet bezposrednio w m2_client_bridge / rust_bridge.
#[no_mangle]
pub extern "C" fn SendQuickSlotAddPacket(wpos: u8, type_: u8, pos: u8) -> bool {
    safe_ffi_call(false, || {
        let packet = TPacketCGQuickSlotAdd::new(wpos, type_, pos);
        let bytes = packet.to_bytes();
        enqueue_outgoing_packet(bytes)
    })
}

/// Zastepuje m2_send_script_answer wywolywane ewentualnie przez CPythonNetworkStream::SendScriptAnswerPacket,
/// ktora kopiuje sekwencje bitow do pre-allokowanego bufora out_buf.
/// Zwraca liczbe zapisanych bajtow lub 0 w przypadku bledu.
#[no_mangle]
pub unsafe extern "C" fn m2_send_script_answer(idx: u8, out_buf: *mut u8) -> usize {
    safe_ffi_call(0, || {
        if out_buf.is_null() {
            return 0;
        }

        let packet = TPacketCGScriptAnswer::new(idx);
        let bytes = packet.to_bytes();
        
        unsafe {
            std::ptr::copy_nonoverlapping(bytes.as_ptr(), out_buf, bytes.len());
        }

        bytes.len()
    })
}

pub const HEADER_CG_QUEST_INPUT_STRING: u16 = 0x0904;

#[repr(C, packed)]
pub struct TPacketCGQuestInputString {
    pub header: u16,
    pub length: u16,
    pub msg: [u8; 65],
}

impl TPacketCGQuestInputString {
    pub const HEADER: u16 = HEADER_CG_QUEST_INPUT_STRING;
    pub const SIZE: u16 = std::mem::size_of::<Self>() as u16;

    pub fn new(msg: &str) -> Self {
        let mut msg_arr = [0u8; 65];
        let bytes = msg.as_bytes();
        let len = std::cmp::min(bytes.len(), 64);
        msg_arr[..len].copy_from_slice(&bytes[..len]);
        
        Self {
            header: Self::HEADER,
            length: Self::SIZE,
            msg: msg_arr,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(Self::SIZE as usize);
        buf.extend_from_slice(&self.header.to_le_bytes());
        buf.extend_from_slice(&self.length.to_le_bytes());
        buf.extend_from_slice(&self.msg);
        buf
    }
}

#[no_mangle]
pub extern "C" fn m2_send_quest_input_string(msg_ptr: *const std::os::raw::c_char) -> bool {
    safe_ffi_call(false, || {
        if msg_ptr.is_null() {
            return false;
        }
        let c_str = unsafe { std::ffi::CStr::from_ptr(msg_ptr) };
        if let Ok(msg) = c_str.to_str() {
            let packet = TPacketCGQuestInputString::new(msg);
            return enqueue_outgoing_packet(packet.to_bytes());
        }
        false
    })
}

/// Zastepuje CPythonNetworkStream::SendQuickSlotDelPacket.
#[no_mangle]
pub extern "C" fn SendQuickSlotDelPacket(pos: u8) -> bool {
    safe_ffi_call(false, || {
        let mut buf = Vec::with_capacity(5);
        buf.extend_from_slice(&0x050Au16.to_le_bytes()); // HEADER_CG_QUICKSLOT_DEL
        buf.extend_from_slice(&5u16.to_le_bytes());
        buf.push(pos);
        enqueue_outgoing_packet(buf)
    })
}

/// Zastepuje CPythonNetworkStream::SendQuickSlotMovePacket.
#[no_mangle]
pub extern "C" fn SendQuickSlotMovePacket(pos: u8, change_pos: u8) -> bool {
    safe_ffi_call(false, || {
        let mut buf = Vec::with_capacity(6);
        buf.extend_from_slice(&0x050Bu16.to_le_bytes()); // HEADER_CG_QUICKSLOT_SWAP
        buf.extend_from_slice(&6u16.to_le_bytes());
        buf.push(pos);
        buf.push(change_pos);
        enqueue_outgoing_packet(buf)
    })
}

