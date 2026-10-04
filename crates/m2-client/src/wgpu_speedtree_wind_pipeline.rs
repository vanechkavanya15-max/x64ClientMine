use glam::{Mat4, Vec3};
use std::f32::consts::PI;

/// Zwraca kod zrodlowy WGSL shadera wiatru SpeedTree
pub fn get_speedtree_wind_wgsl() -> &'static str {
    r#"
struct CameraUniform {
    view_proj: mat4x4<f32>,
    view: mat4x4<f32>,
    position: vec3<f32>,
}

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

struct WindUniform {
    matrices: array<mat4x4<f32>, 4>,
    wind_strength: f32,
    lod_alpha_3d: f32,
    lod_alpha_billboard: f32,
}

@group(1) @binding(0)
var<uniform> wind: WindUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    @location(3) corner: vec2<f32>, // offset for billboard
    @location(4) wind_index: f32,
    @location(5) wind_weight: f32,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) world_pos: vec3<f32>,
    @location(3) alpha: f32,
}

// Shader 3D (liscie, galezie) z wiatrem
@vertex
fn vs_main_3d(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    
    // Obliczenie wplywu wiatru (Wind Sway) wzorem z C++ klienta:
    // P_wind = P_orig + (WindMatrix[wind_index] * P_orig - P_orig) * wind_weight
    
    let wind_idx = u32(model.wind_index);
    let p_orig = vec4<f32>(model.position, 1.0);
    
    // Uzywamy odpowiedniej macierzy na podstawie indeksu (0-3)
    let p_wind_full = wind.matrices[wind_idx] * p_orig;
    let diff = p_wind_full - p_orig;
    let final_pos = p_orig + diff * model.wind_weight;
    
    out.world_pos = final_pos.xyz;
    out.clip_position = camera.view_proj * final_pos;
    out.tex_coords = model.uv;
    out.normal = model.normal;
    out.alpha = wind.lod_alpha_3d; // Plynne przenikanie LOD
    
    return out;
}

// Shader Billboard (distant trees LOD)
@vertex
fn vs_main_billboard(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    
    // Billboard musi byc obrocony w strone kamery.
    // model.position = center of the billboard (root)
    // model.corner = 2D offset from the center (-1..1)
    
    let center = vec4<f32>(model.position, 1.0);
    
    // Ekstrakcja wektorow Right i Up z macierzy widoku (zeby usunac rotacje kamery)
    let camera_right = vec3<f32>(camera.view[0][0], camera.view[1][0], camera.view[2][0]);
    let camera_up = vec3<f32>(camera.view[0][1], camera.view[1][1], camera.view[2][1]);
    
    // Konstrukcja pozycji wierzcholka wzgledem osi kamery
    let final_pos = center.xyz 
                  + camera_right * model.corner.x
                  + camera_up * model.corner.y;
                  
    out.world_pos = final_pos;
    out.clip_position = camera.view_proj * vec4<f32>(final_pos, 1.0);
    out.tex_coords = model.uv;
    out.normal = -vec3<f32>(camera.view[0][2], camera.view[1][2], camera.view[2][2]); // W strone kamery
    out.alpha = wind.lod_alpha_billboard; // Plynne przenikanie LOD (odwrotnosc alpha_3d)
    
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Prosty output, zeby przetestowac przejscie alpha dla LOD
    return vec4<f32>(in.normal * 0.5 + 0.5, in.alpha);
}
    "#
}

/// Oblicza przemieszczenie wierzcholka liscia wywolane wiatrem (dla testow).
/// Wzor: P_wind = P_orig + (WindMatrix[wind_index] * P_orig - P_orig) * wind_weight
pub fn compute_wind_displacement(
    position: Vec3,
    wind_matrix: Mat4,
    wind_weight: f32,
) -> Vec3 {
    let p_orig = position.extend(1.0);
    let p_wind_full = wind_matrix * p_orig;
    let diff = p_wind_full - p_orig;
    let final_pos = p_orig + diff * wind_weight;
    
    final_pos.truncate()
}

/// Oblicza pozycje wierzcholka billboardu obroconego w strone kamery (View-facing).
/// center: centrum (root) drzewa
/// offset_x, offset_y: wektor 2D (+/- width/2, height)
/// view_matrix: macierz widoku kamery
pub fn compute_billboard_view_facing(
    center: Vec3,
    offset_x: f32,
    offset_y: f32,
    view_matrix: Mat4,
) -> Vec3 {
    // Wektor prawo to pierwsza kolumna z transponowanej macierzy (czyli z macierzy widoku)
    let right = Vec3::new(view_matrix.x_axis.x, view_matrix.y_axis.x, view_matrix.z_axis.x);
    // Wektor w gore to druga kolumna 
    let up = Vec3::new(view_matrix.x_axis.y, view_matrix.y_axis.y, view_matrix.z_axis.y);
    
    center + (right * offset_x) + (up * offset_y)
}
