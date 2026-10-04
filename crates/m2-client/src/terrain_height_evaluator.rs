#[allow(dead_code)]
pub const TERRAIN_SIZE: usize = 128;
pub const CELLSCALE: usize = 200;
pub const HEIGHTMAP_RAW_XSIZE: usize = TERRAIN_SIZE + 3;
pub const HEIGHTMAP_RAW_YSIZE: usize = TERRAIN_SIZE + 3;
pub const ATTRMAP_XSIZE: usize = TERRAIN_SIZE * 2;
pub const ATTRMAP_YSIZE: usize = TERRAIN_SIZE * 2;
pub const TERRAIN_XSIZE: usize = TERRAIN_SIZE * CELLSCALE;
pub const TERRAIN_YSIZE: usize = TERRAIN_SIZE * CELLSCALE;

#[allow(dead_code)]
pub const ATTRIBUTE_BLOCK: u8 = 1 << 0;
pub const ATTRIBUTE_WATER: u8 = 1 << 1;
pub const ATTRIBUTE_BANPK: u8 = 1 << 2;

#[derive(Clone)]
pub struct TerrainHeightEvaluator {
    pub raw_height_map: Vec<u16>,
    pub attr_map: Vec<u8>,
    pub height_scale: f32,
    pub map_x: u16,
    pub map_y: u16,
}

impl TerrainHeightEvaluator {
    pub fn new(raw_height_map: Vec<u16>, attr_map: Vec<u8>, height_scale: f32, map_x: u16, map_y: u16) -> Self {
        assert_eq!(raw_height_map.len(), HEIGHTMAP_RAW_XSIZE * HEIGHTMAP_RAW_YSIZE, "Invalid heightmap size");
        assert_eq!(attr_map.len(), ATTRMAP_XSIZE * ATTRMAP_YSIZE, "Invalid attrmap size");

        Self {
            raw_height_map,
            attr_map,
            height_scale,
            map_x,
            map_y,
        }
    }

    pub fn get_height_map_value(&self, sx: i32, sy: i32) -> u16 {
        // According to Terrain.h: m_awRawHeightMap[(sy + 1) * HEIGHTMAP_RAW_XSIZE + sx + 1]
        let x = (sx + 1).clamp(0, HEIGHTMAP_RAW_XSIZE as i32 - 1) as usize;
        let y = (sy + 1).clamp(0, HEIGHTMAP_RAW_YSIZE as i32 - 1) as usize;
        self.raw_height_map[y * HEIGHTMAP_RAW_XSIZE + x]
    }

    pub fn get_height(&self, x: f32, y: f32) -> f32 {
        let mut ix = x as i32;
        let mut iy = y as i32;

        ix -= self.map_x as i32 * TERRAIN_SIZE as i32 * CELLSCALE as i32;
        iy -= self.map_y as i32 * TERRAIN_SIZE as i32 * CELLSCALE as i32;

        if ix < 0 || iy < 0 || ix > (TERRAIN_SIZE * CELLSCALE) as i32 || iy > (TERRAIN_SIZE * CELLSCALE) as i32 {
            return 0.0;
        }

        let mut p = [[0.0; 4]; 4];

        let cell_x = ix / CELLSCALE as i32;
        let cell_y = iy / CELLSCALE as i32;

        for j in -1..=2 {
            for i in -1..=2 {
                let val = self.get_height_map_value(cell_x + i, cell_y + j) as f32 * self.height_scale;
                p[(j + 1) as usize][(i + 1) as usize] = val;
            }
        }

        let dx = (ix % CELLSCALE as i32) as f32 / CELLSCALE as f32;
        let dy = (iy % CELLSCALE as i32) as f32 / CELLSCALE as f32;

        fn cubic_interpolate(p: &[f32; 4], x: f32) -> f32 {
            p[1] + 0.5 * x * (p[2] - p[0] + x * (2.0 * p[0] - 5.0 * p[1] + 4.0 * p[2] - p[3] + x * (3.0 * (p[1] - p[2]) + p[3] - p[0])))
        }

        let mut arr = [0.0; 4];
        arr[0] = cubic_interpolate(&p[0], dx);
        arr[1] = cubic_interpolate(&p[1], dx);
        arr[2] = cubic_interpolate(&p[2], dx);
        arr[3] = cubic_interpolate(&p[3], dx);

        cubic_interpolate(&arr, dy)
    }

    pub fn is_attr_on(&self, x: f32, y: f32, attr_flag: u8) -> bool {
        let mut ix = x as i32;
        let mut iy = y as i32;

        ix -= self.map_x as i32 * TERRAIN_SIZE as i32 * CELLSCALE as i32;
        iy -= self.map_y as i32 * TERRAIN_SIZE as i32 * CELLSCALE as i32;

        if ix < 0 || iy < 0 || ix >= (TERRAIN_SIZE * CELLSCALE) as i32 || iy >= (TERRAIN_SIZE * CELLSCALE) as i32 {
            return false;
        }

        let attr_x = ix / (CELLSCALE as i32 / 2);
        let attr_y = iy / (CELLSCALE as i32 / 2);

        if attr_x >= ATTRMAP_XSIZE as i32 || attr_y >= ATTRMAP_YSIZE as i32 {
            return false;
        }

        let map_attr = self.attr_map[attr_y as usize * ATTRMAP_XSIZE + attr_x as usize];

        if attr_flag < 16 {
            (map_attr & attr_flag) != 0
        } else {
            (attr_flag / 16) == (map_attr / 16)
        }
    }
}

impl TerrainHeightEvaluator {
    /// Performs a line-of-sight raycast between two terrain points to check for collisions against the terrain surface.
    /// Returns `true` if there is a line of sight (no collision), `false` if it hits the terrain.
    pub fn raycast(&self, x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32) -> bool {
        let dx = x2 - x1;
        let dy = y2 - y1;
        let dz = z2 - z1;
        let dist = (dx * dx + dy * dy).sqrt();
        if dist < 1.0 {
            return true;
        }

        let steps = (dist / (CELLSCALE as f32 / 2.0)).ceil() as i32;
        let step_x = dx / steps as f32;
        let step_y = dy / steps as f32;
        let step_z = dz / steps as f32;

        let mut current_x = x1;
        let mut current_y = y1;
        let mut current_z = z1;

        for _ in 0..steps {
            current_x += step_x;
            current_y += step_y;
            current_z += step_z;

            let h = self.get_height(current_x, current_y);
            if current_z < h {
                return false;
            }
        }

        true
    }
}

#[no_mangle]
pub extern "C" fn m2_terrain_get_height_precise(evaluator: *const TerrainHeightEvaluator, x: f32, y: f32) -> f32 {
    if evaluator.is_null() {
        return 0.0;
    }
    let eval = unsafe { &*evaluator };
    eval.get_height(x, y)
}
