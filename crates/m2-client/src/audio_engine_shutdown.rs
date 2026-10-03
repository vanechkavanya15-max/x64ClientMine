use rodio::{OutputStream, Sink, buffer::SamplesBuffer};
use std::sync::{mpsc, Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;
use std::collections::HashMap;

pub enum AudioEngineCmd {
    Play(String, f32),
    Shutdown,
}

pub struct AudioShutdownManager {
    sender: Option<mpsc::Sender<AudioEngineCmd>>,
    thread_handle: Option<JoinHandle<()>>,
    pub is_running: Arc<Mutex<bool>>,
    buffers_cleared: Arc<Mutex<bool>>,
}

impl Default for AudioShutdownManager {
    fn default() -> Self {
        Self::new()
    }
}

impl AudioShutdownManager {
    pub fn new() -> Self {
        Self {
            sender: None,
            thread_handle: None,
            is_running: Arc::new(Mutex::new(false)),
            buffers_cleared: Arc::new(Mutex::new(false)),
        }
    }

    pub fn start(&mut self) -> Result<(), String> {
        let mut running = self.is_running.lock().unwrap();
        if *running {
            return Err("Engine is already running".to_string());
        }
        *running = true;
        drop(running);

        let (tx, rx) = mpsc::channel::<AudioEngineCmd>();
        self.sender = Some(tx);
        
        let is_running_clone = Arc::clone(&self.is_running);
        let buffers_cleared_clone = Arc::clone(&self.buffers_cleared);

        let handle = thread::spawn(move || {
            let mut buffers: HashMap<String, Vec<f32>> = HashMap::new();
            buffers.insert("dummy".to_string(), vec![0.0; 1024]);
            
            // Note: in a headless CI environment, try_default might fail. 
            // We ignore errors to allow the thread loop to process commands gracefully.
            let _audio_context = OutputStream::try_default();

            loop {
                match rx.recv_timeout(Duration::from_millis(50)) {
                    Ok(AudioEngineCmd::Play(_name, _vol)) => {
                        // Playback logic here
                    },
                    Ok(AudioEngineCmd::Shutdown) => {
                        buffers.clear();
                        let mut cleared = buffers_cleared_clone.lock().unwrap();
                        *cleared = true;
                        break;
                    },
                    Err(mpsc::RecvTimeoutError::Timeout) => {
                        let running = is_running_clone.lock().unwrap();
                        if !*running {
                            buffers.clear();
                            let mut cleared = buffers_cleared_clone.lock().unwrap();
                            *cleared = true;
                            break;
                        }
                    },
                    Err(mpsc::RecvTimeoutError::Disconnected) => {
                        buffers.clear();
                        let mut cleared = buffers_cleared_clone.lock().unwrap();
                        *cleared = true;
                        break;
                    }
                }
            }
            
            let mut running = is_running_clone.lock().unwrap();
            *running = false;
        });

        self.thread_handle = Some(handle);
        Ok(())
    }

    pub fn shutdown(&mut self) -> Result<(), String> {
        let mut running = self.is_running.lock().unwrap();
        if !*running {
            return Ok(());
        }
        
        if let Some(sender) = self.sender.take() {
            let _ = sender.send(AudioEngineCmd::Shutdown);
        }
        
        *running = false;
        drop(running);

        if let Some(handle) = self.thread_handle.take() {
            // Join without deadlocking
            let _ = handle.join();
        }

        Ok(())
    }

    pub fn is_active(&self) -> bool {
        let running = self.is_running.lock().unwrap();
        *running
    }

    pub fn are_buffers_cleared(&self) -> bool {
        let cleared = self.buffers_cleared.lock().unwrap();
        *cleared
    }
}
