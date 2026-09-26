#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WavAudio {
    pub channels: u16,
    pub sample_rate: u32,
    pub bits_per_sample: u16,
    pub samples: Vec<i16>,
}

impl WavAudio {
    pub fn duration_ms(&self) -> u32 {
        if self.sample_rate == 0 || self.channels == 0 {
            return 0;
        }
        
        let total_samples = self.samples.len() as u32 / self.channels as u32;
        let duration_sec = total_samples as f64 / self.sample_rate as f64;
        (duration_sec * 1000.0) as u32
    }

    pub fn from_bytes(data: &[u8]) -> Result<WavAudio, &'static str> {
        if data.len() < 44 {
            return Err("Plik jest za krotki aby zawieral poprawny naglowek WAV");
        }

        // RIFF header
        if &data[0..4] != b"RIFF" {
            return Err("Nie znaleziono naglowka RIFF");
        }

        if &data[8..12] != b"WAVE" {
            return Err("Nie znaleziono naglowka WAVE");
        }

        // Parse chunks
        let mut offset = 12;
        let mut channels = 0;
        let mut sample_rate = 0;
        let mut bits_per_sample = 0;
        let mut samples = Vec::new();
        let mut fmt_found = false;
        let mut data_found = false;

        while offset + 8 <= data.len() {
            let chunk_id = &data[offset..offset + 4];
            let chunk_size = u32::from_le_bytes(data[offset + 4..offset + 8].try_into().unwrap()) as usize;
            offset += 8;

            if offset + chunk_size > data.len() {
                return Err("Nieprawidlowy rozmiar chunka (przekracza plik)");
            }

            match chunk_id {
                b"fmt " => {
                    if chunk_size < 16 {
                        return Err("Nieprawidlowy rozmiar chunka fmt");
                    }
                    
                    let audio_format = u16::from_le_bytes(data[offset..offset + 2].try_into().unwrap());
                    if audio_format != 1 {
                        return Err("Nieobsługiwany format audio (wymagany jest PCM format 1)");
                    }

                    channels = u16::from_le_bytes(data[offset + 2..offset + 4].try_into().unwrap());
                    sample_rate = u32::from_le_bytes(data[offset + 4..offset + 8].try_into().unwrap());
                    bits_per_sample = u16::from_le_bytes(data[offset + 14..offset + 16].try_into().unwrap());
                    
                    fmt_found = true;
                }
                b"data" => {
                    if !fmt_found {
                        return Err("Chunk data wystapil przed chunkiem fmt");
                    }

                    let raw_data = &data[offset..offset + chunk_size];
                    
                    if bits_per_sample == 8 {
                        // 8-bit unsigned PCM to 16-bit signed
                        samples = raw_data.iter().map(|&b| ((b as i16) - 128) * 256).collect();
                    } else if bits_per_sample == 16 {
                        // 16-bit signed PCM
                        if raw_data.len() % 2 != 0 {
                            return Err("Nieprawidlowy rozmiar danych dla probek 16-bitowych");
                        }
                        samples = raw_data.chunks_exact(2).map(|c| i16::from_le_bytes(c.try_into().unwrap())).collect();
                    } else {
                        return Err("Nieobsługiwana rozdzielczość (tylko 8-bit lub 16-bit)");
                    }
                    
                    data_found = true;
                }
                _ => {}
            }
            
            offset += chunk_size;
            // Pad byte to word alignment
            if chunk_size % 2 != 0 {
                offset += 1;
            }
        }

        if !fmt_found {
            return Err("Nie znaleziono chunka fmt");
        }

        if !data_found {
            return Err("Nie znaleziono chunka data");
        }

        Ok(WavAudio {
            channels,
            sample_rate,
            bits_per_sample,
            samples,
        })
    }
}
