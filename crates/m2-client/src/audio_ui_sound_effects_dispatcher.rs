use rodio::{Decoder, OutputStreamHandle, Sink};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

/// Definicja predefiniowanych zdarzen dzwiekowych powiazanych z interfejsem uzytkownika.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub enum UiSoundEvent {
    UI_CLICK_BUTTON,
    UI_OPEN_WINDOW,
    UI_CLOSE_WINDOW,
    UI_DROP_ITEM,
    UI_PICK_ITEM,
    UI_ERROR_NOTIFY,
}

/// Błędy routera / dispatchera dźwięków UI.
#[derive(Debug, PartialEq, Eq)]
pub enum DispatcherError {
    FileNotFound,
    DecodeError,
    SinkCreationError,
    NoStreamHandle,
    UnregisteredEvent,
}

/// Klasa dispatchera odtwarzania dźwięków UI bez przestrzennego tłumienia.
pub struct AudioUiSoundEffectsDispatcher {
    stream_handle: Option<OutputStreamHandle>,
    event_registry: HashMap<UiSoundEvent, String>,
}

impl AudioUiSoundEffectsDispatcher {
    /// Tworzy nowa instancje dispatchera z uchwytem strumienia.
    pub fn new(stream_handle: OutputStreamHandle) -> Self {
        Self {
            stream_handle: Some(stream_handle),
            event_registry: Self::default_registry(),
        }
    }

    /// Tworzy instancje dispatchera do testow bez uchwytu strumienia.
    pub fn new_for_test() -> Self {
        Self {
            stream_handle: None,
            event_registry: Self::default_registry(),
        }
    }

    /// Zwraca standardowe mapowanie dźwięków wg konwencji klienta.
    fn default_registry() -> HashMap<UiSoundEvent, String> {
        let mut registry = HashMap::new();
        registry.insert(
            UiSoundEvent::UI_CLICK_BUTTON,
            "sound/ui/click.wav".to_string(),
        );
        registry.insert(
            UiSoundEvent::UI_OPEN_WINDOW,
            "sound/ui/open_window.wav".to_string(),
        );
        registry.insert(
            UiSoundEvent::UI_CLOSE_WINDOW,
            "sound/ui/close_window.wav".to_string(),
        );
        registry.insert(UiSoundEvent::UI_DROP_ITEM, "sound/ui/drop.wav".to_string());
        registry.insert(UiSoundEvent::UI_PICK_ITEM, "sound/ui/pick.wav".to_string());
        registry.insert(
            UiSoundEvent::UI_ERROR_NOTIFY,
            "sound/ui/error.wav".to_string(),
        );
        registry
    }

    /// Aktualizuje uchwyt strumienia audio.
    #[allow(dead_code)]
    pub fn set_stream_handle(&mut self, stream_handle: OutputStreamHandle) {
        self.stream_handle = Some(stream_handle);
    }

    /// Rejestruje lub nadpisuje sciezke pliku przypisana do eventu UI.
    pub fn register_sound_event(&mut self, event: UiSoundEvent, path: &str) {
        self.event_registry.insert(event, path.to_string());
    }

    /// Pobiera aktualnie zarejestrowaną ścieżkę dźwięku dla zdarzenia.
    pub fn get_sound_path(&self, event: UiSoundEvent) -> Option<&String> {
        self.event_registry.get(&event)
    }

    /// Odtwarza dźwięk 2D w pełnej głośności stereo dla danego zdarzenia UI.
    pub fn play_sound(&self, event: UiSoundEvent) -> Result<Sink, DispatcherError> {
        let stream_handle = match &self.stream_handle {
            Some(handle) => handle,
            None => return Err(DispatcherError::NoStreamHandle),
        };

        let path = match self.get_sound_path(event) {
            Some(p) => p,
            None => return Err(DispatcherError::UnregisteredEvent),
        };

        let file = match File::open(path) {
            Ok(f) => f,
            Err(_) => return Err(DispatcherError::FileNotFound),
        };

        let buf_reader = BufReader::new(file);

        let decoder = match Decoder::new(buf_reader) {
            Ok(d) => d,
            Err(_) => return Err(DispatcherError::DecodeError),
        };

        let sink: Sink = match Sink::try_new(stream_handle) {
            Ok(s) => s,
            Err(_) => return Err(DispatcherError::SinkCreationError),
        };

        // UI Sounds są odtwarzane w pełnej głośności.
        sink.set_volume(1.0);
        sink.append(decoder);

        sink.sleep_until_end();

        Ok(sink)
    }
}
