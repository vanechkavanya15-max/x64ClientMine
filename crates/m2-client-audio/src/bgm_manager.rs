//! Modul bgm_manager dla skrzynki m2-client-audio
//! Menedzer muzyki w tle z plynna zmiana i petlami (fade in/out)
//! Plynne przejscia miedzy utworami w miastach i na expowiskach, crossfading

/// Stany przejscia dla menedzera muzyki w tle
#[derive(Debug, Clone, PartialEq)]
pub enum TransitionState {
    Idle,
    FadeIn { duration: f32, elapsed: f32 },
    FadeOut { duration: f32, elapsed: f32 },
    Crossfade { duration: f32, elapsed: f32 },
}

impl Default for TransitionState {
    fn default() -> Self {
        TransitionState::Idle
    }
}

/// Glowna struktura implementacyjna menedzera muzyki w tle
#[derive(Debug, Clone, Default)]
pub struct BgmManager {
    pub initialized: bool,
    pub global_volume: f32,
    
    // Sciezka glowna
    pub primary_track: Option<String>,
    pub primary_volume: f32,
    pub primary_loop: bool,

    // Sciezka poboczna uzywana wylacznie podczas crossfade'u
    pub secondary_track: Option<String>,
    pub secondary_volume: f32,

    pub transition_state: TransitionState,
}

impl BgmManager {
    pub fn new() -> Self {
        Self {
            initialized: true,
            global_volume: 1.0,
            primary_track: None,
            primary_volume: 0.0,
            primary_loop: false,
            secondary_track: None,
            secondary_volume: 0.0,
            transition_state: TransitionState::Idle,
        }
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }
    
    /// Ustawia globalna glosnosc (0.0 do 1.0)
    pub fn set_global_volume(&mut self, volume: f32) {
        self.global_volume = volume.clamp(0.0, 1.0);
    }
    
    /// Zwraca efektywna glosnosc glownej sciezki
    pub fn effective_primary_volume(&self) -> f32 {
        self.primary_volume * self.global_volume
    }

    /// Zwraca efektywna glosnosc sciezki pobocznej
    pub fn effective_secondary_volume(&self) -> f32 {
        self.secondary_volume * self.global_volume
    }

    /// Natychmiastowe odtwarzanie (bez przejscia)
    pub fn play(&mut self, track: &str, looped: bool) {
        self.primary_track = Some(track.to_string());
        self.primary_volume = 1.0;
        self.primary_loop = looped;
        
        self.secondary_track = None;
        self.secondary_volume = 0.0;
        
        self.transition_state = TransitionState::Idle;
    }

    /// Natychmiastowe zatrzymanie muzyki
    pub fn stop(&mut self) {
        self.primary_track = None;
        self.primary_volume = 0.0;
        self.primary_loop = false;
        
        self.secondary_track = None;
        self.secondary_volume = 0.0;
        
        self.transition_state = TransitionState::Idle;
    }

    /// Rozpoczecie odtwarzania z plynnego pojawiania sie
    pub fn play_with_fade_in(&mut self, track: &str, duration: f32, looped: bool) {
        if duration <= 0.0 {
            self.play(track, looped);
            return;
        }

        self.primary_track = Some(track.to_string());
        self.primary_volume = 0.0; // Start od zera
        self.primary_loop = looped;
        
        self.secondary_track = None;
        self.secondary_volume = 0.0;
        
        self.transition_state = TransitionState::FadeIn {
            duration,
            elapsed: 0.0,
        };
    }

    /// Zatrzymanie muzyki z plynnym zanikaniem
    pub fn stop_with_fade_out(&mut self, duration: f32) {
        if self.primary_track.is_none() {
            return;
        }

        if duration <= 0.0 {
            self.stop();
            return;
        }

        self.transition_state = TransitionState::FadeOut {
            duration,
            elapsed: 0.0,
        };
    }

    /// Plynne przejscie miedzy aktualna sciezka a nowa
    pub fn crossfade_to(&mut self, new_track: &str, duration: f32, looped: bool) {
        if self.primary_track.is_none() {
            // Jesli nie ma aktualnej sciezki, po prostu zrob FadeIn
            self.play_with_fade_in(new_track, duration, looped);
            return;
        }

        if let Some(ref current_track) = self.primary_track {
            if current_track == new_track {
                // Ta sama sciezka jest juz grana, ignorujemy zeby uniknac ciecia
                return;
            }
        }

        if duration <= 0.0 {
            self.play(new_track, looped);
            return;
        }

        // Stara sciezka staje sie sciezka poboczna i bedzie zanikac
        self.secondary_track = self.primary_track.take();
        self.secondary_volume = self.primary_volume;

        // Nowa sciezka staje sie glowna i bedzie sie pojawiac
        self.primary_track = Some(new_track.to_string());
        self.primary_volume = 0.0;
        self.primary_loop = looped;

        self.transition_state = TransitionState::Crossfade {
            duration,
            elapsed: 0.0,
        };
    }

    /// Aktualizuje stany przejsc (musi byc wywolywane w petli gry)
    pub fn update(&mut self, delta_time: f32) {
        let mut new_state = self.transition_state.clone();

        match &mut new_state {
            TransitionState::Idle => {}
            TransitionState::FadeIn { duration, elapsed } => {
                *elapsed += delta_time;
                if *elapsed >= *duration {
                    self.primary_volume = 1.0;
                    new_state = TransitionState::Idle;
                } else {
                    self.primary_volume = *elapsed / *duration;
                }
            }
            TransitionState::FadeOut { duration, elapsed } => {
                *elapsed += delta_time;
                if *elapsed >= *duration {
                    self.stop(); // Zeruje wszystko
                    new_state = TransitionState::Idle;
                } else {
                    self.primary_volume = 1.0 - (*elapsed / *duration);
                }
            }
            TransitionState::Crossfade { duration, elapsed } => {
                *elapsed += delta_time;
                if *elapsed >= *duration {
                    self.primary_volume = 1.0;
                    
                    // Usuwamy sciezke poboczna
                    self.secondary_track = None;
                    self.secondary_volume = 0.0;
                    
                    new_state = TransitionState::Idle;
                } else {
                    let progress = *elapsed / *duration;
                    self.primary_volume = progress;
                    // Prosta interpolacja zanikania, zazwyczaj krzywa mocy (sqrt) jest lepsza dla dzwieku, ale liniowa tez wystarczy do podstaw
                    self.secondary_volume = 1.0 - progress; 
                }
            }
        }

        self.transition_state = new_state;
    }
}
