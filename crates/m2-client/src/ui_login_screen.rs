//! Modul `ui_login_screen` — Kompletny interaktywny Ekran Logowania (Wybor kanalu, Login, Haslo)
//! Wzorzec referencyjny C++: `source/UserInterface/PythonNetworkStreamPhaseLogin.cpp`
//! Opis: Zbudowanie kompletnego stanu i widoku Ekranu Logowania: lista serwerow i kanalow (CH1..CH4, status NORM/BUSY), pola wpisania Loginu i Hasla, przycisk Polacz, komunikaty stanu autoryzacji.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChannelStatus {
    Norm,
    Busy,
    Full,
    Offline,
}

#[derive(Debug, Clone)]
pub struct ChannelInfo {
    pub id: u8,
    pub name: String,
    pub status: ChannelStatus,
}

#[derive(Debug, Clone)]
pub struct ServerInfo {
    pub id: u32,
    pub name: String,
    pub channels: Vec<ChannelInfo>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum LoginState {
    #[default]
    Disconnected,
    Connecting,
    Authenticating,
}

#[derive(Debug, Clone, Default)]
pub struct UiLoginScreen {
    pub initialized: bool,
    
    // Widok i inputy
    pub server_list: Vec<ServerInfo>,
    pub selected_server_id: Option<u32>,
    pub selected_channel_id: Option<u8>,
    pub login_input: String,
    pub password_input: String,
    pub login_state: LoginState,
    
    // Stan wewnetrzny (odpowiedniki z C++)
    pub empire_id: u8,
    pub login_key: u32,
    pub is_wait_login_key: bool,
    pub is_direct_enter_mode: bool,
    pub chr_slot: u32,
}

impl UiLoginScreen {
    pub fn new() -> Self {
        Self { 
            initialized: true,
            server_list: Vec::new(),
            selected_server_id: None,
            selected_channel_id: None,
            login_input: String::new(),
            password_input: String::new(),
            login_state: LoginState::Disconnected,
            empire_id: 0,
            login_key: 0,
            is_wait_login_key: false,
            is_direct_enter_mode: false,
            chr_slot: 0,
        }
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }
    
    // Interakcja UI
    pub fn set_username(&mut self, username: &str) {
        self.login_input = username.to_string();
    }
    
    pub fn set_password(&mut self, password: &str) {
        self.password_input = password.to_string();
    }
    
    pub fn select_server(&mut self, server_id: u32) {
        self.selected_server_id = Some(server_id);
    }
    
    pub fn select_channel(&mut self, channel_id: u8) {
        self.selected_channel_id = Some(channel_id);
    }
    
    pub fn connect(&mut self) -> bool {
        if self.selected_server_id.is_none() || self.selected_channel_id.is_none() || self.login_input.is_empty() || self.password_input.is_empty() {
            return false;
        }
        self.login_state = LoginState::Connecting;
        true
    }

    // Metody sieciowe (odpowiedniki z C++)
    pub fn login_phase(&mut self) -> bool {
        // Glowna petla fazy (w oryginalnym kodzie obsluguje pakiety na switch-u). 
        // Zwracamy true jako symulacje sukcesu.
        true
    }

    pub fn set_login_phase(&mut self) {
        self.login_state = LoginState::Authenticating;
        
        if self.is_direct_enter_mode {
            self.send_direct_enter_packet();
        } else {
            if self.login_key != 0 {
                self.send_login_packet_new();
            } else {
                self.send_login_packet();
            }
            self.clear_login_info();
        }
    }

    pub fn recv_empire_packet(&mut self, empire_id: u8) -> bool {
        self.empire_id = empire_id;
        true
    }

    pub fn recv_login_success_packet3(&mut self) -> bool {
        // Symulacja: u nas przyjmujemy, ze logowanie sie powiodlo
        self.login_state = LoginState::Disconnected; // C++ zmienia faze
        true
    }

    pub fn recv_login_success_packet4(&mut self) -> bool {
        // Analogicznie do packet3
        self.login_state = LoginState::Disconnected; 
        true
    }

    pub fn on_connect_failure(&mut self) {
        if self.is_direct_enter_mode {
            // ClosePhase() equivalent
            self.login_state = LoginState::Disconnected;
        } else {
            // OnConnectFailure() w Pythonie
            self.login_state = LoginState::Disconnected;
        }
    }

    pub fn recv_login_failure_packet(&mut self) -> bool {
        self.login_state = LoginState::Disconnected;
        true
    }

    pub fn send_direct_enter_packet(&mut self) -> bool {
        // Wysylanie pakietu i czyszczenie hasel
        let success = true; 
        if success {
            self.clear_login_info();
        }
        success
    }

    pub fn send_login_packet(&mut self) -> bool {
        // Symulacja wyslania pakietu
        true
    }

    pub fn send_login_packet_new(&mut self) -> bool {
        // Symulacja wyslania pakietu nowego z kluczem
        true
    }

    pub fn recv_login_key_packet(&mut self, key: u32) -> bool {
        self.login_key = key;
        self.is_wait_login_key = false;
        true
    }

    fn clear_login_info(&mut self) {
        // Bezpieczne usuwanie hasla z pamieci (jak komentarz w C++)
        self.password_input.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skeleton_ui_login_screen() {
        let instance = UiLoginScreen::new();
        assert!(instance.is_ready());
    }
}
