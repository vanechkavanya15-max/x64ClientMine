use m2_client::main_binary_entry::{AppConfig, Metin2Engine, write_syserr};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = AppConfig::parse_args(args.iter().map(|s| s.as_str()));
    
    let mut engine = Metin2Engine::new();
    
    if let Err(e) = engine.run_interactive(config) {
        let err_msg = format!("Engine failed to start: {:?}", e);
        write_syserr(&err_msg);
        std::process::exit(1);
    }
}
