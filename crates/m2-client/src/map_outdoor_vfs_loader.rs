use std::collections::HashMap;

pub const TERRAIN_XSIZE: i32 = 128;
pub const TERRAIN_YSIZE: i32 = 128;
pub const CELLSCALE: i32 = 200;

pub const HEIGHTMAP_RAW_XSIZE: usize = 131;
pub const HEIGHTMAP_RAW_YSIZE: usize = 131;
pub const HEIGHTMAP_RAW_BYTES: usize = HEIGHTMAP_RAW_XSIZE * HEIGHTMAP_RAW_YSIZE * 2;

pub const TILEMAP_RAW_XSIZE: usize = 258;
pub const TILEMAP_RAW_YSIZE: usize = 258;
pub const TILEMAP_RAW_BYTES: usize = TILEMAP_RAW_XSIZE * TILEMAP_RAW_YSIZE;

pub const ATTRMAP_XSIZE: usize = 256;
pub const ATTRMAP_YSIZE: usize = 256;
pub const ATTRMAP_MAGIC: u16 = 2634;

pub const ATTRIBUTE_BLOCK: u8 = 1 << 0;
pub const ATTRIBUTE_WATER: u8 = 1 << 1;
pub const ATTRIBUTE_BANPK: u8 = 1 << 2;

#[derive(Debug, Clone, PartialEq)]
pub struct MapSetting {
    pub map_size_x: i32,
    pub map_size_y: i32,
    pub texture_set: String,
    pub water_height: i32,
    pub base_x: i32,
    pub base_y: i32,
    pub view_radius: i32,
    pub height_scale: f32,
}

impl Default for MapSetting {
    fn default() -> Self {
        Self {
            map_size_x: 1,
            map_size_y: 1,
            texture_set: String::new(),
            water_height: 0,
            base_x: 0,
            base_y: 0,
            view_radius: 0,
            height_scale: 0.0,
        }
    }
}

pub struct RawHeightMap {
    heights: Vec<u16>,
}

impl RawHeightMap {
    pub fn new(heights: Vec<u16>) -> Self {
        Self { heights }
    }
    
    pub fn get_height(&self, x: usize, y: usize) -> u16 {
        if x < HEIGHTMAP_RAW_XSIZE && y < HEIGHTMAP_RAW_YSIZE {
            self.heights[y * HEIGHTMAP_RAW_XSIZE + x]
        } else {
            0
        }
    }
}

pub struct RawTileMap {
    tiles: Vec<u8>,
}

impl RawTileMap {
    pub fn new(tiles: Vec<u8>) -> Self {
        Self { tiles }
    }
    
    pub fn get_tile(&self, x: usize, y: usize) -> u8 {
        if x < TILEMAP_RAW_XSIZE && y < TILEMAP_RAW_YSIZE {
            self.tiles[y * TILEMAP_RAW_XSIZE + x]
        } else {
            0
        }
    }
}

#[repr(C, packed)]
struct SAttrMapHeader {
    magic: u16,
    width: u16,
    height: u16,
}

pub struct RawAttrMap {
    attributes: Vec<u8>,
}

impl RawAttrMap {
    pub fn new(attributes: Vec<u8>) -> Self {
        Self { attributes }
    }
    
    pub fn get_attr(&self, x: usize, y: usize) -> u8 {
        if x < ATTRMAP_XSIZE && y < ATTRMAP_YSIZE {
            self.attributes[y * ATTRMAP_XSIZE + x]
        } else {
            0
        }
    }
}

pub struct MapOutdoorVfsLoader {
    pub setting: MapSetting,
    pub height_map: Option<RawHeightMap>,
    pub tile_map: Option<RawTileMap>,
    pub attr_map: Option<RawAttrMap>,
}

impl MapOutdoorVfsLoader {
    pub fn new() -> Self {
        Self {
            setting: MapSetting::default(),
            height_map: None,
            tile_map: None,
            attr_map: None,
        }
    }

    pub fn load_setting(&mut self, content: &str) -> Result<(), &'static str> {
        let mut map: HashMap<String, Vec<String>> = HashMap::new();
        let mut lines = content.lines().peekable();

        while let Some(line) = lines.next() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            let mut parts: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();
            if parts.is_empty() {
                continue;
            }

            let key = parts[0].to_lowercase();
            parts.remove(0);
            map.insert(key, parts);
        }

        let script_type = map.get("scripttype")
            .and_then(|v| v.first())
            .ok_or("FIND 'scripttype' - FAILED")?
            .to_string();

        if script_type != "MapSetting" && script_type != "mapsetting" {
            return Err("Resource Type ERROR");
        }

        let view_radius = map.get("viewradius")
            .and_then(|v| v.first())
            .and_then(|s| s.parse::<i32>().ok())
            .ok_or("FIND 'viewradius' - FAILED")?;

        if view_radius <= 0 {
            return Err("VIEWRADIUS IS NOT GREATER THAN 0");
        }

        let height_scale = map.get("heightscale")
            .and_then(|v| v.first())
            .and_then(|s| s.parse::<f32>().ok())
            .ok_or("FIND 'heightscale' - FAILED")?;

        let map_size = map.get("mapsize")
            .ok_or("FIND 'mapsize' - FAILED")?;

        if map_size.len() < 2 {
            return Err("FIND 'mapsize' - FAILED");
        }

        let map_size_x = map_size[0].parse::<i32>().unwrap_or(0);
        let map_size_y = map_size[1].parse::<i32>().unwrap_or(0);

        let texture_set = map.get("textureset")
            .map(|v| v.join(" "))
            .ok_or("FIND 'textureset' - FAILED")?;

        let mut base_x = 0;
        let mut base_y = 0;
        if let Some(base_pos) = map.get("baseposition") {
            if base_pos.len() >= 2 {
                base_x = base_pos[0].parse::<i32>().unwrap_or(0);
                base_y = base_pos[1].parse::<i32>().unwrap_or(0);
            }
        }
        
        let water_height = map.get("waterheight")
            .and_then(|v| v.first())
            .and_then(|s| s.parse::<i32>().ok())
            .unwrap_or(0); // If not found, default to 0 as it's optional sometimes, but original C++ doesn't explicitly parse WaterHeight in LoadSetting, it parses it in MapOutdoorWater. We'll set it here if exists.

        self.setting = MapSetting {
            map_size_x,
            map_size_y,
            texture_set,
            water_height,
            base_x,
            base_y,
            view_radius,
            height_scale,
        };

        Ok(())
    }

    pub fn load_height_map(&mut self, data: &[u8]) -> Result<(), &'static str> {
        if data.len() != HEIGHTMAP_RAW_BYTES {
            return Err("Invalid heightmap size");
        }

        let mut heights = vec![0u16; HEIGHTMAP_RAW_XSIZE * HEIGHTMAP_RAW_YSIZE];
        for i in 0..heights.len() {
            let offset = i * 2;
            heights[i] = u16::from_le_bytes([data[offset], data[offset + 1]]);
        }

        self.height_map = Some(RawHeightMap::new(heights));
        Ok(())
    }

    pub fn load_tile_map(&mut self, data: &[u8]) -> Result<(), &'static str> {
        if data.len() != TILEMAP_RAW_BYTES {
            return Err("Invalid tilemap size");
        }

        let tiles = data.to_vec();
        self.tile_map = Some(RawTileMap::new(tiles));
        Ok(())
    }

    pub fn load_attr_map(&mut self, data: &[u8]) -> Result<(), &'static str> {
        let header_size = std::mem::size_of::<SAttrMapHeader>();
        if data.len() < header_size {
            return Err("Invalid attrmap size");
        }

        let mut magic_bytes = [0u8; 2];
        magic_bytes.copy_from_slice(&data[0..2]);
        let magic = u16::from_le_bytes(magic_bytes);

        if magic != ATTRMAP_MAGIC {
            return Err("MAGIC NUMBER ERROR");
        }

        let mut width_bytes = [0u8; 2];
        width_bytes.copy_from_slice(&data[2..4]);
        let width = u16::from_le_bytes(width_bytes);

        if width as usize != ATTRMAP_XSIZE {
            return Err("WIDTH ERROR");
        }

        let mut height_bytes = [0u8; 2];
        height_bytes.copy_from_slice(&data[4..6]);
        let height = u16::from_le_bytes(height_bytes);

        if height as usize != ATTRMAP_YSIZE {
            return Err("HEIGHT ERROR");
        }

        let expected_size = header_size + (ATTRMAP_XSIZE * ATTRMAP_YSIZE);
        if data.len() != expected_size {
            return Err("FILE DATA SIZE ERROR");
        }

        let attributes = data[header_size..].to_vec();
        self.attr_map = Some(RawAttrMap::new(attributes));
        Ok(())
    }

    pub fn get_terrain_height_at(&self, mut world_x: f32, mut world_y: f32) -> f32 {
        if world_x < 0.0 { world_x = -world_x; }
        if world_y < 0.0 { world_y = -world_y; }

        let lx = world_x.floor() as i32;
        let ly = world_y.floor() as i32;

        let terrain_x_size = TERRAIN_XSIZE * CELLSCALE;
        let terrain_y_size = TERRAIN_YSIZE * CELLSCALE;

        // Base coordinates check
        // Original code offsets it based on m_wX and m_wY in CTerrain::GetHeight,
        // but here we just take local coordinates. We assume this VfsLoader corresponds
        // to one specific terrain patch (since it loads one height.raw).
        // Let's assume lx, ly are ALREADY local to this terrain patch for simplicity,
        // OR we subtract the base_x / base_y.
        
        let local_x = lx - self.setting.base_x;
        let local_y = ly - self.setting.base_y;

        if local_x < 0 || local_y < 0 || local_x > terrain_x_size || local_y > terrain_y_size {
            return 0.0;
        }

        let height_map = match &self.height_map {
            Some(h) => h,
            None => return 0.0,
        };

        let mut x = local_x;
        let mut y = local_y;

        let xdist = (x % CELLSCALE) as f32;
        let ydist = (y % CELLSCALE) as f32;

        let ooscale = 1.0 / (CELLSCALE as f32);
        x /= CELLSCALE;
        y /= CELLSCALE;

        let cell_x = x as usize;
        let cell_y = y as usize;

        // Note: CTerrainImpl::GetHeightMapValue uses (sy+1) and (sx+1) because of the padding
        let sx1 = cell_x + 1;
        let sy1 = cell_y + 1;
        
        let h1 = height_map.get_height(sx1, sy1) as f32 * self.setting.height_scale;
        let h2 = height_map.get_height(sx1 + 1, sy1 + 1) as f32 * self.setting.height_scale;

        if xdist <= ydist {
            let h3 = height_map.get_height(sx1, sy1 + 1) as f32 * self.setting.height_scale;
            let xslope = (h2 - h3) * ooscale;
            let yslope = (h3 - h1) * ooscale;
            h1 + (xdist * xslope + ydist * yslope)
        } else {
            let h3 = height_map.get_height(sx1 + 1, sy1) as f32 * self.setting.height_scale;
            let xslope = (h3 - h1) * ooscale;
            let yslope = (h2 - h3) * ooscale;
            h1 + (xdist * xslope + ydist * yslope)
        }
    }
}
