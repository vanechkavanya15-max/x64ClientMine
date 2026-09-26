pub const TERRAIN_PATCHSIZE: usize = 16;
pub const TERRAIN_PATCHCOUNT: usize = 8;
pub const CELL_SIZE: f32 = 200.0;

#[derive(Debug, Clone, PartialEq)]
pub struct TerrainPatch {
    pub patch_x: usize,
    pub patch_y: usize,
    pub min_z: f32,
    pub max_z: f32,
    pub dominant_texture_id: u8,
}

#[derive(Debug, Clone)]
pub struct RawHeightMap {
    pub width: usize,
    pub height: usize,
    pub heights: Vec<f32>,
}

impl RawHeightMap {
    pub fn new(width: usize, height: usize, heights: Vec<f32>) -> Self {
        assert_eq!(
            width * height,
            heights.len(),
            "Invalid height map dimensions"
        );
        Self {
            width,
            height,
            heights,
        }
    }

    pub fn get_height(&self, x: usize, y: usize) -> f32 {
        let x = x.min(self.width.saturating_sub(1));
        let y = y.min(self.height.saturating_sub(1));
        self.heights[y * self.width + x]
    }
}

#[derive(Debug, Clone)]
pub struct HeightPatchGrid {
    pub patches: Vec<TerrainPatch>,
}

impl HeightPatchGrid {
    pub fn build(height_map: &RawHeightMap) -> Self {
        let mut patches = Vec::with_capacity(TERRAIN_PATCHCOUNT * TERRAIN_PATCHCOUNT);

        for py in 0..TERRAIN_PATCHCOUNT {
            for px in 0..TERRAIN_PATCHCOUNT {
                let mut min_z = f32::MAX;
                let mut max_z = f32::MIN;

                let start_x = px * TERRAIN_PATCHSIZE;
                let start_y = py * TERRAIN_PATCHSIZE;

                // Iterate over the 16x16 vertices of the patch to find min_z and max_z
                for dy in 0..TERRAIN_PATCHSIZE {
                    for dx in 0..TERRAIN_PATCHSIZE {
                        let vx = start_x + dx;
                        let vy = start_y + dy;

                        let h = height_map.get_height(vx, vy);
                        if h < min_z {
                            min_z = h;
                        }
                        if h > max_z {
                            max_z = h;
                        }
                    }
                }

                if min_z == f32::MAX {
                    min_z = 0.0;
                }
                if max_z == f32::MIN {
                    max_z = 0.0;
                }

                patches.push(TerrainPatch {
                    patch_x: px,
                    patch_y: py,
                    min_z,
                    max_z,
                    dominant_texture_id: 0,
                });
            }
        }

        Self { patches }
    }

    pub fn ray_intersects_patch(
        &self,
        patch_x: usize,
        patch_y: usize,
        ray_start: (f32, f32, f32),
        ray_dir: (f32, f32, f32),
    ) -> bool {
        let patch = self
            .patches
            .iter()
            .find(|p| p.patch_x == patch_x && p.patch_y == patch_y);
            
        let patch = match patch {
            Some(p) => p,
            None => return false,
        };

        let min_x = (patch.patch_x * TERRAIN_PATCHSIZE) as f32 * CELL_SIZE;
        let min_y = (patch.patch_y * TERRAIN_PATCHSIZE) as f32 * CELL_SIZE;
        let max_x = min_x + (TERRAIN_PATCHSIZE as f32 * CELL_SIZE);
        let max_y = min_y + (TERRAIN_PATCHSIZE as f32 * CELL_SIZE);

        let min_z = patch.min_z;
        let max_z = patch.max_z;

        let bounds_min = [min_x, min_y, min_z];
        let bounds_max = [max_x, max_y, max_z];
        let ray_orig = [ray_start.0, ray_start.1, ray_start.2];
        let ray_dir_vec = [ray_dir.0, ray_dir.1, ray_dir.2];

        let mut tmin = 0.0f32;
        let mut tmax = f32::MAX;

        for i in 0..3 {
            if ray_dir_vec[i].abs() < 1e-6 {
                if ray_orig[i] < bounds_min[i] || ray_orig[i] > bounds_max[i] {
                    return false;
                }
            } else {
                let ood = 1.0 / ray_dir_vec[i];
                let mut t1 = (bounds_min[i] - ray_orig[i]) * ood;
                let mut t2 = (bounds_max[i] - ray_orig[i]) * ood;

                if t1 > t2 {
                    std::mem::swap(&mut t1, &mut t2);
                }

                if t1 > tmin {
                    tmin = t1;
                }
                if t2 < tmax {
                    tmax = t2;
                }

                if tmin > tmax {
                    return false;
                }
            }
        }

        true
    }
}
