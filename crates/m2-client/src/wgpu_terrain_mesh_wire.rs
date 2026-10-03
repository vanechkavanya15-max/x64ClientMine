use crate::terrain_height_patch::{RawHeightMap, CELL_SIZE, TERRAIN_PATCHSIZE};

// Using Metin2's exact C++ Normal Vector calculation algorithm
pub fn calculate_normal_metin2(
    x: usize,
    y: usize,
    start_x: usize,
    start_y: usize,
    height_map: &RawHeightMap,
    height_scale: f32,
) -> [f32; 3] {
    let global_x = start_x + x;
    let global_y = start_y + y;

    // normal.x = -m_fHeightScale * ((float)GetHeightMapValue((x-1),y)-(float)GetHeightMapValue((x+1),y));
    let hl = height_map.get_height(global_x.saturating_sub(1), global_y);
    let hr = height_map.get_height(global_x + 1, global_y);
    let mut nx = -height_scale * (hl - hr);

    // normal.y = -m_fHeightScale * ((float)GetHeightMapValue(x,(y-1))-(float)GetHeightMapValue(x,(y+1)));
    let hd = height_map.get_height(global_x, global_y.saturating_sub(1));
    let hu = height_map.get_height(global_x, global_y + 1);
    let mut ny = -height_scale * (hd - hu);

    // normal.z = 2.0f * static_cast<float>(CELLSCALE);
    let mut nz = 2.0 * CELL_SIZE;

    // normal *= 127.0f / D3DXVec3Length(&normal);
    let len = (nx * nx + ny * ny + nz * nz).sqrt();
    if len > 0.0 {
        let scale = 127.0 / len;
        nx *= scale;
        ny *= scale;
        nz *= scale;
    }

    [nx, ny, nz]
}

// Generate wireframe indices
pub fn build_patch_indices_wireframe(lod_level: u32) -> Vec<u16> {
    let step = match lod_level {
        0 => 1,
        1 => 2,
        _ => 4,
    };

    let mut indices = Vec::new();
    let verts_per_row = (TERRAIN_PATCHSIZE + 1) as u16;

    for y in (0..TERRAIN_PATCHSIZE).step_by(step) {
        for x in (0..TERRAIN_PATCHSIZE).step_by(step) {
            let current = (y as u16 * verts_per_row) + x as u16;
            let next_x = current + step as u16;
            let next_y = ((y + step) as u16 * verts_per_row) + x as u16;
            let next_xy = next_y + step as u16;

            // Wireframe lines
            // Top edge
            indices.push(current);
            indices.push(next_x);
            // Left edge
            indices.push(current);
            indices.push(next_y);
            // Diagonal
            indices.push(current);
            indices.push(next_xy);
            // Right edge
            indices.push(next_x);
            indices.push(next_xy);
            // Bottom edge
            indices.push(next_y);
            indices.push(next_xy);
        }
    }

    indices
}
