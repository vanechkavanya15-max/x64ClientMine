use m2_client_pyapi::py_runtime::PyRuntime;

#[test]
fn test_integration_py_runtime() {
    let mut instance = PyRuntime::new();
    assert!(!instance.is_ready());
    assert!(instance.initialize_gil().is_ok());
    assert!(instance.is_ready());
}

#[test]
fn test_gil_initialization_and_finalization() {
    let mut runtime = PyRuntime::new();
    
    // Inicjalizacja GIL
    assert!(!runtime.is_ready());
    assert!(runtime.initialize_gil().is_ok());
    assert!(runtime.is_ready());
    
    // Proba ponownej inicjalizacji powinna zwrocic blad
    assert!(runtime.initialize_gil().is_err());
    
    // Zwalnianie GIL
    assert!(runtime.finalize_gil().is_ok());
    assert!(!runtime.is_ready());
    
    // Proba ponownego zwolnienia powinna zwrocic blad
    assert!(runtime.finalize_gil().is_err());
}

#[test]
fn test_module_registration() {
    let mut runtime = PyRuntime::new();
    
    // Rejestracja modulu
    assert!(runtime.register_module("app", "Modul aplikacji").is_ok());
    assert!(runtime.register_module("player", "Modul gracza").is_ok());
    
    // Pobieranie opisu
    assert_eq!(runtime.get_module_description("app").unwrap(), "Modul aplikacji");
    assert_eq!(runtime.get_module_description("player").unwrap(), "Modul gracza");
    assert!(runtime.get_module_description("nieistniejacy").is_none());
    
    // Proba podwojnej rejestracji
    assert!(runtime.register_module("app", "Inny opis").is_err());
    
    // Lista modulow
    let modules = runtime.get_registered_modules();
    assert_eq!(modules.len(), 2);
    assert_eq!(modules[0], "app");
    assert_eq!(modules[1], "player");
}

#[test]
fn test_script_execution() {
    let mut runtime = PyRuntime::new();
    
    // Proba wykonania przed GIL
    assert!(runtime.execute_script("print('Hello')").is_err());
    
    // Wykonanie po GIL
    assert!(runtime.initialize_gil().is_ok());
    assert!(runtime.execute_script("print('Hello')").is_ok());
    assert!(runtime.execute_script("x = 5").is_ok());
    
    // Sprawdzanie historii
    let scripts = runtime.get_executed_scripts();
    assert_eq!(scripts.len(), 2);
    assert_eq!(scripts[0], "print('Hello')");
    assert_eq!(scripts[1], "x = 5");
}

#[test]
fn test_reset() {
    let mut runtime = PyRuntime::new();
    
    // Ustawienie poczatkowego stanu
    assert!(runtime.initialize_gil().is_ok());
    assert!(runtime.register_module("test_mod", "opis").is_ok());
    assert!(runtime.execute_script("test_script()").is_ok());
    
    // Reset
    runtime.reset();
    
    // Sprawdzenie stanu po resecie
    assert!(!runtime.is_ready());
    assert_eq!(runtime.get_registered_modules().len(), 0);
    assert_eq!(runtime.get_executed_scripts().len(), 0);
}
