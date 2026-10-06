pub const CELLSCALE: usize = 200;
pub const XSIZE: usize = 128;
pub const YSIZE: usize = 128;

pub struct TerrainHeightData {
    pub raw_heights: [u16; (XSIZE + 1) * (YSIZE + 1)],
    pub height_scale: f32,
    pub terrain_x: i32,
    pub terrain_y: i32,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct TerrainSnapperResult {
    pub z_height: f32,
    pub pitch: f32,
    pub roll: f32,
    pub normal: (f32, f32, f32),
}

impl TerrainHeightData {
    pub fn new(
        raw_heights: [u16; (XSIZE + 1) * (YSIZE + 1)],
        height_scale: f32,
        terrain_x: i32,
        terrain_y: i32,
    ) -> Self {
        Self {
            raw_heights,
            height_scale,
            terrain_x,
            terrain_y,
        }
    }

    fn get_height_map_value(&self, x: i32, y: i32) -> u16 {
        let mut cl_x = x;
        let mut cl_y = y;

        if cl_x < 0 {
            cl_x = 0;
        } else if cl_x > XSIZE as i32 {
            cl_x = XSIZE as i32;
        }

        if cl_y < 0 {
            cl_y = 0;
        } else if cl_y > YSIZE as i32 {
            cl_y = YSIZE as i32;
        }

        self.raw_heights[(cl_y as usize) * (XSIZE + 1) + (cl_x as usize)]
    }

    /// Implementacja Bilinear Heightfield Sampling - odpowiednik CTerrain::GetHeight
    pub fn get_height_bilinear(&self, mut fx: f32, mut fy: f32) -> f32 {
        fx -= (self.terrain_x as f32) * (XSIZE as f32) * (CELLSCALE as f32);
        fy -= (self.terrain_y as f32) * (YSIZE as f32) * (CELLSCALE as f32);

        if fx < 0.0
            || fy < 0.0
            || fx > (XSIZE * CELLSCALE) as f32
            || fy > (YSIZE * CELLSCALE) as f32
        {
            return 0.0;
        }

        let fx_int = fx as i32;
        let fy_int = fy as i32;

        let xdist = fx_int % (CELLSCALE as i32);
        let ydist = fy_int % (CELLSCALE as i32);

        let ooscale = 1.0 / (CELLSCALE as f32);
        let x = fx_int / (CELLSCALE as i32);
        let y = fy_int / (CELLSCALE as i32);

        let h1 = self.get_height_map_value(x, y) as f32 * self.height_scale;
        let h2 = self.get_height_map_value(x + 1, y + 1) as f32 * self.height_scale;

        let xslope;
        let yslope;

        if xdist <= ydist {
            let h3 = self.get_height_map_value(x, y + 1) as f32 * self.height_scale;
            xslope = (h2 - h3) * ooscale;
            yslope = (h3 - h1) * ooscale;
        } else {
            let h3 = self.get_height_map_value(x + 1, y) as f32 * self.height_scale;
            xslope = (h3 - h1) * ooscale;
            yslope = (h2 - h3) * ooscale;
        }

        h1 + (xdist as f32 * xslope + ydist as f32 * yslope)
    }

    /// Implementacja obliczania normalnej - odpowiednik CTerrain::CalculateNormal
    pub fn calculate_normal(&self, fx: f32, fy: f32) -> (f32, f32, f32) {
        let local_x = fx - (self.terrain_x as f32) * (XSIZE as f32) * (CELLSCALE as f32);
        let local_y = fy - (self.terrain_y as f32) * (YSIZE as f32) * (CELLSCALE as f32);

        let mut x = (local_x / CELLSCALE as f32) as i32;
        let mut y = (local_y / CELLSCALE as f32) as i32;

        if x < 0 {
            x = 0;
        }
        if x > XSIZE as i32 {
            x = XSIZE as i32;
        }
        if y < 0 {
            y = 0;
        }
        if y > YSIZE as i32 {
            y = YSIZE as i32;
        }

        let h_left = self.get_height_map_value(x - 1, y) as f32;
        let h_right = self.get_height_map_value(x + 1, y) as f32;
        let h_up = self.get_height_map_value(x, y - 1) as f32;
        let h_down = self.get_height_map_value(x, y + 1) as f32;

        let nx = -self.height_scale * (h_left - h_right);
        let ny = -self.height_scale * (h_up - h_down);
        let nz = 2.0 * CELLSCALE as f32;

        let len = (nx * nx + ny * ny + nz * nz).sqrt();
        if len > 0.0 {
            (nx / len, ny / len, nz / len)
        } else {
            (0.0, 0.0, 1.0)
        }
    }
}

pub fn adjust_to_terrain_height(
    actor_x: f32,
    actor_y: f32,
    dir_x: f32,
    dir_y: f32,
    terrain: &TerrainHeightData,
) -> TerrainSnapperResult {
    let z_height = terrain.get_height_bilinear(actor_x, actor_y);
    let normal = terrain.calculate_normal(actor_x, actor_y);

    // Calculate slope pitch and roll based on actor orientation (dir_x, dir_y) and terrain normal
    // Using a simplified adaptation logic commonly found in game clients
    // Dir is assumed to be normalized

    let mut dir_len = (dir_x * dir_x + dir_y * dir_y).sqrt();
    let (ndx, ndy) = if dir_len > 0.0001 {
        (dir_x / dir_len, dir_y / dir_len)
    } else {
        (1.0, 0.0) // fallback direction
    };

    // Right vector (dir_y, -dir_x)
    let right_x = ndy;
    let right_y = -ndx;

    // Project normal onto forward and right vectors to find slopes
    let forward_slope = -normal.0 * ndx - normal.1 * ndy;
    let right_slope = -normal.0 * right_x - normal.1 * right_y;

    // Calculate pitch and roll angles in degrees
    // We assume normal.z is mostly 1.0, and forward_slope is the tangent of the pitch angle
    let pitch = forward_slope.atan2(normal.2).to_degrees();
    let roll = right_slope.atan2(normal.2).to_degrees();

    TerrainSnapperResult {
        z_height,
        pitch,
        roll,
        normal,
    }
}
