//! Skrzynka m2-client-render
//! Nowoczesny potok renderowania 64-bit wgpu/DirectX z shaderami WGSL

pub mod gpu_context;
pub mod camera_3d;
pub mod frustum_culler;
pub mod shader_manager;
pub mod pipeline_terrain;
pub mod pipeline_water;
pub mod pipeline_model;
pub mod pipeline_shadow;
pub mod pipeline_skybox;
pub mod pipeline_effect;
pub mod pipeline_ui_2d;
pub mod light_environment;
pub mod font_atlas;
pub mod render_target;
pub mod render_scene;
pub mod render_ffi;
