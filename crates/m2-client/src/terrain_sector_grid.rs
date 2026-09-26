pub const SECTOR_SIZE_CM: i32 = 25600;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SectorCoords {
    pub col: u32,
    pub row: u32,
    pub local_cm_x: f32,
    pub local_cm_y: f32,
    pub tile_x: f32,
    pub tile_y: f32,
}

pub fn global_to_sector(global_x: i32, global_y: i32, base_x: i32, base_y: i32) -> SectorCoords {
    let rel_x = global_x - base_x;
    let rel_y = global_y - base_y;

    let col = (rel_x / SECTOR_SIZE_CM) as u32;
    let row = (rel_y / SECTOR_SIZE_CM) as u32;
    
    let local_cm_x = (rel_x % SECTOR_SIZE_CM) as f32;
    let local_cm_y = (rel_y % SECTOR_SIZE_CM) as f32;
    
    let tile_x = local_cm_x / 200.0;
    let tile_y = local_cm_y / 200.0;

    SectorCoords {
        col,
        row,
        local_cm_x,
        local_cm_y,
        tile_x,
        tile_y,
    }
}

pub fn get_surrounding_sectors(center_col: u32, center_row: u32, max_w: u32, max_h: u32) -> Vec<(u32, u32)> {
    let mut sectors = Vec::with_capacity(9);
    
    if max_w == 0 || max_h == 0 {
        return sectors;
    }

    let start_c = center_col.saturating_sub(1);
    let end_c = (center_col + 1).min(max_w - 1);
    
    let start_r = center_row.saturating_sub(1);
    let end_r = (center_row + 1).min(max_h - 1);
    
    for r in start_r..=end_r {
        for c in start_c..=end_c {
            sectors.push((c, r));
        }
    }
    
    sectors
}
