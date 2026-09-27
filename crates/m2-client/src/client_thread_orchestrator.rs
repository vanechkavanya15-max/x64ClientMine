use std::collections::VecDeque;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;

#[derive(Debug, Clone, PartialEq)]
pub enum PacketCmd {
    Connect { ip: String, port: u16 },
    SendPacket(Vec<u8>),
    Disconnect,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PacketEvent {
    Connected,
    PacketReceived(Vec<u8>),
    Disconnected,
    Error(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum AssetLoadRequest {
    LoadTexture(String),
    LoadModel(String),
    Terminate,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AssetLoadEvent {
    TextureLoaded {
        path: String,
        data: Vec<u8>,
    },
    ModelLoaded {
        path: String,
        vertices: usize,
    },
    LoadFailed {
        path: String,
        reason: String,
    },
}

pub struct ThreadChannels {
    pub net_tx: Sender<PacketCmd>,
    pub net_rx: Receiver<PacketEvent>,
    pub asset_tx: Sender<AssetLoadRequest>,
    pub asset_rx: Receiver<AssetLoadEvent>,
}

pub struct ThreadPoolController {
    channels: ThreadChannels,
    network_thread: Option<JoinHandle<()>>,
    asset_threads: Vec<JoinHandle<()>>,
    
    // Internal channels for worker threads
    asset_worker_tx: Sender<AssetLoadRequest>,
}

impl ThreadPoolController {
    pub fn new() -> Self {
        let (net_cmd_tx, net_cmd_rx) = mpsc::channel();
        let (net_event_tx, net_event_rx) = mpsc::channel();
        
        let (asset_req_tx, asset_req_rx) = mpsc::channel();
        let (asset_event_tx, asset_event_rx) = mpsc::channel();
        
        let channels = ThreadChannels {
            net_tx: net_cmd_tx.clone(),
            net_rx: net_event_rx,
            asset_tx: asset_req_tx.clone(),
            asset_rx: asset_event_rx,
        };

        Self {
            channels,
            network_thread: None,
            asset_threads: Vec::new(),
            asset_worker_tx: asset_req_tx,
        }
    }

    pub fn get_channels(&self) -> &ThreadChannels {
        &self.channels
    }
    
    pub fn get_net_tx(&self) -> Sender<PacketCmd> {
        self.channels.net_tx.clone()
    }

    pub fn get_asset_tx(&self) -> Sender<AssetLoadRequest> {
        self.channels.asset_tx.clone()
    }

    pub fn spawn_network_thread(&mut self) -> Result<(), String> {
        if self.network_thread.is_some() {
            return Err("Network thread is already running".to_string());
        }

        let (net_cmd_tx, net_cmd_rx) = mpsc::channel::<PacketCmd>();
        let net_event_tx = self.channels.net_tx.clone();
        
        // Actually we need to replace the channels to wire them correctly
        // We need a channel to receive commands FROM the main thread
        let (main_to_net_tx, main_to_net_rx) = mpsc::channel::<PacketCmd>();
        // We need a channel to send events TO the main thread
        let (net_to_main_tx, net_to_main_rx) = mpsc::channel::<PacketEvent>();
        
        self.channels.net_tx = main_to_net_tx;
        self.channels.net_rx = net_to_main_rx;

        let handle = thread::spawn(move || {
            let mut stream: Option<TcpStream> = None;
            
            loop {
                // Non-blocking check for commands
                match main_to_net_rx.try_recv() {
                    Ok(PacketCmd::Connect { ip, port }) => {
                        let addr = format!("{}:{}", ip, port);
                        match TcpStream::connect_timeout(&addr.parse().unwrap(), Duration::from_millis(500)) {
                            Ok(s) => {
                                s.set_nonblocking(true).unwrap();
                                stream = Some(s);
                                let _ = net_to_main_tx.send(PacketEvent::Connected);
                            }
                            Err(e) => {
                                let _ = net_to_main_tx.send(PacketEvent::Error(format!("Connection failed: {}", e)));
                            }
                        }
                    }
                    Ok(PacketCmd::SendPacket(data)) => {
                        if let Some(ref mut s) = stream {
                            if let Err(e) = s.write_all(&data) {
                                let _ = net_to_main_tx.send(PacketEvent::Error(format!("Write error: {}", e)));
                            }
                        } else {
                            let _ = net_to_main_tx.send(PacketEvent::Error("Not connected".to_string()));
                        }
                    }
                    Ok(PacketCmd::Disconnect) => {
                        stream = None;
                        let _ = net_to_main_tx.send(PacketEvent::Disconnected);
                        break;
                    }
                    Err(mpsc::TryRecvError::Empty) => {}
                    Err(mpsc::TryRecvError::Disconnected) => {
                        break; // Main thread dropped the sender
                    }
                }

                // Non-blocking read from network if connected
                if let Some(ref mut s) = stream {
                    let mut buffer = [0u8; 1024];
                    match s.read(&mut buffer) {
                        Ok(0) => {
                            // Connection closed
                            stream = None;
                            let _ = net_to_main_tx.send(PacketEvent::Disconnected);
                        }
                        Ok(n) => {
                            let _ = net_to_main_tx.send(PacketEvent::PacketReceived(buffer[..n].to_vec()));
                        }
                        Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                            // No data available right now, this is expected
                        }
                        Err(e) => {
                            stream = None;
                            let _ = net_to_main_tx.send(PacketEvent::Error(format!("Read error: {}", e)));
                            let _ = net_to_main_tx.send(PacketEvent::Disconnected);
                        }
                    }
                }

                thread::sleep(Duration::from_millis(1)); // Yield to prevent 100% CPU
            }
        });

        self.network_thread = Some(handle);
        Ok(())
    }

    pub fn spawn_background_asset_loader(&mut self, worker_count: usize) -> Result<(), String> {
        if !self.asset_threads.is_empty() {
            return Err("Asset loaders are already running".to_string());
        }

        let (main_to_asset_tx, main_to_asset_rx) = mpsc::channel::<AssetLoadRequest>();
        let (asset_to_main_tx, asset_to_main_rx) = mpsc::channel::<AssetLoadEvent>();

        self.channels.asset_tx = main_to_asset_tx.clone();
        self.channels.asset_rx = asset_to_main_rx;
        self.asset_worker_tx = main_to_asset_tx;
        
        let shared_receiver = Arc::new(Mutex::new(main_to_asset_rx));

        for _ in 0..worker_count {
            let rx = Arc::clone(&shared_receiver);
            let tx = asset_to_main_tx.clone();

            let handle = thread::spawn(move || {
                loop {
                    let request = {
                        let rx_lock = rx.lock().unwrap();
                        rx_lock.recv()
                    };

                    match request {
                        Ok(AssetLoadRequest::Terminate) => {
                            break;
                        }
                        Ok(AssetLoadRequest::LoadTexture(path)) => {
                            // Simulate loading time
                            thread::sleep(Duration::from_millis(10));
                            // Mock data based on path length
                            let data = vec![0u8; path.len() * 1024]; 
                            let _ = tx.send(AssetLoadEvent::TextureLoaded { path, data });
                        }
                        Ok(AssetLoadRequest::LoadModel(path)) => {
                            // Simulate loading time
                            thread::sleep(Duration::from_millis(20));
                            let vertices = path.len() * 100;
                            let _ = tx.send(AssetLoadEvent::ModelLoaded { path, vertices });
                        }
                        Err(_) => {
                            break; // Channel closed
                        }
                    }
                }
            });
            
            self.asset_threads.push(handle);
        }

        Ok(())
    }

    pub fn synchronize_main_thread_events(&mut self) -> (Vec<PacketEvent>, Vec<AssetLoadEvent>) {
        let mut net_events = Vec::new();
        while let Ok(event) = self.channels.net_rx.try_recv() {
            net_events.push(event);
        }

        let mut asset_events = Vec::new();
        while let Ok(event) = self.channels.asset_rx.try_recv() {
            asset_events.push(event);
        }

        (net_events, asset_events)
    }
    
    pub fn stop_all(&mut self) {
        let _ = self.channels.net_tx.send(PacketCmd::Disconnect);
        
        for _ in 0..self.asset_threads.len() {
            let _ = self.channels.asset_tx.send(AssetLoadRequest::Terminate);
        }
        
        if let Some(handle) = self.network_thread.take() {
            let _ = handle.join();
        }
        
        for handle in self.asset_threads.drain(..) {
            let _ = handle.join();
        }
    }
}

impl Drop for ThreadPoolController {
    fn drop(&mut self) {
        self.stop_all();
    }
}
