use std::f32;

/// A simple 3D vector.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let len = self.length();
        if len > 0.0 {
            Self {
                x: self.x / len,
                y: self.y / len,
                z: self.z / len,
            }
        } else {
            Self { x: 0.0, y: 0.0, z: 0.0 }
        }
    }
}

/// A chunk of terrain splatting data.
/// We support up to 16 layers of splats for `Texture2DArray`.
#[derive(Debug, Clone)]
pub struct TerrainArraySplatter {
    /// 256x256 alpha map per chunk. Each element represents weights for up to 16 layers.
    /// In practice, maybe 1 byte per layer (16 bytes per texel).
    /// Size: 256 * 256 * 16 bytes = 1MB per chunk.
    pub alpha_map: Vec<u8>,
    pub width: usize,
    pub height: usize,
    pub num_layers: usize,
}

impl TerrainArraySplatter {
    /// Creates a new TerrainArraySplatter.
    pub fn new(width: usize, height: usize, num_layers: usize) -> Self {
        assert!(num_layers <= 16, "Maximum of 16 layers supported.");
        Self {
            alpha_map: vec![0; width * height * num_layers],
            width,
            height,
            num_layers,
        }
    }

    /// Sets the weight for a specific layer at (x, y).
    pub fn set_weight(&mut self, x: usize, y: usize, layer: usize, weight: u8) {
        if x < self.width && y < self.height && layer < self.num_layers {
            let index = (y * self.width + x) * self.num_layers + layer;
            self.alpha_map[index] = weight;
        }
    }

    /// Gets the weight for a specific layer at (x, y).
    pub fn get_weight(&self, x: usize, y: usize, layer: usize) -> u8 {
        if x < self.width && y < self.height && layer < self.num_layers {
            let index = (y * self.width + x) * self.num_layers + layer;
            self.alpha_map[index]
        } else {
            0
        }
    }

    /// Implements bilinear interpolation of alpha blend weights between neighboring height squares.
    pub fn get_bilinear_alpha_weight(&self, fx: f32, fy: f32, layer: usize) -> f32 {
        let x0 = fx.floor() as usize;
        let y0 = fy.floor() as usize;
        
        let tx = fx - fx.floor();
        let ty = fy - fy.floor();

        let x1 = (x0 + 1).min(self.width.saturating_sub(1));
        let y1 = (y0 + 1).min(self.height.saturating_sub(1));

        let w00 = self.get_weight(x0, y0, layer) as f32 / 255.0;
        let w10 = self.get_weight(x1, y0, layer) as f32 / 255.0;
        let w01 = self.get_weight(x0, y1, layer) as f32 / 255.0;
        let w11 = self.get_weight(x1, y1, layer) as f32 / 255.0;

        let top = w00 * (1.0 - tx) + w10 * tx;
        let bottom = w01 * (1.0 - tx) + w11 * tx;

        top * (1.0 - ty) + bottom * ty
    }
}

/// Calculate heightmap normals for Diffuse lighting (using Heightmap Normals logic from C++).
/// Takes a raw heightmap and outputs [i8; 3] normals mapped to -127..127.
pub fn calculate_heightmap_normals(
    height_map: &[u16],
    width: usize,
    height: usize,
    height_scale: f32,
    cell_scale: f32,
) -> Vec<[i8; 3]> {
    let mut normal_map = vec![[0, 0, 0]; width * height];
    
    // NormalZ = 2.0f * CELLSCALE
    let f_normal_z = 2.0 * cell_scale;
    let f_normal_scale = 127.0;

    for y in 0..height {
        for x in 0..width {
            // For edges, we clamp to border values
            let get_height = |hx: usize, hy: usize| -> f32 {
                let hx = hx.clamp(0, width - 1);
                let hy = hy.clamp(0, height - 1);
                height_map[hy * width + hx] as f32
            };

            // Calculate differences (using Central Difference)
            // Left vs Right
            let h_left = get_height(x.saturating_sub(1), y);
            let h_right = get_height(x + 1, y);
            let nx = -height_scale * (h_left - h_right);

            // Top vs Bottom
            let h_top = get_height(x, y.saturating_sub(1));
            let h_bottom = get_height(x, y + 1);
            let ny = -height_scale * (h_top - h_bottom);

            let nz = f_normal_z;

            let length = (nx * nx + ny * ny + nz * nz).sqrt();
            let inv_len = f_normal_scale / length;

            let fnx = nx * inv_len;
            let fny = ny * inv_len;
            let fnz = nz * inv_len;

            normal_map[y * width + x] = [
                fnx.round() as i8,
                fny.round() as i8,
                fnz.round() as i8,
            ];
        }
    }

    normal_map
}

