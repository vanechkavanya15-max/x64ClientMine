//! Modul `phase_game_command` — Parser polecen serwerowych ServerCommand
//! Wzorzec referencyjny C++: `source/UserInterface/PythonNetworkStreamCommand.cpp`
//! Opis: Odwzorowanie RecvCommandPacket: parsowanie komend tekstowych od serwera (day, night, mall, gift, RefineSuceeded, RefineFailed, BINARY_Update).

#[derive(Debug, Clone, Default)]
pub struct PhaseGameCommand {
    pub initialized: bool,
}

impl PhaseGameCommand {
    pub fn new() -> Self {
        Self { initialized: true }
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }
}

/// Dzieli ciag znakow na tokeny, obslugujac cytaty (np. `" "`) 
/// tak samo jak funkcja SplitToken w C++.
pub fn split_token(line: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut base_pos = 0;
    
    while base_pos < line.len() {
        let begin_pos_opt = line[base_pos..].find(|c: char| c != ' ').map(|i| base_pos + i);
        
        if let Some(mut begin) = begin_pos_opt {
            let end_pos;
            if line[begin..].starts_with('"') {
                begin += 1;
                if let Some(quote_end) = line[begin..].find('"') {
                    end_pos = begin + quote_end;
                    tokens.push(line[begin..end_pos].to_string());
                    base_pos = end_pos + 1;
                } else {
                    return Vec::new(); 
                }
            } else {
                if let Some(space_pos) = line[begin..].find(' ') {
                    end_pos = begin + space_pos;
                    tokens.push(line[begin..end_pos].to_string());
                    base_pos = end_pos;
                } else {
                    end_pos = line.len();
                    tokens.push(line[begin..end_pos].to_string());
                    base_pos = end_pos;
                }
            }
        } else {
            break;
        }
    }
    
    tokens
}

#[derive(Debug, Clone, PartialEq)]
pub enum GameCommand {
    ConsoleEnable,
    Quit,
    Gift,
    Cube { args: Vec<String> },
    ObserverCount { count: u32 },
    ObserverMode { mode: u32 },
    StoneDetect { vid: u32, distance: u8, angle: f32 },
    StartStaminaConsume { consume_per_sec: u32, current_stamina: u32 },
    StopStaminaConsume { current_stamina: u32 },
    Sms,
    NoSms,
    MobileAuth,
    MessengerAuth { name: String },
    Combo { flag: i32 },
    SetBlockMode { flag: i32 },
    Emotion { name: String, args: Vec<String> },
    ScriptCommand { command: String, args: Vec<String> },
}

pub fn parse_server_command(command: &str) -> Option<GameCommand> {
    if command.eq_ignore_ascii_case("ConsoleEnable") {
        return Some(GameCommand::ConsoleEnable);
    }
    
    let tokens = split_token(command);
    if tokens.is_empty() {
        return None;
    }
    
    let cmd = tokens[0].to_ascii_lowercase();
    
    match cmd.as_str() {
        "quit" => Some(GameCommand::Quit),
        "gift" => Some(GameCommand::Gift),
        "cube" => {
            let mut args = Vec::new();
            if tokens.len() > 1 {
                args = tokens[1..].to_vec();
            }
            Some(GameCommand::Cube { args })
        },
        "observercount" => {
            if tokens.len() == 2 {
                if let Ok(count) = tokens[1].parse::<u32>() {
                    return Some(GameCommand::ObserverCount { count });
                }
            }
            None
        },
        "observermode" => {
            if tokens.len() == 2 {
                if let Ok(mode) = tokens[1].parse::<u32>() {
                    return Some(GameCommand::ObserverMode { mode });
                }
            }
            None
        },
        "stonedetect" => {
            if tokens.len() == 4 {
                if let (Ok(vid), Ok(distance), Ok(angle)) = (
                    tokens[1].parse::<u32>(),
                    tokens[2].parse::<u8>(),
                    tokens[3].parse::<f32>(),
                ) {
                    let f_angle = (540.0 - angle) % 360.0;
                    // Adjust to be strictly positive matching C++ fmod if angle > 540
                    let f_angle = if f_angle < 0.0 { f_angle + 360.0 } else { f_angle };
                    return Some(GameCommand::StoneDetect { vid, distance, angle: f_angle });
                }
            }
            None
        },
        "startstaminaconsume" => {
            if tokens.len() == 3 {
                if let (Ok(consume_per_sec), Ok(current_stamina)) = (
                    tokens[1].parse::<u32>(),
                    tokens[2].parse::<u32>(),
                ) {
                    return Some(GameCommand::StartStaminaConsume { consume_per_sec, current_stamina });
                }
            }
            None
        },
        "stopstaminaconsume" => {
            if tokens.len() == 2 {
                if let Ok(current_stamina) = tokens[1].parse::<u32>() {
                    return Some(GameCommand::StopStaminaConsume { current_stamina });
                }
            }
            None
        },
        "sms" => Some(GameCommand::Sms),
        "nosms" => Some(GameCommand::NoSms),
        "mobile_auth" => Some(GameCommand::MobileAuth),
        "messenger_auth" => {
            if tokens.len() >= 2 {
                Some(GameCommand::MessengerAuth { name: tokens[1].clone() })
            } else {
                None
            }
        },
        "combo" => {
            if tokens.len() == 2 {
                if let Ok(flag) = tokens[1].parse::<i32>() {
                    return Some(GameCommand::Combo { flag });
                }
            }
            None
        },
        "setblockmode" => {
            if tokens.len() == 2 {
                if let Ok(flag) = tokens[1].parse::<i32>() {
                    return Some(GameCommand::SetBlockMode { flag });
                }
            }
            None
        },
        "french_kiss" | "kiss" | "slap" | "clap" | "cheer1" | "cheer2" | "dance1" | "dance2" | "dig_motion" |
        "dance3" | "dance4" | "dance5" | "dance6" | "congratulation" | "forgive" | "angry" | "attractive" | 
        "sad" | "shy" | "cheerup" | "banter" | "joy" => {
            let mut args = Vec::new();
            if tokens.len() > 1 {
                args = tokens[1..].to_vec();
            }
            Some(GameCommand::Emotion { name: tokens[0].clone(), args })
        },
        _ => {
            // Wszystkie inne komendy (np. day, night, mall, RefineSuceeded, itp.) 
            // sa traktowane jako komendy skryptowe przekazywane do Pythona.
            let args = if tokens.len() > 1 { tokens[1..].to_vec() } else { Vec::new() };
            Some(GameCommand::ScriptCommand { 
                command: tokens[0].clone(), 
                args 
            })
        }
    }
}
