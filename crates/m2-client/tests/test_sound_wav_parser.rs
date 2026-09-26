#[path = "../src/sound_wav_parser.rs"]
mod sound_wav_parser;
use sound_wav_parser::WavAudio;

fn build_wav(
    channels: u16,
    sample_rate: u32,
    bits_per_sample: u16,
    samples_data: &[u8],
) -> Vec<u8> {
    let mut data = Vec::new();
    
    // RIFF
    data.extend_from_slice(b"RIFF");
    let file_size = 36 + samples_data.len() as u32;
    data.extend_from_slice(&file_size.to_le_bytes());
    data.extend_from_slice(b"WAVE");
    
    // fmt chunk
    data.extend_from_slice(b"fmt ");
    data.extend_from_slice(&16u32.to_le_bytes()); // chunk size
    data.extend_from_slice(&1u16.to_le_bytes()); // audio format PCM
    data.extend_from_slice(&channels.to_le_bytes());
    data.extend_from_slice(&sample_rate.to_le_bytes());
    let byte_rate = sample_rate * channels as u32 * (bits_per_sample / 8) as u32;
    data.extend_from_slice(&byte_rate.to_le_bytes());
    let block_align = channels * (bits_per_sample / 8);
    data.extend_from_slice(&block_align.to_le_bytes());
    data.extend_from_slice(&bits_per_sample.to_le_bytes());
    
    // data chunk
    data.extend_from_slice(b"data");
    data.extend_from_slice(&(samples_data.len() as u32).to_le_bytes());
    data.extend_from_slice(samples_data);
    
    data
}

#[test]
fn test_wav_invalid_header() {
    let data = b"RIFX.......................................X".to_vec();
    let err = WavAudio::from_bytes(&data).unwrap_err();
    assert_eq!(err, "Nie znaleziono naglowka RIFF");
    
    let mut data2 = b"RIFF....XXXX...............................X".to_vec();
    data2[4..8].copy_from_slice(&40u32.to_le_bytes());
    let err = WavAudio::from_bytes(&data2).unwrap_err();
    assert_eq!(err, "Nie znaleziono naglowka WAVE");
}

#[test]
fn test_wav_16bit_mono() {
    let samples_16bit: Vec<i16> = vec![0, 1000, -1000, 32767, -32768];
    let mut samples_bytes = Vec::new();
    for &s in &samples_16bit {
        samples_bytes.extend_from_slice(&s.to_le_bytes());
    }
    
    let wav_data = build_wav(1, 44100, 16, &samples_bytes);
    let audio = WavAudio::from_bytes(&wav_data).expect("Poprawne parsowanie 16-bit mono");
    
    assert_eq!(audio.channels, 1);
    assert_eq!(audio.sample_rate, 44100);
    assert_eq!(audio.bits_per_sample, 16);
    assert_eq!(audio.samples.len(), 5);
    assert_eq!(audio.samples, samples_16bit);
}

#[test]
fn test_wav_8bit_stereo() {
    let samples_8bit: Vec<u8> = vec![128, 200, 56, 0];
    let wav_data = build_wav(2, 22050, 8, &samples_8bit);
    let audio = WavAudio::from_bytes(&wav_data).expect("Poprawne parsowanie 8-bit stereo");
    
    assert_eq!(audio.channels, 2);
    assert_eq!(audio.sample_rate, 22050);
    assert_eq!(audio.bits_per_sample, 8);
    
    // 128 -> 0
    // 200 -> (200-128)*256 = 72 * 256 = 18432
    // 56 -> (56-128)*256 = -72 * 256 = -18432
    // 0 -> (0-128)*256 = -128 * 256 = -32768
    let expected_samples: Vec<i16> = vec![0, 18432, -18432, -32768];
    assert_eq!(audio.samples, expected_samples);
}

#[test]
fn test_wav_duration_calculation() {
    // 44100 Hz, stereo, 16-bit.
    // 44100 probek (dla obu kanalow, wiec 44100*2 probek liczbowych)
    // To daje równe 1000 ms.
    let num_samples_per_channel = 44100;
    let total_samples = num_samples_per_channel * 2;
    let samples_bytes = vec![0u8; total_samples * 2];
    
    let wav_data = build_wav(2, 44100, 16, &samples_bytes);
    let audio = WavAudio::from_bytes(&wav_data).expect("Parsowanie duzego pliku");
    
    assert_eq!(audio.duration_ms(), 1000);
    
    // Test for fractional ms calculation
    let num_samples_per_channel_2 = 22050; // 0.5 sekundy
    let total_samples_2 = num_samples_per_channel_2 * 2;
    let samples_bytes_2 = vec![0u8; total_samples_2 * 2];
    
    let wav_data_2 = build_wav(2, 44100, 16, &samples_bytes_2);
    let audio_2 = WavAudio::from_bytes(&wav_data_2).unwrap();
    assert_eq!(audio_2.duration_ms(), 500);
}

#[test]
fn test_wav_unsupported_format() {
    let mut data = build_wav(1, 44100, 16, &[0, 0, 0, 0]);
    // Zmien format na inny niz 1 (PCM)
    // offset do chunk_id "fmt " w build_wav to:
    // 0..4 RIFF
    // 4..8 size
    // 8..12 WAVE
    // 12..16 fmt 
    // 16..20 chunk size
    // 20..22 format
    data[20] = 3; // Format IEEE Float np.
    let err = WavAudio::from_bytes(&data).unwrap_err();
    assert_eq!(err, "Nieobsługiwany format audio (wymagany jest PCM format 1)");
}
