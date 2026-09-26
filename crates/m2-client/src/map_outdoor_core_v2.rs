#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct OutdoorMapCoordinate {
    pub terrain_coord_x: i16,
    pub terrain_coord_y: i16,
}

pub const TERRAIN_XSIZE: f32 = 25600.0;
pub const TERRAIN_YSIZE: f32 = 25600.0;
pub const HALF_CELLSCALE: f32 = 100.0;
pub const HEIGHT_TILE_XRATIO: u8 = 2; // TILEMAP_XSIZE / XSIZE = 256 / 128 = 2
pub const HEIGHT_TILE_YRATIO: u8 = 2; // TILEMAP_YSIZE / YSIZE = 256 / 128 = 2
pub const LOAD_SIZE_WIDTH: i16 = 1;
pub const AROUND_AREA_NUM: u8 = 9; // 1+(LOAD_SIZE_WIDTH*2)*(LOAD_SIZE_WIDTH*2)*2 ... 3x3 sector ring

#[derive(Debug, Clone, Default)]
pub struct MapOutdoorCore {
    pub cur_coordinate: OutdoorMapCoordinate,
    pub prev_coordinate: OutdoorMapCoordinate,
}

impl MapOutdoorCore {
    pub fn new() -> Self {
        Self {
            cur_coordinate: OutdoorMapCoordinate {
                terrain_coord_x: -1,
                terrain_coord_y: -1,
            },
            prev_coordinate: OutdoorMapCoordinate {
                terrain_coord_x: -1,
                terrain_coord_y: -1,
            },
        }
    }

    pub fn get_terrain_num_from_coord(&self, coord_x: u16, coord_y: u16) -> Option<u8> {
        let diff_x = coord_x as i16 - self.cur_coordinate.terrain_coord_x;
        let diff_y = coord_y as i16 - self.cur_coordinate.terrain_coord_y;

        if diff_x < -LOAD_SIZE_WIDTH || diff_x > LOAD_SIZE_WIDTH {
            return None;
        }

        if diff_y < -LOAD_SIZE_WIDTH || diff_y > LOAD_SIZE_WIDTH {
            return None;
        }

        let terrain_num = (diff_y + LOAD_SIZE_WIDTH) * 3 + (diff_x + LOAD_SIZE_WIDTH);

        if terrain_num < 0 || terrain_num >= AROUND_AREA_NUM as i16 {
            None
        } else {
            Some(terrain_num as u8)
        }
    }

    pub fn convert_to_map_coords(
        fx: f32,
        fy: f32,
    ) -> (i32, i32, u8, u8, u16, u16) {
        let mut fx = fx;
        let mut fy = fy;
        if fy < 0.0 {
            fy = -fy;
        }

        let ix = fx as i32;
        let iy = fy as i32;

        let terrain_num_x = (ix / TERRAIN_XSIZE as i32) as u16;
        let terrain_num_y = (iy / TERRAIN_YSIZE as i32) as u16;

        let maxx = TERRAIN_XSIZE;
        let maxy = TERRAIN_YSIZE;

        while fx < 0.0 {
            fx += maxx;
        }

        while fy < 0.0 {
            fy += maxy;
        }

        while fx >= maxx {
            fx -= maxx;
        }

        while fy >= maxy {
            fy -= maxy;
        }

        let fooscale = 1.0 / HALF_CELLSCALE;

        let cell_x = fx * fooscale;
        let cell_y = fy * fooscale;

        let i_cell_x = cell_x as i32;
        let i_cell_y = cell_y as i32;

        let ratioooscale = (HEIGHT_TILE_XRATIO as f32) * fooscale;

        let subcell_x = fx * ratioooscale;
        let subcell_y = fy * ratioooscale;

        let mut puc_subcell_x = subcell_x as u8;
        let mut puc_subcell_y = subcell_y as u8;

        puc_subcell_x %= HEIGHT_TILE_XRATIO;
        puc_subcell_y %= HEIGHT_TILE_YRATIO;

        (i_cell_x, i_cell_y, puc_subcell_x, puc_subcell_y, terrain_num_x, terrain_num_y)
    }
}

#[repr(C)]
pub struct TerrainLocalCoords {
    pub cell_x: f32,
    pub cell_y: f32,
    pub terrain_x: u16,
    pub terrain_y: u16,
}

#[no_mangle]
pub extern "C" fn m2_terrain_world_to_local(gx: f32, gy: f32) -> TerrainLocalCoords {
    let (i_cell_x, i_cell_y, sub_x, sub_y, terrain_x, terrain_y) =
        MapOutdoorCore::convert_to_map_coords(gx, gy);

    TerrainLocalCoords {
        cell_x: i_cell_x as f32 + (sub_x as f32 / HEIGHT_TILE_XRATIO as f32),
        cell_y: i_cell_y as f32 + (sub_y as f32 / HEIGHT_TILE_YRATIO as f32),
        terrain_x,
        terrain_y,
    }
}
