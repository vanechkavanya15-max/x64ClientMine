#[cfg(test)]
mod tests {
    use m2_client::wgpu_skybox_renderer::{
        generate_skybox_geometry, interpolate_color_local, update_time_of_day,
    };
    use m2_client::wgpu_skybox_lighting::EnvironmentUniforms;
    use glam::{Mat4, Vec3, Vec4};

    #[test]
    fn test_skybox_geometry_generation() {
        let (vertices, indices) = generate_skybox_geometry();

        // 6 faces * 4 vertices = 24 vertices
        assert_eq!(vertices.len(), 24, "Skybox should have exactly 24 vertices");

        // 6 faces * 2 triangles * 3 indices = 36 indices
        assert_eq!(indices.len(), 36, "Skybox should have exactly 36 indices");

        // Check bounds (all coordinates should be -1.0 or 1.0)
        for vertex in vertices {
            assert!(vertex.position[0].abs() == 1.0);
            assert!(vertex.position[1].abs() == 1.0);
            assert!(vertex.position[2].abs() == 1.0);
        }
    }

    #[test]
    fn test_interpolate_color_local() {
        let c1 = [0.0, 0.0, 0.0, 1.0];
        let c2 = [1.0, 1.0, 1.0, 1.0];

        let mid = interpolate_color_local(c1, c2, 0.5);
        assert_eq!(mid, [0.5, 0.5, 0.5, 1.0]);

        let beyond = interpolate_color_local(c1, c2, 1.5); // should clamp
        assert_eq!(beyond, [1.0, 1.0, 1.0, 1.0]);

        let below = interpolate_color_local(c1, c2, -0.5); // should clamp
        assert_eq!(below, [0.0, 0.0, 0.0, 1.0]);
    }

    #[test]
    fn test_update_time_of_day() {
        // Test keyframes
        let (ambient_m, diffuse_m, fog_m, sun_m) = update_time_of_day(0.0); // Morning start / Night end
        let (ambient_n, diffuse_n, fog_n, sun_n) = update_time_of_day(0.25); // Noon start
        let (ambient_s, diffuse_s, fog_s, sun_s) = update_time_of_day(0.5); // Sunset start
        let (ambient_night, diffuse_night, fog_night, sun_night) = update_time_of_day(0.75); // Night start

        // Values shouldn't be all zero
        assert_ne!(ambient_m, [0.0; 4]);
        assert_ne!(ambient_n, [0.0; 4]);
        assert_ne!(ambient_s, [0.0; 4]);
        assert_ne!(ambient_night, [0.0; 4]);

        // Sun direction should be normalized (length = 1.0)
        let check_len = |sun: [f32; 3]| {
            let len_sq = sun[0] * sun[0] + sun[1] * sun[1] + sun[2] * sun[2];
            assert!((len_sq - 1.0).abs() < 1e-4);
        };
        
        check_len(sun_m);
        check_len(sun_n);
        check_len(sun_s);
        check_len(sun_night);
    }
    
    #[test]
    fn test_skybox_view_proj_calculation() {
        let mut uniforms = EnvironmentUniforms::default();
        let view = Mat4::look_at_rh(
            Vec3::new(10.0, 20.0, 30.0), 
            Vec3::new(10.0, 20.0, 29.0), 
            Vec3::Y
        );
        let proj = Mat4::perspective_rh(std::f32::consts::PI / 4.0, 1.0, 0.1, 1000.0);
        
        // Remove translation from view matrix (this is what update_camera in wgpu_skybox_lighting does)
        let mut view_no_trans = view;
        view_no_trans.w_axis = Vec4::new(0.0, 0.0, 0.0, 1.0);
        
        let view_proj = proj * view_no_trans;
        uniforms.skybox_view_proj = view_proj.to_cols_array_2d();
        
        // Ensure translation is gone by transforming origin
        let origin = Vec4::new(0.0, 0.0, 0.0, 1.0);
        let transformed_origin = view_proj * origin;
        
        // In perspective projection, origin should map to (0,0,x,0) if translation is 0
        assert!(transformed_origin.x.abs() < 1e-5);
        assert!(transformed_origin.y.abs() < 1e-5);
        // z depends on near/far planes
    }
}
