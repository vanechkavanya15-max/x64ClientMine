use m2_client_audio::ogg_decoder::{OggDecoder, TrackMetadata};
use std::io::Cursor;

fn create_dummy_metadata() -> TrackMetadata {
    TrackMetadata {
        title: "Test Track".to_string(),
        artist: "Test Artist".to_string(),
        sample_rate: 44100,
        channels: 2,
        length_in_samples: 1000,
    }
}

#[test]
fn test_initialization() {
    let data = vec![0u8; 1024];
    let cursor = Cursor::new(data);
    let metadata = create_dummy_metadata();
    
    let decoder = OggDecoder::new(cursor, metadata.clone());
    
    assert!(decoder.is_ready());
    assert_eq!(decoder.get_metadata(), &metadata);
    assert_eq!(decoder.current_sample(), 0);
}

#[test]
fn test_loop_points() {
    let data = vec![0u8; 1024];
    let cursor = Cursor::new(data);
    let mut decoder = OggDecoder::new(cursor, create_dummy_metadata());

    // Poprawne ustawienie (0 - 500)
    assert!(decoder.set_loop_points(100, 500));
    
    // Zle ustawienie (start >= end)
    assert!(!decoder.set_loop_points(500, 500));
    assert!(!decoder.set_loop_points(600, 500));
    
    // Zle ustawienie (end > length)
    assert!(!decoder.set_loop_points(100, 2000));
    
    decoder.clear_loop_points();
}

#[test]
fn test_read_samples_no_loop() {
    let data = vec![0u8; 1024];
    let cursor = Cursor::new(data);
    let mut decoder = OggDecoder::new(cursor, create_dummy_metadata()); // length = 1000

    let read = decoder.read_samples(400);
    assert_eq!(read, 400);
    assert_eq!(decoder.current_sample(), 400);

    let read2 = decoder.read_samples(700);
    assert_eq!(read2, 600); // Tylko 600 zostalo do konca (1000 max)
    assert_eq!(decoder.current_sample(), 1000);

    let read3 = decoder.read_samples(100);
    assert_eq!(read3, 0); // Koniec utworu
}

#[test]
fn test_read_samples_with_loop() {
    let data = vec![0u8; 1024];
    let cursor = Cursor::new(data);
    let mut decoder = OggDecoder::new(cursor, create_dummy_metadata()); // length = 1000

    assert!(decoder.set_loop_points(200, 500));

    // Odczyt przed petla (z 0 do 200)
    let read = decoder.read_samples(150);
    assert_eq!(read, 150);
    assert_eq!(decoder.current_sample(), 150);

    // Odczyt wchodzacy w punkt konca petli. Chcemy przeczytac 400 (dojdzie do 550, ale ograniczy do 500)
    // 500 - 150 = 350 probek
    let read2 = decoder.read_samples(400);
    assert_eq!(read2, 350);
    // Skoro doszedl do 500 (koniec petli), aktualna probka to 200 (poczatek petli)
    assert_eq!(decoder.current_sample(), 200);

    // Kolejny odczyt po zapetleniu. Znowu jestesmy w probce 200. Zazadajmy 100 probek (200 + 100 = 300)
    let read3 = decoder.read_samples(100);
    assert_eq!(read3, 100);
    assert_eq!(decoder.current_sample(), 300);
}

#[test]
fn test_seek() {
    let data = vec![0u8; 1024];
    let cursor = Cursor::new(data);
    let mut decoder = OggDecoder::new(cursor, create_dummy_metadata()); // length = 1000

    assert!(decoder.seek(500));
    assert_eq!(decoder.current_sample(), 500);

    // Poza zakres
    assert!(!decoder.seek(1500));
    assert_eq!(decoder.current_sample(), 500); // Bez zmian
}
