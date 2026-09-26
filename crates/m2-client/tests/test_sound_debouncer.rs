use m2_client::sound_debouncer::SoundDebouncer;

#[test]
fn test_sound_debouncer_first_play() {
    let mut debouncer = SoundDebouncer::new();
    // Powinno zwrocic true, brak historii
    assert!(debouncer.can_play(100, 500, 1000));
}

#[test]
fn test_sound_debouncer_blocked_after_100ms() {
    let mut debouncer = SoundDebouncer::new();
    assert!(debouncer.can_play(100, 500, 1000));
    // Powinno zwrocic false, minelo tylko 100ms (< 300ms)
    assert!(!debouncer.can_play(100, 500, 1100));
}

#[test]
fn test_sound_debouncer_allowed_after_301ms() {
    let mut debouncer = SoundDebouncer::new();
    assert!(debouncer.can_play(100, 500, 1000));
    // Powinno zwrocic true, minelo 301ms (>= 300ms)
    assert!(debouncer.can_play(100, 500, 1301));
}

#[test]
fn test_sound_debouncer_allowed_exactly_300ms() {
    let mut debouncer = SoundDebouncer::new();
    assert!(debouncer.can_play(100, 500, 1000));
    // Powinno zwrocic true, minelo rowno 300ms (>= 300ms)
    assert!(debouncer.can_play(100, 500, 1300));
}

#[test]
fn test_sound_debouncer_cleanup_old_records() {
    let mut debouncer = SoundDebouncer::new();
    assert!(debouncer.can_play(100, 500, 1000));
    assert!(debouncer.can_play(101, 501, 15000));

    // Usuwamy po uplywie 10000ms od czasu "cur_time_ms", tutaj 16000
    // Wpis dla (100, 500) ma 15000 (16000 - 1000) > 10000 -> ZOSTANIE USUNIETY
    debouncer.cleanup_old_records(16000);

    // Po usunieciu mozna znowu odtworzyc dzwiek dla (100, 500), mimo ze nie uzywamy czasu
    // (powinno dzialac jako first play bo historia czysta dla (100, 500))
    assert!(debouncer.can_play(100, 500, 16100));
    
    // (101, 501) zostal zachowany (mial czas 15000, 16000 - 15000 = 1000 <= 10000)
    // Zatem zablokuje probe odtworzenia po 100ms
    assert!(!debouncer.can_play(101, 501, 15100));
}

#[test]
fn test_sound_debouncer_time_wrap() {
    let mut debouncer = SoundDebouncer::new();
    // Czas blisko granicy typu u32
    let t1 = u32::MAX - 100;
    assert!(debouncer.can_play(100, 500, t1));
    
    // Uplynelo 150ms -> u32::MAX - 100 + 150 = u32::MAX + 50 -> to bedzie 49 (przy zawinieciu)
    let t2 = t1.wrapping_add(150);
    assert!(!debouncer.can_play(100, 500, t2));
    
    // Uplynelo 310ms -> u32::MAX - 100 + 310 = u32::MAX + 210 -> to bedzie 209
    let t3 = t1.wrapping_add(310);
    assert!(debouncer.can_play(100, 500, t3));
}
