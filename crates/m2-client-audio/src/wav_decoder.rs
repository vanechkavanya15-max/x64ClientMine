//! Modul wav_decoder dla skrzynki m2-client-audio
//! Dekoder nieskompresowanych plikow WAV PCM (8/16-bit)
//! Parser naglowkow RIFF/WAVE, format PCM, odczyt probek audio mono/stereo

use std::io::{Read, Seek, SeekFrom};
use std::fmt;

/// Reprezentuje bledy podczas dekodowania WAV
#[derive(Debug)]
pub enum WavError {
    IoError(std::io::Error),
    InvalidHeader(&'static str),
    UnsupportedFormat(&'static str),
    UnexpectedEof,
}

impl fmt::Display for WavError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WavError::IoError(err) => write!(f, "Blad wejscia/wyjscia: {}", err),
            WavError::InvalidHeader(msg) => write!(f, "Nieprawidlowy naglowek WAV: {}", msg),
            WavError::UnsupportedFormat(msg) => write!(f, "Nieobslugiwany format WAV: {}", msg),
            WavError::UnexpectedEof => write!(f, "Nieoczekiwany koniec pliku (EOF)"),
        }
    }
}

impl std::error::Error for WavError {}

impl From<std::io::Error> for WavError {
    fn from(err: std::io::Error) -> Self {
        if err.kind() == std::io::ErrorKind::UnexpectedEof {
            WavError::UnexpectedEof
        } else {
            WavError::IoError(err)
        }
    }
}

/// Naglowek z metadanymi pliku WAV
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WavHeader {
    pub channels: u16,
    pub sample_rate: u32,
    pub byte_rate: u32,
    pub block_align: u16,
    pub bits_per_sample: u16,
}

/// Wynik pomyslnego zdekodowania
#[derive(Debug, Clone)]
pub struct WavData {
    pub header: WavHeader,
    pub samples: Vec<u8>,
}

/// Glowna struktura implementacyjna
#[derive(Debug, Clone, Default)]
pub struct WavDecoder {
    pub initialized: bool,
}

impl WavDecoder {
    pub fn new() -> Self {
        Self { initialized: true }
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }

    /// Dekoduje plik WAV z danego czytnika
    pub fn decode<R: Read + Seek>(&self, mut reader: R) -> Result<WavData, WavError> {
        let mut buf4 = [0u8; 4];
        let mut buf2 = [0u8; 2];

        // Naglowek RIFF
        reader.read_exact(&mut buf4)?;
        if &buf4 != b"RIFF" {
            return Err(WavError::InvalidHeader("Brak sygnatury RIFF"));
        }

        let _file_size = read_u32_le(&mut reader)?;

        // Naglowek WAVE
        reader.read_exact(&mut buf4)?;
        if &buf4 != b"WAVE" {
            return Err(WavError::InvalidHeader("Brak sygnatury WAVE"));
        }

        let mut header: Option<WavHeader> = None;
        let mut samples: Option<Vec<u8>> = None;

        loop {
            // Czytamy ID chunk'a i rozmiar
            match reader.read_exact(&mut buf4) {
                Ok(_) => {}
                Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => break,
                Err(e) => return Err(e.into()),
            }
            
            let chunk_size = read_u32_le(&mut reader)?;
            let chunk_id = buf4;

            if &chunk_id == b"fmt " {
                if chunk_size < 16 {
                    return Err(WavError::InvalidHeader("Chunk fmt jest za krotki"));
                }

                reader.read_exact(&mut buf2)?;
                let audio_format = u16::from_le_bytes(buf2);
                if audio_format != 1 { // 1 = PCM
                    return Err(WavError::UnsupportedFormat("Wspierany tylko format PCM (wspolczynnik 1)"));
                }

                reader.read_exact(&mut buf2)?;
                let channels = u16::from_le_bytes(buf2);

                let sample_rate = read_u32_le(&mut reader)?;
                let byte_rate = read_u32_le(&mut reader)?;
                
                reader.read_exact(&mut buf2)?;
                let block_align = u16::from_le_bytes(buf2);
                
                reader.read_exact(&mut buf2)?;
                let bits_per_sample = u16::from_le_bytes(buf2);

                if bits_per_sample != 8 && bits_per_sample != 16 {
                    return Err(WavError::UnsupportedFormat("Wspierane tylko 8 lub 16 bitow na probke"));
                }

                if channels != 1 && channels != 2 {
                    return Err(WavError::UnsupportedFormat("Wspierane tylko mono(1) lub stereo(2)"));
                }

                header = Some(WavHeader {
                    channels,
                    sample_rate,
                    byte_rate,
                    block_align,
                    bits_per_sample,
                });

                // Przesuwamy sie jezeli chunk fmt jest dluzszy niz 16 bajtow (dla zbalansowania)
                if chunk_size > 16 {
                    reader.seek(SeekFrom::Current((chunk_size - 16) as i64))?;
                }

            } else if &chunk_id == b"data" {
                if header.is_none() {
                    return Err(WavError::InvalidHeader("Brak chunk'a fmt przed data"));
                }
                
                let mut data = vec![0u8; chunk_size as usize];
                reader.read_exact(&mut data)?;
                samples = Some(data);
                
            } else {
                // Nieznany chunk, ignoruj
                reader.seek(SeekFrom::Current(chunk_size as i64))?;
            }
            
            // Jesli rozmiar chunka jest nieparzysty, padding 1 bajt (specyfikacja RIFF)
            if chunk_size % 2 != 0 {
                // Sprobuj odczytac padding, ale nie przejmuj sie jesli to koniec pliku
                let mut padding = [0u8; 1];
                let _ = reader.read_exact(&mut padding);
            }
        }

        let header = header.ok_or(WavError::InvalidHeader("Nie odnaleziono chunk'a fmt"))?;
        let samples = samples.ok_or(WavError::InvalidHeader("Nie odnaleziono chunk'a data"))?;

        Ok(WavData { header, samples })
    }
}

// Funkcja pomocnicza do odczytu u32 w formacie little-endian
fn read_u32_le<R: Read>(mut reader: R) -> Result<u32, std::io::Error> {
    let mut buf = [0u8; 4];
    reader.read_exact(&mut buf)?;
    Ok(u32::from_le_bytes(buf))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skeleton_wav_decoder() {
        let obj = WavDecoder::new();
        assert!(obj.is_ready());
    }
}
