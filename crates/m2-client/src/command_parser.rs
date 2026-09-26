#[derive(Debug, PartialEq)]
pub enum ServerCommand {
    Day,
    Night,
    RefineSucceeded,
    RefineFailed,
    Unknown(String),
}

pub trait GameStateHandler {
    fn set_day_mode(&mut self, is_day: bool);
    fn on_refine_succeeded(&mut self);
    fn on_refine_failed(&mut self);
    fn on_unknown_command(&mut self, cmd: &str);
}

pub fn tokenize(command: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut chars = command.chars().peekable();

    while let Some(&c) = chars.peek() {
        if c.is_whitespace() {
            chars.next();
            continue;
        }

        if c == '"' {
            chars.next(); // consume opening quote
            let mut token = String::new();
            while let Some(c) = chars.next() {
                if c == '"' {
                    break;
                }
                token.push(c);
            }
            tokens.push(token);
        } else {
            let mut token = String::new();
            while let Some(&c) = chars.peek() {
                if c.is_whitespace() || c == '"' {
                    break;
                }
                token.push(c);
                chars.next();
            }
            tokens.push(token);
        }
    }

    tokens
}

pub fn parse_server_command(command: &str) -> Result<ServerCommand, String> {
    let tokens = tokenize(command);
    if tokens.is_empty() {
        return Err("Empty command".to_string());
    }

    let cmd = tokens[0].to_lowercase();
    match cmd.as_str() {
        "day" => Ok(ServerCommand::Day),
        "night" => Ok(ServerCommand::Night),
        "refinesuceeded" | "refinesucceeded" => Ok(ServerCommand::RefineSucceeded), // Handling both correct and typoed versions
        "refinefailed" => Ok(ServerCommand::RefineFailed),
        _ => Ok(ServerCommand::Unknown(tokens[0].clone())),
    }
}

pub fn dispatch_command<H: GameStateHandler>(handler: &mut H, command: ServerCommand) {
    match command {
        ServerCommand::Day => handler.set_day_mode(true),
        ServerCommand::Night => handler.set_day_mode(false),
        ServerCommand::RefineSucceeded => handler.on_refine_succeeded(),
        ServerCommand::RefineFailed => handler.on_refine_failed(),
        ServerCommand::Unknown(cmd) => handler.on_unknown_command(&cmd),
    }
}
