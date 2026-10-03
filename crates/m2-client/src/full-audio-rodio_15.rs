use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink, SpatialSink};
use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, Mutex};

pub struct RodioAudioEngine {
    _stream: OutputStream,
    stream_handle: OutputStreamHandle,
    bgm_sink: Arc<Mutex<Option<Sink>>>,
    spatial_sinks: Arc<Mutex<Vec<SpatialSink>>>,
    master_volume: Arc<Mutex<f32>>,
}

impl RodioAudioEngine {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let (_stream, stream_handle) = OutputStream::try_default()?;
        Ok(Self {
            _stream,
            stream_handle,
            bgm_sink: Arc::new(Mutex::new(None)),
            spatial_sinks: Arc::new(Mutex::new(Vec::new())),
            master_volume: Arc::new(Mutex::new(1.0)),
        })
    }

    pub fn play_bgm(&self, file: &str) {
        if let Ok(file_obj) = File::open(file) {
            let buf_reader = BufReader::new(file_obj);
            if let Ok(source) = Decoder::new(buf_reader) {
                let mut sink_guard = self.bgm_sink.lock().unwrap();
                let volume = *self.master_volume.lock().unwrap();
                if let Ok(sink) = Sink::try_new(&self.stream_handle) {
                    sink.set_volume(volume);
                    sink.append(source);
                    sink.play();
                    *sink_guard = Some(sink);
                }
            }
        }
    }

    pub fn play_sound_3d(&self, file: &str, pos: [f32; 3]) {
        if let Ok(file_obj) = File::open(file) {
            let buf_reader = BufReader::new(file_obj);
            if let Ok(source) = Decoder::new(buf_reader) {
                let volume = *self.master_volume.lock().unwrap();
                if let Ok(sink) = SpatialSink::try_new(
                    &self.stream_handle,
                    pos,
                    [-1.0, 0.0, 0.0],
                    [1.0, 0.0, 0.0],
                ) {
                    sink.set_volume(volume);
                    sink.append(source);
                    sink.play();
                    self.spatial_sinks.lock().unwrap().push(sink);
                }
            }
        }
    }

    pub fn set_master_volume(&self, vol: f32) {
        let clamped = vol.clamp(0.0, 1.0);
        *self.master_volume.lock().unwrap() = clamped;

        if let Some(bgm) = self.bgm_sink.lock().unwrap().as_ref() {
            bgm.set_volume(clamped);
        }

        let mut spatial_guard = self.spatial_sinks.lock().unwrap();
        for sink in spatial_guard.iter_mut() {
            sink.set_volume(clamped);
        }
    }

    pub fn stop_all(&self) {
        let mut bgm_guard = self.bgm_sink.lock().unwrap();
        if let Some(sink) = bgm_guard.take() {
            sink.stop();
        }

        let mut spatial_guard = self.spatial_sinks.lock().unwrap();
        for sink in spatial_guard.drain(..) {
            sink.stop();
        }
    }
}
