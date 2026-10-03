use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

use crate::texture_vfs_pipeline::read_vfs_bytes;

#[derive(Debug, Clone, PartialEq)]
pub enum AssetType {
    Texture,
    Model,
}

#[derive(Debug, Clone, PartialEq)]
pub enum VfsAssetLoadRequest {
    Load { path: String, asset_type: AssetType },
    Terminate,
}

#[derive(Debug, Clone, PartialEq)]
pub enum VfsAssetLoadEvent {
    Loaded {
        path: String,
        asset_type: AssetType,
        data: Vec<u8>,
    },
    LoadFailed {
        path: String,
        reason: String,
    },
}

pub struct VfsThreadedAssetLoader {
    request_tx: Sender<VfsAssetLoadRequest>,
    event_rx: Receiver<VfsAssetLoadEvent>,
    worker_threads: Vec<JoinHandle<()>>,
}

impl VfsThreadedAssetLoader {
    pub fn new(worker_count: usize) -> Self {
        let (request_tx, request_rx) = mpsc::channel::<VfsAssetLoadRequest>();
        let (event_tx, event_rx) = mpsc::channel::<VfsAssetLoadEvent>();

        let shared_request_rx = Arc::new(Mutex::new(request_rx));
        let mut worker_threads = Vec::with_capacity(worker_count);

        for _ in 0..worker_count {
            let rx = Arc::clone(&shared_request_rx);
            let tx = event_tx.clone();

            let handle = thread::spawn(move || {
                loop {
                    let request = {
                        let rx_lock = rx.lock().unwrap();
                        rx_lock.recv()
                    };

                    match request {
                        Ok(VfsAssetLoadRequest::Terminate) => {
                            break;
                        }
                        Ok(VfsAssetLoadRequest::Load { path, asset_type }) => {
                            // Prawdziwe ładowanie danych bajt po bajcie z VFS
                            match read_vfs_bytes(&path) {
                                Some(data) => {
                                    let _ = tx.send(VfsAssetLoadEvent::Loaded {
                                        path,
                                        asset_type,
                                        data,
                                    });
                                }
                                None => {
                                    let _ = tx.send(VfsAssetLoadEvent::LoadFailed {
                                        path,
                                        reason: "File not found in VFS or disk".to_string(),
                                    });
                                }
                            }
                        }
                        Err(_) => {
                            // Channel closed
                            break;
                        }
                    }
                }
            });
            worker_threads.push(handle);
        }

        Self {
            request_tx,
            event_rx,
            worker_threads,
        }
    }

    pub fn get_request_sender(&self) -> Sender<VfsAssetLoadRequest> {
        self.request_tx.clone()
    }

    pub fn try_recv_event(&self) -> Option<VfsAssetLoadEvent> {
        self.event_rx.try_recv().ok()
    }
    
    pub fn sync_all_events(&self) -> Vec<VfsAssetLoadEvent> {
        let mut events = Vec::new();
        while let Ok(event) = self.event_rx.try_recv() {
            events.push(event);
        }
        events
    }

    pub fn stop_all(&mut self) {
        for _ in 0..self.worker_threads.len() {
            let _ = self.request_tx.send(VfsAssetLoadRequest::Terminate);
        }

        for handle in self.worker_threads.drain(..) {
            let _ = handle.join();
        }
    }
}

impl Drop for VfsThreadedAssetLoader {
    fn drop(&mut self) {
        self.stop_all();
    }
}
