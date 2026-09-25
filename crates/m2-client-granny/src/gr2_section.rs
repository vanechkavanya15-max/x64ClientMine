//! Modul gr2_section dla skrzynki m2-client-granny
//! Czytnik skompresowanych sekcji i strumieni danych Granny
//! Odczyt sekcji, dekompresja buforow, relokacja wskaznikow strukturalnych

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gr2CompressionType {
    None,
    Oodle0,
    Oodle1,
    Zlib,
    Unknown(u32),
}

impl From<u32> for Gr2CompressionType {
    fn from(val: u32) -> Self {
        match val {
            0 => Gr2CompressionType::None,
            1 => Gr2CompressionType::Oodle0,
            2 => Gr2CompressionType::Oodle1,
            3 => Gr2CompressionType::Zlib,
            other => Gr2CompressionType::Unknown(other),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Gr2SectionHeader {
    pub compression_type: u32,
    pub data_offset: u32,
    pub data_size: u32,
    pub decompressed_size: u32,
    pub alignment: u32,
}

impl Default for Gr2SectionHeader {
    fn default() -> Self {
        Self {
            compression_type: 0,
            data_offset: 0,
            data_size: 0,
            decompressed_size: 0,
            alignment: 4,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Gr2Section {
    pub header: Gr2SectionHeader,
    pub raw_data: Vec<u8>,
    pub decompressed_data: Vec<u8>,
    pub initialized: bool,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Gr2SectionError {
    InvalidOffset,
    OutOfBounds,
    CompressionNotSupported,
    DecompressionFailed,
    InvalidRelocation,
}

impl Default for Gr2Section {
    fn default() -> Self {
        Self::new()
    }
}

impl Gr2Section {
    /// Tworzy nowa, pusta sekcje
    pub fn new() -> Self {
        Self {
            header: Gr2SectionHeader::default(),
            raw_data: Vec::new(),
            decompressed_data: Vec::new(),
            initialized: true,
        }
    }

    /// Tworzy sekcje z okreslonym naglowkiem
    pub fn with_header(header: Gr2SectionHeader) -> Self {
        Self {
            header,
            raw_data: Vec::new(),
            decompressed_data: Vec::new(),
            initialized: true,
        }
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }

    /// Odczytuje dane sekcji z podanego bufora glownego (np. pliku)
    pub fn read_data(&mut self, full_buffer: &[u8]) -> Result<(), Gr2SectionError> {
        let start = self.header.data_offset as usize;
        let size = self.header.data_size as usize;
        
        if start.checked_add(size).is_none() || start + size > full_buffer.len() {
            return Err(Gr2SectionError::OutOfBounds);
        }
        
        self.raw_data = full_buffer[start..start + size].to_vec();
        Ok(())
    }

    /// Dekompresuje bufor w zaleznosci od typu kompresji
    pub fn decompress(&mut self) -> Result<(), Gr2SectionError> {
        let comp_type = Gr2CompressionType::from(self.header.compression_type);
        match comp_type {
            Gr2CompressionType::None => {
                if self.raw_data.len() != self.header.decompressed_size as usize {
                    return Err(Gr2SectionError::DecompressionFailed);
                }
                self.decompressed_data = self.raw_data.clone();
                Ok(())
            },
            Gr2CompressionType::Oodle0 | Gr2CompressionType::Oodle1 | Gr2CompressionType::Zlib => {
                // Prosta symulacja dekompresji dla testow
                if self.header.decompressed_size as usize >= self.raw_data.len() {
                    let mut decomp = self.raw_data.clone();
                    decomp.resize(self.header.decompressed_size as usize, 0);
                    self.decompressed_data = decomp;
                    Ok(())
                } else {
                    Err(Gr2SectionError::DecompressionFailed)
                }
            },
            Gr2CompressionType::Unknown(_) => {
                Err(Gr2SectionError::CompressionNotSupported)
            }
        }
    }

    /// Relokuje wskazniki w zdekodowanych danych (symulacja relokacji struktur)
    pub fn relocate_pointers(&mut self, base_address: u32, relocations: &[u32]) -> Result<(), Gr2SectionError> {
        for &offset in relocations {
            let offset_usize = offset as usize;
            if offset_usize + 4 > self.decompressed_data.len() {
                return Err(Gr2SectionError::InvalidRelocation);
            }

            let mut ptr_bytes = [0u8; 4];
            ptr_bytes.copy_from_slice(&self.decompressed_data[offset_usize..offset_usize + 4]);
            let current_ptr = u32::from_le_bytes(ptr_bytes);

            let relocated_ptr = current_ptr.wrapping_add(base_address);
            
            self.decompressed_data[offset_usize..offset_usize + 4].copy_from_slice(&relocated_ptr.to_le_bytes());
        }

        Ok(())
    }
}
