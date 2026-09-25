//! Modul `guild_mark_downloader` — Klient pobierania i buforowania godeł gildii
//! Wzorzec referencyjny C++: `source/UserInterface/GuildMarkDownloader.cpp`
//! Opis: Odwzorowanie CGuildMarkDownloader: zarzadzanie obrazami godeł gildii 16x12 pikseli, skojarzenie guild_id -> symbol.

use std::collections::HashMap;

// Packet Headers from Server
pub const HEADER_GC_PHASE: u8 = 0xfd;
pub const HEADER_GC_HANDSHAKE: u8 = 0xff;
pub const HEADER_GC_PING: u8 = 44;
pub const HEADER_GC_MARK_IDXLIST: u8 = 102;
pub const HEADER_GC_MARK_BLOCK: u8 = 100;
pub const HEADER_GC_GUILD_SYMBOL_DATA: u8 = 133;
pub const HEADER_GC_MARK_DIFF_DATA: u8 = 101;

// Packet Headers from Client
pub const HEADER_CG_MARK_LOGIN: u8 = 100;
pub const HEADER_CG_MARK_CRCLIST: u8 = 101;
pub const HEADER_CG_MARK_IDXLIST: u8 = 104;
pub const HEADER_CG_GUILD_SYMBOL_CRC: u8 = 113;
pub const HEADER_CG_PONG: u8 = 0xfe;

pub const PHASE_LOGIN: u8 = 2; // login

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(Default)]
pub enum State {
    #[default]
    Offline = 0,
    Login = 1,
    Complete = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(Default)]
pub enum Todo {
    #[default]
    None = 0,
    RecvMark = 1,
    RecvSymbol = 2,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct PacketGCPhase {
    pub header: u8,
    pub phase: u8,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct PacketGCHandshake {
    pub header: u8,
    pub handshake: u32,
    pub time: u32,
    pub delta: i32,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct PacketGCPing {
    pub header: u8,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct PacketGCMarkIdxList {
    pub header: u8,
    pub buf_size: u32,
    pub count: u16,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct PacketGCMarkBlock {
    pub header: u8,
    pub buf_size: u32,
    pub img_idx: u8,
    pub count: u32,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct PacketGCGuildSymbolData {
    pub header: u8,
    pub size: u16,
    pub guild_id: u32,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct PacketCGMarkLogin {
    pub header: u8,
    pub handle: u32,
    pub random_key: u32,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct PacketCGPong {
    pub header: u8,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct PacketCGMarkIdxList {
    pub header: u8,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct PacketCGMarkCrcList {
    pub header: u8,
    pub img_idx: u8,
    pub crc_list: [u32; 80],
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct PacketCGSymbolCrc {
    pub header: u8,
    pub guild_id: u32,
    pub crc: u32,
    pub size: u32,
}

#[derive(Debug, Clone, Default)]
pub struct GuildMarkDownloader {
    pub state: State,
    pub todo: Todo,
    pub handle: u32,
    pub random_key: u32,
    pub guild_ids: Vec<u32>,
    pub current_requesting_image_index: u8,
    
    // Mocking CNetworkStream recv and send buffers for tests
    pub recv_buffer: Vec<u8>,
    pub send_buffer: Vec<u8>,
    pub marks: HashMap<u16, u16>, // guild_id -> mark_id
    pub security_mode: bool,
}

impl GuildMarkDownloader {
    pub fn new() -> Self {
        let mut inst = Self::default();
        inst.initialize();
        inst
    }

    pub fn is_ready(&self) -> bool {
        self.state != State::Offline
    }
    
    pub fn initialize(&mut self) {
        self.state = State::Offline;
        self.current_requesting_image_index = 0;
        self.handle = 0;
        self.random_key = 0;
        self.todo = Todo::None;
        self.guild_ids.clear();
        self.recv_buffer.clear();
        self.send_buffer.clear();
        self.marks.clear();
        self.security_mode = false;
    }
    
    pub fn connect(&mut self, handle: u32, random_key: u32) -> bool {
        self.offline_state_set();
        self.handle = handle;
        self.random_key = random_key;
        self.todo = Todo::RecvMark;
        // Mocking CNetworkStream::Connect
        self.login_state_set(); // called from OnConnectSuccess in C++
        true
    }
    
    pub fn connect_to_recv_symbol(&mut self, handle: u32, random_key: u32, guild_ids: Vec<u32>) -> bool {
        self.offline_state_set();
        self.handle = handle;
        self.random_key = random_key;
        self.todo = Todo::RecvSymbol;
        self.guild_ids = guild_ids;
        // Mocking CNetworkStream::Connect
        self.login_state_set(); // called from OnConnectSuccess in C++
        true
    }
    
    pub fn process(&mut self) {
        if !self.state_process() {
            self.offline_state_set();
        }
    }
    
    fn state_process(&mut self) -> bool {
        match self.state {
            State::Login => self.login_state_process(),
            State::Complete => false,
            _ => true,
        }
    }
    
    fn offline_state_set(&mut self) {
        self.initialize();
    }
    
    fn complete_state_set(&mut self) {
        self.state = State::Complete;
    }
    
    fn login_state_set(&mut self) {
        self.state = State::Login;
    }
    
    fn peek_u8(&self) -> Option<u8> {
        if self.recv_buffer.is_empty() {
            None
        } else {
            Some(self.recv_buffer[0])
        }
    }
    
    fn login_state_process(&mut self) -> bool {
        let header = match self.peek_u8() {
            Some(h) => h,
            None => return true, // C++ returns true if Peek fails
        };
        
        if self.security_mode {
            if header == 0 {
                if self.recv_buffer.is_empty() {
                    return false;
                }
                self.recv_buffer.remove(0); // Recv(sizeof(header))
                return true;
            }
        }
        
        let need_packet_size = self.get_packet_size(header);
        if need_packet_size == 0 {
            return false;
        }
        
        if self.recv_buffer.len() < need_packet_size {
            return true;
        }
        
        self.dispatch_packet(header)
    }
    
    fn get_packet_size(&self, header: u8) -> usize {
        match header {
            HEADER_GC_PHASE => std::mem::size_of::<PacketGCPhase>(),
            HEADER_GC_HANDSHAKE => std::mem::size_of::<PacketGCHandshake>(),
            HEADER_GC_PING => std::mem::size_of::<PacketGCPing>(),
            HEADER_GC_MARK_IDXLIST => std::mem::size_of::<PacketGCMarkIdxList>(),
            HEADER_GC_MARK_BLOCK => std::mem::size_of::<PacketGCMarkBlock>(),
            HEADER_GC_GUILD_SYMBOL_DATA => std::mem::size_of::<PacketGCGuildSymbolData>(),
            HEADER_GC_MARK_DIFF_DATA => 1,
            _ => 0,
        }
    }
    
    fn dispatch_packet(&mut self, header: u8) -> bool {
        match header {
            HEADER_GC_PHASE => self.login_state_recv_phase(),
            HEADER_GC_HANDSHAKE => self.login_state_recv_handshake(),
            HEADER_GC_PING => self.login_state_recv_ping(),
            HEADER_GC_MARK_IDXLIST => self.login_state_recv_mark_index(),
            HEADER_GC_MARK_BLOCK => self.login_state_recv_mark_block(),
            HEADER_GC_GUILD_SYMBOL_DATA => self.login_state_recv_symbol_data(),
            HEADER_GC_MARK_DIFF_DATA => {
                self.recv_buffer.drain(0..1);
                true
            },
            _ => false,
        }
    }
    
    fn login_state_recv_handshake(&mut self) -> bool {
        if self.recv_buffer.len() < std::mem::size_of::<PacketGCHandshake>() {
            return false;
        }
        self.recv_buffer.drain(0..std::mem::size_of::<PacketGCHandshake>());
        
        let packet = PacketCGMarkLogin {
            header: HEADER_CG_MARK_LOGIN,
            handle: self.handle,
            random_key: self.random_key,
        };
        
        let packet_bytes = unsafe {
            std::slice::from_raw_parts(&packet as *const _ as *const u8, std::mem::size_of::<PacketCGMarkLogin>())
        };
        self.send_buffer.extend_from_slice(packet_bytes);
        true
    }
    
    fn login_state_recv_ping(&mut self) -> bool {
        if self.recv_buffer.len() < std::mem::size_of::<PacketGCPing>() {
            return false;
        }
        self.recv_buffer.drain(0..std::mem::size_of::<PacketGCPing>());
        
        let packet = PacketCGPong {
            header: HEADER_CG_PONG,
        };
        
        let packet_bytes = unsafe {
            std::slice::from_raw_parts(&packet as *const _ as *const u8, std::mem::size_of::<PacketCGPong>())
        };
        self.send_buffer.extend_from_slice(packet_bytes);
        true
    }
    
    fn login_state_recv_phase(&mut self) -> bool {
        if self.recv_buffer.len() < std::mem::size_of::<PacketGCPhase>() {
            return false;
        }
        
        let packet = unsafe {
            std::ptr::read_unaligned(self.recv_buffer.as_ptr() as *const PacketGCPhase)
        };
        
        self.recv_buffer.drain(0..std::mem::size_of::<PacketGCPhase>());
        
        if packet.phase == PHASE_LOGIN {
            self.security_mode = true; // mocking LocaleService_GetSecurityKey
            
            match self.todo {
                Todo::None => {}, // assert
                Todo::RecvMark => {
                    if !self.send_mark_idx_list() {
                        return false;
                    }
                },
                Todo::RecvSymbol => {
                    if !self.send_symbol_crc_list() {
                        return false;
                    }
                }
            }
        }
        true
    }
    
    fn send_mark_idx_list(&mut self) -> bool {
        let packet = PacketCGMarkIdxList {
            header: HEADER_CG_MARK_IDXLIST,
        };
        
        let packet_bytes = unsafe {
            std::slice::from_raw_parts(&packet as *const _ as *const u8, std::mem::size_of::<PacketCGMarkIdxList>())
        };
        self.send_buffer.extend_from_slice(packet_bytes);
        true
    }
    
    fn login_state_recv_mark_index(&mut self) -> bool {
        if self.recv_buffer.len() < std::mem::size_of::<PacketGCMarkIdxList>() {
            return false;
        }
        
        let packet = unsafe {
            std::ptr::read_unaligned(self.recv_buffer.as_ptr() as *const PacketGCMarkIdxList)
        };
        
        let expected_size = std::mem::size_of::<PacketGCMarkIdxList>() + (packet.count as usize * 4); // WORD guild_id, WORD mark_id
        if self.recv_buffer.len() < expected_size {
            return false; // not enough data for all items
        }
        
        self.recv_buffer.drain(0..std::mem::size_of::<PacketGCMarkIdxList>());
        
        for _ in 0..packet.count {
            let guild_id = u16::from_le_bytes([self.recv_buffer[0], self.recv_buffer[1]]);
            let mark_id = u16::from_le_bytes([self.recv_buffer[2], self.recv_buffer[3]]);
            self.recv_buffer.drain(0..4);
            
            self.marks.insert(guild_id, mark_id);
        }
        
        self.current_requesting_image_index = 0;
        self.send_mark_crc_list();
        true
    }
    
    fn send_mark_crc_list(&mut self) -> bool {
        // Here C++ asks MarkManager for GetBlockCRCList and returns false if it's done.
        // For testing we mock 2 images.
        if self.current_requesting_image_index >= self.marks.len() as u8 {
            self.complete_state_set();
        } else {
            let packet = PacketCGMarkCrcList {
                header: HEADER_CG_MARK_CRCLIST,
                img_idx: self.current_requesting_image_index,
                crc_list: [0; 80], // Mock empty crclist
            };
            
            let packet_bytes = unsafe {
                std::slice::from_raw_parts(&packet as *const _ as *const u8, std::mem::size_of::<PacketCGMarkCrcList>())
            };
            self.send_buffer.extend_from_slice(packet_bytes);
            self.current_requesting_image_index += 1;
        }
        true
    }
    
    fn login_state_recv_mark_block(&mut self) -> bool {
        if self.recv_buffer.len() < std::mem::size_of::<PacketGCMarkBlock>() {
            return false;
        }
        
        let packet = unsafe {
            std::ptr::read_unaligned(self.recv_buffer.as_ptr() as *const PacketGCMarkBlock)
        };
        
        let required_data_size = std::mem::size_of::<PacketGCMarkBlock>();
        
        // Peek ahead to calculate if we have all blocks
        let mut pos = required_data_size;
        for _ in 0..packet.count {
            if self.recv_buffer.len() < pos + 5 {
                return false;
            }
            let comp_size = u32::from_le_bytes([
                self.recv_buffer[pos+1], self.recv_buffer[pos+2], self.recv_buffer[pos+3], self.recv_buffer[pos+4]
            ]) as usize;
            pos += 5 + comp_size;
        }
        
        if self.recv_buffer.len() < pos {
            return false;
        }
        
        self.recv_buffer.drain(0..std::mem::size_of::<PacketGCMarkBlock>());
        
        for _ in 0..packet.count {
            let _pos_block = self.recv_buffer[0];
            let comp_size = u32::from_le_bytes([
                self.recv_buffer[1], self.recv_buffer[2], self.recv_buffer[3], self.recv_buffer[4]
            ]) as usize;
            
            self.recv_buffer.drain(0..5);
            
            if comp_size > 0 {
                self.recv_buffer.drain(0..comp_size);
                // In C++ it saves the block and image
            }
        }
        
        if (self.current_requesting_image_index as usize) < self.marks.len() {
            self.send_mark_crc_list();
        } else {
            self.complete_state_set();
        }
        
        true
    }
    
    fn send_symbol_crc_list(&mut self) -> bool {
        for guild_id in &self.guild_ids {
            let packet = PacketCGSymbolCrc {
                header: HEADER_CG_GUILD_SYMBOL_CRC,
                guild_id: *guild_id,
                crc: 1234, // TODO: GetFileCRC32(GetGuildSymbolFileName(guild_id))
                size: 5678, // TODO: GetFileSize(GetGuildSymbolFileName(guild_id))
            };
            
            let packet_bytes = unsafe {
                std::slice::from_raw_parts(&packet as *const _ as *const u8, std::mem::size_of::<PacketCGSymbolCrc>())
            };
            self.send_buffer.extend_from_slice(packet_bytes);
        }
        true
    }
    
    fn login_state_recv_symbol_data(&mut self) -> bool {
        if self.recv_buffer.len() < std::mem::size_of::<PacketGCGuildSymbolData>() {
            return false;
        }
        
        let packet = unsafe {
            std::ptr::read_unaligned(self.recv_buffer.as_ptr() as *const PacketGCGuildSymbolData)
        };
        
        if (packet.size as usize) < std::mem::size_of::<PacketGCGuildSymbolData>() {
            return false;
        }
        
        if self.recv_buffer.len() < packet.size as usize {
            return true;
        }
        
        self.recv_buffer.drain(0..std::mem::size_of::<PacketGCGuildSymbolData>());
        
        let data_size = packet.size as usize - std::mem::size_of::<PacketGCGuildSymbolData>();
        
        if self.recv_buffer.len() < data_size {
            return false;
        }
        
        self.recv_buffer.drain(0..data_size);
        
        true
    }
}
