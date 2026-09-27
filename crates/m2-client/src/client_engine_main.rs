#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WindowConfig {
    pub width: u32,
    pub height: u32,
    pub is_windowed: bool,
    pub vsync: bool,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            width: 800,
            height: 600,
            is_windowed: true,
            vsync: true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnginePhase {
    Init,
    Login,
    Select,
    Loading,
    Game,
    Exit,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EngineMessage {
    Quit,
    PhaseChange(EnginePhase),
    Resize(u32, u32),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FrameEvent {
    PollMessages,
    UpdateNetwork,
    UpdatePhysicsAndActors,
    Render3DWorldAndTerrain,
    Render2DGUI,
    PresentFrame,
}

pub struct ClientEngine {
    pub window: WindowConfig,
    pub is_running: bool,
    pub current_phase: EnginePhase,
    pub message_queue: Vec<EngineMessage>,
    pub frame_events: Vec<FrameEvent>,
    pub frame_count: u64,
    initialized: bool,
}

impl ClientEngine {
    pub fn new(window_config: WindowConfig) -> Self {
        Self {
            window: window_config,
            is_running: false,
            current_phase: EnginePhase::Init,
            message_queue: Vec::new(),
            frame_events: Vec::new(),
            frame_count: 0,
            initialized: false,
        }
    }

    pub fn initialize(&mut self) -> Result<(), String> {
        if self.initialized {
            return Err("Engine is already initialized".to_string());
        }
        
        self.is_running = true;
        self.initialized = true;
        self.current_phase = EnginePhase::Init;
        Ok(())
    }

    pub fn shutdown(&mut self) {
        self.is_running = false;
        self.current_phase = EnginePhase::Exit;
        self.initialized = false;
    }

    pub fn switch_phase(&mut self, next: EnginePhase) {
        if !self.initialized {
            return;
        }
        self.current_phase = next;
    }

    pub fn push_message(&mut self, message: EngineMessage) {
        self.message_queue.push(message);
    }

    pub fn run_frame(&mut self) {
        if !self.initialized || !self.is_running {
            return;
        }

        self.frame_events.clear();

        // 1. Poll Messages
        self.poll_messages();

        // If a message (e.g. Quit) stopped the engine, abort frame early
        if !self.is_running {
            return;
        }

        // 2. Update Network
        self.update_network();

        // 3. Update Physics & Actors
        self.update_physics_and_actors();

        // 4. Render 3D World & Terrain
        self.render_3d_world_and_terrain();

        // 5. Render 2D GUI
        self.render_2d_gui();

        // 6. Present Frame
        self.present_frame();
    }

    fn poll_messages(&mut self) {
        self.frame_events.push(FrameEvent::PollMessages);
        
        // Process all messages in queue
        let messages = std::mem::take(&mut self.message_queue);
        for msg in messages {
            match msg {
                EngineMessage::Quit => {
                    self.shutdown();
                }
                EngineMessage::PhaseChange(phase) => {
                    self.switch_phase(phase);
                }
                EngineMessage::Resize(width, height) => {
                    self.window.width = width;
                    self.window.height = height;
                }
            }
        }
    }

    fn update_network(&mut self) {
        self.frame_events.push(FrameEvent::UpdateNetwork);
    }

    fn update_physics_and_actors(&mut self) {
        self.frame_events.push(FrameEvent::UpdatePhysicsAndActors);
    }

    fn render_3d_world_and_terrain(&mut self) {
        self.frame_events.push(FrameEvent::Render3DWorldAndTerrain);
    }

    fn render_2d_gui(&mut self) {
        self.frame_events.push(FrameEvent::Render2DGUI);
    }

    fn present_frame(&mut self) {
        self.frame_events.push(FrameEvent::PresentFrame);
        self.frame_count += 1;
    }
}
