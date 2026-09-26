use std::sync::{OnceLock, RwLock};

pub const CELLSCALE: i32 = 200;
pub const XSIZE: i32 = 128;
pub const YSIZE: i32 = 128;
pub const HEIGHTMAP_RAW_XSIZE: i32 = XSIZE + 3;
pub const HEIGHTMAP_RAW_YSIZE: i32 = YSIZE + 3;
pub const HEIGHTMAP_RAW_SIZE: usize = (HEIGHTMAP_RAW_XSIZE * HEIGHTMAP_RAW_YSIZE) as usize;

pub struct TerrainBicubicHeightV2 {
    raw_heights: Vec<u16>,
    height_scale: f32,
    wx: u16,
    wy: u16,
}

impl TerrainBicubicHeightV2 {
    pub fn new(raw_heights: Vec<u16>, height_scale: f32, wx: u16, wy: u16) -> Self {
        assert_eq!(
            raw_heights.len(),
            HEIGHTMAP_RAW_SIZE,
            "Invalid heightmap size"
        );
        Self {
            raw_heights,
            height_scale,
            wx,
            wy,
        }
    }

    #[inline]
    pub fn get_height_map_value(&self, sx: i16, sy: i16) -> u16 {
        // inline WORD GetHeightMapValue(short sx, short sy) {
        //     return m_awRawHeightMap[(sy + 1) * HEIGHTMAP_RAW_XSIZE + sx + 1];
        // }
        // C++ allows sx and sy to be -1 up to XSIZE+1 because of raw padding.
        // We will clamp to prevent out of bounds panics, although callers should respect bounds.
        let clamped_sx = sx.clamp(-1, (HEIGHTMAP_RAW_XSIZE - 2) as i16) as i32;
        let clamped_sy = sy.clamp(-1, (HEIGHTMAP_RAW_YSIZE - 2) as i16) as i32;
        
        let index = ((clamped_sy + 1) * HEIGHTMAP_RAW_XSIZE + clamped_sx + 1) as usize;
        self.raw_heights[index]
    }

    pub fn get_height(&self, mut x: i32, mut y: i32) -> f32 {
        x -= (self.wx as i32) * XSIZE * CELLSCALE;
        y -= (self.wy as i32) * YSIZE * CELLSCALE;

        if x < 0 || y < 0 || x > XSIZE * CELLSCALE || y > XSIZE * CELLSCALE {
            return 0.0;
        }

        let xdist = (x % CELLSCALE) as f32;
        let ydist = (y % CELLSCALE) as f32;

        let ooscale = 1.0 / (CELLSCALE as f32);
        
        let cell_x = (x / CELLSCALE) as i16;
        let cell_y = (y / CELLSCALE) as i16;

        let h1 = self.get_height_map_value(cell_x, cell_y) as f32 * self.height_scale;
        let h2 = self.get_height_map_value(cell_x + 1, cell_y + 1) as f32 * self.height_scale;

        if xdist <= ydist {
            let h3 = self.get_height_map_value(cell_x, cell_y + 1) as f32 * self.height_scale;
            let xslope = (h2 - h3) * ooscale;
            let yslope = (h3 - h1) * ooscale;
            h1 + (xdist * xslope + ydist * yslope)
        } else {
            let h3 = self.get_height_map_value(cell_x + 1, cell_y) as f32 * self.height_scale;
            let xslope = (h3 - h1) * ooscale;
            let yslope = (h2 - h3) * ooscale;
            h1 + (xdist * xslope + ydist * yslope)
        }
    }

    pub fn get_normal_vector(&self, x: i32, y: i32) -> (f32, f32, f32) {
        // In CTerrain::CalculateNormal(long x, long y):
        // normal.x = -m_fHeightScale * ((float)GetHeightMapValue((x-1),y)-(float)GetHeightMapValue((x+1),y));
        // normal.y = -m_fHeightScale * ((float)GetHeightMapValue(x,(y-1))-(float)GetHeightMapValue(x,(y+1)));
        // normal.z = 2.0f * static_cast<float>(CELLSCALE);
        // normal *= 127.0f / D3DXVec3Length(&normal);
        
        // This calculates the normal vector at a specific grid coordinate, not arbitrary world coordinates.
        // It expects x and y to be grid cell coordinates.
        let cell_x = x as i16;
        let cell_y = y as i16;

        let h_left = self.get_height_map_value(cell_x - 1, cell_y) as f32;
        let h_right = self.get_height_map_value(cell_x + 1, cell_y) as f32;
        let h_up = self.get_height_map_value(cell_x, cell_y - 1) as f32;
        let h_down = self.get_height_map_value(cell_x, cell_y + 1) as f32;

        let mut nx = -self.height_scale * (h_left - h_right);
        let mut ny = -self.height_scale * (h_up - h_down);
        let mut nz = 2.0 * (CELLSCALE as f32);

        let length = (nx * nx + ny * ny + nz * nz).sqrt();
        let scale = 127.0 / length;

        nx *= scale;
        ny *= scale;
        nz *= scale;

        (nx, ny, nz)
    }

    pub fn get_height_and_normal(&self, x: f32, y: f32) -> (f32, (f32, f32, f32)) {
        let h = self.get_height(x as i32, y as i32);
        
        // Calculate normal based on the local grid cell
        let local_x = (x as i32) - (self.wx as i32) * XSIZE * CELLSCALE;
        let local_y = (y as i32) - (self.wy as i32) * YSIZE * CELLSCALE;
        
        let mut cell_x = local_x / CELLSCALE;
        let mut cell_y = local_y / CELLSCALE;
        
        cell_x = cell_x.clamp(0, XSIZE);
        cell_y = cell_y.clamp(0, YSIZE);

        let n = self.get_normal_vector(cell_x, cell_y);

        (h, n)
    }
}

pub static TERRAIN_V2_CTX: OnceLock<RwLock<Option<TerrainBicubicHeightV2>>> = OnceLock::new();

pub fn get_terrain_v2_ctx() -> &'static RwLock<Option<TerrainBicubicHeightV2>> {
    TERRAIN_V2_CTX.get_or_init(|| RwLock::new(None))
}

pub fn set_terrain_v2_context_for_test(terrain: TerrainBicubicHeightV2) {
    let ctx = get_terrain_v2_ctx();
    if let Ok(mut lock) = ctx.write() {
        *lock = Some(terrain);
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct TerrainHeightAndNormalResult {
    pub height: f32,
    pub normal_x: f32,
    pub normal_y: f32,
    pub normal_z: f32,
}

#[no_mangle]
pub extern "C" fn m2_terrain_get_height_and_normal(x: f32, y: f32) -> TerrainHeightAndNormalResult {
    let ctx = get_terrain_v2_ctx();
    if let Ok(lock) = ctx.read() {
        if let Some(terrain) = &*lock {
            let (h, (nx, ny, nz)) = terrain.get_height_and_normal(x, y);
            return TerrainHeightAndNormalResult {
                height: h,
                normal_x: nx,
                normal_y: ny,
                normal_z: nz,
            };
        }
    }
    TerrainHeightAndNormalResult {
        height: 0.0,
        normal_x: 0.0,
        normal_y: 0.0,
        normal_z: 1.0,
    }
}
