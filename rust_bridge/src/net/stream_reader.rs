// Modul stream_reader - bufor odbierajacy pociete ramki TCP 
// i skladajacy je w calosciowe pakiety z obsluga deszyfrowania TEA.
// Wszystkie komentarze po polsku bez polskich znakow diakrytycznych.

use crate::tea::tea_decrypt;
use crate::ffi_guard::safe_ffi_call;
use crate::net::constants::PACKET_HEADER_SIZE;
use once_cell::sync::Lazy;
use std::sync::Mutex;

pub const STREAM_READER: u16 = 0x0205;

pub struct PacketStreamReader {
    pub buffer: Vec<u8>,
    pub head: usize,
    pub tail: usize,
    pub tea_key: Option<[u32; 4]>,
}

impl PacketStreamReader {
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
            head: 0,
            tail: 0,
            tea_key: None,
        }
    }

    pub fn set_tea_key(&mut self, key: [u32; 4]) {
        self.tea_key = Some(key);
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
        self.head = 0;
        self.tail = 0;
    }

    pub fn append_bytes(&mut self, data: &[u8]) {
        self.buffer.extend_from_slice(data);
        
        if let Some(key) = &self.tea_key {
            while self.buffer.len() - self.tail >= 8 {
                let mut temp = [0u8; 8];
                temp.copy_from_slice(&self.buffer[self.tail..self.tail + 8]);
                tea_decrypt(&mut temp, &self.buffer[self.tail..self.tail + 8], key);
                self.buffer[self.tail..self.tail + 8].copy_from_slice(&temp);
                self.tail += 8;
            }
        } else {
            self.tail = self.buffer.len();
        }
    }

    pub fn read_next_packet(&mut self) -> Option<Vec<u8>> {
        let available = self.tail - self.head;
        if available >= PACKET_HEADER_SIZE {
            let _header = u16::from_le_bytes([self.buffer[self.head], self.buffer[self.head + 1]]);
            let length = u16::from_le_bytes([self.buffer[self.head + 2], self.buffer[self.head + 3]]) as usize;

            if length < PACKET_HEADER_SIZE {
                // Nieprawidlowa dlugosc pakietu ponizej minimalnego naglowka (4 bajty)
                return None;
            }

            if available >= length {
                let packet = self.buffer[self.head..self.head + length].to_vec();
                self.head += length;

                self.buffer.drain(..self.head);
                self.tail -= self.head;
                self.head = 0;

                return Some(packet);
            }
        }
        None
    }
}

static GLOBAL_STREAM_READER: Lazy<Mutex<PacketStreamReader>> = Lazy::new(|| Mutex::new(PacketStreamReader::new()));

#[no_mangle]
pub unsafe extern "C" fn m2_net_stream_feed(data: *const u8, len: u32) -> bool {
    safe_ffi_call(false, || {
        if data.is_null() {
            return false;
        }
        let slice = std::slice::from_raw_parts(data, len as usize);
        let mut reader = GLOBAL_STREAM_READER.lock().unwrap();
        reader.append_bytes(slice);
        true
    })
}

#[no_mangle]
pub unsafe extern "C" fn m2_net_free_buffer(ptr: *mut u8, len: u32, capacity: u32) {
    safe_ffi_call((), || {
        if ptr.is_null() {
            return;
        }
        let _ = Vec::from_raw_parts(ptr, len as usize, capacity as usize);
    })
}
