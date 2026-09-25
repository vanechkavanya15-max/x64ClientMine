    fn login_state_recv_symbol_data(&mut self) -> bool {
        if self.recv_buffer.len() < std::mem::size_of::<PacketGCGuildSymbolData>() {
            return false;
        }
        
        let packet = unsafe {
            std::ptr::read_unaligned(self.recv_buffer.as_ptr() as *const PacketGCGuildSymbolData)
        };
        
        if (packet.size as usize) < std::mem::size_of::<PacketGCGuildSymbolData>() {
            return false; // protect against underflow
        }
        
        if self.recv_buffer.len() < packet.size as usize {
            return true; // wait for more data like C++ does with GetRecvBufferSize
        }
        
        self.recv_buffer.drain(0..std::mem::size_of::<PacketGCGuildSymbolData>());
        
        let data_size = packet.size as usize - std::mem::size_of::<PacketGCGuildSymbolData>();
        
        if self.recv_buffer.len() < data_size {
            return false; // Error if not enough after drain, shouldn't happen based on above check
        }
        
        self.recv_buffer.drain(0..data_size);
        
        // Data would be saved to file here.
        true
    }
