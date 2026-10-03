use std::collections::HashMap;

use crate::terrain_attr_collision_v2::TerrainSectorAttr;
use crate::terrain_bicubic_height_v2::{TerrainBicubicHeightV2, CELLSCALE, XSIZE, YSIZE};

pub const SECTOR_SIZE_CM: i32 = 25600;

pub struct MapOutdoorSystemValidator {
    pub active_sectors: HashMap<(i32, i32), (TerrainBicubicHeightV2, TerrainSectorAttr)>,
    pub streaming_radius: i32,
}

impl MapOutdoorSystemValidator {
    pub fn new(radius: i32) -> Self {
        Self {
            active_sectors: HashMap::new(),
            streaming_radius: radius,
        }
    }

    pub fn update_streaming_center(&mut self, center_x: i32, center_y: i32) {
        let center_col = center_x.div_euclid(SECTOR_SIZE_CM);
        let center_row = center_y.div_euclid(SECTOR_SIZE_CM);

        let radius = self.streaming_radius;
        self.active_sectors.retain(|&(col, row), _| {
            let dx = (col - center_col).abs();
            let dy = (row - center_row).abs();
            dx <= radius && dy <= radius
        });
    }

    pub fn add_sector_data(
        &mut self,
        x: i32,
        y: i32,
        height_data: Vec<u16>,
        attr_data: Vec<u8>,
    ) -> Result<(), &'static str> {
        let height_map = TerrainBicubicHeightV2::new(height_data, 1.0, x as u16, y as u16);
        let attr_map = TerrainSectorAttr::new(attr_data)?;

        self.active_sectors.insert((x, y), (height_map, attr_map));
        Ok(())
    }

    pub fn is_walkable(&self, global_x: f32, global_y: f32) -> bool {
        let gx = global_x as i32;
        let gy = global_y as i32;
        
        let col = gx.div_euclid(SECTOR_SIZE_CM);
        let row = gy.div_euclid(SECTOR_SIZE_CM);
        
        if let Some((_, attr_map)) = self.active_sectors.get(&(col, row)) {
            // Relative coordinates within the sector
            let local_x = gx.rem_euclid(SECTOR_SIZE_CM);
            let local_y = gy.rem_euclid(SECTOR_SIZE_CM);

            // Attr map is 128x128
            // Each cell in the attr map corresponds to CELLSCALE (200 cm)
            let cell_x = (local_x / CELLSCALE) as usize;
            let cell_y = (local_y / CELLSCALE) as usize;

            attr_map.is_walkable(cell_x, cell_y)
        } else {
            false // If sector is not loaded, it's not walkable
        }
    }

    pub fn get_height(&self, global_x: f32, global_y: f32) -> Option<f32> {
        let gx = global_x as i32;
        let gy = global_y as i32;

        let col = gx.div_euclid(SECTOR_SIZE_CM);
        let row = gy.div_euclid(SECTOR_SIZE_CM);

        if let Some((height_map, _)) = self.active_sectors.get(&(col, row)) {
            Some(height_map.get_height(gx, gy))
        } else {
            None
        }
    }
}
