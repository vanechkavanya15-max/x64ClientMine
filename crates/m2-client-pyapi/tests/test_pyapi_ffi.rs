use std::ffi::CString;
use m2_client_pyapi::pyapi_ffi::*;

#[test]
fn test_integration_pyapi_ffi() {
    unsafe {
        assert!(m2_pyapi_init());

        let event = CString::new("BINARY_OnItemUse").unwrap();
        let payload = CString::new("{\"slot\": 5}").unwrap();

        assert!(m2_pyapi_dispatch_event(event.as_ptr(), payload.as_ptr()));
        assert!(!m2_pyapi_dispatch_event(std::ptr::null(), payload.as_ptr()));

        m2_pyapi_shutdown();
        assert!(!m2_pyapi_dispatch_event(event.as_ptr(), payload.as_ptr()));
    }
}
