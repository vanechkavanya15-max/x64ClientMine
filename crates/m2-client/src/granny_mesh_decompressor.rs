use std::os::raw::c_void;

#[repr(C)]
pub struct GrannyPwnt3432Vertex {
    pub position: [f32; 3],
    pub bone_weights: [u8; 4],
    pub bone_indices: [u8; 4],
    pub normal: [f32; 3],
    pub uv: [f32; 2],
}

#[repr(C)]
pub struct GrannyTriMaterialGroup {
    pub material_index: i32,
    pub tri_first: i32,
    pub tri_count: i32,
}

#[repr(C)]
pub struct GrannyTriTopology {
    pub group_count: i32,
    pub groups: *mut GrannyTriMaterialGroup,
    pub index_count: i32,
    pub indices: *mut i32,
    pub index16_count: i32,
    pub indices16: *mut u16,
    pub vertex_to_vertex_count: i32,
    pub vertex_to_vertex_map: *mut i32,
    pub vertex_to_triangle_count: i32,
    pub vertex_to_triangle_map: *mut i32,
    pub side_to_neighbor_count: i32,
    pub side_to_neighbor_map: *mut u32,
    pub polygon_index_start_count: i32,
    pub polygon_index_starts: *mut i32,
}

#[repr(C)]
pub struct GrannyVertexData {
    pub vertex_type: *mut c_void, // granny_data_type_definition*
    pub vertex_count: i32,
    pub vertices: *mut u8,
    pub vertex_component_name_count: i32,
    pub vertex_component_names: *mut *const i8,
    pub vertex_annotation_set_count: i32,
    pub vertex_annotation_sets: *mut c_void, // granny_vertex_annotation_set*
}

#[repr(C)]
pub struct GrannyMesh {
    pub name: *const i8,
    pub primary_vertex_data: *mut GrannyVertexData,
    pub morph_target_count: i32,
    pub morph_targets: *mut c_void, // granny_morph_target*
    pub primary_topology: *mut GrannyTriTopology,
    pub material_binding_count: i32,
    pub material_bindings: *mut c_void, // granny_material_binding*
    pub bone_binding_count: i32,
    pub bone_bindings: *mut c_void, // granny_bone_binding*
    pub extended_data: [u8; 24], // granny_variant (pointer, pointer, array size)
}

#[derive(Debug)]
pub struct DecompressedMesh {
    pub vertex_count: usize,
    pub triangle_count: usize,
    pub positions: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub uvs: Vec<[f32; 2]>,
    pub bone_indices: Vec<[u8; 4]>,
    pub bone_weights: Vec<[f32; 4]>,
    pub indices: Vec<u16>,
}

pub fn decompress_granny_mesh(raw_mesh_ptr: *const c_void) -> Result<DecompressedMesh, String> {
    if raw_mesh_ptr.is_null() {
        return Err("raw_mesh_ptr is null".to_string());
    }

    let mesh = unsafe { &*(raw_mesh_ptr as *const GrannyMesh) };

    if mesh.primary_vertex_data.is_null() {
        return Err("primary_vertex_data is null".to_string());
    }

    let vertex_data = unsafe { &*mesh.primary_vertex_data };
    let vertex_count = vertex_data.vertex_count as usize;

    if vertex_data.vertices.is_null() {
        return Err("vertices array is null".to_string());
    }

    let mut positions = Vec::with_capacity(vertex_count);
    let mut normals = Vec::with_capacity(vertex_count);
    let mut uvs = Vec::with_capacity(vertex_count);
    let mut bone_indices = Vec::with_capacity(vertex_count);
    let mut bone_weights = Vec::with_capacity(vertex_count);

    let vertices_slice = unsafe {
        std::slice::from_raw_parts(
            vertex_data.vertices as *const GrannyPwnt3432Vertex,
            vertex_count,
        )
    };

    for v in vertices_slice {
        positions.push(v.position);
        normals.push(v.normal);
        uvs.push(v.uv);
        bone_indices.push(v.bone_indices);

        let w_sum = v.bone_weights.iter().map(|&w| w as u32).sum::<u32>();
        
        if w_sum == 0 {
            bone_weights.push([1.0, 0.0, 0.0, 0.0]);
        } else {
            let inv_sum = 1.0 / (w_sum as f32);
            bone_weights.push([
                (v.bone_weights[0] as f32) * inv_sum,
                (v.bone_weights[1] as f32) * inv_sum,
                (v.bone_weights[2] as f32) * inv_sum,
                (v.bone_weights[3] as f32) * inv_sum,
            ]);
        }
    }

    if mesh.primary_topology.is_null() {
        return Err("primary_topology is null".to_string());
    }

    let topology = unsafe { &*mesh.primary_topology };
    
    let mut indices = Vec::new();
    let index16_count = topology.index16_count as usize;
    let index32_count = topology.index_count as usize;

    if index16_count > 0 {
        if topology.indices16.is_null() {
             return Err("indices16 array is null".to_string());
        }
        let indices_slice = unsafe {
            std::slice::from_raw_parts(topology.indices16, index16_count)
        };
        indices.extend_from_slice(indices_slice);
    } else if index32_count > 0 {
         if topology.indices.is_null() {
             return Err("indices array is null".to_string());
        }
        let indices_slice = unsafe {
            std::slice::from_raw_parts(topology.indices, index32_count)
        };
        for &idx in indices_slice {
            indices.push(idx as u16);
        }
    }

    let triangle_count = indices.len() / 3;

    Ok(DecompressedMesh {
        vertex_count,
        triangle_count,
        positions,
        normals,
        uvs,
        bone_indices,
        bone_weights,
        indices,
    })
}

pub fn calculate_bounding_box(mesh: &DecompressedMesh) -> ([f32; 3], [f32; 3]) {
    if mesh.positions.is_empty() {
        return ([0.0, 0.0, 0.0], [0.0, 0.0, 0.0]);
    }

    let mut min_box = mesh.positions[0];
    let mut max_box = mesh.positions[0];

    for p in mesh.positions.iter().skip(1) {
        if p[0] < min_box[0] { min_box[0] = p[0]; }
        if p[1] < min_box[1] { min_box[1] = p[1]; }
        if p[2] < min_box[2] { min_box[2] = p[2]; }

        if p[0] > max_box[0] { max_box[0] = p[0]; }
        if p[1] > max_box[1] { max_box[1] = p[1]; }
        if p[2] > max_box[2] { max_box[2] = p[2]; }
    }

    (min_box, max_box)
}
