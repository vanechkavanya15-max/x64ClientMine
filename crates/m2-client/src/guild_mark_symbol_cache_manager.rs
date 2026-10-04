use std::collections::{HashMap, HashSet};

pub const MARK_WIDTH: usize = 16;
pub const MARK_HEIGHT: usize = 12;
pub const MARK_SIZE: usize = MARK_WIDTH * MARK_HEIGHT; // 192 pixels

// Pixel represented as u32 (RGBA)
pub type Pixel = u32;

#[derive(Clone, Debug, PartialEq)]
pub struct GuildMark {
    pub pixels: [Pixel; MARK_SIZE],
}

impl GuildMark {
    pub fn new() -> Self {
        Self {
            pixels: [0; MARK_SIZE],
        }
    }

    pub fn is_empty(&self) -> bool {
        self.pixels.iter().all(|&p| p == 0)
    }

    pub fn from_rgb565(data: &[u8]) -> Option<Self> {
        if data.len() < MARK_SIZE * 2 {
            return None;
        }

        let mut mark = Self::new();
        for i in 0..MARK_SIZE {
            let p1 = data[i * 2] as u16;
            let p2 = data[i * 2 + 1] as u16;
            let val = (p2 << 8) | p1;

            let r = (((val >> 11) & 0x1F) as u32) * 255 / 31;
            let g = (((val >> 5) & 0x3F) as u32) * 255 / 63;
            let b = ((val & 0x1F) as u32) * 255 / 31;
            let a = 255;

            mark.pixels[i] = (a << 24) | (b << 16) | (g << 8) | r;
        }
        Some(mark)
    }

    pub fn from_rgba(data: &[u8]) -> Option<Self> {
        if data.len() < MARK_SIZE * 4 {
            return None;
        }

        let mut mark = Self::new();
        for i in 0..MARK_SIZE {
            let r = data[i * 4] as u32;
            let g = data[i * 4 + 1] as u32;
            let b = data[i * 4 + 2] as u32;
            let a = data[i * 4 + 3] as u32;

            mark.pixels[i] = (a << 24) | (b << 16) | (g << 8) | r;
        }
        Some(mark)
    }
}

pub struct GuildMarkBlock {
    // 64x48 pixels
    pub pixels: [Pixel; 64 * 48],
}

pub const ATLAS_WIDTH: usize = 512;
pub const ATLAS_HEIGHT: usize = 512;

pub const BLOCK_ROW_COUNT: usize = ATLAS_HEIGHT / 48; // 10
pub const BLOCK_COL_COUNT: usize = ATLAS_WIDTH / 64; // 8
pub const BLOCK_TOTAL_COUNT: usize = BLOCK_ROW_COUNT * BLOCK_COL_COUNT; // 80

pub const MARK_ROW_COUNT: usize = BLOCK_ROW_COUNT * 4; // 40
pub const MARK_COL_COUNT: usize = BLOCK_COL_COUNT * 4; // 32
pub const MARK_TOTAL_COUNT: usize = MARK_ROW_COUNT * MARK_COL_COUNT; // 1280

pub const MAX_IMAGE_COUNT: u32 = 5;
pub const INVALID_MARK_ID: u32 = 0xFFFFFFFF;

pub struct GuildMarkImage {
    pub pixels: Vec<Pixel>,
}

impl GuildMarkImage {
    pub fn new() -> Self {
        Self {
            pixels: vec![0; ATLAS_WIDTH * ATLAS_HEIGHT],
        }
    }

    pub fn put_mark(&mut self, pos_mark: usize, mark: &GuildMark) -> bool {
        if pos_mark >= MARK_TOTAL_COUNT {
            return false;
        }

        let col = pos_mark % MARK_COL_COUNT;
        let row = pos_mark / MARK_COL_COUNT;

        let start_x = col * MARK_WIDTH;
        let start_y = row * MARK_HEIGHT;

        for y in 0..MARK_HEIGHT {
            for x in 0..MARK_WIDTH {
                let img_idx = (start_y + y) * ATLAS_WIDTH + (start_x + x);
                let mark_idx = y * MARK_WIDTH + x;
                self.pixels[img_idx] = mark.pixels[mark_idx];
            }
        }
        true
    }

    pub fn get_mark(&self, pos_mark: usize) -> Option<GuildMark> {
        if pos_mark >= MARK_TOTAL_COUNT {
            return None;
        }

        let col = pos_mark % MARK_COL_COUNT;
        let row = pos_mark / MARK_COL_COUNT;

        let start_x = col * MARK_WIDTH;
        let start_y = row * MARK_HEIGHT;

        let mut mark = GuildMark::new();

        for y in 0..MARK_HEIGHT {
            for x in 0..MARK_WIDTH {
                let img_idx = (start_y + y) * ATLAS_WIDTH + (start_x + x);
                let mark_idx = y * MARK_WIDTH + x;
                mark.pixels[mark_idx] = self.pixels[img_idx];
            }
        }

        Some(mark)
    }

    pub fn delete_mark(&mut self, pos_mark: usize) -> bool {
        if pos_mark >= MARK_TOTAL_COUNT {
            return false;
        }

        let col = pos_mark % MARK_COL_COUNT;
        let row = pos_mark / MARK_COL_COUNT;

        let start_x = col * MARK_WIDTH;
        let start_y = row * MARK_HEIGHT;

        for y in 0..MARK_HEIGHT {
            for x in 0..MARK_WIDTH {
                let img_idx = (start_y + y) * ATLAS_WIDTH + (start_x + x);
                self.pixels[img_idx] = 0;
            }
        }
        true
    }
}

pub struct GuildSymbol {
    pub crc: u32,
    pub raw: Vec<u8>,
}

pub struct GuildMarkSymbolCacheManager {
    images: HashMap<u32, GuildMarkImage>,
    gid_to_mark_id: HashMap<u32, u32>,
    free_mark_ids: HashSet<u32>,
    symbols: HashMap<u32, GuildSymbol>,
}

impl GuildMarkSymbolCacheManager {
    pub fn new() -> Self {
        let mut free_mark_ids = HashSet::new();
        for i in 0..(MAX_IMAGE_COUNT * MARK_TOTAL_COUNT as u32) {
            free_mark_ids.insert(i);
        }

        Self {
            images: HashMap::new(),
            gid_to_mark_id: HashMap::new(),
            free_mark_ids,
            symbols: HashMap::new(),
        }
    }

    fn crc32(data: &[u8]) -> u32 {
        let mut crc = 0xFFFFFFFF;
        for &b in data {
            crc ^= b as u32;
            for _ in 0..8 {
                if (crc & 1) != 0 {
                    crc = (crc >> 1) ^ 0xEDB88320;
                } else {
                    crc >>= 1;
                }
            }
        }
        crc ^ 0xFFFFFFFF
    }

    pub fn upload_symbol(&mut self, guild_id: u32, data: &[u8]) {
        let crc = Self::crc32(data);
        self.symbols.insert(
            guild_id,
            GuildSymbol {
                crc,
                raw: data.to_vec(),
            },
        );
    }

    pub fn get_guild_symbol(&self, guild_id: u32) -> Option<&GuildSymbol> {
        self.symbols.get(&guild_id)
    }

    fn get_or_create_image(&mut self, img_idx: u32) -> &mut GuildMarkImage {
        self.images.entry(img_idx).or_insert_with(GuildMarkImage::new)
    }

    pub fn add_mark_id_by_guild_id(&mut self, guild_id: u32, mark_id: u32) -> bool {
        if mark_id >= MAX_IMAGE_COUNT * (MARK_TOTAL_COUNT as u32) {
            return false;
        }

        self.gid_to_mark_id.insert(guild_id, mark_id);
        self.free_mark_ids.remove(&mark_id);
        true
    }

    pub fn get_mark_id(&self, guild_id: u32) -> u32 {
        *self.gid_to_mark_id.get(&guild_id).unwrap_or(&INVALID_MARK_ID)
    }

    pub fn alloc_mark_id(&mut self, guild_id: u32) -> u32 {
        if let Some(&min_free) = self.free_mark_ids.iter().min() {
            let img_idx = min_free / (MARK_TOTAL_COUNT as u32);
            self.get_or_create_image(img_idx);
            
            if self.add_mark_id_by_guild_id(guild_id, min_free) {
                return min_free;
            }
        }
        INVALID_MARK_ID
    }

    pub fn save_mark(&mut self, guild_id: u32, mark: &GuildMark) -> u32 {
        let mut id_mark = self.get_mark_id(guild_id);

        if id_mark == INVALID_MARK_ID {
            id_mark = self.alloc_mark_id(guild_id);
            if id_mark == INVALID_MARK_ID {
                return INVALID_MARK_ID;
            }
        }

        let img_idx = id_mark / (MARK_TOTAL_COUNT as u32);
        let pos_mark = (id_mark % (MARK_TOTAL_COUNT as u32)) as usize;

        let image = self.get_or_create_image(img_idx);
        if image.put_mark(pos_mark, mark) {
            id_mark
        } else {
            INVALID_MARK_ID
        }
    }

    pub fn delete_mark(&mut self, guild_id: u32) {
        if let Some(mark_id) = self.gid_to_mark_id.remove(&guild_id) {
            let img_idx = mark_id / (MARK_TOTAL_COUNT as u32);
            let pos_mark = (mark_id % (MARK_TOTAL_COUNT as u32)) as usize;

            if let Some(image) = self.images.get_mut(&img_idx) {
                image.delete_mark(pos_mark);
            }

            self.free_mark_ids.insert(mark_id);
        }
    }

    pub fn get_mark_image_count(&self) -> u32 {
        self.images.len() as u32
    }

    pub fn get_mark_count(&self) -> u32 {
        self.gid_to_mark_id.len() as u32
    }

    pub fn get_mark(&self, guild_id: u32) -> Option<GuildMark> {
        let mark_id = self.get_mark_id(guild_id);
        if mark_id == INVALID_MARK_ID {
            return None;
        }

        let img_idx = mark_id / (MARK_TOTAL_COUNT as u32);
        let pos_mark = (mark_id % (MARK_TOTAL_COUNT as u32)) as usize;

        self.images.get(&img_idx).and_then(|img| img.get_mark(pos_mark))
    }
}
