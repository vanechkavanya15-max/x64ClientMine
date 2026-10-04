use glam::{Vec2, Vec3};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VertexPNTTB {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub tex_coord: [f32; 2],
    pub tangent: [f32; 3],
    pub bitangent: [f32; 3],
}

pub struct MeshTangentGenerator;

impl MeshTangentGenerator {
    pub fn generate_tangents(
        indices: &[u32],
        positions: &[[f32; 3]],
        normals: &[[f32; 3]],
        uvs: &[[f32; 2]],
    ) -> Vec<VertexPNTTB> {
        let vertex_count = positions.len();
        if normals.len() != vertex_count || uvs.len() != vertex_count {
            return Vec::new();
        }

        let mut tangents = vec![Vec3::ZERO; vertex_count];
        let mut bitangents = vec![Vec3::ZERO; vertex_count];

        for chunk in indices.chunks_exact(3) {
            let i0 = chunk[0] as usize;
            let i1 = chunk[1] as usize;
            let i2 = chunk[2] as usize;

            if i0 >= vertex_count || i1 >= vertex_count || i2 >= vertex_count {
                continue;
            }

            let p0 = Vec3::from(positions[i0]);
            let p1 = Vec3::from(positions[i1]);
            let p2 = Vec3::from(positions[i2]);

            let uv0 = Vec2::from(uvs[i0]);
            let uv1 = Vec2::from(uvs[i1]);
            let uv2 = Vec2::from(uvs[i2]);

            let edge1 = p1 - p0;
            let edge2 = p2 - p0;
            let delta_uv1 = uv1 - uv0;
            let delta_uv2 = uv2 - uv0;

            let determinant = delta_uv1.x * delta_uv2.y - delta_uv2.x * delta_uv1.y;
            let f = if determinant.abs() > 1e-6 { 1.0 / determinant } else { 1.0 };
            
            let tangent = Vec3::new(
                f * (delta_uv2.y * edge1.x - delta_uv1.y * edge2.x),
                f * (delta_uv2.y * edge1.y - delta_uv1.y * edge2.y),
                f * (delta_uv2.y * edge1.z - delta_uv1.y * edge2.z),
            );

            let bitangent = Vec3::new(
                f * (-delta_uv2.x * edge1.x + delta_uv1.x * edge2.x),
                f * (-delta_uv2.x * edge1.y + delta_uv1.x * edge2.y),
                f * (-delta_uv2.x * edge1.z + delta_uv1.x * edge2.z),
            );

            tangents[i0] += tangent;
            tangents[i1] += tangent;
            tangents[i2] += tangent;

            bitangents[i0] += bitangent;
            bitangents[i1] += bitangent;
            bitangents[i2] += bitangent;
        }

        let mut result = Vec::with_capacity(vertex_count);
        for i in 0..vertex_count {
            let n = Vec3::from(normals[i]);
            let t = tangents[i];
            let b = bitangents[i];

            // Gram-Schmidt orthogonalization
            // T = normalize(t - n * dot(n, t))
            let n_dot_t = n.dot(t);
            let mut tangent = t - n * n_dot_t;
            if tangent.length_squared() > 1e-6 {
                tangent = tangent.normalize();
            } else {
                tangent = Vec3::X; // fallback
            }

            // Calculate handedness (sign)
            // let sign = if n.cross(t).dot(b) < 0.0 { -1.0 } else { 1.0 };

            // Orthogonalize bitangent
            let n_dot_b = n.dot(b);
            let t_dot_b = tangent.dot(b);
            let mut bitangent = b - n * n_dot_b - tangent * t_dot_b;
            if bitangent.length_squared() > 1e-6 {
                bitangent = bitangent.normalize();
            } else {
                bitangent = n.cross(tangent).normalize_or_zero();
            }

            result.push(VertexPNTTB {
                position: positions[i],
                normal: normals[i],
                tex_coord: uvs[i],
                tangent: tangent.into(),
                bitangent: bitangent.into(),
            });
        }

        result
    }
}
