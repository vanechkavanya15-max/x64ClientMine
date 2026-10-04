use std::cmp;

/// A simple ring buffer for accumulating TCP chunks and reassembling them into Metin2 packets.
/// Metin2 packets have the format:
///   [0..2]: u16 Header
///   [2..4]: u16 Length (total length including header and length field)
///   [4..Length]: Payload
pub struct TcpStreamReassembler {
    buffer: Vec<u8>,
    read_pos: usize,
    write_pos: usize,
}

impl TcpStreamReassembler {
    /// Creates a new reassembler with the given initial capacity.
    pub fn new(initial_capacity: usize) -> Self {
        Self {
            buffer: vec![0; initial_capacity],
            read_pos: 0,
            write_pos: 0,
        }
    }

    /// Appends incoming TCP data to the buffer.
    pub fn write(&mut self, data: &[u8]) {
        if data.is_empty() {
            return;
        }

        self.ensure_writable(data.len());
        self.buffer[self.write_pos..self.write_pos + data.len()].copy_from_slice(data);
        self.write_pos += data.len();
    }

    /// Attempts to read the next complete packet from the buffer.
    /// Returns `Some(packet_data)` if a full packet is available, `None` otherwise.
    pub fn read_packet(&mut self) -> Option<Vec<u8>> {
        let readable = self.readable_bytes();

        // Need at least 4 bytes for the header and length fields
        if readable < 4 {
            return None;
        }

        // Read the packet length (u16 at offset 2, little endian)
        let length_bytes = [
            self.buffer[self.read_pos + 2],
            self.buffer[self.read_pos + 3],
        ];
        let packet_length = u16::from_le_bytes(length_bytes) as usize;

        // If the packet is larger than the buffer's capacity or the readable bytes,
        // we might have a corrupted stream or an overly large packet, but for this
        // simple reassembler, we just wait until we have enough bytes.
        // Wait until we have the full packet
        if readable < packet_length {
            return None;
        }

        // Extract the packet
        let packet = self.buffer[self.read_pos..self.read_pos + packet_length].to_vec();
        self.read_pos += packet_length;

        self.maybe_compact();

        Some(packet)
    }

    /// Returns the number of readable bytes in the buffer.
    fn readable_bytes(&self) -> usize {
        self.write_pos - self.read_pos
    }

    /// Returns the number of writable bytes at the end of the buffer.
    fn writable_bytes(&self) -> usize {
        self.buffer.len() - self.write_pos
    }

    /// Ensures there is enough contiguous space to write `len` bytes.
    fn ensure_writable(&mut self, len: usize) {
        if self.writable_bytes() >= len {
            return;
        }

        // Try compaction first
        self.compact();
        if self.writable_bytes() >= len {
            return;
        }

        // Need to grow
        let needed = self.write_pos + len;
        let mut new_size = cmp::max(self.buffer.len(), 1024);
        while new_size < needed {
            new_size *= 2;
        }

        self.buffer.resize(new_size, 0);
    }

    /// Moves remaining readable bytes to the start of the buffer.
    fn compact(&mut self) {
        if self.read_pos == 0 {
            return;
        }

        let readable = self.readable_bytes();
        if readable > 0 {
            self.buffer.copy_within(self.read_pos..self.write_pos, 0);
        }

        self.read_pos = 0;
        self.write_pos = readable;
    }

    /// Compacts the buffer if it's more than half empty and we've read at least half.
    fn maybe_compact(&mut self) {
        if self.read_pos == self.write_pos {
            self.read_pos = 0;
            self.write_pos = 0;
        } else if self.read_pos > self.buffer.len() / 2 {
            self.compact();
        }
    }
}
