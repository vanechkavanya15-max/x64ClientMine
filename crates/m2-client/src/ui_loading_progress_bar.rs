#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoadingPhase {
    Init,
    MapLoading,
    ResourceLoading,
    Finishing,
    Complete,
}

pub struct LoadingProgressBar {
    current_progress: f32,
    target_progress: f32,
    animation_speed: f32,
    phase: LoadingPhase,
}

impl Default for LoadingProgressBar {
    fn default() -> Self {
        Self::new()
    }
}

impl LoadingProgressBar {
    pub fn new() -> Self {
        Self {
            current_progress: 0.0,
            target_progress: 0.0,
            animation_speed: 100.0,
            phase: LoadingPhase::Init,
        }
    }

    pub fn with_animation_speed(mut self, speed: f32) -> Self {
        self.animation_speed = speed.max(0.1);
        self
    }

    pub fn set_phase(&mut self, phase: LoadingPhase) {
        self.phase = phase;
        match phase {
            LoadingPhase::Init => self.set_target_progress(5.0),
            LoadingPhase::MapLoading => self.set_target_progress(10.0),
            LoadingPhase::ResourceLoading => self.set_target_progress(50.0),
            LoadingPhase::Finishing => self.set_target_progress(90.0),
            LoadingPhase::Complete => self.set_target_progress(100.0),
        }
    }

    pub fn set_target_progress(&mut self, target: f32) {
        self.target_progress = target.clamp(0.0, 100.0);
        if self.current_progress > self.target_progress {
            self.current_progress = self.target_progress;
        }
    }

    pub fn add_progress(&mut self, amount: f32) {
        self.set_target_progress(self.target_progress + amount);
    }

    pub fn update(&mut self, delta_time: f32) {
        if self.current_progress < self.target_progress {
            self.current_progress += self.animation_speed * delta_time;
            if self.current_progress > self.target_progress {
                self.current_progress = self.target_progress;
            }
        }
    }

    pub fn force_complete(&mut self) {
        self.phase = LoadingPhase::Complete;
        self.target_progress = 100.0;
        self.current_progress = 100.0;
    }

    pub fn get_current_progress(&self) -> f32 {
        self.current_progress
    }

    pub fn get_target_progress(&self) -> f32 {
        self.target_progress
    }

    pub fn get_phase(&self) -> LoadingPhase {
        self.phase
    }

    pub fn is_fully_loaded(&self) -> bool {
        self.current_progress >= 100.0 && self.phase == LoadingPhase::Complete
    }
}
