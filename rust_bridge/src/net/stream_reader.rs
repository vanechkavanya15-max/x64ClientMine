// Modul stream_reader - bufor kolowy odbierajacy pociete ramki TCP 
// i skladajacy je w calosciowe pakiety GC z deszyfrowaniem TEA.
// Wszystkie komentarze po polsku bez polskich znakow diakrytycznych.

use crate::tea::tea_decrypt;
use crate::ffi_guard::safe_ffi_call;
use once_cell::sync::Lazy;
use std::sync::Mutex;

pub const STREAM_READER: u8 = 0;

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

    pub fn append_bytes(&mut self, data: &[u8]) {
        self.buffer.extend_from_slice(data);
        
        while self.buffer.len() - self.tail >= 8 {
            if let Some(key) = &self.tea_key {
                let mut temp = [0u8; 8];
                temp.copy_from_slice(&self.buffer[self.tail..self.tail + 8]);
                tea_decrypt(&mut temp, &self.buffer[self.tail..self.tail + 8], key);
                self.buffer[self.tail..self.tail + 8].copy_from_slice(&temp);
            }
            self.tail += 8;
        }
    }

    pub fn read_next_packet(&mut self) -> Option<Vec<u8>> {
        if self.tail - self.head > 0 {
            let header = self.buffer[self.head];
            
            if let Some(size) = get_packet_size(header, &self.buffer, self.head, self.tail) {
                if self.tail - self.head >= size {
                    let packet = self.buffer[self.head..self.head + size].to_vec();
                    self.head += size;
                    
                    self.buffer.drain(..self.head);
                    self.tail -= self.head;
                    self.head = 0;
                    
                    return Some(packet);
                }
            }
        }
        None
    }
}

fn get_packet_size(header: u8, buffer: &[u8], head: usize, tail: usize) -> Option<usize> {
    match header {
        4 => { // HEADER_GC_CHAT
            if tail - head >= 3 {
                let size = u16::from_le_bytes([buffer[head + 1], buffer[head + 2]]) as usize;
                Some(size)
            } else {
                None
            }
        }
        STREAM_READER => Some(13),
        _ => None, // Nieznany naglowek
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
