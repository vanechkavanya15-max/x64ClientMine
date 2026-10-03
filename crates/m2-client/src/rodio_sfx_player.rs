use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use std::collections::HashMap;
use std::io::Cursor;
use std::sync::{Arc, Mutex};

pub struct RodioSfxPlayer {
    _stream: Option<OutputStream>,
    stream_handle: Option<OutputStreamHandle>,
    sinks: Arc<Mutex<HashMap<u32, Sink>>>,
    next_sink_id: u32,
    volume: f32,
}

impl RodioSfxPlayer {
    pub fn new() -> Result<Self, String> {
        let (stream, stream_handle) = match OutputStream::try_default() {
            Ok((s, h)) => (Some(s), Some(h)),
            Err(e) => {
                eprintln!("Warning: Failed to initialize audio output: {}", e);
                (None, None)
            }
        };

        Ok(Self {
            _stream: stream,
            stream_handle,
            sinks: Arc::new(Mutex::new(HashMap::new())),
            next_sink_id: 1,
            volume: 1.0,
        })
    }

    pub fn play_wav_buffer(&mut self, buffer: &[u8]) -> Option<u32> {
        let handle = self.stream_handle.as_ref()?;
        
        let cursor = Cursor::new(buffer.to_vec());
        let decoder = Decoder::new(cursor).ok()?;
        
        let sink = Sink::try_new(handle).ok()?;
        sink.set_volume(self.volume);
        sink.append(decoder);
        
        // Remove empty sinks to avoid memory leaks
        if let Ok(mut sinks_lock) = self.sinks.lock() {
            sinks_lock.retain(|_, s| !s.empty());
            
            let id = self.next_sink_id;
            self.next_sink_id = self.next_sink_id.wrapping_add(1);
            
            sinks_lock.insert(id, sink);
            Some(id)
        } else {
            None
        }
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume.clamp(0.0, 1.0);
        if let Ok(sinks_lock) = self.sinks.lock() {
            for sink in sinks_lock.values() {
                sink.set_volume(self.volume);
            }
        }
    }
    
    pub fn stop_all(&mut self) {
        if let Ok(mut sinks_lock) = self.sinks.lock() {
            for sink in sinks_lock.values() {
                sink.stop();
            }
            sinks_lock.clear();
        }
    }
    
    pub fn stop(&mut self, id: u32) {
        if let Ok(mut sinks_lock) = self.sinks.lock() {
            if let Some(sink) = sinks_lock.remove(&id) {
                sink.stop();
            }
        }
    }
    
    pub fn is_playing(&self, id: u32) -> bool {
        if let Ok(sinks_lock) = self.sinks.lock() {
            sinks_lock.get(&id).map(|s| !s.empty()).unwrap_or(false)
        } else {
            false
        }
    }
}
