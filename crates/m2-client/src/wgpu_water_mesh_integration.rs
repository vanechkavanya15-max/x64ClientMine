use std::cmp;
use std::mem;

use crate::terrain_height_patch::{RawHeightMap, CELL_SIZE, TERRAIN_PATCHSIZE};

pub const OPAQUE_WATER_DEPTH: f32 = 400.0;
pub const TRANSPARENT_WATER_DEPTH: f32 = 0.8 * OPAQUE_WATER_DEPTH;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct WaterVertex {
    pub position: [f32; 3],
    pub diffuse: u32,
}

impl WaterVertex {
    pub const fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<WaterVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Uint32,
                },
            ],
        }
    }
}

pub fn calculate_water_fresnel(normal: glam::Vec3, view_dir: glam::Vec3, r0: f32) -> f32 {
    let cos_theta = normal.dot(view_dir).clamp(0.0, 1.0);
    r0 + (1.0 - r0) * (1.0 - cos_theta).powi(5)
}

pub fn calculate_water_foam(alpha: f32, _time: f32, foam_threshold: f32) -> f32 {
    if alpha >= foam_threshold {
        return 0.0;
    }
    (1.0 - alpha / foam_threshold).clamp(0.0, 1.0)
}

fn float_to_int(f: f32) -> u32 {
    (f as i32) as u32
}

pub struct WaterMeshBuilder;

impl WaterMeshBuilder {
    pub fn build_water_vertices(
        patch_x: usize,
        patch_y: usize,
        height_map: &RawHeightMap,
        water_map: &[u8],
        water_heights: &[i32],
        height_scale: f32,
    ) -> Vec<WaterVertex> {
        let mut vertices = Vec::new();
        
        let start_x = patch_x * TERRAIN_PATCHSIZE;
        let start_y = patch_y * TERRAIN_PATCHSIZE;
        
        let ooo_paque_water_depth = 1.0 / OPAQUE_WATER_DEPTH;

        for y in 0..TERRAIN_PATCHSIZE {
            for x in 0..TERRAIN_PATCHSIZE {
                let global_x = start_x + x;
                let global_y = start_y + y;
                
                if global_x >= height_map.width - 1 || global_y >= height_map.height - 1 {
                    continue;
                }
                
                let water_idx = global_y * height_map.width + global_x;
                if water_idx >= water_map.len() {
                    continue;
                }
                
                let by_num_water = water_map[water_idx];
                if by_num_water == 0xFF {
                    continue;
                }
                
                if (by_num_water as usize) >= water_heights.len() {
                    continue;
                }
                
                let l_water_height = water_heights[by_num_water as usize];
                if l_water_height == -1 {
                    continue;
                }
                
                let hgt = height_map.get_height(global_x, global_y);
                let hgt_x1 = height_map.get_height(global_x + 1, global_y);
                let hgt_y1 = height_map.get_height(global_x, global_y + 1);
                let hgt_xy = height_map.get_height(global_x + 1, global_y + 1);
                
                let mut diff0 = l_water_height as f32 - hgt;
                let mut diff1 = l_water_height as f32 - hgt_y1;
                let mut diff2 = l_water_height as f32 - hgt_x1;
                let mut diff3 = l_water_height as f32 - hgt_xy;
                
                diff0 = diff0.clamp(0.0, TRANSPARENT_WATER_DEPTH);
                diff1 = diff1.clamp(0.0, TRANSPARENT_WATER_DEPTH);
                diff2 = diff2.clamp(0.0, TRANSPARENT_WATER_DEPTH);
                diff3 = diff3.clamp(0.0, TRANSPARENT_WATER_DEPTH);
                
                let alpha0 = float_to_int(diff0 * ooo_paque_water_depth * 255.0);
                let alpha1 = float_to_int(diff1 * ooo_paque_water_depth * 255.0);
                let alpha2 = float_to_int(diff2 * ooo_paque_water_depth * 255.0);
                let alpha3 = float_to_int(diff3 * ooo_paque_water_depth * 255.0);
                
                let alpha_key = (alpha0 << 24) | (alpha1 << 16) | (alpha2 << 8) | alpha3;
                
                if alpha_key != 0 {
                    let fx = (global_x as f32) * CELL_SIZE;
                    let fy = (global_y as f32) * CELL_SIZE;
                    let z = (l_water_height as f32) * height_scale;
                    
                    // CCW mapping matching the C++ loop logic
                    
                    // Vertex 1: top-left
                    vertices.push(WaterVertex {
                        position: [fx, -fy, z],
                        diffuse: ((alpha0 << 24) & 0xFF000000) | 0x000000FF, // 0x000F939B from original source but uses 0xFF here
                    });
                    
                    // Vertex 2: bottom-left
                    vertices.push(WaterVertex {
                        position: [fx, -fy - CELL_SIZE, z],
                        diffuse: ((alpha1 << 24) & 0xFF000000) | 0x00FFFFFF,
                    });
                    
                    // Vertex 3: top-right
                    vertices.push(WaterVertex {
                        position: [fx + CELL_SIZE, -fy, z],
                        diffuse: ((alpha2 << 24) & 0xFF000000) | 0x00FFFFFF,
                    });
                    
                    // Vertex 4: top-right
                    vertices.push(WaterVertex {
                        position: [fx + CELL_SIZE, -fy, z],
                        diffuse: ((alpha2 << 24) & 0xFF000000) | 0x00FFFFFF,
                    });
                    
                    // Vertex 5: bottom-left
                    vertices.push(WaterVertex {
                        position: [fx, -fy - CELL_SIZE, z],
                        diffuse: ((alpha1 << 24) & 0xFF000000) | 0x00FFFFFF,
                    });
                    
                    // Vertex 6: bottom-right
                    vertices.push(WaterVertex {
                        position: [fx + CELL_SIZE, -fy - CELL_SIZE, z],
                        diffuse: ((alpha3 << 24) & 0xFF0000FF) | 0x00FFFFFF,
                    });
                }
            }
        }
        
        vertices
    }
}
