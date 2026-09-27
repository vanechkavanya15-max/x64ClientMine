use m2_client::granny_library_loader::*;

#[test]
fn test_granny_availability_initial_state() {
    // Should be false initially unless tests leak state, but we ensure clean start.
    unload_granny();
    assert!(!is_granny_available());
}

#[test]
fn test_load_granny_invalid_path() {
    unload_granny();
    let res = load_granny2_library("invalid_or_missing_granny2.dll");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err(), "Failed to load library");
    assert!(!is_granny_available());
}

#[test]
fn test_load_granny_null_bytes_in_path() {
    unload_granny();
    let res = load_granny2_library("invalid\0path");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err(), "Invalid DLL path");
}

#[test]
fn test_unload_granny_safe_when_not_loaded() {
    unload_granny();
    // Should not panic or crash
    unload_granny();
    assert!(!is_granny_available());
}

#[test]
fn test_granny_api_struct_defaults() {
    let api = GrannyApi::default();
    assert_eq!(api.load_file, 0);
    assert_eq!(api.free_file, 0);
    assert_eq!(api.get_mesh, 0);
    assert_eq!(api.sample_model, 0);
    assert!(!api.is_loaded);
}
