use m2_client::client_thread_orchestrator::{
    AssetLoadEvent, AssetLoadRequest, PacketCmd, PacketEvent, ThreadPoolController,
};
use std::net::TcpListener;
use std::thread;
use std::time::Duration;

#[test]
fn test_orchestrator_initialization() {
    let mut orchestrator = ThreadPoolController::new();
    // Spawn it first so channels are initialized and receiver isn't disconnected
    orchestrator.spawn_network_thread().unwrap();
    let net_tx = orchestrator.get_net_tx();

    // Verify channels are set up correctly by sending/receiving on them directly
    net_tx
        .send(PacketCmd::Connect {
            ip: "127.0.0.1".to_string(),
            port: 8080,
        })
        .unwrap();
}

#[test]
fn test_network_thread_connection_failure() {
    let mut orchestrator = ThreadPoolController::new();
    orchestrator.spawn_network_thread().unwrap();

    let net_tx = orchestrator.get_net_tx();

    // Connect to a blackhole IP/port
    net_tx
        .send(PacketCmd::Connect {
            ip: "127.0.0.1".to_string(),
            port: 1, // Port 1 is likely closed
        })
        .unwrap();

    // Wait for response
    thread::sleep(Duration::from_millis(600));

    let (net_events, _) = orchestrator.synchronize_main_thread_events();

    assert!(
        net_events.iter().any(|e| matches!(e, PacketEvent::Error(_))),
        "Expected a connection error event"
    );
}

#[test]
fn test_background_asset_loader() {
    let mut orchestrator = ThreadPoolController::new();
    orchestrator.spawn_background_asset_loader(2).unwrap();

    let asset_tx = orchestrator.get_asset_tx();

    asset_tx
        .send(AssetLoadRequest::LoadTexture("test/path.png".to_string()))
        .unwrap();
    asset_tx
        .send(AssetLoadRequest::LoadModel("test/model.gr2".to_string()))
        .unwrap();

    // Give it time to load
    thread::sleep(Duration::from_millis(100));

    let (_, asset_events) = orchestrator.synchronize_main_thread_events();

    assert_eq!(asset_events.len(), 2, "Expected 2 asset events");

    let has_texture = asset_events.iter().any(|e| {
        if let AssetLoadEvent::TextureLoaded { path, data } = e {
            path == "test/path.png" && data.len() == "test/path.png".len() * 1024
        } else {
            false
        }
    });

    let has_model = asset_events.iter().any(|e| {
        if let AssetLoadEvent::ModelLoaded { path, vertices } = e {
            path == "test/model.gr2" && *vertices == "test/model.gr2".len() * 100
        } else {
            false
        }
    });

    assert!(has_texture, "Texture load event missing or incorrect");
    assert!(has_model, "Model load event missing or incorrect");
}

#[test]
fn test_main_thread_zero_lag_synchronization() {
    let mut orchestrator = ThreadPoolController::new();
    orchestrator.spawn_network_thread().unwrap();
    orchestrator.spawn_background_asset_loader(1).unwrap();

    // Should return immediately without blocking
    let start = std::time::Instant::now();
    let (net_events, asset_events) = orchestrator.synchronize_main_thread_events();
    let elapsed = start.elapsed();

    assert!(elapsed.as_millis() < 5, "Synchronization should not block");
    assert_eq!(net_events.len(), 0);
    assert_eq!(asset_events.len(), 0);
}

#[test]
fn test_network_thread_disconnect() {
    let mut orchestrator = ThreadPoolController::new();
    orchestrator.spawn_network_thread().unwrap();

    let net_tx = orchestrator.get_net_tx();

    // Start a mock server
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();

    net_tx
        .send(PacketCmd::Connect {
            ip: "127.0.0.1".to_string(),
            port,
        })
        .unwrap();

    thread::sleep(Duration::from_millis(100));
    
    // Accept connection
    let _ = listener.accept();

    net_tx.send(PacketCmd::Disconnect).unwrap();

    thread::sleep(Duration::from_millis(100));

    let (net_events, _) = orchestrator.synchronize_main_thread_events();
    assert!(
        net_events.contains(&PacketEvent::Disconnected),
        "Expected Disconnected event"
    );
}

#[test]
fn test_worker_termination() {
    let mut orchestrator = ThreadPoolController::new();
    orchestrator.spawn_background_asset_loader(2).unwrap();

    // Explicitly call drop which calls stop_all
    drop(orchestrator);

    // If it doesn't hang, it terminated properly
}
