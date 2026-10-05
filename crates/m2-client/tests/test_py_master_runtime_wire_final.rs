use m2_client::py_master_runtime_bridge::inject_master_runtime_modules;
use m2_client::py_master_runtime_wire_final::*;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyTuple};
use std::sync::atomic::{AtomicBool, Ordering};

static INIT: AtomicBool = AtomicBool::new(false);

fn init_python() {
    if !INIT.swap(true, Ordering::SeqCst) {
        pyo3::prepare_freethreaded_python();
        inject_master_runtime_modules().expect("Failed to inject bridge modules");
    }
}

#[test]
fn test_app_module_lambdas_replaced() {
    init_python();

    Python::with_gil(|py| {
        let sys = py.import_bound("sys").unwrap();
        let modules = sys.getattr("modules").unwrap();
        let app_module = modules.get_item("app").unwrap().unwrap();

        // Check if the functions are no longer lambda (in Python, lambdas are functions, C-FFI are builtin_function_or_method or staticmethod wrapping CFuncPtr)
        // A simple test: GetRandom was lambda a,b: randint. Now it's mapped to FFI.
        // Let's call it and see if it works without raising.
        
        // 1. GetRandom
        let args = PyTuple::new_bound(py, &[1, 10]);
        let rand_val: i32 = app_module.call_method1("GetRandom", args).unwrap().extract().unwrap();
        assert!((1..=10).contains(&rand_val));

        // 2. SetFrameSkip
        let args_fs = PyTuple::new_bound(py, &[1]);
        app_module.call_method1("SetFrameSkip", args_fs).unwrap();
        assert_eq!(*FRAME_SKIP.lock().unwrap(), true);

        // 3. GetGlobalTimeStamp
        let timestamp: i32 = app_module.call_method0("GetGlobalTimeStamp").unwrap().extract().unwrap();
        assert!(timestamp > 0);

        // 4. SetHairColorEnable
        let args_hair = PyTuple::new_bound(py, &[1]);
        app_module.call_method1("SetHairColorEnable", args_hair).unwrap();
        assert_eq!(*HAIR_COLOR_ENABLE.lock().unwrap(), true);
        
        // 5. GetLocaleName
        let locale: String = app_module.call_method0("GetLocaleName").unwrap().extract().unwrap();
        assert_eq!(locale, "en");

        // 6. GetImageInfo (wrapped logic)
        // Because of the wrapper logic, it should return a tuple (False, 0, 0) for non-existent image
        let args_img = PyTuple::new_bound(py, &["does_not_exist.png"]);
        let img_info: (bool, i32, i32) = app_module.call_method1("GetImageInfo", args_img).unwrap().extract().unwrap();
        assert_eq!(img_info, (false, 0, 0));
        
        // 7. IsExistFile
        let args_file = PyTuple::new_bound(py, &["does_not_exist.txt"]);
        let is_exist: i32 = app_module.call_method1("IsExistFile", args_file).unwrap().extract().unwrap();
        assert_eq!(is_exist, 0);

        // 8. SetGuildMarkPath
        let args_mark = PyTuple::new_bound(py, &["my_mark.tga"]);
        app_module.call_method1("SetGuildMarkPath", args_mark).unwrap();
        assert_eq!(*GUILD_MARK_PATH.lock().unwrap(), "my_mark");

        // 9. SetCenterPosition
        let args_pos = PyTuple::new_bound(py, &[100.0f32, 200.0f32, 300.0f32]);
        app_module.call_method1("SetCenterPosition", args_pos).unwrap();
        assert_eq!(*CENTER_POSITION.lock().unwrap(), (100.0, -200.0, 300.0));
    });
}
