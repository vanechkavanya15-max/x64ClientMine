pub const MAX_TERRAIN_TEXTURES: usize = 256;
pub const TERRAIN_SIZE: usize = 128;
pub const TERRAIN_PATCHSIZE: usize = 16;
pub const TERRAIN_PATCHCOUNT: usize = TERRAIN_SIZE / TERRAIN_PATCHSIZE;

pub const TILEMAP_XSIZE: usize = TERRAIN_SIZE * 2;
pub const TILEMAP_YSIZE: usize = TERRAIN_SIZE * 2;
pub const TILEMAP_RAW_XSIZE: usize = TILEMAP_XSIZE + 2;
pub const TILEMAP_RAW_YSIZE: usize = TILEMAP_YSIZE + 2;
pub const SPLATALPHA_RAW_XSIZE: usize = 64; // Based on the requirement: Odczyt mapy wag alpha (alpha weight map 64x64).
pub const SPLATALPHA_RAW_YSIZE: usize = 64;

pub struct TerrainSplat {
    pub active: bool,
    pub needs_update: bool,
    pub alpha_map: Option<Vec<u8>>,
}

pub struct TerrainSplatPatch {
    pub tile_count: [u32; MAX_TERRAIN_TEXTURES],
    pub patch_tile_count: [[u32; MAX_TERRAIN_TEXTURES]; TERRAIN_PATCHCOUNT * TERRAIN_PATCHCOUNT],
    pub splats: Vec<TerrainSplat>,
    pub needs_update: bool,
}

impl TerrainSplatPatch {
    pub fn new() -> Self {
        let mut splats = Vec::with_capacity(MAX_TERRAIN_TEXTURES);
        for _ in 0..MAX_TERRAIN_TEXTURES {
            splats.push(TerrainSplat {
                active: false,
                needs_update: false,
                alpha_map: None,
            });
        }
        
        Self {
            tile_count: [0; MAX_TERRAIN_TEXTURES],
            patch_tile_count: [[0; MAX_TERRAIN_TEXTURES]; TERRAIN_PATCHCOUNT * TERRAIN_PATCHCOUNT],
            splats,
            needs_update: false,
        }
    }
}

pub struct Terrain {
    pub terrain_splat_patch: TerrainSplatPatch,
    pub tile_map: Vec<u8>,
    pub texture_count: u32,
}

impl Terrain {
    pub fn new(texture_count: u32) -> Self {
        Self {
            terrain_splat_patch: TerrainSplatPatch::new(),
            tile_map: vec![0; SPLATALPHA_RAW_XSIZE * SPLATALPHA_RAW_YSIZE],
            texture_count,
        }
    }

    pub fn generate_splat(&mut self) {
        if !self.terrain_splat_patch.needs_update {
            return;
        }

        self.terrain_splat_patch.needs_update = false;

        for i in 1..self.texture_count as usize {
            let splat = &mut self.terrain_splat_patch.splats[i];
            
            if splat.needs_update {
                if self.terrain_splat_patch.tile_count[i] > 0 {
                    splat.active = true;
                    splat.needs_update = false;

                    let mut alpha_map = vec![0u8; SPLATALPHA_RAW_XSIZE * SPLATALPHA_RAW_YSIZE];
                    
                    let stride = SPLATALPHA_RAW_XSIZE;
                    let target_byte = i as u8;

                    for y in 0..SPLATALPHA_RAW_YSIZE {
                        for x in 0..SPLATALPHA_RAW_XSIZE {
                            let tile_num = self.tile_map[y * stride + x];

                            let alpha_val = if tile_num == target_byte {
                                0xFF
                            } else if tile_num > target_byte {
                                let mut found = false;

                                // Check horizontal
                                if x > 0 && self.tile_map[y * stride + (x - 1)] == target_byte {
                                    found = true;
                                } else if x < SPLATALPHA_RAW_XSIZE - 1 && self.tile_map[y * stride + (x + 1)] == target_byte {
                                    found = true;
                                }
                                
                                // Check Up
                                else if y > 0 {
                                    let row_up = (y - 1) * stride;
                                    if self.tile_map[row_up + x] == target_byte {
                                        found = true;
                                    } else if x > 0 && self.tile_map[row_up + (x - 1)] == target_byte {
                                        found = true;
                                    } else if x < SPLATALPHA_RAW_XSIZE - 1 && self.tile_map[row_up + (x + 1)] == target_byte {
                                        found = true;
                                    }
                                }

                                // Check Down
                                if !found && y < SPLATALPHA_RAW_YSIZE - 1 {
                                    let row_down = (y + 1) * stride;
                                    if self.tile_map[row_down + x] == target_byte {
                                        found = true;
                                    } else if x > 0 && self.tile_map[row_down + (x - 1)] == target_byte {
                                        found = true;
                                    } else if x < SPLATALPHA_RAW_XSIZE - 1 && self.tile_map[row_down + (x + 1)] == target_byte {
                                        found = true;
                                    }
                                }

                                if found { 0xFF } else { 0x00 }
                            } else {
                                0x00
                            };
                            
                            alpha_map[y * SPLATALPHA_RAW_XSIZE + x] = alpha_val;
                        }
                    }

                    splat.alpha_map = Some(alpha_map);
                } else {
                    splat.needs_update = false;
                    splat.active = false;
                    splat.alpha_map = None;
                }
            }
        }
    }
}

// Global terrain state pointer for FFI 
// A static pointer to a Terrain instance to be able to retrieve the layers for the patch
static mut GLOBAL_TERRAIN_PTR: *const Terrain = std::ptr::null();

#[no_mangle]
pub extern "C" fn m2_terrain_set_context(terrain_ptr: *const Terrain) {
    unsafe {
        GLOBAL_TERRAIN_PTR = terrain_ptr;
    }
}

#[no_mangle]
pub extern "C" fn m2_terrain_get_texture_layers(patch_x: u16, patch_y: u16) -> u8 {
    if patch_x as usize >= TERRAIN_PATCHCOUNT || patch_y as usize >= TERRAIN_PATCHCOUNT {
        return 0;
    }
    
    let patch_index = (patch_y as usize) * TERRAIN_PATCHCOUNT + (patch_x as usize);
    let mut layer_count = 0;
    
    unsafe {
        if !GLOBAL_TERRAIN_PTR.is_null() {
            let terrain = &*GLOBAL_TERRAIN_PTR;
            
            // Loop through all textures and check if they are used in this patch
            for i in 1..terrain.texture_count as usize {
                if terrain.terrain_splat_patch.patch_tile_count[patch_index][i] > 0 {
                    layer_count += 1;
                    
                    // Hardware limit per C++ codebase (limit is 8 layers per patch)
                    if layer_count >= 8 {
                        return 8;
                    }
                }
            }
        }
    }
    
    layer_count
}
