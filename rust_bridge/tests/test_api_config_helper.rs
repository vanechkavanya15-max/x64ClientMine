use m2_client_bridge::api::config_helper::{m2_config_is_valid, ClientConfig};
use std::ptr;

#[test]
fn test_m2_config_is_valid_null_ptr() {
    let result = m2_config_is_valid(ptr::null());
    assert_eq!(result, 0, "Powinno zwrocic 0 dla pustego wskaznika");
}

#[test]
fn test_m2_config_is_valid_correct_config() {
    let config = ClientConfig {
        width: 1920,
        height: 1080,
        bpp: 32,
        shadow_quality: 3,
    };
    let result = m2_config_is_valid(&config as *const ClientConfig);
    assert_eq!(result, 1, "Powinno zwrocic 1 dla poprawnych ustawien");
}

#[test]
fn test_m2_config_is_valid_incorrect_resolution_too_small() {
    let config = ClientConfig {
        width: 640,
        height: 480,
        bpp: 32,
        shadow_quality: 3,
    };
    let result = m2_config_is_valid(&config as *const ClientConfig);
    assert_eq!(result, 0, "Powinno zwrocic 0 dla zbyt malej rozdzielczosci");
}

#[test]
fn test_m2_config_is_valid_incorrect_resolution_too_large() {
    let config = ClientConfig {
        width: 8000,
        height: 5000,
        bpp: 32,
        shadow_quality: 3,
    };
    let result = m2_config_is_valid(&config as *const ClientConfig);
    assert_eq!(result, 0, "Powinno zwrocic 0 dla zbyt duzej rozdzielczosci");
}

#[test]
fn test_m2_config_is_valid_incorrect_bpp() {
    let config = ClientConfig {
        width: 1920,
        height: 1080,
        bpp: 24, // niepoprawne bpp
        shadow_quality: 3,
    };
    let result = m2_config_is_valid(&config as *const ClientConfig);
    assert_eq!(result, 0, "Powinno zwrocic 0 dla niepoprawnego bpp");
}

#[test]
fn test_m2_config_is_valid_incorrect_shadow_quality() {
    let config = ClientConfig {
        width: 1920,
        height: 1080,
        bpp: 32,
        shadow_quality: 10, // za wysoka jakosc
    };
    let result = m2_config_is_valid(&config as *const ClientConfig);
    assert_eq!(result, 0, "Powinno zwrocic 0 dla zbyt wysokiej jakosci cieni");
}
