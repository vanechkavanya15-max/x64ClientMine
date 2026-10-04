// Modul game_net_phase_machine.rs
// W pelni zastapiony kod PythonNetworkStream.cpp (faza sieciowa)
// Komentarze wylacznie po polsku bez polskich znakow diakrytycznych.


#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    OffLine = 0,
    Handshake,
    Login,
    Select,
    Loading,
    Game,
    Dead,
    ClientConnecting,
}

#[repr(u32)]
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

pub struct GameNetPhaseMachine {
    pub current_phase: Phase,
    pub changing_phase_time: u32,
    pub phase_windows: [usize; PhaseWindow::Num as usize],
}

impl GameNetPhaseMachine {
    pub fn new() -> Self {
        Self {
            current_phase: Phase::OffLine,
            changing_phase_time: 0,
            phase_windows: [0; PhaseWindow::Num as usize],
        }
    }
}

impl GameNetPhaseMachine {
    pub fn set_phase_window(&mut self, window_type: PhaseWindow, handler: usize) {
        if (window_type as usize) < PhaseWindow::Num as usize {
            self.phase_windows[window_type as usize] = handler;
            // Aktualizacja okien w pythonie powiazana z py_bridge_net (opcjonalna synchronizacja z globalnym m2_net)
        }
    }

    pub fn clear_phase_window(&mut self, window_type: PhaseWindow, handler: usize) {
        if (window_type as usize) < PhaseWindow::Num as usize {
            if self.phase_windows[window_type as usize] == handler {
                self.phase_windows[window_type as usize] = 0;
            }
        }
    }

    fn get_time_msec() -> u32 {
        // Implementacja get time msec, zalezna od systemu (np. Instant::now())
        // Uproszczona wersja uzywajaca czasu systemowego jako mock dla ELTimer_GetMSec()
        // W m2-client normalnie uzylibysmy wewnetrznego zegara, np crate::frame_delta_time_clock
        if let Ok(dur) = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
            dur.as_millis() as u32
        } else {
            0
        }
    }

    pub fn set_offline_phase(&mut self) {
        self.current_phase = Phase::OffLine;
        self.changing_phase_time = Self::get_time_msec();
        // C++: m_strPhase = "OffLine"
    }

    pub fn set_handshake_phase(&mut self) {
        self.current_phase = Phase::Handshake;
        self.changing_phase_time = Self::get_time_msec();
    }

    pub fn set_login_phase(&mut self) {
        self.current_phase = Phase::Login;
        self.changing_phase_time = Self::get_time_msec();

        // Logika LeaveLoginPhase zazwyczaj dzieje sie w C++, ale jako ze to maszyna stanow,
        // przelaczanie fazy wykonuje tylko wewnetrzny update state.
    }

    pub fn set_select_phase(&mut self) {
        self.current_phase = Phase::Select;
        self.changing_phase_time = Self::get_time_msec();
    }

    pub fn set_loading_phase(&mut self) {
        self.current_phase = Phase::Loading;
        self.changing_phase_time = Self::get_time_msec();
    }

    pub fn set_game_phase(&mut self) {
        self.current_phase = Phase::Game;
        self.changing_phase_time = Self::get_time_msec();
    }

    pub fn close_phase(&mut self) {
        self.set_offline_phase();
        // C++ std::fill(m_apoPhaseWnd, m_apoPhaseWnd+PHASE_WINDOW_NUM, NULL);
        self.phase_windows = [0; PhaseWindow::Num as usize];
    }

    pub fn process_phase_packet(&mut self, next_phase: Phase) {
        match next_phase {
            Phase::OffLine | Phase::ClientConnecting => self.set_offline_phase(),
            Phase::Handshake => self.set_handshake_phase(),
            Phase::Login => self.set_login_phase(),
            Phase::Select => self.set_select_phase(),
            Phase::Loading => self.set_loading_phase(),
            Phase::Game => self.set_game_phase(),
            Phase::Dead => {
                // SetGamePhase dla dead
                self.set_game_phase();
            }
        }
    }
}

impl GameNetPhaseMachine {
    pub fn is_packet_allowed(&self, header: u16) -> bool {
        // Zawsze dozwolone w kazdej fazie ze wzgledu na EterLib / ControlPackets:
        const GC_PING: u16 = 0x0007;
        const GC_PHASE: u16 = 0x0008;
        const GC_KEY_CHALLENGE: u16 = 0x000B;
        const GC_KEY_COMPLETE: u16 = 0x000C;

        if header == GC_PING
            || header == GC_PHASE
            || header == GC_KEY_CHALLENGE
            || header == GC_KEY_COMPLETE
        {
            return true;
        }

        // Filtrowanie zalezne od fazy, oparte o Register*Handlers w PythonNetworkStream.cpp
        match self.current_phase {
            Phase::OffLine => {
                // OffLine akceptuje tylko PHASE, PING itp (sprawdzone wyzej)
                false
            }
            Phase::Handshake => {
                // Handshake Phase (zazwyczaj kontrolne pakiety sprawdzane wyzej)
                false
            }
            Phase::Login => {
                const GC_LOGIN_SUCCESS3: u16 = 0x0104;
                const GC_LOGIN_SUCCESS4: u16 = 0x0105;
                const GC_LOGIN_FAILURE: u16 = 0x0106;
                const GC_LOGIN_KEY: u16 = 0x0107;
                const GC_EMPIRE: u16 = 0x0109;

                matches!(
                    header,
                    GC_LOGIN_SUCCESS3
                        | GC_LOGIN_SUCCESS4
                        | GC_LOGIN_FAILURE
                        | GC_LOGIN_KEY
                        | GC_EMPIRE
                )
            }
            Phase::Select => {
                const GC_LOGIN_SUCCESS3: u16 = 0x0104;
                const GC_LOGIN_SUCCESS4: u16 = 0x0105;
                const GC_EMPIRE: u16 = 0x0109;
                const GC_PLAYER_CREATE_SUCCESS: u16 = 0x020C;
                const GC_PLAYER_CREATE_FAILURE: u16 = 0x020D;
                const GC_PLAYER_DELETE_SUCCESS: u16 = 0x020E;
                const GC_PLAYER_DELETE_WRONG_SOCIAL_ID: u16 = 0x020F;
                const GC_CHANGE_NAME: u16 = 0x010C;
                const GC_PLAYER_POINT_CHANGE: u16 = 0x0215; // W kodzie rejestrowany w SelectPhase

                matches!(
                    header,
                    GC_LOGIN_SUCCESS3
                        | GC_LOGIN_SUCCESS4
                        | GC_EMPIRE
                        | GC_PLAYER_CREATE_SUCCESS
                        | GC_PLAYER_CREATE_FAILURE
                        | GC_PLAYER_DELETE_SUCCESS
                        | GC_PLAYER_DELETE_WRONG_SOCIAL_ID
                        | GC_CHANGE_NAME
                        | GC_PLAYER_POINT_CHANGE
                )
            }
            Phase::Loading => {
                // C++: m_loadingHandlers = m_gameHandlers;
                // Zatem akceptujemy wszystko to co gra
                true
            }
            Phase::Game | Phase::Dead => {
                // W GamePhase akceptujemy wiekszosc pakietow serwerowych. W naszym filtrze zwracamy true dla bezpieczenstwa,
                // blokada game packets ma glownie dotyczyc zablokowania ich w Login/Select, zeby nie psuly stanu.
                true
            }
            Phase::ClientConnecting => false,
        }
    }
}
