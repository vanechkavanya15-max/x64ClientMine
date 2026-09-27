#[cfg(test)]
mod tests {
    use m2_client::d3d9_effect_billboard_renderer::*;

    fn assert_f32_eq(a: f32, b: f32) {
        assert!((a - b).abs() < 1e-4, "Expected {}, got {}", b, a);
    }

    fn assert_vec3_eq(a: [f32; 3], b: [f32; 3]) {
        assert_f32_eq(a[0], b[0]);
        assert_f32_eq(a[1], b[1]);
        assert_f32_eq(a[2], b[2]);
    }

    #[test]
    fn test_spherical_alignment() {
        let quad = BillboardQuad {
            center: [10.0, 20.0, 30.0],
            size_x: 2.0,
            size_y: 4.0,
            rotation_rad: 0.0,
            color: 0xFFFFFFFF,
            uv_rect: [0.0, 0.0, 1.0, 1.0],
        };

        // Looking along +Y, so Right is +X, Up is +Z
        let cam_right = [1.0, 0.0, 0.0];
        let cam_up = [0.0, 0.0, 1.0];

        let corners = align_billboard_to_camera(&quad, cam_right, cam_up, false);
        
        // 0: Top-Left (-half_w, half_h) -> (-1, 2)
        assert_vec3_eq(corners[0], [9.0, 20.0, 32.0]);
        
        // 1: Top-Right (half_w, half_h) -> (1, 2)
        assert_vec3_eq(corners[1], [11.0, 20.0, 32.0]);
        
        // 2: Bottom-Left (-half_w, -half_h) -> (-1, -2)
        assert_vec3_eq(corners[2], [9.0, 20.0, 28.0]);
        
        // 3: Bottom-Right (half_w, -half_h) -> (1, -2)
        assert_vec3_eq(corners[3], [11.0, 20.0, 28.0]);
    }

    #[test]
    fn test_cylindrical_alignment() {
        let quad = BillboardQuad {
            center: [0.0, 0.0, 0.0],
            size_x: 2.0,
            size_y: 4.0,
            rotation_rad: 0.0,
            color: 0xFFFFFFFF,
            uv_rect: [0.0, 0.0, 1.0, 1.0],
        };

        // Camera is pitched, looking down slightly along +Y. Right is still +X.
        let cam_right = [1.0, 0.0, 0.0];
        // Up is tilted
        let cam_up = [0.0, 0.707, 0.707];

        let corners = align_billboard_to_camera(&quad, cam_right, cam_up, true);
        
        // In cylindrical, up is forced to [0, 0, 1]
        // 0: Top-Left (-half_w, half_h) -> (-1, 2)
        assert_vec3_eq(corners[0], [-1.0, 0.0, 2.0]);
        
        // 3: Bottom-Right (half_w, -half_h) -> (1, -2)
        assert_vec3_eq(corners[3], [1.0, 0.0, -2.0]);
    }

    #[test]
    fn test_cylindrical_look_down_edge_case() {
        let quad = BillboardQuad {
            center: [0.0, 0.0, 0.0],
            size_x: 2.0,
            size_y: 4.0,
            rotation_rad: 0.0,
            color: 0xFFFFFFFF,
            uv_rect: [0.0, 0.0, 1.0, 1.0],
        };

        // Looking perfectly straight down. Camera right might be zeroed out in XY if we rely on look dir,
        // but here we pass cam_right directly. If cam_right is also pointing down (invalid normally),
        // we test the fallback logic.
        let cam_right = [0.0, 0.0, -1.0]; // Zero length in XY plane
        let cam_up = [0.0, 1.0, 0.0];

        let corners = align_billboard_to_camera(&quad, cam_right, cam_up, true);
        
        // Fallback right is [1, 0, 0], up is [0, 0, 1]
        // Top-Right: (1, 2)
        assert_vec3_eq(corners[1], [1.0, 0.0, 2.0]);
    }

    #[test]
    fn test_quad_rotation() {
        let quad = BillboardQuad {
            center: [0.0, 0.0, 0.0],
            size_x: 2.0,
            size_y: 2.0,
            rotation_rad: std::f32::consts::PI / 2.0, // 90 degrees
            color: 0xFFFFFFFF,
            uv_rect: [0.0, 0.0, 1.0, 1.0],
        };

        let cam_right = [1.0, 0.0, 0.0];
        let cam_up = [0.0, 1.0, 0.0];

        let corners = align_billboard_to_camera(&quad, cam_right, cam_up, false);
        
        // Local without rotation:
        // TL: (-1, 1), TR: (1, 1), BL: (-1, -1), BR: (1, -1)
        // Rotate 90 deg (cos = 0, sin = 1):
        // rx = x * 0 - y * 1 = -y
        // ry = x * 1 + y * 0 = x
        // For TL (-1, 1): rx = -1, ry = -1 -> (-1, -1, 0)
        assert_vec3_eq(corners[0], [-1.0, -1.0, 0.0]);
        // For TR (1, 1): rx = -1, ry = 1 -> (-1, 1, 0)
        assert_vec3_eq(corners[1], [-1.0, 1.0, 0.0]);
    }

    #[test]
    fn test_transform_batch_size() {
        let quad = BillboardQuad {
            center: [0.0, 0.0, 0.0],
            size_x: 2.0,
            size_y: 2.0,
            rotation_rad: 0.0,
            color: 0xFFFFFFFF,
            uv_rect: [0.0, 0.0, 1.0, 1.0],
        };

        let particles = vec![quad.clone(), quad.clone(), quad.clone()];
        
        let buffer = transform_particles_to_screen_batch(
            &particles,
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            false
        );

        // 3 quads * 6 vertices * 24 bytes = 432 bytes
        assert_eq!(buffer.len(), 432);
    }

    #[test]
    fn test_vertex_buffer_data() {
        let quad = BillboardQuad {
            center: [0.0, 0.0, 0.0],
            size_x: 2.0,
            size_y: 2.0,
            rotation_rad: 0.0,
            color: 0x12345678,
            uv_rect: [0.1, 0.2, 0.9, 0.8], // u_min, v_min, u_max, v_max
        };

        let buffer = transform_particles_to_screen_batch(
            &[quad],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            false
        );

        // A single vertex is 24 bytes.
        // Vertex 0: Top-Left (-1, 1), color: 0x12345678, uv: (0.1, 0.2)
        let v0_bytes = &buffer[0..24];
        
        let pos_x = f32::from_le_bytes(v0_bytes[0..4].try_into().unwrap());
        let pos_y = f32::from_le_bytes(v0_bytes[4..8].try_into().unwrap());
        let pos_z = f32::from_le_bytes(v0_bytes[8..12].try_into().unwrap());
        let color = u32::from_le_bytes(v0_bytes[12..16].try_into().unwrap());
        let u = f32::from_le_bytes(v0_bytes[16..20].try_into().unwrap());
        let v = f32::from_le_bytes(v0_bytes[20..24].try_into().unwrap());

        assert_f32_eq(pos_x, -1.0);
        assert_f32_eq(pos_y, 1.0);
        assert_f32_eq(pos_z, 0.0);
        assert_eq!(color, 0x12345678);
        assert_f32_eq(u, 0.1);
        assert_f32_eq(v, 0.2);

        // Vertex 1: Top-Right (1, 1), uv: (0.9, 0.2)
        let v1_bytes = &buffer[24..48];
        let pos_x1 = f32::from_le_bytes(v1_bytes[0..4].try_into().unwrap());
        let u1 = f32::from_le_bytes(v1_bytes[16..20].try_into().unwrap());
        assert_f32_eq(pos_x1, 1.0);
        assert_f32_eq(u1, 0.9);
    }
}
