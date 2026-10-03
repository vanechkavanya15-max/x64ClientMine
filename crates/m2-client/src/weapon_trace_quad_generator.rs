use glam::{Vec3, Vec4};

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub struct WeaponTraceQuadVertex {
    pub position: Vec3,
    pub tex_coord: [f32; 2],
    pub color: [f32; 4],
}

// For proper memory layout compatibility with wgpu
unsafe impl bytemuck::Zeroable for WeaponTraceQuadVertex {}
unsafe impl bytemuck::Pod for WeaponTraceQuadVertex {}

pub struct WeaponTraceQuadGenerator;

impl WeaponTraceQuadGenerator {
    /// Generates a triangle list from a sequence of interleaved trace vertices
    /// which are assumed to be structured as pairs of (long_edge, short_edge).
    /// `input_vertices` format: [long0, short0, long1, short1, long2, short2, ...]
    /// `base_color` format: RGB color vector, alpha will be blended with vertex alpha.
    pub fn generate_quad_strip(
        input_vertices: &[crate::weapon_trace::TraceVertex],
        base_color: Vec3,
    ) -> Vec<WeaponTraceQuadVertex> {
        let mut output = Vec::new();
        let pair_count = input_vertices.len() / 2;

        if pair_count < 2 {
            return output;
        }

        // Generate triangles for the quad strip
        // For each segment between pair `i` and `i+1`:
        // L_i   --- L_i+1
        //  |   \     |
        // S_i   --- S_i+1
        //
        // Triangles: (L_i, S_i, L_i+1) and (S_i, S_i+1, L_i+1)

        for i in 0..(pair_count - 1) {
            let idx = i * 2;
            let l0 = &input_vertices[idx];
            let s0 = &input_vertices[idx + 1];
            let l1 = &input_vertices[idx + 2];
            let s1 = &input_vertices[idx + 3];

            let convert_vertex = |v: &crate::weapon_trace::TraceVertex| -> WeaponTraceQuadVertex {
                WeaponTraceQuadVertex {
                    position: v.position,
                    tex_coord: [v.u, v.v],
                    color: [base_color.x, base_color.y, base_color.z, v.alpha.clamp(0.0, 1.0)],
                }
            };

            let vl0 = convert_vertex(l0);
            let vs0 = convert_vertex(s0);
            let vl1 = convert_vertex(l1);
            let vs1 = convert_vertex(s1);

            // First triangle (L_i, S_i, L_i+1)
            output.push(vl0);
            output.push(vs0);
            output.push(vl1);

            // Second triangle (S_i, S_i+1, L_i+1)
            output.push(vs0);
            output.push(vs1);
            output.push(vl1);
        }

        output
    }
}
