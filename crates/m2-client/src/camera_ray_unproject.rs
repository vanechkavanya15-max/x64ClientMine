use glam::{Mat4, Vec3, Vec4};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ray3D {
    pub origin: [f32; 3],
    pub direction: [f32; 3],
}

pub fn screen_to_world_ray(
    screen_x: f32,
    screen_y: f32,
    w: f32,
    h: f32,
    view_proj: &[[f32; 4]; 4],
) -> Ray3D {
    // Handling edge cases for dimensions to avoid division by zero or invalid Ndc.
    if w <= 0.0 || h <= 0.0 {
        return Ray3D {
            origin: [0.0, 0.0, 0.0],
            direction: [0.0, 0.0, 1.0], // Safe fallback direction
        };
    }

    let ndc_x = (2.0 * screen_x / w) - 1.0;
    let ndc_y = 1.0 - (2.0 * screen_y / h);

    let view_proj_mat = Mat4::from_cols_array_2d(view_proj);
    let inv_vp = view_proj_mat.inverse();

    // Near point projection (Z = 0.0)
    let near_vec = Vec4::new(ndc_x, ndc_y, 0.0, 1.0);
    let near_unprojected = inv_vp * near_vec;
    let p_near = if near_unprojected.w != 0.0 {
        near_unprojected.truncate() / near_unprojected.w
    } else {
        near_unprojected.truncate()
    };

    // Far point projection (Z = 1.0)
    let far_vec = Vec4::new(ndc_x, ndc_y, 1.0, 1.0);
    let far_unprojected = inv_vp * far_vec;
    let p_far = if far_unprojected.w != 0.0 {
        far_unprojected.truncate() / far_unprojected.w
    } else {
        far_unprojected.truncate()
    };

    let origin = p_near;
    let direction_vec = p_far - p_near;
    
    // Normalize direction, fallback to forward if vector is 0
    let direction = if direction_vec.length_squared() > f32::EPSILON {
        direction_vec.normalize()
    } else {
        Vec3::new(0.0, 0.0, 1.0)
    };

    Ray3D {
        origin: [origin.x, origin.y, origin.z],
        direction: [direction.x, direction.y, direction.z],
    }
}
