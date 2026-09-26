use m2_client::hair_head_attach_v2::BONE_HEAD_NAME;
use m2_client::hair_head_attach_v2::*;
use std::sync::Mutex;

static TEST_LOCK: Mutex<()> = Mutex::new(());

#[test]
fn test_bone_name_is_correct() {
    assert_eq!(BONE_HEAD_NAME, "Bip01 Head");
}

#[test]
fn test_default_state_is_not_attached() {
    let _lock = TEST_LOCK.lock().unwrap();
    m2_head_clear_state(0);

    let mut out_mat = [0.0f32; 16];

    // Bez ustawionych wlosow/glowy nie powinno zwracac transformacji
    let result = m2_head_get_transform(out_mat.as_mut_ptr());
    assert!(!result);
}

#[test]
fn test_null_pointers() {
    let _lock = TEST_LOCK.lock().unwrap();
    assert!(!m2_head_get_transform(std::ptr::null_mut()));
    assert!(!m2_head_update_transform(0, std::ptr::null()));
}

#[test]
fn test_set_hair_allows_get_transform() {
    let _lock = TEST_LOCK.lock().unwrap();
    m2_head_clear_state(0);

    let mut out_mat = [0.0f32; 16];

    assert!(m2_head_set_hair(0, 1000));

    let result = m2_head_get_transform(out_mat.as_mut_ptr());
    assert!(result);

    // Sprawdzamy czy to macierz jednostkowa (domyslna)
    assert_eq!(out_mat[0], 1.0);
    assert_eq!(out_mat[5], 1.0);
    assert_eq!(out_mat[10], 1.0);
    assert_eq!(out_mat[15], 1.0);
}

#[test]
fn test_set_head_allows_get_transform() {
    let _lock = TEST_LOCK.lock().unwrap();
    m2_head_clear_state(0);

    let mut out_mat = [0.0f32; 16];

    assert!(m2_head_set_head(0, 2000));

    let result = m2_head_get_transform(out_mat.as_mut_ptr());
    assert!(result);
}

#[test]
fn test_update_transform() {
    let _lock = TEST_LOCK.lock().unwrap();
    m2_head_clear_state(0);

    let custom_mat = [
        2.0, 0.0, 0.0, 0.0, 0.0, 3.0, 0.0, 0.0, 0.0, 0.0, 4.0, 0.0, 10.0, 20.0, 30.0, 1.0,
    ];

    // Aktualizujemy transformacje (dla VID 0)
    assert!(m2_head_update_transform(0, custom_mat.as_ptr()));

    // Zeby get_transform dzialalo, musimy miec wlosy lub nakrycie glowy
    assert!(m2_head_set_hair(0, 1001));

    let mut out_mat = [0.0f32; 16];
    assert!(m2_head_get_transform(out_mat.as_mut_ptr()));

    assert_eq!(out_mat, custom_mat);
}

#[test]
fn test_multiple_vids_isolation() {
    let _lock = TEST_LOCK.lock().unwrap();
    m2_head_clear_state(0);
    m2_head_clear_state(1);

    assert!(m2_head_set_hair(1, 999));

    let custom_mat = [
        0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0,
    ];

    assert!(m2_head_update_transform(1, custom_mat.as_ptr()));

    // Dla VID 0 nie powinno zwracac nic, bo stan jest pusty
    let mut out_mat = [0.0f32; 16];
    assert!(!m2_head_get_transform(out_mat.as_mut_ptr()));

    // (Opcjonalnie mozna by zrobic test dla innych VID, ale
    // eksportowana funkcja m2_head_get_transform obsluguje tylko VID 0
    // zgodnie z definicja C++)
}

#[test]
fn test_clear_state_resets_behavior() {
    let _lock = TEST_LOCK.lock().unwrap();
    m2_head_clear_state(0);

    assert!(m2_head_set_head(0, 123));
    let mut out_mat = [0.0f32; 16];
    assert!(m2_head_get_transform(out_mat.as_mut_ptr()));

    m2_head_clear_state(0);

    assert!(!m2_head_get_transform(out_mat.as_mut_ptr()));
}
