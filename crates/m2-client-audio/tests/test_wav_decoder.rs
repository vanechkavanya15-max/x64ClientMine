use m2_client_audio::wav_decoder::{WavDecoder, WavError};
use std::io::Cursor;

fn create_valid_wav() -> Vec<u8> {
    let mut data = Vec::new();
    
    // RIFF header
    data.extend_from_slice(b"RIFF");
    data.extend_from_slice(&[36, 0, 0, 0]); // File size (36)
    data.extend_from_slice(b"WAVE");
    
    // fmt chunk
    data.extend_from_slice(b"fmt ");
    data.extend_from_slice(&[16, 0, 0, 0]); // Chunk size (16)
    data.extend_from_slice(&[1, 0]); // Audio format (PCM = 1)
    data.extend_from_slice(&[1, 0]); // Channels (1 = Mono)
    data.extend_from_slice(&[0x44, 0xac, 0, 0]); // Sample rate (44100)
    data.extend_from_slice(&[0x88, 0x58, 0x01, 0]); // Byte rate (44100 * 2)
    data.extend_from_slice(&[2, 0]); // Block align
    data.extend_from_slice(&[16, 0]); // Bits per sample (16)
    
    // data chunk
    data.extend_from_slice(b"data");
    data.extend_from_slice(&[4, 0, 0, 0]); // Chunk size (4 bytes of audio data)
    data.extend_from_slice(&[0, 0, 255, 127]); // Some audio samples
    
    data
}

fn create_invalid_wav_no_riff() -> Vec<u8> {
    let mut data = Vec::new();
    data.extend_from_slice(b"BADD");
    data.extend_from_slice(&[36, 0, 0, 0]);
    data.extend_from_slice(b"WAVE");
    data
}

fn create_invalid_wav_unsupported_format() -> Vec<u8> {
    let mut data = Vec::new();
    data.extend_from_slice(b"RIFF");
    data.extend_from_slice(&[36, 0, 0, 0]);
    data.extend_from_slice(b"WAVE");
    
    data.extend_from_slice(b"fmt ");
    data.extend_from_slice(&[16, 0, 0, 0]);
    data.extend_from_slice(&[3, 0]); // Format 3 = IEEE Float (unsupported)
    data.extend_from_slice(&[1, 0]);
    data.extend_from_slice(&[0x44, 0xac, 0, 0]);
    data.extend_from_slice(&[0x88, 0x58, 0x01, 0]);
    data.extend_from_slice(&[2, 0]);
    data.extend_from_slice(&[16, 0]);
    
    data.extend_from_slice(b"data");
    data.extend_from_slice(&[4, 0, 0, 0]);
    data.extend_from_slice(&[0, 0, 255, 127]);
    
    data
}

#[test]
fn test_integration_wav_decoder() {
    let instance = WavDecoder::new();
    assert!(instance.is_ready());
}

#[test]
fn test_decode_valid_wav() {
    let data = create_valid_wav();
    let cursor = Cursor::new(data);
    let decoder = WavDecoder::new();
    
    let result = decoder.decode(cursor);
    assert!(result.is_ok());
    
    let wav_data = result.unwrap();
    assert_eq!(wav_data.header.channels, 1);
    assert_eq!(wav_data.header.sample_rate, 44100);
    assert_eq!(wav_data.header.bits_per_sample, 16);
    assert_eq!(wav_data.samples.len(), 4);
}

#[test]
fn test_decode_invalid_riff() {
    let data = create_invalid_wav_no_riff();
    let cursor = Cursor::new(data);
    let decoder = WavDecoder::new();
    
    let result = decoder.decode(cursor);
    assert!(result.is_err());
    
    match result.unwrap_err() {
        WavError::InvalidHeader(msg) => assert_eq!(msg, "Brak sygnatury RIFF"),
        _ => panic!("Oczekiwano bledu InvalidHeader"),
    }
}

#[test]
fn test_decode_unsupported_format() {
    let data = create_invalid_wav_unsupported_format();
    let cursor = Cursor::new(data);
    let decoder = WavDecoder::new();
    
    let result = decoder.decode(cursor);
    assert!(result.is_err());
    
    match result.unwrap_err() {
        WavError::UnsupportedFormat(msg) => assert_eq!(msg, "Wspierany tylko format PCM (wspolczynnik 1)"),
        _ => panic!("Oczekiwano bledu UnsupportedFormat"),
    }
}

#[test]
fn test_decode_unexpected_eof() {
    let mut data = create_valid_wav();
    data.truncate(20); // Ucinamy w srodku naglowka fmt
    
    let cursor = Cursor::new(data);
    let decoder = WavDecoder::new();
    
    let result = decoder.decode(cursor);
    assert!(result.is_err());
    
    match result.unwrap_err() {
        WavError::UnexpectedEof | WavError::IoError(_) => {}, // Obie akceptowalne zaleznie od momentu EOF
        _ => panic!("Oczekiwano bledu EOF"),
    }
}
