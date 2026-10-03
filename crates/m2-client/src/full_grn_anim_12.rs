use crate::wgpu_granny_actor_renderer::GrannyActorVertex;
use crate::granny_mesh_decompressor::GrannyPwnt3432Vertex;

pub fn extract_vertex_weights(vertices: &[GrannyPwnt3432Vertex]) -> Result<Vec<GrannyActorVertex>, String> {
    if vertices.is_empty() {
        return Ok(Vec::new());
    }

    let mut result = Vec::with_capacity(vertices.len());
    let inv_255 = 1.0 / 255.0;

    for v in vertices {
        let bw = v.bone_weights;
        let bi = v.bone_indices;
        
        result.push(GrannyActorVertex {
            position: v.position,
            normal: v.normal,
            tex_coord: v.uv,
            bone_indices: [
                bi[0] as u32,
                bi[1] as u32,
                bi[2] as u32,
                bi[3] as u32,
            ],
            bone_weights: [
                bw[0] as f32 * inv_255,
                bw[1] as f32 * inv_255,
                bw[2] as f32 * inv_255,
                bw[3] as f32 * inv_255,
            ],
        });
    }

    Ok(result)
}
