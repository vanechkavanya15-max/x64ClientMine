use crate::terrain_height_patch::{RawHeightMap, TERRAIN_PATCHCOUNT};
use crate::terrain_water_map::WaterMap;
use crate::wgpu_water_mesh_integration::{WaterMeshBuilder, WaterVertex};

pub struct WaterTileMeshLoader;

impl WaterTileMeshLoader {
    pub fn load_sector_water(
        water_data: &[u8],
        height_map: &RawHeightMap,
        height_scale: f32,
    ) -> Result<Vec<WaterVertex>, &'static str> {
        let water_map = WaterMap::from_bytes(water_data)?;
        
        let mut water_heights = Vec::new();
        let mut raw_grid = vec![0xFFu8; 128 * 128];

        for y in 0..128 {
            for x in 0..128 {
                let idx = y * 128 + x;
                if water_map.has_water(x as u8, y as u8) {
                    if let Some(h) = water_map.get_water_height(x as u8, y as u8) {
                        let h_int = h as i32;
                        let height_idx = if let Some(i) = water_heights.iter().position(|&x| x == h_int) {
                            i
                        } else {
                            let new_idx = water_heights.len();
                            water_heights.push(h_int);
                            new_idx
                        };
                        
                        // Prevents overflow in the extreme edge case
                        if height_idx < 255 {
                            raw_grid[idx] = height_idx as u8;
                        }
                    }
                }
            }
        }

        let mut vertices = Vec::new();
        for py in 0..TERRAIN_PATCHCOUNT {
            for px in 0..TERRAIN_PATCHCOUNT {
                let patch_verts = WaterMeshBuilder::build_water_vertices(
                    px,
                    py,
                    height_map,
                    &raw_grid,
                    &water_heights,
                    height_scale,
                );
                vertices.extend(patch_verts);
            }
        }
        
        Ok(vertices)
    }
}
