use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use std::fs::File;
use std::io::{BufReader, Cursor};
use std::path::Path;

pub struct BgmStreamer {
    _stream: OutputStream,
    stream_handle: OutputStreamHandle,
    sink: Sink,
    current_volume: f32,
}

impl BgmStreamer {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let (_stream, stream_handle) = OutputStream::try_default()?;
        let sink = Sink::try_new(&stream_handle)?;
        
        Ok(Self {
            _stream,
            stream_handle,
            sink,
            current_volume: 1.0,
        })
    }

    pub fn play_from_file<P: AsRef<Path>>(&mut self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let decoder = Decoder::new(reader)?;
        
        self.sink.stop();
        self.sink = Sink::try_new(&self.stream_handle)?;
        self.sink.set_volume(self.current_volume);
        self.sink.append(decoder);
        self.sink.play();
        
        Ok(())
    }

    pub fn play_from_memory(&mut self, data: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
        let cursor = Cursor::new(data);
        let decoder = Decoder::new(cursor)?;
        
        self.sink.stop();
        self.sink = Sink::try_new(&self.stream_handle)?;
        self.sink.set_volume(self.current_volume);
        self.sink.append(decoder);
        self.sink.play();
        
        Ok(())
    }

    pub fn pause(&self) {
        self.sink.pause();
    }

    pub fn resume(&self) {
        self.sink.play();
    }

    pub fn stop(&mut self) {
        self.sink.stop();
        if let Ok(new_sink) = Sink::try_new(&self.stream_handle) {
            new_sink.set_volume(self.current_volume);
            self.sink = new_sink;
        }
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.current_volume = volume.clamp(0.0, 1.0);
        self.sink.set_volume(self.current_volume);
    }
    
    pub fn current_volume(&self) -> f32 {
        self.current_volume
    }

    pub fn is_playing(&self) -> bool {
        !self.sink.is_paused() && !self.sink.empty()
    }
}
