// Modul mapowania i odtwarzania dzwiekow interfejsu (UI Sound Event Router).
// Czysty kod bez polskich znakow diakrytycznych. Zaimplementowano na silniku Rodio.

use rodio::{Decoder, OutputStreamHandle, Sink};
use std::fs::File;
use std::io::BufReader;

/// Definicja zdarzen dzwiekowych powiazanych z interfejsem uzytkownika.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UiSoundEvent {
    ButtonClick,
    OpenInventory,
    PickYang,
    DropItem,
}

pub struct UiSoundEventRouter {
    stream_handle: Option<OutputStreamHandle>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum RouterError {
    FileNotFound,
    DecodeError,
    SinkCreationError,
    NoStreamHandle,
}

impl UiSoundEventRouter {
    /// Tworzy nowa instancje routera dzwiekow UI, przyjmujac uchwyt do strumienia rodio.
    pub fn new(stream_handle: OutputStreamHandle) -> Self {
        Self { stream_handle: Some(stream_handle) }
    }

    /// Tworzy instancje routera bez uchwytu, zeby latwiej testowac mapping jesli audio device nie jest dostepne.
    pub fn new_for_test() -> Self {
        Self { stream_handle: None }
    }
    
    pub fn set_stream_handle(&mut self, stream_handle: OutputStreamHandle) {
        self.stream_handle = Some(stream_handle);
    }

    /// Pobiera sciezke do pliku dzwiekowego zaleznie od typu zdarzenia UI.
    /// Zgodnie z konwencja Metin2:
    pub fn get_sound_path(&self, event: UiSoundEvent) -> &'static str {
        match event {
            UiSoundEvent::ButtonClick => "sound/ui/click.wav",
            UiSoundEvent::OpenInventory => "sound/ui/open_inventory.wav", // sciezki mozna modyfikowac w zaleznosci od vfs
            UiSoundEvent::PickYang => "sound/ui/money.wav",
            UiSoundEvent::DropItem => "sound/ui/drop.wav",
        }
    }

    /// Odtwarza dzwiek bezposrednio dla przekazanego eventu.
    /// Funkcja zwraca pomyslnie utworzony Sink jezeli operacja sie powiodla, lub blad.
    pub fn play_sound(&self, event: UiSoundEvent) -> Result<Sink, RouterError> {
        let stream_handle = match &self.stream_handle {
            Some(handle) => handle,
            None => return Err(RouterError::NoStreamHandle),
        };
        
        let path = self.get_sound_path(event);
        
        let file = match File::open(path) {
            Ok(f) => f,
            Err(_) => return Err(RouterError::FileNotFound),
        };
        
        let buf_reader = BufReader::new(file);
        
        let decoder = match Decoder::new(buf_reader) {
            Ok(d) => d,
            Err(_) => return Err(RouterError::DecodeError),
        };
        
        let sink: Sink = match Sink::try_new(stream_handle) {
            Ok(s) => s,
            Err(_) => return Err(RouterError::SinkCreationError),
        };
        
        sink.append(decoder);
        Ok(sink)
    }
}
