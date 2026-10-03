use crate::vfs_pck_memory_mapped::{mount_archive, read_file_from_archive};
use crate::tga_decoder::TgaImage;
use crate::bmp_decoder::BmpImage;

const MARK_WIDTH: u32 = 16;
const MARK_HEIGHT: u32 = 12;

#[derive(Debug, PartialEq, Eq)]
pub enum GuildMarkError {
    ArchiveMountError(String),
    FileNotFound,
    DecodeError(&'static str),
    InvalidDimensions(u32, u32),
    AtlasFull,
}

pub struct GuildMarkAtlas {
    pub width: u32,
    pub height: u32,
    pub rgba_data: Vec<u8>,
    pub next_x: u32,
    pub next_y: u32,
}

impl GuildMarkAtlas {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            rgba_data: vec![0; (width * height * 4) as usize],
            next_x: 0,
            next_y: 0,
        }
    }

    pub fn add_mark(&mut self, mark_rgba: &[u8]) -> Result<(u32, u32), GuildMarkError> {
        if mark_rgba.len() != (MARK_WIDTH * MARK_HEIGHT * 4) as usize {
            return Err(GuildMarkError::DecodeError("Invalid mark data size"));
        }

        if self.next_x + MARK_WIDTH > self.width {
            self.next_x = 0;
            self.next_y += MARK_HEIGHT;
        }

        if self.next_y + MARK_HEIGHT > self.height {
            return Err(GuildMarkError::AtlasFull);
        }

        let current_x = self.next_x;
        let current_y = self.next_y;

        for y in 0..MARK_HEIGHT {
            for x in 0..MARK_WIDTH {
                let src_idx = ((y * MARK_WIDTH + x) * 4) as usize;
                let dst_idx = (((current_y + y) * self.width + (current_x + x)) * 4) as usize;
                
                self.rgba_data[dst_idx] = mark_rgba[src_idx];
                self.rgba_data[dst_idx + 1] = mark_rgba[src_idx + 1];
                self.rgba_data[dst_idx + 2] = mark_rgba[src_idx + 2];
                self.rgba_data[dst_idx + 3] = mark_rgba[src_idx + 3];
            }
        }

        self.next_x += MARK_WIDTH;

        Ok((current_x, current_y))
    }
}

pub struct GuildMarkAtlasLoader {
    pub atlas: GuildMarkAtlas,
}

impl GuildMarkAtlasLoader {
    pub fn new(atlas: GuildMarkAtlas) -> Self {
        Self { atlas }
    }

    pub fn load_from_pck(
        &mut self,
        archive_path: &str,
        file_name: &str,
    ) -> Result<(u32, u32), GuildMarkError> {
        let archive = mount_archive(archive_path)
            .map_err(|e| GuildMarkError::ArchiveMountError(e))?;

        let file_data = read_file_from_archive(&archive, file_name)
            .ok_or(GuildMarkError::FileNotFound)?;

        let mut mark_width = 0;
        let mut mark_height = 0;
        let mut mark_rgba = Vec::new();

        if let Ok(tga) = TgaImage::from_bytes(&file_data) {
            mark_width = tga.width as u32;
            mark_height = tga.height as u32;
            mark_rgba = tga.rgba_data;
        } else if let Ok(bmp) = BmpImage::from_bytes(&file_data) {
            mark_width = bmp.width;
            mark_height = bmp.height;
            // BMP data is vec of u32 RGBA. Let's convert to u8.
            mark_rgba = Vec::with_capacity((mark_width * mark_height * 4) as usize);
            for pixel in bmp.data {
                let r = (pixel & 0xFF) as u8;
                let g = ((pixel >> 8) & 0xFF) as u8;
                let b = ((pixel >> 16) & 0xFF) as u8;
                let a = ((pixel >> 24) & 0xFF) as u8;
                mark_rgba.push(r);
                mark_rgba.push(g);
                mark_rgba.push(b);
                mark_rgba.push(a);
            }
        } else {
            return Err(GuildMarkError::DecodeError("Could not decode image as TGA or BMP"));
        }

        if mark_width != MARK_WIDTH || mark_height != MARK_HEIGHT {
            return Err(GuildMarkError::InvalidDimensions(mark_width, mark_height));
        }

        self.atlas.add_mark(&mark_rgba)
    }
}
