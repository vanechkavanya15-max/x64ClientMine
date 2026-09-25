//! Modul motion_blender dla skrzynki m2-client-granny
//! Plynne blendowanie i crossfading pomiedzy stanami animacji
//! Plynne przejscia z biegu do ataku bez szarpniec, czas przenikania 0.1s-0.2s

/// Struktura reprezentujaca stan pojedynczej animacji
#[derive(Debug, Clone, Default, PartialEq)]
pub struct AnimationState {
    pub name: String,
    pub weight: f32,
    pub time: f32,
}

impl AnimationState {
    pub fn new(name: &str, weight: f32) -> Self {
        Self {
            name: name.to_string(),
            weight,
            time: 0.0,
        }
    }
}

/// Struktura reprezentujaca przejscie pomiedzy dwoma stanami animacji
#[derive(Debug, Clone)]
pub struct Transition {
    pub source: AnimationState,
    pub target: AnimationState,
    pub duration: f32,
    pub elapsed: f32,
}

/// Glowna struktura implementacyjna obslugujaca blendowanie animacji
#[derive(Debug, Clone, Default)]
pub struct MotionBlender {
    pub initialized: bool,
    pub current_state: Option<AnimationState>,
    pub transition: Option<Transition>,
}

impl MotionBlender {
    pub fn new() -> Self {
        Self { 
            initialized: true,
            current_state: None,
            transition: None,
        }
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }

    /// Twarde ustawienie stanu animacji bez blendowania
    pub fn set_state(&mut self, name: &str) {
        self.current_state = Some(AnimationState::new(name, 1.0));
        self.transition = None;
    }

    /// Inicjalizuje plynne przejscie do nowej animacji
    pub fn blend_to_state(&mut self, name: &str, duration: f32) -> Result<(), &'static str> {
        if duration <= 0.0 {
            return Err("Czas przejscia musi byc wiekszy niz zero");
        }
        
        let target_state = AnimationState::new(name, 0.0);

        if let Some(ref current) = self.current_state {
            if self.transition.is_none() {
                self.transition = Some(Transition {
                    source: current.clone(),
                    target: target_state,
                    duration,
                    elapsed: 0.0,
                });
                self.current_state = None;
            } else {
                // Jesli juz jestesmy w trakcie przejscia, uzywamy aktualnego celu jako zrodla nowego przejscia
                if let Some(ref mut t) = self.transition {
                    t.source = t.target.clone();
                    t.source.weight = 1.0;
                    t.target = target_state;
                    t.elapsed = 0.0;
                    t.duration = duration;
                }
            }
        } else {
            // Jesli brak stanu poczatkowego, ustawiamy twardo
            self.set_state(name);
        }

        Ok(())
    }

    /// Aktualizuje czas (delta_time), przelicza wagi i wymienia stany po zakonczeniu przejscia
    pub fn update(&mut self, delta_time: f32) {
        if let Some(ref mut state) = self.current_state {
            state.time += delta_time;
        }

        if let Some(mut trans) = self.transition.take() {
            trans.elapsed += delta_time;
            trans.source.time += delta_time;
            trans.target.time += delta_time;

            if trans.elapsed >= trans.duration {
                // Przejscie zakonczone, nowy stan to 100% celu
                let mut final_state = trans.target;
                final_state.weight = 1.0;
                self.current_state = Some(final_state);
            } else {
                // Przeliczanie wag od 0.0 do 1.0
                let factor = trans.elapsed / trans.duration;
                trans.source.weight = 1.0 - factor;
                trans.target.weight = factor;
                self.transition = Some(trans);
            }
        }
    }
}
