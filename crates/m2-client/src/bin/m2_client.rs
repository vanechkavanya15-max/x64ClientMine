// m2_client.rs - Main executable binary entry point for Metin2 Pure Rust Client
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let config = m2_client::main_binary_entry::AppConfig::parse_args(&args[1..]);
    let mut engine = m2_client::main_binary_entry::Metin2Engine::new();
    if let Err(e) = engine.run_interactive(config) {
        eprintln!("Bledne zakonczenie dzialania silnika: {:?}", e);
    }
}
