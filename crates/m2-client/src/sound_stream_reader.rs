pub const CHUNK_SIZE: usize = 16_384;
pub const MAX_BUFFER_BYTES: usize = CHUNK_SIZE * 3;

pub struct StreamReader {
    buffer: Vec<u8>,
    head: usize,
    tail: usize,
    count: usize,
    total_bytes: usize,
    bytes_fed: usize,
}

impl StreamReader {
    pub fn new(total_bytes: usize) -> Self {
        Self {
            buffer: vec![0; MAX_BUFFER_BYTES],
            head: 0,
            tail: 0,
            count: 0,
            total_bytes,
            bytes_fed: 0,
        }
    }

    pub fn feed_chunk(&mut self, chunk: &[u8]) -> Result<(), &'static str> {
        if self.bytes_fed + chunk.len() > self.total_bytes {
            return Err("Overfeeding: exceeds total_bytes");
        }
        
        if self.count + chunk.len() > MAX_BUFFER_BYTES {
            return Err("Buffer capacity limit reached");
        }

        for &byte in chunk {
            self.buffer[self.tail] = byte;
            self.tail = (self.tail + 1) % MAX_BUFFER_BYTES;
            self.count += 1;
            self.bytes_fed += 1;
        }

        Ok(())
    }

    pub fn read_samples(&mut self, out_buffer: &mut [i16]) -> usize {
        let mut samples_read = 0;

        for sample in out_buffer.iter_mut() {
            if self.count < 2 {
                break; // Not enough bytes for a 16-bit sample
            }

            let b1 = self.buffer[self.head];
            self.head = (self.head + 1) % MAX_BUFFER_BYTES;
            
            let b2 = self.buffer[self.head];
            self.head = (self.head + 1) % MAX_BUFFER_BYTES;
            
            self.count -= 2;

            // Construct i16 from two bytes (assuming little-endian as it's common for PCM audio, though the specific endianness depends on the decoding, typical for wav/ogg on x64).
            // Actually, we'll construct it exactly as read (low byte, high byte).
            *sample = i16::from_le_bytes([b1, b2]);
            samples_read += 1;
        }

        samples_read
    }

    pub fn is_end_of_stream(&self) -> bool {
        self.bytes_fed == self.total_bytes && self.count < 2
    }

    pub fn seek_to_start(&mut self) {
        self.head = 0;
        self.tail = 0;
        self.count = 0;
        self.bytes_fed = 0;
    }
}
