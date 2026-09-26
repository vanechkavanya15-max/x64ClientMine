#[path = "../src/sound_stream_reader.rs"]
mod sound_stream_reader;

use sound_stream_reader::{StreamReader, MAX_BUFFER_BYTES};

#[test]
fn test_standard_feeding_and_reading() {
    let mut reader = StreamReader::new(10);
    
    let chunk = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06];
    assert!(reader.feed_chunk(&chunk).is_ok());

    let mut samples = [0i16; 2];
    let read_count = reader.read_samples(&mut samples);
    assert_eq!(read_count, 2);
    
    // 0x0201 -> 513
    assert_eq!(samples[0], 0x0201);
    // 0x0403 -> 1027
    assert_eq!(samples[1], 0x0403);
    
    // We have 2 bytes left in buffer: 0x05, 0x06
    let mut remaining = [0i16; 1];
    let read_count_2 = reader.read_samples(&mut remaining);
    assert_eq!(read_count_2, 1);
    assert_eq!(remaining[0], 0x0605);
}

#[test]
fn test_end_of_stream_detection() {
    let mut reader = StreamReader::new(4);
    
    let chunk = [0x0A, 0x0B, 0x0C, 0x0D];
    assert!(reader.feed_chunk(&chunk).is_ok());

    assert!(!reader.is_end_of_stream()); // still bytes in buffer

    let mut samples = [0i16; 2];
    assert_eq!(reader.read_samples(&mut samples), 2);

    assert!(reader.is_end_of_stream()); // All read
}

#[test]
fn test_seek_to_start() {
    let mut reader = StreamReader::new(6);
    let chunk = [0xAA, 0xBB, 0xCC, 0xDD];
    assert!(reader.feed_chunk(&chunk).is_ok());
    
    let mut out = [0i16; 1];
    reader.read_samples(&mut out);

    reader.seek_to_start();

    // Now it should act like it's brand new
    assert_eq!(reader.is_end_of_stream(), false);

    let new_chunk = [0x11, 0x22, 0x33, 0x44, 0x55, 0x66];
    assert!(reader.feed_chunk(&new_chunk).is_ok());
    
    let mut out2 = [0i16; 3];
    assert_eq!(reader.read_samples(&mut out2), 3);
    assert_eq!(out2[0], 0x2211);
    assert_eq!(out2[1], 0x4433);
    assert_eq!(out2[2], 0x6655);

    assert!(reader.is_end_of_stream());
}

#[test]
fn test_overfeeding_error() {
    let mut reader = StreamReader::new(5);
    
    let chunk = [0x1, 0x2, 0x3, 0x4, 0x5, 0x6];
    assert_eq!(reader.feed_chunk(&chunk), Err("Overfeeding: exceeds total_bytes"));
}

#[test]
fn test_buffer_capacity_limit_error() {
    let total_bytes = MAX_BUFFER_BYTES + 10;
    let mut reader = StreamReader::new(total_bytes);
    
    let chunk = vec![0; MAX_BUFFER_BYTES];
    assert!(reader.feed_chunk(&chunk).is_ok());
    
    let chunk_too_much = vec![0; 1];
    assert_eq!(reader.feed_chunk(&chunk_too_much), Err("Buffer capacity limit reached"));
}

#[test]
fn test_reading_across_boundary() {
    let mut reader = StreamReader::new(MAX_BUFFER_BYTES * 2);
    
    // Fill up the buffer exactly to MAX_BUFFER_BYTES - 1
    let large_chunk = vec![0x11; MAX_BUFFER_BYTES - 1];
    assert!(reader.feed_chunk(&large_chunk).is_ok());
    
    // Read most of it
    let mut big_read = vec![0i16; (MAX_BUFFER_BYTES - 1) / 2];
    reader.read_samples(&mut big_read);
    
    // Now there's only 1 byte left at the end of the buffer (if MAX_BUFFER_BYTES is odd, actually MAX_BUFFER_BYTES = 49152 which is even.
    // Let's be precise.
    // MAX_BUFFER_BYTES = 49152
    // We feed 49151 bytes.
    // We read 24575 samples (49150 bytes).
    // Remaining in buffer: 1 byte, located at index 49150.
    // Wait, head will be at 49150.
    
    let next_chunk = [0x22, 0x33, 0x44];
    assert!(reader.feed_chunk(&next_chunk).is_ok());
    
    // Now buffer has 4 bytes.
    // At head 49150: 0x11
    // At head 49151: 0x22
    // At head 0: 0x33
    // At head 1: 0x44
    
    let mut small_read = [0i16; 2];
    assert_eq!(reader.read_samples(&mut small_read), 2);
    
    // First sample: 0x11 and 0x22 -> 0x2211
    assert_eq!(small_read[0], 0x2211);
    
    // Second sample: 0x33 and 0x44 -> 0x4433
    assert_eq!(small_read[1], 0x4433);
}

#[test]
fn test_odd_total_bytes_gracefully_marks_eof() {
    let mut reader = StreamReader::new(3);
    
    let chunk = [0x11, 0x22, 0x33];
    assert!(reader.feed_chunk(&chunk).is_ok());
    
    let mut out = [0i16; 1];
    assert_eq!(reader.read_samples(&mut out), 1);
    assert_eq!(out[0], 0x2211);
    
    // 1 byte remaining, total_bytes fed. It cannot form a full sample.
    assert!(reader.is_end_of_stream());
    
    // Reading again should return 0
    let mut out2 = [0i16; 1];
    assert_eq!(reader.read_samples(&mut out2), 0);
}
