use m2_client_render::shader_manager::{ShaderCategory, ShaderError, ShaderManager, ShaderType};

#[test]
fn test_integration_shader_manager() {
    let instance = ShaderManager::new();
    assert!(instance.is_ready());
}

#[test]
fn test_register_shader_success() {
    let mut manager = ShaderManager::new();
    
    let result = manager.register_shader(
        "ui_shader",
        "fn main() -> vec4<f32> { return vec4<f32>(1.0); }",
        ShaderType::Wgsl,
        ShaderCategory::Interface,
    );
    
    assert!(result.is_ok());
    
    let shader = manager.get_shader("ui_shader");
    assert!(shader.is_some());
    let shader = shader.unwrap();
    assert_eq!(shader.name, "ui_shader");
    assert_eq!(shader.shader_type, ShaderType::Wgsl);
    assert_eq!(shader.category, ShaderCategory::Interface);
    assert!(!shader.is_compiled);
}

#[test]
fn test_register_shader_already_exists() {
    let mut manager = ShaderManager::new();
    
    assert!(manager.register_shader("test_shader", "content", ShaderType::Wgsl, ShaderCategory::Terrain).is_ok());
    let result = manager.register_shader("test_shader", "other content", ShaderType::Hlsl, ShaderCategory::Model3D);
    
    assert_eq!(result, Err(ShaderError::AlreadyExists));
}

#[test]
fn test_register_shader_empty_source() {
    let mut manager = ShaderManager::new();
    
    let result = manager.register_shader("empty_shader", "   \n  \t ", ShaderType::Wgsl, ShaderCategory::Terrain);
    assert_eq!(result, Err(ShaderError::EmptySource));
}

#[test]
fn test_compile_shader_success() {
    let mut manager = ShaderManager::new();
    manager.register_shader("test_shader", "valid_source", ShaderType::Wgsl, ShaderCategory::Terrain).unwrap();
    
    let result = manager.compile_shader("test_shader");
    assert!(result.is_ok());
    
    let shader = manager.get_shader("test_shader").unwrap();
    assert!(shader.is_compiled);
}

#[test]
fn test_compile_shader_not_found() {
    let mut manager = ShaderManager::new();
    let result = manager.compile_shader("non_existent");
    assert_eq!(result, Err(ShaderError::NotFound));
}

#[test]
fn test_compile_shader_failed() {
    let mut manager = ShaderManager::new();
    manager.register_shader("bad_shader", "invalid_syntax", ShaderType::Wgsl, ShaderCategory::Terrain).unwrap();
    
    let result = manager.compile_shader("bad_shader");
    assert_eq!(result, Err(ShaderError::CompilationFailed));
    
    let shader = manager.get_shader("bad_shader").unwrap();
    assert!(!shader.is_compiled);
}

#[test]
fn test_get_shaders_by_category() {
    let mut manager = ShaderManager::new();
    
    manager.register_shader("s1", "s", ShaderType::Wgsl, ShaderCategory::Terrain).unwrap();
    manager.register_shader("s2", "s", ShaderType::Wgsl, ShaderCategory::Model3D).unwrap();
    manager.register_shader("s3", "s", ShaderType::Wgsl, ShaderCategory::Terrain).unwrap();
    
    let terrain_shaders = manager.get_shaders_by_category(ShaderCategory::Terrain);
    assert_eq!(terrain_shaders.len(), 2);
    
    let model_shaders = manager.get_shaders_by_category(ShaderCategory::Model3D);
    assert_eq!(model_shaders.len(), 1);
    
    let interface_shaders = manager.get_shaders_by_category(ShaderCategory::Interface);
    assert_eq!(interface_shaders.len(), 0);
}

#[test]
fn test_remove_shader() {
    let mut manager = ShaderManager::new();
    manager.register_shader("s1", "s", ShaderType::Wgsl, ShaderCategory::Terrain).unwrap();
    
    assert!(manager.get_shader("s1").is_some());
    
    let result = manager.remove_shader("s1");
    assert!(result.is_ok());
    
    assert!(manager.get_shader("s1").is_none());
    
    let result = manager.remove_shader("s1");
    assert_eq!(result, Err(ShaderError::NotFound));
}

#[test]
fn test_clear() {
    let mut manager = ShaderManager::new();
    manager.register_shader("s1", "s", ShaderType::Wgsl, ShaderCategory::Terrain).unwrap();
    manager.register_shader("s2", "s", ShaderType::Wgsl, ShaderCategory::Model3D).unwrap();
    
    assert_eq!(manager.get_shaders_by_category(ShaderCategory::Terrain).len(), 1);
    
    manager.clear();
    
    assert!(manager.get_shader("s1").is_none());
    assert!(manager.get_shader("s2").is_none());
}
