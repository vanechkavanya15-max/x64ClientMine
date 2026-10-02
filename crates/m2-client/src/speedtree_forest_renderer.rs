use std::ffi::{c_char, c_void};
use std::slice;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BranchVertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub uv: [f32; 2],
    pub wind_index: f32,
    pub wind_weight: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LeafVertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub uv: [f32; 2],
    pub corner: [f32; 2], // 2D offset for billboard
    pub wind_index: f32,
    pub wind_weight: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Material {
    pub ambient: [f32; 4],
    pub diffuse: [f32; 4],
    pub specular: [f32; 4],
    pub emission: [f32; 4],
    pub shininess: f32,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct TreeGeometry {
    pub branches: Vec<BranchVertex>,
    pub branches_indices: Vec<u16>,
    
    pub fronds: Vec<BranchVertex>,
    pub fronds_indices: Vec<u16>,
    
    pub leaves: Vec<LeafVertex>,
    
    pub branch_material: Material,
    pub frond_material: Material,
    pub leaf_material: Material,
    
    pub extents: [f32; 6], // min x, min y, min z, max x, max y, max z
}

impl Default for Material {
    fn default() -> Self {
        Self {
            ambient: [0.2, 0.2, 0.2, 1.0],
            diffuse: [0.8, 0.8, 0.8, 1.0],
            specular: [0.0, 0.0, 0.0, 1.0],
            emission: [0.0, 0.0, 0.0, 1.0],
            shininess: 0.0,
        }
    }
}

pub const NUM_WIND_MATRICES: usize = 4;

#[derive(Debug, Clone)]
pub struct WindMatrices {
    pub matrices: [[f32; 16]; NUM_WIND_MATRICES],
    pub wind_strength: f32,
    pub matrix_times: [f32; NUM_WIND_MATRICES],
    pub old_strength: f32,
    pub time_of_last_call: f32,
}

impl Default for WindMatrices {
    fn default() -> Self {
        Self {
            matrices: [[0.0; 16]; NUM_WIND_MATRICES],
            wind_strength: 0.0,
            matrix_times: [0.0; NUM_WIND_MATRICES],
            old_strength: 0.0,
            time_of_last_call: 0.0,
        }
    }
}

impl WindMatrices {
    pub fn setup_wind_matrices(&mut self, time_in_secs: f32) {
        let frequencies: [[f32; 2]; NUM_WIND_MATRICES] = [
            [0.15, 0.17],
            [0.25, 0.15],
            [0.19, 0.05],
            [0.15, 0.22],
        ];

        let time_since_last_call = time_in_secs - self.time_of_last_call;
        self.time_of_last_call = time_in_secs;

        for i in 0..NUM_WIND_MATRICES {
            self.matrix_times[i] += time_since_last_call;
        }

        let base_angle = self.wind_strength * 35.0;

        for j in 0..NUM_WIND_MATRICES {
            if self.wind_strength != 0.0 {
                self.matrix_times[j] = (self.matrix_times[j] * self.old_strength) / self.wind_strength;
            }

            let base_freq = self.wind_strength * 20.0;
            let freq = frequencies[j % NUM_WIND_MATRICES];
            
            let x_percent = (base_freq * freq[0] * self.matrix_times[j]).sin();
            let y_percent = (base_freq * freq[1] * self.matrix_times[j]).cos();

            let deg_to_rad = 57.2957795;
            let sin_x = (base_angle * x_percent / deg_to_rad).sin();
            let sin_y = (base_angle * y_percent / deg_to_rad).sin();
            let cos_x = (base_angle * x_percent / deg_to_rad).cos();
            let cos_y = (base_angle * y_percent / deg_to_rad).cos();

            let mut matrix = [0.0; 16];
            matrix[0] = cos_y;
            matrix[2] = -sin_y;
            matrix[4] = sin_x * sin_y;
            matrix[5] = cos_x;
            matrix[6] = sin_x * cos_y;
            matrix[8] = sin_y * cos_x;
            matrix[9] = -sin_x;
            matrix[10] = cos_x * cos_y;
            matrix[15] = 1.0;

            self.matrices[j] = matrix;
        }

        self.old_strength = self.wind_strength;
    }
}

use crate::speedtree_forest_mgr_v2::SptDefinition;
use m2_client_bridge::ffi_guard::safe_ffi_call;
use std::sync::{OnceLock, RwLock};
use std::collections::HashMap;
use crate::speedtree_billboard_lod_v2::{calculate_speedtree_lod, SpeedTreeLodInfo};

static SPEEDTREE_RENDERER_CACHE: OnceLock<RwLock<HashMap<u32, TreeGeometry>>> = OnceLock::new();

fn get_renderer_cache() -> &'static RwLock<HashMap<u32, TreeGeometry>> {
    SPEEDTREE_RENDERER_CACHE.get_or_init(|| RwLock::new(HashMap::new()))
}

#[no_mangle]
pub extern "C" fn m2_speedtree_parse_geometry(tree_id: u32, data_ptr: *const u8, data_len: u32) -> bool {
    safe_ffi_call(false, || {
        if data_ptr.is_null() || data_len == 0 {
            return false;
        }

        let data = unsafe { slice::from_raw_parts(data_ptr, data_len as usize) };
        if let Ok(spt) = SptDefinition::parse(data) {
            let mut geometry = TreeGeometry {
                branches: vec![],
                branches_indices: vec![],
                fronds: vec![],
                fronds_indices: vec![],
                leaves: vec![],
                branch_material: Material::default(),
                frond_material: Material::default(),
                leaf_material: Material::default(),
                extents: [f32::MAX, f32::MAX, f32::MAX, f32::MIN, f32::MIN, f32::MIN],
            };

            let mut min_x = f32::MAX; let mut min_y = f32::MAX; let mut min_z = f32::MAX;
            let mut max_x = f32::MIN; let mut max_y = f32::MIN; let mut max_z = f32::MIN;

            let mut offset = 0;
            let data = &spt.raw_data;

            let read_u32 = |offset: &mut usize| -> Option<u32> {
                if *offset + 4 <= data.len() {
                    let val = u32::from_le_bytes([data[*offset], data[*offset + 1], data[*offset + 2], data[*offset + 3]]);
                    *offset += 4;
                    Some(val)
                } else {
                    None
                }
            };

            let read_f32 = |offset: &mut usize| -> Option<f32> {
                if *offset + 4 <= data.len() {
                    let val = f32::from_le_bytes([data[*offset], data[*offset + 1], data[*offset + 2], data[*offset + 3]]);
                    *offset += 4;
                    Some(val)
                } else {
                    None
                }
            };

            if let Some(num_branches) = read_u32(&mut offset) {
                for i in 0..num_branches {
                    if let (Some(px), Some(py), Some(pz), Some(nx), Some(ny), Some(nz), Some(u), Some(v)) = 
                        (read_f32(&mut offset), read_f32(&mut offset), read_f32(&mut offset),
                         read_f32(&mut offset), read_f32(&mut offset), read_f32(&mut offset),
                         read_f32(&mut offset), read_f32(&mut offset)) 
                    {
                        min_x = min_x.min(px); max_x = max_x.max(px);
                        min_y = min_y.min(py); max_y = max_y.max(py);
                        min_z = min_z.min(pz); max_z = max_z.max(pz);

                        geometry.branches.push(BranchVertex {
                            position: [px, py, pz],
                            normal: [nx, ny, nz],
                            uv: [u, v],
                            wind_index: (i as usize % NUM_WIND_MATRICES) as f32,
                            wind_weight: 1.0,
                        });
                        geometry.branches_indices.push(geometry.branches.len() as u16 - 1);
                    }
                }
            }

            if let Some(num_leaves) = read_u32(&mut offset) {
                for i in 0..num_leaves {
                    if let (Some(px), Some(py), Some(pz), Some(nx), Some(ny), Some(nz), Some(u), Some(v)) = 
                        (read_f32(&mut offset), read_f32(&mut offset), read_f32(&mut offset),
                         read_f32(&mut offset), read_f32(&mut offset), read_f32(&mut offset),
                         read_f32(&mut offset), read_f32(&mut offset)) 
                    {
                        geometry.leaves.push(LeafVertex {
                            position: [px, py, pz],
                            normal: [nx, ny, nz],
                            uv: [u, v],
                            corner: [0.0, 0.0],
                            wind_index: (i as usize % NUM_WIND_MATRICES) as f32,
                            wind_weight: 1.0,
                        });
                    }
                }
            }

            geometry.extents = [min_x, min_y, min_z, max_x, max_y, max_z];

            if let Ok(mut cache) = get_renderer_cache().write() {
                cache.insert(tree_id, geometry);
                return true;
            }
        }
        false
    })
}

#[no_mangle]
pub extern "C" fn m2_speedtree_get_vertex_buffer(tree_id: u32, out_ptr: *mut *const BranchVertex, out_count: *mut u32) -> bool {
    safe_ffi_call(false, || {
        if out_ptr.is_null() || out_count.is_null() {
            return false;
        }

        if let Ok(cache) = get_renderer_cache().read() {
            if let Some(geometry) = cache.get(&tree_id) {
                unsafe {
                    *out_ptr = geometry.branches.as_ptr();
                    *out_count = geometry.branches.len() as u32;
                }
                return true;
            }
        }
        false
    })
}

pub fn clear_geometry_cache() {
    if let Ok(mut cache) = get_renderer_cache().write() {
        cache.clear();
    }
}

pub fn calculate_tree_lod_alpha(distance: f32) -> SpeedTreeLodInfo {
    calculate_speedtree_lod(distance)
}

#[no_mangle]
pub extern "C" fn m2_speedtree_get_lod_alpha(distance: f32, out_3d_alpha: *mut f32, out_billboard_alpha: *mut f32) -> bool {
    safe_ffi_call(false, || {
        if out_3d_alpha.is_null() || out_billboard_alpha.is_null() {
            return false;
        }
        let lod = calculate_tree_lod_alpha(distance);
        unsafe {
            *out_3d_alpha = lod.alpha_3d;
            *out_billboard_alpha = lod.alpha_billboard;
        }
        true
    })
}

pub const WIND_SHADER_WGSL: &str = r#"
struct WindMatrices {
    matrices: array<mat4x4<f32>, 4>,
};

@group(0) @binding(0) var<uniform> wind: WindMatrices;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    @location(3) wind_index: f32,
    @location(4) wind_weight: f32,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) normal: vec3<f32>,
};

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    
    let index = i32(model.wind_index);
    let wind_matrix = wind.matrices[index];
    
    // Apply wind matrix if weight > 0
    var pos = vec4<f32>(model.position, 1.0);
    if (model.wind_weight > 0.0) {
        let wind_offset = (wind_matrix * pos).xyz - pos.xyz;
        pos = vec4<f32>(pos.xyz + (wind_offset * model.wind_weight), 1.0);
    }
    
    out.clip_position = pos;
    out.uv = model.uv;
    out.normal = model.normal;
    return out;
}
"#;

pub const LEAF_SHADER_WGSL: &str = r#"
struct CameraUniforms {
    view_proj: mat4x4<f32>,
    camera_pos: vec3<f32>,
};

struct WindMatrices {
    matrices: array<mat4x4<f32>, 4>,
};

@group(0) @binding(0) var<uniform> camera: CameraUniforms;
@group(0) @binding(1) var<uniform> wind: WindMatrices;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    @location(3) corner: vec2<f32>,
    @location(4) wind_index: f32,
    @location(5) wind_weight: f32,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) normal: vec3<f32>,
};

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    
    let index = i32(model.wind_index);
    let wind_matrix = wind.matrices[index];
    
    // Apply wind matrix
    var base_pos = vec4<f32>(model.position, 1.0);
    if (model.wind_weight > 0.0) {
        let wind_offset = (wind_matrix * base_pos).xyz - base_pos.xyz;
        base_pos = vec4<f32>(base_pos.xyz + (wind_offset * model.wind_weight), 1.0);
    }
    
    // Billboard offset
    let right = vec3<f32>(camera.view_proj[0][0], camera.view_proj[1][0], camera.view_proj[2][0]);
    let up = vec3<f32>(camera.view_proj[0][1], camera.view_proj[1][1], camera.view_proj[2][1]);
    
    let offset_pos = base_pos.xyz + (right * model.corner.x) + (up * model.corner.y);
    
    out.clip_position = camera.view_proj * vec4<f32>(offset_pos, 1.0);
    out.uv = model.uv;
    out.normal = model.normal;
    return out;
}
"#;
