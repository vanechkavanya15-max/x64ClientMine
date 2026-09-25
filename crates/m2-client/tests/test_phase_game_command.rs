use m2_client::phase_game_command::*;

#[test]
fn test_skeleton_phase_game_command() {
    let instance = PhaseGameCommand::new();
    assert!(instance.is_ready());
}

#[test]
fn test_split_token_basic() {
    let line = "cube r_list 20383 4 72723,1/72725,1";
    let tokens = split_token(line);
    assert_eq!(tokens, vec!["cube", "r_list", "20383", "4", "72723,1/72725,1"]);
}

#[test]
fn test_split_token_with_quotes() {
    let line = "say \"Hello World\" 123";
    let tokens = split_token(line);
    assert_eq!(tokens, vec!["say", "Hello World", "123"]);
}

#[test]
fn test_split_token_multiple_spaces() {
    let line = "gift    test  ";
    let tokens = split_token(line);
    assert_eq!(tokens, vec!["gift", "test"]);
}

#[test]
fn test_split_token_unclosed_quote() {
    let line = "say \"Hello World";
    let tokens = split_token(line);
    assert!(tokens.is_empty()); // Zgodnie z C++, bledny format -> false/pusty wynik
}

#[test]
fn test_parse_console_enable() {
    assert_eq!(parse_server_command("ConsoleEnable"), Some(GameCommand::ConsoleEnable));
    assert_eq!(parse_server_command("consoleenable"), Some(GameCommand::ConsoleEnable));
}

#[test]
fn test_parse_quit() {
    assert_eq!(parse_server_command("quit"), Some(GameCommand::Quit));
    assert_eq!(parse_server_command("QUIT"), Some(GameCommand::Quit));
}

#[test]
fn test_parse_gift() {
    assert_eq!(parse_server_command("gift"), Some(GameCommand::Gift));
}

#[test]
fn test_parse_cube() {
    assert_eq!(
        parse_server_command("cube m_info 0 5 125,1|126,2"),
        Some(GameCommand::Cube {
            args: vec!["m_info".to_string(), "0".to_string(), "5".to_string(), "125,1|126,2".to_string()]
        })
    );
}

#[test]
fn test_parse_observer_count() {
    assert_eq!(
        parse_server_command("ObserverCount 123"),
        Some(GameCommand::ObserverCount { count: 123 })
    );
    assert_eq!(parse_server_command("ObserverCount"), None); // Brakuje parametru
    assert_eq!(parse_server_command("ObserverCount abc"), None); // Bledny parametr
    assert_eq!(parse_server_command("ObserverCount 123 456"), None); // Za duzo parametrow
}

#[test]
fn test_parse_observer_mode() {
    assert_eq!(
        parse_server_command("ObserverMode 1"),
        Some(GameCommand::ObserverMode { mode: 1 })
    );
    assert_eq!(parse_server_command("ObserverMode"), None);
}

#[test]
fn test_parse_stone_detect() {
    assert_eq!(
        parse_server_command("StoneDetect 10 2 180.0"),
        Some(GameCommand::StoneDetect { vid: 10, distance: 2, angle: 0.0 })
    );
    assert_eq!(parse_server_command("StoneDetect 10 2"), None);
}

#[test]
fn test_parse_stamina() {
    assert_eq!(
        parse_server_command("StartStaminaConsume 5 100"),
        Some(GameCommand::StartStaminaConsume { consume_per_sec: 5, current_stamina: 100 })
    );
    assert_eq!(
        parse_server_command("StopStaminaConsume 90"),
        Some(GameCommand::StopStaminaConsume { current_stamina: 90 })
    );
}

#[test]
fn test_parse_auth_flags() {
    assert_eq!(parse_server_command("sms"), Some(GameCommand::Sms));
    assert_eq!(parse_server_command("nosms"), Some(GameCommand::NoSms));
    assert_eq!(parse_server_command("mobile_auth"), Some(GameCommand::MobileAuth));
    assert_eq!(
        parse_server_command("messenger_auth testuser"),
        Some(GameCommand::MessengerAuth { name: "testuser".to_string() })
    );
}

#[test]
fn test_parse_combo_blockmode() {
    assert_eq!(
        parse_server_command("combo 1"),
        Some(GameCommand::Combo { flag: 1 })
    );
    assert_eq!(
        parse_server_command("setblockmode 0"),
        Some(GameCommand::SetBlockMode { flag: 0 })
    );
}

#[test]
fn test_parse_emotions() {
    assert_eq!(
        parse_server_command("french_kiss 1 2"),
        Some(GameCommand::Emotion {
            name: "french_kiss".to_string(),
            args: vec!["1".to_string(), "2".to_string()]
        })
    );
    assert_eq!(
        parse_server_command("clap 1"),
        Some(GameCommand::Emotion {
            name: "clap".to_string(),
            args: vec!["1".to_string()]
        })
    );
    assert_eq!(
        parse_server_command("dance3 5"),
        Some(GameCommand::Emotion {
            name: "dance3".to_string(),
            args: vec!["5".to_string()]
        })
    );
}

#[test]
fn test_parse_script_commands() {
    assert_eq!(
        parse_server_command("day"),
        Some(GameCommand::ScriptCommand {
            command: "day".to_string(),
            args: vec![]
        })
    );

    assert_eq!(
        parse_server_command("night"),
        Some(GameCommand::ScriptCommand {
            command: "night".to_string(),
            args: vec![]
        })
    );

    assert_eq!(
        parse_server_command("mall"),
        Some(GameCommand::ScriptCommand {
            command: "mall".to_string(),
            args: vec![]
        })
    );

    assert_eq!(
        parse_server_command("RefineSuceeded"),
        Some(GameCommand::ScriptCommand {
            command: "RefineSuceeded".to_string(),
            args: vec![]
        })
    );

    assert_eq!(
        parse_server_command("RefineFailed"),
        Some(GameCommand::ScriptCommand {
            command: "RefineFailed".to_string(),
            args: vec![]
        })
    );

    assert_eq!(
        parse_server_command("BINARY_Update 100 200"),
        Some(GameCommand::ScriptCommand {
            command: "BINARY_Update".to_string(),
            args: vec!["100".to_string(), "200".to_string()]
        })
    );
}

#[test]
fn test_empty_command() {
    assert_eq!(parse_server_command(""), None);
    assert_eq!(parse_server_command("   "), None);
}
