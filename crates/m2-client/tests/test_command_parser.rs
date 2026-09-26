use m2_client::command_parser::{dispatch_command, parse_server_command, tokenize, GameStateHandler, ServerCommand};

#[derive(Default)]
struct TestHandler {
    is_day: Option<bool>,
    refine_successes: u32,
    refine_failures: u32,
    unknown_commands: Vec<String>,
}

impl GameStateHandler for TestHandler {
    fn set_day_mode(&mut self, is_day: bool) {
        self.is_day = Some(is_day);
    }

    fn on_refine_succeeded(&mut self) {
        self.refine_successes += 1;
    }

    fn on_refine_failed(&mut self) {
        self.refine_failures += 1;
    }

    fn on_unknown_command(&mut self, cmd: &str) {
        self.unknown_commands.push(cmd.to_string());
    }
}

#[test]
fn test_tokenize() {
    assert_eq!(tokenize("day"), vec!["day"]);
    assert_eq!(tokenize("  night  "), vec!["night"]);
    assert_eq!(
        tokenize("cube info 10 20 \"this is a test\""),
        vec!["cube", "info", "10", "20", "this is a test"]
    );
    assert_eq!(
        tokenize("gift   \"multiple  spaces\"  end"),
        vec!["gift", "multiple  spaces", "end"]
    );
    assert_eq!(
        tokenize("RefineSuceeded"),
        vec!["RefineSuceeded"]
    );
    assert_eq!(
        tokenize(""),
        Vec::<String>::new()
    );
}

#[test]
fn test_parse_server_command() {
    assert_eq!(parse_server_command("day"), Ok(ServerCommand::Day));
    assert_eq!(parse_server_command("DAY"), Ok(ServerCommand::Day));
    assert_eq!(parse_server_command("night"), Ok(ServerCommand::Night));
    assert_eq!(parse_server_command("NIGHT"), Ok(ServerCommand::Night));
    assert_eq!(parse_server_command("RefineSuceeded"), Ok(ServerCommand::RefineSucceeded));
    assert_eq!(parse_server_command("refinesucceeded"), Ok(ServerCommand::RefineSucceeded));
    assert_eq!(parse_server_command("RefineFailed"), Ok(ServerCommand::RefineFailed));
    assert_eq!(parse_server_command("unknowncmd"), Ok(ServerCommand::Unknown("unknowncmd".to_string())));
    
    assert!(parse_server_command("").is_err());
    assert!(parse_server_command("   ").is_err());
}

#[test]
fn test_dispatch_day_night() {
    let mut handler = TestHandler::default();
    
    dispatch_command(&mut handler, parse_server_command("day").unwrap());
    assert_eq!(handler.is_day, Some(true));
    
    dispatch_command(&mut handler, parse_server_command("night").unwrap());
    assert_eq!(handler.is_day, Some(false));
}

#[test]
fn test_dispatch_refine() {
    let mut handler = TestHandler::default();
    
    dispatch_command(&mut handler, parse_server_command("RefineSuceeded").unwrap());
    assert_eq!(handler.refine_successes, 1);
    assert_eq!(handler.refine_failures, 0);
    
    dispatch_command(&mut handler, parse_server_command("RefineFailed").unwrap());
    assert_eq!(handler.refine_successes, 1);
    assert_eq!(handler.refine_failures, 1);
}

#[test]
fn test_dispatch_unknown() {
    let mut handler = TestHandler::default();
    
    dispatch_command(&mut handler, parse_server_command("random_command_123").unwrap());
    assert_eq!(handler.unknown_commands, vec!["random_command_123"]);
}
