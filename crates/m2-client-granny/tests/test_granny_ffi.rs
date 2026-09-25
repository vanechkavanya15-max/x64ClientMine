use m2_client_granny::granny_ffi::{
    m2_granny_load_model, 
    m2_granny_evaluate_pose, 
    m2_granny_free,
    GrannyFfi
};
use std::ffi::CString;
use std::ptr;

#[test]
fn test_integration_granny_ffi() {
    let instance = GrannyFfi::new("test".to_string());
    assert!(instance.is_ready());
}

#[test]
fn test_m2_granny_load_model_valid() {
    let path = CString::new("test_model.gr2").unwrap();
    let model_ptr = unsafe { m2_granny_load_model(path.as_ptr()) };
    
    assert!(!model_ptr.is_null());
    
    // Upewniamy sie ze model jest poprawnie zaladowany
    let model_ref = unsafe { &*model_ptr };
    assert!(model_ref.is_ready());
    assert_eq!(model_ref.model_path, "test_model.gr2");
    assert_eq!(model_ref.current_time, 0.0);
    
    unsafe { m2_granny_free(model_ptr) };
}

#[test]
fn test_m2_granny_load_model_invalid_null() {
    let model_ptr = unsafe { m2_granny_load_model(ptr::null()) };
    assert!(model_ptr.is_null());
}

#[test]
fn test_m2_granny_evaluate_pose_valid() {
    let path = CString::new("test_model.gr2").unwrap();
    let model_ptr = unsafe { m2_granny_load_model(path.as_ptr()) };
    assert!(!model_ptr.is_null());
    
    let result = unsafe { m2_granny_evaluate_pose(model_ptr, 1.5) };
    assert!(result);
    
    let model_ref = unsafe { &*model_ptr };
    assert_eq!(model_ref.current_time, 1.5);
    
    unsafe { m2_granny_free(model_ptr) };
}

#[test]
fn test_m2_granny_evaluate_pose_invalid_null() {
    let result = unsafe { m2_granny_evaluate_pose(ptr::null_mut(), 1.0) };
    assert!(!result);
}

#[test]
fn test_m2_granny_free_null() {
    // Sprawdzamy czy nie ma bledu przy probie zwolnienia null
    unsafe { m2_granny_free(ptr::null_mut()) };
}
