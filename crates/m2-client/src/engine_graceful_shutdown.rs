use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::collections::VecDeque;

#[derive(Debug, PartialEq)]
pub enum ShutdownError {
    ThreadJoinError,
    QueueLockError,
}

pub struct SocketQueues {
    pub send_queue: Mutex<VecDeque<Vec<u8>>>,
    pub recv_queue: Mutex<VecDeque<Vec<u8>>>,
}

impl SocketQueues {
    pub fn new() -> Self {
        Self {
            send_queue: Mutex::new(VecDeque::new()),
            recv_queue: Mutex::new(VecDeque::new()),
        }
    }

    pub fn clear(&self) -> Result<(), ShutdownError> {
        let mut had_error = false;
        if let Ok(mut send) = self.send_queue.lock() {
            send.clear();
        } else {
            had_error = true;
        }

        if let Ok(mut recv) = self.recv_queue.lock() {
            recv.clear();
        } else {
            had_error = true;
        }

        if had_error {
            Err(ShutdownError::QueueLockError)
        } else {
            Ok(())
        }
    }
}

pub struct EngineShutdownManager {
    threads: Vec<JoinHandle<()>>,
    socket_queues: Arc<SocketQueues>,
    wgpu_device: Option<wgpu::Device>,
}

impl EngineShutdownManager {
    pub fn new(socket_queues: Arc<SocketQueues>, wgpu_device: Option<wgpu::Device>) -> Self {
        Self {
            threads: Vec::new(),
            socket_queues,
            wgpu_device,
        }
    }

    pub fn register_thread(&mut self, handle: JoinHandle<()>) {
        self.threads.push(handle);
    }

    pub fn shutdown(mut self) -> Result<(), ShutdownError> {
        // Zwalnianie kolejek socketu
        let queue_res = self.socket_queues.clear();

        // Sekwencyjne zatrzymywanie watkow, log error and continue if a thread panics.
        let mut had_join_error = false;
        while let Some(handle) = self.threads.pop() {
            if handle.join().is_err() {
                had_join_error = true;
            }
        }

        // Niszczenie urzadzenia wgpu
        if let Some(device) = self.wgpu_device.take() {
            device.destroy();
        }
        
        // Return errors (QueueLockError takes precedence if both fail or we just return the first we find)
        queue_res?;
        if had_join_error {
            return Err(ShutdownError::ThreadJoinError);
        }

        Ok(())
    }
}
