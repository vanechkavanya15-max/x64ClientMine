#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TerrainPatchRenderData {
    pub patch_x: u32,
    pub patch_y: u32,
    pub height_map: [f32; 17 * 17],
    pub normal_map: [[f32; 3]; 17 * 17],
    pub splat_texture_ids: [u32; 4],
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerrainLodLevel {
    Lod0 = 0,
    Lod1 = 1,
    Lod2 = 2,
}

impl TerrainLodLevel {
    pub fn step_size(self) -> usize {
        match self {
            Self::Lod0 => 1,
            Self::Lod1 => 2,
            Self::Lod2 => 4,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TerrainVertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
}

pub fn generate_patch_vertices(patch: &TerrainPatchRenderData, lod: TerrainLodLevel) -> Vec<u8> {
    let step = lod.step_size();
    let cell_size = 200.0;
    
    // Each patch is 16x16 cells.
    let patch_base_x = (patch.patch_x as f32) * 16.0 * cell_size;
    let patch_base_y = (patch.patch_y as f32) * 16.0 * cell_size;
    
    let mut vertices = Vec::new();
    
    for y in (0..16).step_by(step) {
        for x in (0..16).step_by(step) {
            let v00 = get_vertex(patch, x, y, patch_base_x, patch_base_y, cell_size);
            let v10 = get_vertex(patch, x + step, y, patch_base_x, patch_base_y, cell_size);
            let v01 = get_vertex(patch, x, y + step, patch_base_x, patch_base_y, cell_size);
            let v11 = get_vertex(patch, x + step, y + step, patch_base_x, patch_base_y, cell_size);
            
            // Triangle right (xdist > ydist): v00, v10, v11
            vertices.push(v00);
            vertices.push(v10);
            vertices.push(v11);

            // Triangle left (xdist <= ydist): v00, v11, v01
            vertices.push(v00);
            vertices.push(v11);
            vertices.push(v01);
        }
    }
    
    let ptr = vertices.as_ptr() as *const u8;
    let size = vertices.len() * std::mem::size_of::<TerrainVertex>();
    let bytes = unsafe { std::slice::from_raw_parts(ptr, size) };
    
    bytes.to_vec()
}

fn get_vertex(
    patch: &TerrainPatchRenderData, 
    x: usize, 
    y: usize, 
    base_x: f32, 
    base_y: f32, 
    cell_size: f32
) -> TerrainVertex {
    let idx = y * 17 + x;
    let z = patch.height_map[idx];
    let normal = patch.normal_map[idx];
    
    TerrainVertex {
        position: [
            base_x + (x as f32) * cell_size,
            base_y + (y as f32) * cell_size,
            z
        ],
        normal,
    }
}

pub fn is_patch_in_frustum(
    patch_min: [f32; 3],
    patch_max: [f32; 3],
    frustum_planes: &[[f32; 4]; 6],
) -> bool {
    for plane in frustum_planes.iter() {
        let mut p_x = patch_min[0];
        let mut p_y = patch_min[1];
        let mut p_z = patch_min[2];
        
        if plane[0] >= 0.0 { p_x = patch_max[0]; }
        if plane[1] >= 0.0 { p_y = patch_max[1]; }
        if plane[2] >= 0.0 { p_z = patch_max[2]; }
        
        let dist = plane[0] * p_x + plane[1] * p_y + plane[2] * p_z + plane[3];
        
        if dist < 0.0 {
            return false;
        }
    }
    
    true
}
