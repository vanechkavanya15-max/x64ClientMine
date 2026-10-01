use m2_client::py_native_shim::*;
use pyo3::ffi::*;
use pyo3::Python;
use serial_test::serial;
use std::ffi::{CStr, CString};
use std::ptr;

// Dummy method to register
unsafe extern "C" fn dummy_method(_self: *mut PyObject, _args: *mut PyObject) -> *mut PyObject {
    Py_None()
}

#[test]
#[serial]
fn test_py_init_module() {
    Python::with_gil(|_py| unsafe {
        let module_name = CString::new("test_shim_module").unwrap();
        let method_name = CString::new("dummy_func").unwrap();
        let doc_string = CString::new("Dummy function").unwrap();

        // Create method definitions
        let mut methods = vec![
            PyMethodDef {
                ml_name: method_name.as_ptr(),
                ml_meth: PyMethodDefPointer { PyCFunction: dummy_method },
                ml_flags: METH_VARARGS,
                ml_doc: doc_string.as_ptr(),
            },
            PyMethodDef {
                ml_name: ptr::null(),
                ml_meth: PyMethodDefPointer::zeroed(),
                ml_flags: 0,
                ml_doc: ptr::null(),
            },
        ];

        let module = py_init_module(module_name.as_ptr(), methods.as_mut_ptr());
        
        assert!(!module.is_null(), "Module creation failed");

        // Verify module is in sys.modules
        let sys_modules = PyImport_GetModuleDict();
        let borrowed_module = PyDict_GetItemString(sys_modules, module_name.as_ptr());
        
        assert_eq!(module, borrowed_module, "Returned module does not match sys.modules entry");
        
        // Verify module type is module
        assert_eq!(PyModule_Check(module), 1, "Created object is not a module");
    });
}

#[test]
#[serial]
fn test_py_string_utils_unicode() {
    Python::with_gil(|_py| unsafe {
        let test_str = CString::new("hello_unicode").unwrap();
        let obj = py_string_from_string(test_str.as_ptr());
        
        assert!(!obj.is_null());
        assert!(py_string_check(obj));
        
        let extracted_ptr = py_string_as_string(obj);
        assert!(!extracted_ptr.is_null());
        
        let extracted_str = CStr::from_ptr(extracted_ptr).to_str().unwrap();
        assert_eq!(extracted_str, "hello_unicode");

        Py_DecRef(obj);
    });
}

#[test]
#[serial]
fn test_py_string_utils_bytes() {
    Python::with_gil(|_py| unsafe {
        let test_str = CString::new("hello_bytes").unwrap();
        let obj = py_string_from_string_and_size(test_str.as_ptr(), test_str.as_bytes().len() as isize);
        
        assert!(!obj.is_null());
        assert!(py_string_check(obj));
        
        let extracted_ptr = py_string_as_string(obj);
        assert!(!extracted_ptr.is_null());
        
        let extracted_str = CStr::from_ptr(extracted_ptr).to_str().unwrap();
        assert_eq!(extracted_str, "hello_bytes");

        Py_DecRef(obj);
    });
}

#[test]
#[serial]
fn test_py_string_utils_invalid() {
    Python::with_gil(|_py| unsafe {
        let obj = py_int_from_long(1234);
        assert!(!obj.is_null());
        
        assert!(!py_string_check(obj));
        
        let extracted_ptr = py_string_as_string(obj);
        assert!(extracted_ptr.is_null());
        
        let err = PyErr_Occurred();
        assert!(!err.is_null());
        PyErr_Clear();

        Py_DecRef(obj);
    });
}

#[test]
#[serial]
fn test_py_int_utils() {
    Python::with_gil(|_py| unsafe {
        let val = 42;
        let obj = py_int_from_long(val);
        
        assert!(!obj.is_null());
        assert!(py_int_check(obj));
        assert_eq!(py_int_as_long(obj), val);

        Py_DecRef(obj);
    });
}

#[test]
#[serial]
fn test_py_string_intern_from_string() {
    Python::with_gil(|_py| unsafe {
        let test_str = CString::new("interned_string").unwrap();
        let obj = py_string_intern_from_string(test_str.as_ptr());
        
        assert!(!obj.is_null());
        assert!(py_string_check(obj));
        
        let extracted_ptr = py_string_as_string_macro(obj);
        assert!(!extracted_ptr.is_null());
        
        let extracted_str = CStr::from_ptr(extracted_ptr).to_str().unwrap();
        assert_eq!(extracted_str, "interned_string");

        Py_DecRef(obj);
    });
}