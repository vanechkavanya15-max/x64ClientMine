use std::cmp;

/// A true ring buffer for accumulating TCP chunks and reassembling them into Metin2 packets.
/// Metin2 packets have the format:
///   [0..2]: u16 Header
///   [2..4]: u16 Length (total length including header and length field)
///   [4..Length]: Payload
pub struct TcpStreamReassembler {
    buffer: Vec<u8>,
    head: usize,
    tail: usize,
    count: usize,
}

impl TcpStreamReassembler {
    /// Creates a new reassembler with the given initial capacity.
    pub fn new(initial_capacity: usize) -> Self {
        Self {
            buffer: vec![0; cmp::max(initial_capacity, 1024)],
            head: 0,
            tail: 0,
            count: 0,
        }
    }

    /// Appends incoming TCP data to the buffer.
    pub fn write(&mut self, data: &[u8]) {
        if data.is_empty() {
            return;
        }

        self.ensure_capacity(data.len());

        let cap = self.buffer.len();
        let end_space = cap - self.tail;

        if data.len() <= end_space {
            self.buffer[self.tail..self.tail + data.len()].copy_from_slice(data);
        } else {
            self.buffer[self.tail..cap].copy_from_slice(&data[..end_space]);
            self.buffer[0..data.len() - end_space].copy_from_slice(&data[end_space..]);
        }

        self.tail = (self.tail + data.len()) % cap;
        self.count += data.len();
    }

    /// Returns the number of bytes currently in the buffer
    pub fn len(&self) -> usize {
        self.count
    }

    /// Returns whether the buffer is currently empty
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    /// Reads a statically sized array of bytes from the ring buffer without consuming them.
    fn peek_array<const N: usize>(&self) -> Option<[u8; N]> {
        if self.count < N {
            return None;
        }

        let cap = self.buffer.len();
        let end_space = cap - self.head;
        let mut result = [0u8; N];

        if N <= end_space {
            result.copy_from_slice(&self.buffer[self.head..self.head + N]);
        } else {
            result[..end_space].copy_from_slice(&self.buffer[self.head..cap]);
            result[end_space..].copy_from_slice(&self.buffer[0..N - end_space]);
        }

        Some(result)
    }

    /// Reads a specific number of bytes from the ring buffer without consuming them.
    fn peek_bytes(&self, size: usize) -> Option<Vec<u8>> {
        if self.count < size {
            return None;
        }

        let cap = self.buffer.len();
        let end_space = cap - self.head;
        let mut result = Vec::with_capacity(size);

        if size <= end_space {
            result.extend_from_slice(&self.buffer[self.head..self.head + size]);
        } else {
            result.extend_from_slice(&self.buffer[self.head..cap]);
            result.extend_from_slice(&self.buffer[0..size - end_space]);
        }

        Some(result)
    }

    /// Consumes a specific number of bytes from the ring buffer.
    fn consume(&mut self, size: usize) {
        if size > self.count {
            panic!("Attempting to consume more bytes than available");
        }
        self.head = (self.head + size) % self.buffer.len();
        self.count -= size;
    }

    /// Attempts to read the next complete packet from the buffer.
    pub fn read_packet(&mut self) -> Option<Vec<u8>> {
        // We need at least 4 bytes to parse header and length
        let header_data = self.peek_array::<4>()?;

        let header = u16::from_le_bytes([header_data[0], header_data[1]]);
        let packet_length = u16::from_le_bytes([header_data[2], header_data[3]]) as usize;

        if header == 0x0603 || header == 0x0519 {
            // These are dynamically sized packets (HEADER_GC_CHAT, HEADER_GC_QUICKSLOT_ADD).
            // We just ensure length covers header and length fields.
            if packet_length < 4 {
                // Invalid dynamic packet
                // Instead of blocking forever, we consume 1 byte to attempt recovery
                self.consume(1);
                return None;
            }
        }

        if self.count < packet_length {
            return None; // Wait for full packet
        }

        let packet = self.peek_bytes(packet_length)?;
        self.consume(packet_length);

        Some(packet)
    }

    fn ensure_capacity(&mut self, needed_space: usize) {
        if self.buffer.len() - self.count >= needed_space {
            return;
        }

        let mut new_cap = self.buffer.len() * 2;
        while new_cap - self.count < needed_space {
            new_cap *= 2;
        }

        let mut new_buffer = vec![0; new_cap];

        if self.count > 0 {
            let cap = self.buffer.len();
            let end_space = cap - self.head;

            if self.count <= end_space {
                new_buffer[0..self.count].copy_from_slice(&self.buffer[self.head..self.head + self.count]);
            } else {
                new_buffer[0..end_space].copy_from_slice(&self.buffer[self.head..cap]);
                new_buffer[end_space..self.count].copy_from_slice(&self.buffer[0..self.count - end_space]);
            }
        }

        self.buffer = new_buffer;
        self.head = 0;
        self.tail = self.count;
    }
}
