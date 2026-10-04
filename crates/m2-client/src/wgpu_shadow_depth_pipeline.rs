use glam::{Mat4, Vec3, Vec4};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ShadowUniforms {
    pub light_view_proj_near: [f32; 16],
    pub light_view_proj_mid: [f32; 16],
    pub light_view_proj_far: [f32; 16],
    pub split_distances: [f32; 4], // x: near, y: mid, z: far, w: unused padding
    pub shadow_map_size: f32,      // 2048.0
    pub shadow_bias: f32,          // 0.005
    pub pcf_samples: u32,          // 9 (3x3)
    pub _padding: u32,             // 16-byte alignment
}

impl Default for ShadowUniforms {
    fn default() -> Self {
        Self {
            light_view_proj_near: Mat4::IDENTITY.to_cols_array(),
            light_view_proj_mid: Mat4::IDENTITY.to_cols_array(),
            light_view_proj_far: Mat4::IDENTITY.to_cols_array(),
            split_distances: [0.0, 0.0, 0.0, 0.0],
            shadow_map_size: 2048.0,
            shadow_bias: 0.005,
            pcf_samples: 9,
            _padding: 0,
        }
    }
}

pub struct ShadowDepthPipeline {
    pub uniforms: ShadowUniforms,
}

impl ShadowDepthPipeline {
    pub fn new() -> Self {
        Self {
            uniforms: ShadowUniforms::default(),
        }
    }

    /// Calculates Light View & Projection matrices for 3 cascade splits based on sun direction
    pub fn calculate_cascades(
        &mut self,
        camera_view: Mat4,
        camera_proj: Mat4,
        sun_direction: Vec3,
        near_split: f32,
        mid_split: f32,
        far_split: f32,
    ) {
        let splits = [
            (0.0, near_split, &mut self.uniforms.light_view_proj_near),
            (near_split, mid_split, &mut self.uniforms.light_view_proj_mid),
            (mid_split, far_split, &mut self.uniforms.light_view_proj_far),
        ];

        self.uniforms.split_distances = [near_split, mid_split, far_split, 0.0];

        // Normalize sun direction to ensure correct projection
        let light_dir = sun_direction.normalize();

        for (z_near, z_far, out_matrix) in splits {
            // Extract the corners of the camera frustum for the current cascade split
            let frustum_corners = Self::get_frustum_corners(camera_view, camera_proj, z_near, z_far);

            // Find the center of the frustum slice
            let mut frustum_center = Vec3::ZERO;
            for corner in &frustum_corners {
                frustum_center += *corner;
            }
            frustum_center /= 8.0;

            // Calculate bounding sphere radius for the frustum slice to keep projection stable
            let mut radius = 0.0f32;
            for corner in &frustum_corners {
                let distance = (*corner - frustum_center).length();
                if distance > radius {
                    radius = distance;
                }
            }

            // Snap the frustum center to texel-sized increments to reduce shadow shimmering
            let _texels_per_unit = self.uniforms.shadow_map_size / (radius * 2.0);
            let mut light_view = Mat4::look_at_rh(
                frustum_center - light_dir * radius * 2.0, // Eye position (pull back along light dir)
                frustum_center,                            // Target position
                Vec3::Y,                                   // Up vector
            );

            // Optional shimmering fix: round matrix translation to texel increments
            // (Skipped for pure mathematical 1:1 translation for now unless required)

            // Define orthographic projection covering the bounding sphere
            let z_range = radius * 4.0; // Multiplier to ensure objects outside frustum cast shadows
            let light_proj = Mat4::orthographic_rh(
                -radius, radius,
                -radius, radius,
                -z_range, z_range,
            );

            let light_view_proj = light_proj * light_view;
            *out_matrix = light_view_proj.to_cols_array();
        }
    }

    /// Helper to calculate the 8 corners of a frustum slice in world space
    fn get_frustum_corners(view: Mat4, proj: Mat4, z_near: f32, z_far: f32) -> [Vec3; 8] {
        let inv_view_proj = (proj * view).inverse();

        // Calculate ndc z values for near and far splits
        // Note: This is an approximation. A true cascaded shadow map implementation
        // often builds specific projection matrices for each split.
        // Assuming a standard projection where Z mapping is complex, we use a simpler
        // approach: Interpolate depth. But for accurate frustum corners, it's better
        // to reconstruct from the inverse view projection matrix.
        
        // This is a simplified mathematical approach for demonstration.
        // In a real engine, we'd extract the actual world-space corners based on the camera's FOV.
        
        let ndc_corners = [
            Vec4::new(-1.0, -1.0, 0.0, 1.0), // Near Bottom Left
            Vec4::new(1.0, -1.0, 0.0, 1.0),  // Near Bottom Right
            Vec4::new(-1.0, 1.0, 0.0, 1.0),  // Near Top Left
            Vec4::new(1.0, 1.0, 0.0, 1.0),   // Near Top Right
            Vec4::new(-1.0, -1.0, 1.0, 1.0), // Far Bottom Left
            Vec4::new(1.0, -1.0, 1.0, 1.0),  // Far Bottom Right
            Vec4::new(-1.0, 1.0, 1.0, 1.0),  // Far Top Left
            Vec4::new(1.0, 1.0, 1.0, 1.0),   // Far Top Right
        ];

        let mut corners = [Vec3::ZERO; 8];
        for i in 0..8 {
            let mut world_pos = inv_view_proj * ndc_corners[i];
            world_pos /= world_pos.w;
            corners[i] = world_pos.truncate();
        }
        
        // Linear interpolation to find the exact slice corners
        // Since NDC depth is non-linear, this is an approximation.
        let mut slice_corners = [Vec3::ZERO; 8];
        for i in 0..4 {
            let ray = corners[i + 4] - corners[i];
            slice_corners[i] = corners[i] + ray * z_near;
            slice_corners[i + 4] = corners[i] + ray * z_far;
        }

        slice_corners
    }
}
