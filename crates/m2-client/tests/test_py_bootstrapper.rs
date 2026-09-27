use m2_client::py_bootstrapper::{
    clear_registered_modules, get_registered_modules, load_initial_game_scripts,
    py_init_module, register_builtin_modules, register_module_app, register_module_chr,
    register_module_net, register_module_player, run_python_bootstrap, BootstrapperConfig,
    BootstrapperError, PyMethodDef,
};
use serial_test::serial;

#[test]
#[serial]
fn test_empty_root_script_fails() {
    let config = BootstrapperConfig {
        root_script: "".to_string(),
        import_modules: vec!["system".to_string()],
    };

    let result = run_python_bootstrap(&config);
    assert!(result.is_err());
    assert!(matches!(result.err().unwrap(), BootstrapperError::EmptyRootScript));
}

#[test]
#[serial]
fn test_empty_module_in_list_fails() {
    let config = BootstrapperConfig {
        root_script: "prototype.py".to_string(),
        import_modules: vec!["system".to_string(), "".to_string()],
    };

    let result = run_python_bootstrap(&config);
    assert!(result.is_err());
    assert!(matches!(result.err().unwrap(), BootstrapperError::ModuleImportFailed(_)));
}

#[test]
#[serial]
fn test_load_initial_game_scripts_fails_in_test_mode() {
    let result = load_initial_game_scripts();
    assert!(result.is_err());
    assert!(matches!(result.err().unwrap(), BootstrapperError::ModuleImportFailed(_)));
}

#[test]
#[serial]
fn test_run_python_bootstrap_fails_module_in_test_mode() {
    let config = BootstrapperConfig {
        root_script: "prototype.py".to_string(),
        import_modules: vec!["system".to_string()],
    };

    let result = run_python_bootstrap(&config);
    assert!(result.is_err());
    assert!(matches!(result.err().unwrap(), BootstrapperError::ModuleImportFailed(_)));
}

#[test]
#[serial]
fn test_register_builtin_modules_app_player_chr_net() {
    clear_registered_modules();

    register_builtin_modules();

    let modules = get_registered_modules();
    assert!(modules.contains(&"app".to_string()), "app module must be registered");
    assert!(modules.contains(&"player".to_string()), "player module must be registered");
    assert!(modules.contains(&"chr".to_string()), "chr module must be registered");
    assert!(modules.contains(&"net".to_string()), "net module must be registered");
}

#[test]
#[serial]
fn test_register_individual_modules() {
    clear_registered_modules();

    register_module_app();
    register_module_player();
    register_module_chr();
    register_module_net();

    let modules = get_registered_modules();
    assert_eq!(modules.len(), 4);
}

#[test]
#[serial]
fn test_py_init_module_null_safety() {
    static SENTINEL: PyMethodDef = PyMethodDef {
        ml_name: std::ptr::null(),
        ml_meth: std::ptr::null(),
        ml_flags: 0,
        ml_doc: std::ptr::null(),
    };

    unsafe {
        // Should not panic or crash even without python27.dll loaded
        let _ = py_init_module("test_module", &SENTINEL);
    }
}

#[test]
#[serial]
fn test_registered_module_methods() {
    let app_methods = m2_client::py_bootstrapper::get_module_methods("app");
    assert!(app_methods.contains(&"GetWidth".to_string()));
    assert!(app_methods.contains(&"GetHeight".to_string()));
    assert!(app_methods.contains(&"GetTime".to_string()));
    assert!(app_methods.contains(&"GetGlobalTime".to_string()));
    assert!(app_methods.contains(&"Exit".to_string()));
    assert!(app_methods.contains(&"SetCamera".to_string()));
    assert!(app_methods.contains(&"IsPressed".to_string()));
    assert!(app_methods.contains(&"EnableSpecialCameraMode".to_string()));

    let player_methods = m2_client::py_bootstrapper::get_module_methods("player");
    assert!(player_methods.contains(&"GetStatus".to_string()));
    assert!(player_methods.contains(&"SetStatus".to_string()));
    assert!(player_methods.contains(&"GetItemIndex".to_string()));
    assert!(player_methods.contains(&"GetItemCount".to_string()));
    assert!(player_methods.contains(&"GetItemCountByVnum".to_string()));
    assert!(player_methods.contains(&"PickCloseItem".to_string()));
    assert!(player_methods.contains(&"SendClickItem".to_string()));
    assert!(player_methods.contains(&"GetMainCharacterPosition".to_string()));

    let chr_methods = m2_client::py_bootstrapper::get_module_methods("chr");
    assert!(chr_methods.contains(&"GetInstance".to_string()));
    assert!(chr_methods.contains(&"SelectInstance".to_string()));
    assert!(chr_methods.contains(&"BlendLoopMotion".to_string()));
    assert!(chr_methods.contains(&"GetPixelPosition".to_string()));
    assert!(chr_methods.contains(&"GetName".to_string()));

    let net_methods = m2_client::py_bootstrapper::get_module_methods("net");
    assert!(net_methods.contains(&"Connect".to_string()));
    assert!(net_methods.contains(&"Disconnect".to_string()));
    assert!(net_methods.contains(&"GetMainStream".to_string()));
    assert!(net_methods.contains(&"SendChatPacket".to_string()));
}
