use crate::client_engine_main::{ClientEngine, WindowConfig, EnginePhase, EngineMessage};

pub struct HeadlessSimulator {
    pub engine: ClientEngine,
}

impl HeadlessSimulator {
    pub fn new() -> Self {
        Self {
            engine: ClientEngine::new(WindowConfig {
                width: 800,
                height: 600,
                is_windowed: false,
                vsync: false,
            }),
        }
    }

    pub fn initialize(&mut self) -> Result<(), String> {
        self.engine.initialize()
    }

    pub fn shutdown(&mut self) {
        self.engine.shutdown();
    }

    pub fn switch_phase(&mut self, next: EnginePhase) {
        self.engine.switch_phase(next);
    }

    pub fn push_message(&mut self, message: EngineMessage) {
        self.engine.push_message(message);
    }

    pub fn run_headless_frame(&mut self) {
        self.engine.run_frame();
    }

    pub fn is_running(&self) -> bool {
        self.engine.is_running
    }

    pub fn current_phase(&self) -> EnginePhase {
        self.engine.current_phase
    }

    pub fn frame_count(&self) -> u64 {
        self.engine.frame_count
    }

    pub fn window(&self) -> WindowConfig {
        self.engine.window
    }
}
