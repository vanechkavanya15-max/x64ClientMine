use m2_client::python_runtime_core::{
    execute_python_string, finalize_python_runtime, init_python_runtime, register_builtin_module,
    PythonEnvironmentConfig, PYTHON_RUNTIME_STATE,
};
use std::os::raw::c_void;
use std::sync::Once;
use serial_test::serial;

static INIT: Once = Once::new();

fn setup_tests() {
    INIT.call_once(|| {
        // Zabezpieczenie przez rzutowanie globalnego stanu na testy (poniewaz C-API w Pythonie to singelton globalny)
    });
}

// Dummy init_fn do testow (dozwolona imitacja C API na zewnątrz dla testów)
extern "C" fn test_module_init() -> *mut c_void {
    std::ptr::null_mut()
}

#[test]
#[serial]
fn test_python_config_creation() {
    let cfg = PythonEnvironmentConfig {
        dll_name: "python3.dll".to_string(),
        search_paths: vec!["lib".to_string(), "scripts".to_string()],
        enable_debug: true,
    };
    
    assert_eq!(cfg.dll_name, "python3.dll");
    assert_eq!(cfg.search_paths.len(), 2);
    assert!(cfg.enable_debug);
}

#[test]
#[serial]
fn test_python_runtime_lifecycle() {
    setup_tests();
    let _lock = PYTHON_RUNTIME_STATE.lock().unwrap();
    drop(_lock);
    
    // Upewniamy się, że środowisko jest sfinalizowane przed naszym testem
    finalize_python_runtime();

    let cfg = PythonEnvironmentConfig {
        dll_name: "python3".to_string(),
        search_paths: vec!["/tmp/python_test_paths".to_string()],
        enable_debug: false,
    };

    // Test: udana inicjalizacja
    let init_result = init_python_runtime(&cfg);
    assert!(init_result.is_ok(), "Python initialization failed");

    let is_init = PYTHON_RUNTIME_STATE.lock().unwrap().is_initialized;
    assert!(is_init, "State should be initialized");

    // Test: podwójna inicjalizacja powinna zwrócić błąd
    let second_init = init_python_runtime(&cfg);
    assert!(second_init.is_err(), "Second initialization should fail");

    // Test: weryfikacja PYTHONPATH
    let pythonpath = std::env::var("PYTHONPATH").unwrap_or_default();
    assert!(pythonpath.contains("/tmp/python_test_paths"));

    // Test: poprawne sfinalizowanie
    finalize_python_runtime();
    let is_init_after = PYTHON_RUNTIME_STATE.lock().unwrap().is_initialized;
    assert!(!is_init_after, "State should be uninitialized after finalize");
}

#[test]
#[serial]
fn test_python_execute_string() {
    setup_tests();
    let _lock = PYTHON_RUNTIME_STATE.lock().unwrap();
    drop(_lock);
    
    finalize_python_runtime();

    let cfg = PythonEnvironmentConfig {
        dll_name: "python3".to_string(),
        search_paths: vec![],
        enable_debug: true,
    };

    init_python_runtime(&cfg).unwrap();

    // Test poprawny skrypt
    let success = execute_python_string("x = 2 + 2\n");
    assert!(success, "Valid Python script should execute successfully");

    // Test niepoprawny skrypt
    let fail = execute_python_string("x = 2 + \n");
    assert!(!fail, "Invalid Python script should fail execution");
    
    finalize_python_runtime();
}

#[test]
#[serial]
fn test_python_register_builtin_module() {
    setup_tests();
    let _lock = PYTHON_RUNTIME_STATE.lock().unwrap();
    drop(_lock);
    
    finalize_python_runtime();

    let cfg = PythonEnvironmentConfig {
        dll_name: "python3".to_string(),
        search_paths: vec![],
        enable_debug: false,
    };

    init_python_runtime(&cfg).unwrap();

    // Test wywołania bindowania - ze wzgledu na brak panicow powinno przejsc
    register_builtin_module("test_mod", test_module_init);

    // Aby upewnic sie ze nie zrobilo crasha, wykonajmy cos po
    let success = execute_python_string("print('module registered successfully')\n");
    assert!(success);

    finalize_python_runtime();
}

#[test]
#[serial]
fn test_finalize_when_not_initialized() {
    setup_tests();
    let _lock = PYTHON_RUNTIME_STATE.lock().unwrap();
    drop(_lock);
    
    finalize_python_runtime();
    
    // Sfinalizowanie już sfinalizowanego środowiska nie powinno sypać błędem ani panikować
    finalize_python_runtime();
    
    let is_init = PYTHON_RUNTIME_STATE.lock().unwrap().is_initialized;
    assert!(!is_init);
}
