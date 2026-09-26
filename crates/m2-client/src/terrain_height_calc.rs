pub const HEIGHTMAP_RAW_XSIZE: usize = 131;
pub const HEIGHTMAP_RAW_YSIZE: usize = 131;
pub const HEIGHTMAP_RAW_SIZE: usize = HEIGHTMAP_RAW_XSIZE * HEIGHTMAP_RAW_YSIZE;

pub struct RawHeightMap {
    data: [u16; HEIGHTMAP_RAW_SIZE],
}

impl RawHeightMap {
    pub fn new(data: [u16; HEIGHTMAP_RAW_SIZE]) -> Self {
        Self { data }
    }

    pub fn get_height_raw(&self, x: usize, y: usize) -> u16 {
        self.data[y * HEIGHTMAP_RAW_XSIZE + x]
    }
}

pub struct TerrainHeightCalculator {
    raw_map: RawHeightMap,
    height_scale: f32,
}

impl TerrainHeightCalculator {
    pub fn new(raw_map: RawHeightMap, height_scale: f32) -> Self {
        Self {
            raw_map,
            height_scale,
        }
    }

    pub fn get_height(&self, fx: f32, fy: f32) -> f32 {
        let clamped_fx = fx.clamp(0.0, 128.0);
        let clamped_fy = fy.clamp(0.0, 128.0);

        let ix = clamped_fx.floor() as usize;
        let iy = clamped_fy.floor() as usize;

        let x_index = ix + 1;
        let y_index = iy + 1;

        let h00 = self.raw_map.get_height_raw(x_index, y_index);
        let h10 = self.raw_map.get_height_raw(x_index + 1, y_index);
        let h01 = self.raw_map.get_height_raw(x_index, y_index + 1);
        let h11 = self.raw_map.get_height_raw(x_index + 1, y_index + 1);

        let tx = clamped_fx - clamped_fx.floor();
        let ty = clamped_fy - clamped_fy.floor();

        let h_bottom = (1.0 - tx) * (h00 as f32) + tx * (h10 as f32);
        let h_top = (1.0 - tx) * (h01 as f32) + tx * (h11 as f32);
        
        let final_height = (1.0 - ty) * h_bottom + ty * h_top;

        final_height * self.height_scale
    }

    /// Dokladna kalkulacja wysokosci na podstawie dwoch trojkatow siatki terenu,
    /// bezposrednio odwzorowujaca algorytm C++ CTerrain::GetHeight (AreaTerrain.cpp:408-433).
    pub fn get_height_triangle(&self, fx: f32, fy: f32) -> f32 {
        let clamped_fx = fx.clamp(0.0, 128.0);
        let clamped_fy = fy.clamp(0.0, 128.0);

        let ix = clamped_fx.floor() as usize;
        let iy = clamped_fy.floor() as usize;

        let x_index = ix + 1;
        let y_index = iy + 1;

        let h1 = self.raw_map.get_height_raw(x_index, y_index) as f32 * self.height_scale;
        let h2 = self.raw_map.get_height_raw(x_index + 1, y_index + 1) as f32 * self.height_scale;

        let xdist = clamped_fx - (ix as f32);
        let ydist = clamped_fy - (iy as f32);

        // Trojkat lewy (xdist <= ydist) vs prawy (xdist > ydist) - zgodnie z C++ AreaTerrain.cpp:408-433
        if xdist <= ydist {
            let h3 = self.raw_map.get_height_raw(x_index, y_index + 1) as f32 * self.height_scale;
            let xslope = h2 - h3;
            let yslope = h3 - h1;
            h1 + (xdist * xslope + ydist * yslope)
        } else {
            let h3 = self.raw_map.get_height_raw(x_index + 1, y_index) as f32 * self.height_scale;
            let xslope = h3 - h1;
            let yslope = h2 - h3;
            h1 + (xdist * xslope + ydist * yslope)
        }
    }
}
