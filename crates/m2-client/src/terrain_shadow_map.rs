pub const SHADOWMAP_XSIZE: usize = 256;
pub const SHADOWMAP_YSIZE: usize = 256;
pub const TERRAIN_SIZE: f32 = 128.0;

pub struct TerrainShadowMap {
    pub shadow_map: [u16; SHADOWMAP_XSIZE * SHADOWMAP_YSIZE],
}

impl TerrainShadowMap {
    pub fn new() -> Self {
        Self {
            shadow_map: [0xFFFF; SHADOWMAP_XSIZE * SHADOWMAP_YSIZE],
        }
    }

    pub fn from_raw(data: &[u16]) -> Self {
        let mut map = Self::new();
        let len = std::cmp::min(data.len(), map.shadow_map.len());
        map.shadow_map[..len].copy_from_slice(&data[..len]);
        map
    }

    pub fn get_shadow_color_at(&self, fx: f32, fy: f32) -> u32 {
        if fx < 0.0 || fy < 0.0 || fx >= TERRAIN_SIZE || fy >= TERRAIN_SIZE {
            return 0xFFFFFFFF;
        }

        let x = ((fx / TERRAIN_SIZE) * 255.0).clamp(0.0, 255.0) as usize;
        let y = ((fy / TERRAIN_SIZE) * 255.0).clamp(0.0, 255.0) as usize;

        let val = self.shadow_map[y * SHADOWMAP_XSIZE + x];

        let r = (((val >> 11) & 0x1F) as u32 * 255) / 31;
        let g = (((val >> 5) & 0x3F) as u32 * 255) / 63;
        let b = ((val & 0x1F) as u32 * 255) / 31;

        0xFF000000 | (r << 16) | (g << 8) | b
    }
}
