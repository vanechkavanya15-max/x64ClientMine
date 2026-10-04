//! Ostateczny Orkiestrator Faz Klienta: Handshake -> Login -> Select -> Loading -> Game
//! Zastępuje CPythonNetworkStream::SetPhase* z C++.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClientPhase {
    Offline = 0,
    Handshake,
    Login,
    Select,
    Loading,
    Game,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhaseWindow {
    Logo = 0,
    Login,
    Select,
    Create,
    Load,
    Game,
    Empire,
    Num,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UiWindowManager {
    pub active_windows: [Option<usize>; PhaseWindow::Num as usize],
    pub direct_enter_mode: bool,
    pub selected_empire: u32,
}

impl Default for UiWindowManager {
    fn default() -> Self {
        Self::new()
    }
}

impl UiWindowManager {
    pub fn new() -> Self {
        Self {
            active_windows: [None; PhaseWindow::Num as usize],
            direct_enter_mode: false,
            selected_empire: 0,
        }
    }

    pub fn set_phase_window(&mut self, window: PhaseWindow, handler: usize) {
        if (window as usize) < PhaseWindow::Num as usize {
            self.active_windows[window as usize] = Some(handler);
        }
    }

    pub fn clear_phase_window(&mut self, window: PhaseWindow) {
        if (window as usize) < PhaseWindow::Num as usize {
            self.active_windows[window as usize] = None;
        }
    }

    pub fn clear_all_windows(&mut self) {
        self.active_windows = [None; PhaseWindow::Num as usize];
    }

    pub fn is_window_active(&self, window: PhaseWindow) -> bool {
        if (window as usize) < PhaseWindow::Num as usize {
            self.active_windows[window as usize].is_some()
        } else {
            false
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WgpuPipelineManager {
    pub initialized: bool,
    pub active_pipelines_mask: u32,
}

impl WgpuPipelineManager {
    pub fn new() -> Self {
        Self {
            initialized: false,
            active_pipelines_mask: 0,
        }
    }

    pub fn initialize_game_pipelines(&mut self) {
        self.initialized = true;
        self.active_pipelines_mask = 0xFFFFFFFF; // Wszystkie potoki aktywne w Game
    }

    pub fn clear_pipelines(&mut self) {
        self.initialized = false;
        self.active_pipelines_mask = 0;
    }
}

pub struct GamePhaseOrchestrator {
    pub current_phase: ClientPhase,
    pub ui_manager: UiWindowManager,
    pub wgpu_manager: WgpuPipelineManager,
    pub changing_phase_time: u32,
    pub is_online: bool,
    pub phase_history: Vec<ClientPhase>,
    pub main_actor_vid: Option<u32>,
}

impl GamePhaseOrchestrator {
    pub fn new() -> Self {
        Self {
            current_phase: ClientPhase::Offline,
            ui_manager: UiWindowManager::new(),
            wgpu_manager: WgpuPipelineManager::new(),
            changing_phase_time: 0,
            is_online: false,
            phase_history: vec![ClientPhase::Offline],
            main_actor_vid: None,
        }
    }

    fn get_time_msec() -> u32 {
        if let Ok(dur) = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
            dur.as_millis() as u32
        } else {
            0
        }
    }

    fn transition_to(&mut self, new_phase: ClientPhase) {
        if self.current_phase != new_phase {
            self.current_phase = new_phase;
            self.changing_phase_time = Self::get_time_msec();
            self.phase_history.push(new_phase);
        }
    }

    pub fn set_offline_phase(&mut self) {
        self.transition_to(ClientPhase::Offline);
        self.is_online = false;
        self.wgpu_manager.clear_pipelines();
    }

    pub fn set_handshake_phase(&mut self) {
        self.transition_to(ClientPhase::Handshake);
        self.is_online = true; // Zgodnie z SetGameOnline()
        
        if !self.ui_manager.direct_enter_mode {
            // C++: PyCallClassMemberFunc(m_apoPhaseWnd[PHASE_WINDOW_LOGIN], "OnHandShake", Py_BuildValue("()"));
            self.ui_manager.set_phase_window(PhaseWindow::Login, 1);
        }
    }

    pub fn set_login_phase(&mut self) {
        self.transition_to(ClientPhase::Login);
        if !self.ui_manager.direct_enter_mode {
             // C++: PyCallClassMemberFunc(m_apoPhaseWnd[PHASE_WINDOW_LOGIN], "OnLoginStart", Py_BuildValue("()"));
             self.ui_manager.set_phase_window(PhaseWindow::Login, 1);
        }
    }

    pub fn set_select_phase(&mut self) {
        self.transition_to(ClientPhase::Select);
        if !self.ui_manager.direct_enter_mode {
            if self.ui_manager.selected_empire > 0 {
                self.ui_manager.set_phase_window(PhaseWindow::Select, 1);
            } else {
                self.ui_manager.set_phase_window(PhaseWindow::Empire, 1);
            }
        }
    }

    pub fn set_loading_phase(&mut self) {
        self.transition_to(ClientPhase::Loading);
        self.main_actor_vid = None; // Resetuje gracza, odpowiednik CPythonPlayer::Instance().Clear()
        self.ui_manager.set_phase_window(PhaseWindow::Load, 1);
    }

    pub fn set_game_phase(&mut self) {
        self.transition_to(ClientPhase::Game);
        self.wgpu_manager.initialize_game_pipelines();
        self.ui_manager.set_phase_window(PhaseWindow::Game, 1);
    }

    pub fn close_phase(&mut self) {
        // CPythonNetworkStream::ClosePhase
        // Zwykle oznacza powrót do loginu / offline i wyczyszczenie okien
        self.set_offline_phase();
        self.ui_manager.clear_all_windows();
        
        // C++: PyCallClassMemberFunc(m_poHandler, "SetLoginPhase", Py_BuildValue("()"));
        // W naszym orchestratorze symulujemy to przejscie:
        self.set_login_phase();
    }

    pub fn handle_network_error(&mut self) {
        if self.ui_manager.direct_enter_mode {
            self.close_phase();
        } else {
            // W normalnym trybie pozostajemy w login phase, wysylamy "OnConnectFailure" do UI (co my modelujemy jako powrot do Login z czyszczeniem)
            self.set_offline_phase();
            self.wgpu_manager.clear_pipelines();
            self.set_login_phase();
        }
    }
    
    // Przebieg pelnego logowania
    pub fn execute_phase_progression_step(&mut self) -> bool {
        match self.current_phase {
            ClientPhase::Offline => {
                self.set_handshake_phase();
                true
            }
            ClientPhase::Handshake => {
                self.set_login_phase();
                true
            }
            ClientPhase::Login => {
                if self.ui_manager.direct_enter_mode {
                    // W C++ przy direct enter wysyla paczke logowania i nie przechodzi do select. 
                    // Po zalogowaniu od razu Loading
                    self.set_loading_phase();
                } else {
                    self.set_select_phase();
                }
                true
            }
            ClientPhase::Select => {
                if self.ui_manager.direct_enter_mode {
                    self.set_loading_phase();
                } else {
                    self.set_loading_phase();
                }
                true
            }
            ClientPhase::Loading => {
                self.set_game_phase();
                true
            }
            ClientPhase::Game => {
                // Koniec lancucha startowego
                false
            }
        }
    }
}
