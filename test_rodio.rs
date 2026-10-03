use rodio::{OutputStream, SpatialSink, Sink};

fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = SpatialSink::try_new(&stream_handle, [0.0, 0.0, 0.0], [-1.0, 0.0, 0.0], [1.0, 0.0, 0.0]).unwrap();
    let bgm = Sink::try_new(&stream_handle).unwrap();
    
    println!("Rodio SpatialSink created!");
}
