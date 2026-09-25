//! Modul ogg_decoder dla skrzynki m2-client-audio
//! Czytnik strumieniowego audio i muzyki tla BGM
//! Strumieniowy czytnik probek, metadane utworu, punkty zapetlenia utworu

use std::io::{Read, Seek, SeekFrom};

/// Metadane utworu strumieniowego.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrackMetadata {
    pub title: String,
    pub artist: String,
    pub sample_rate: u32,
    pub channels: u16,
    pub length_in_samples: u64,
}

impl Default for TrackMetadata {
    fn default() -> Self {
        Self {
            title: String::new(),
            artist: String::new(),
            sample_rate: 44100,
            channels: 2,
            length_in_samples: 0,
        }
    }
}

/// Punkty zapetlenia utworu (w probkach).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LoopPoints {
    pub start_sample: u64,
    pub end_sample: u64,
}

/// OggDecoder - Glowna struktura dekodera OGG.
/// W tej uproszczonej wersji implementujemy mock dekodera do testow w systemie.
pub struct OggDecoder<R: Read + Seek> {
    #[allow(dead_code)]
    reader: R,
    metadata: TrackMetadata,
    loop_points: Option<LoopPoints>,
    current_sample: u64,
    initialized: bool,
}

impl<R: Read + Seek> OggDecoder<R> {
    /// Tworzy nowy dekoder OGG, czytajac strumien.
    pub fn new(mut reader: R, metadata: TrackMetadata) -> Self {
        // Ustawiamy pozycje na 0 (mock)
        let _ = reader.seek(SeekFrom::Start(0));
        
        Self {
            reader,
            metadata,
            loop_points: None,
            current_sample: 0,
            initialized: true,
        }
    }

    /// Zwraca status gotowosci.
    pub fn is_ready(&self) -> bool {
        self.initialized
    }
    
    /// Pobiera metadane utworu.
    pub fn get_metadata(&self) -> &TrackMetadata {
        &self.metadata
    }

    /// Ustawia punkty zapetlenia.
    /// Zwraca `false` jesli parametry sa nieprawidlowe (np. start >= end lub end > dlugosc).
    pub fn set_loop_points(&mut self, start_sample: u64, end_sample: u64) -> bool {
        if start_sample >= end_sample || end_sample > self.metadata.length_in_samples {
            return false;
        }
        self.loop_points = Some(LoopPoints {
            start_sample,
            end_sample,
        });
        true
    }

    /// Czysci punkty zapetlenia.
    pub fn clear_loop_points(&mut self) {
        self.loop_points = None;
    }

    /// Przewija utwor do podanej probki. Zwraca true jesli sie udalo.
    pub fn seek(&mut self, sample: u64) -> bool {
        if sample > self.metadata.length_in_samples {
            return false;
        }
        self.current_sample = sample;
        // W rzeczywistej implementacji tutaj ustawialibysmy `reader.seek(...)` bazujac na probce, bajtach, rozmiarze bitowym probki i naglowku.
        true
    }

    /// Zwraca biezaca pozycje.
    pub fn current_sample(&self) -> u64 {
        self.current_sample
    }

    /// Wczytuje do bufora zadana ilosc probek (tutaj mockujemy to operacja na `current_sample`).
    /// Obsluguje zapetlenia - jesli osiagnie end_sample z LoopPoints, cofa do start_sample.
    /// Wczytywana porcja nie przekroczy w jednej operacji punktu konca lub punktu zapetlenia.
    /// Zwraca ilosc "wczytanych" probek (wartosci u64 w tym mocku uzyte jako offsety).
    pub fn read_samples(&mut self, num_samples: u64) -> u64 {
        if num_samples == 0 {
            return 0;
        }

        // Limitujemy odczyt do konca utworu
        let max_read = self.metadata.length_in_samples.saturating_sub(self.current_sample);
        
        let mut to_read = num_samples.min(max_read);

        if let Some(loop_pts) = self.loop_points {
            if self.current_sample < loop_pts.end_sample {
                // Limit do zapetlenia
                let till_loop_end = loop_pts.end_sample.saturating_sub(self.current_sample);
                to_read = to_read.min(till_loop_end);
            }
        }

        if to_read == 0 {
            return 0; // Osiagnelismy koniec i brak obslugi zapetlenia po osiagnieciu end (jesli wyjdzie)
        }

        self.current_sample += to_read;

        // Jesli mamy petle i osiagnelismy koniec zapetlenia
        if let Some(loop_pts) = self.loop_points {
            if self.current_sample == loop_pts.end_sample {
                self.current_sample = loop_pts.start_sample;
            }
        }

        to_read
    }
}
