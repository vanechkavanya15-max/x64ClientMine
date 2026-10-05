use m2_client::py_bridge_minimap_controls::*;

#[test]
fn test_minimap_size_and_create_destroy() {
    reset_minimap_controls_state();
    
    // Check initial state
    {
        let state = get_minimap_controls_state().read().unwrap();
        assert_eq!(state.is_created, false);
        assert_eq!(state.width, 0.0);
        assert_eq!(state.height, 0.0);
    }
    
    // Create
    assert!(m2_py_minimap_create());
    {
        let state = get_minimap_controls_state().read().unwrap();
        assert_eq!(state.is_created, true);
    }
    
    // Set size
    m2_py_minimap_set_minimap_size(256.0, 256.0);
    {
        let state = get_minimap_controls_state().read().unwrap();
        assert_eq!(state.width, 256.0);
        assert_eq!(state.height, 256.0);
    }
    
    // Destroy
    m2_py_minimap_destroy();
    {
        let state = get_minimap_controls_state().read().unwrap();
        assert_eq!(state.is_created, false);
    }
}

#[test]
fn test_minimap_scale_up_down() {
    reset_minimap_controls_state();
    
    {
        let state = get_minimap_controls_state().read().unwrap();
        assert_eq!(state.scale, 1.0);
    }
    
    m2_py_minimap_scale_up();
    {
        let state = get_minimap_controls_state().read().unwrap();
        assert_eq!(state.scale, 2.0);
    }
    
    m2_py_minimap_scale_up();
    {
        let state = get_minimap_controls_state().read().unwrap();
        assert_eq!(state.scale, 4.0);
    }
    
    // Cap at 4.0
    m2_py_minimap_scale_up();
    {
        let state = get_minimap_controls_state().read().unwrap();
        assert_eq!(state.scale, 4.0);
    }
    
    m2_py_minimap_scale_down();
    {
        let state = get_minimap_controls_state().read().unwrap();
        assert_eq!(state.scale, 2.0);
    }
    
    m2_py_minimap_scale_down();
    m2_py_minimap_scale_down();
    {
        let state = get_minimap_controls_state().read().unwrap();
        assert_eq!(state.scale, 0.5);
    }
    
    // Cap at 0.5
    m2_py_minimap_scale_down();
    {
        let state = get_minimap_controls_state().read().unwrap();
        assert_eq!(state.scale, 0.5);
    }
}

#[test]
fn test_minimap_center_position_and_update() {
    reset_minimap_controls_state();
    
    m2_py_minimap_set_center_position(26600.0, 1000.0);
    {
        let state = get_minimap_controls_state().read().unwrap();
        assert_eq!(state.center_x, 26600.0);
        assert_eq!(state.center_y, 1000.0);
        
        // Terrain size is 25600, cell scale is 200
        // coord X = floor(26600 / 25600) = 1
        // center cell x = (26600 - 1 * 25600) / 200 = 1000 / 200 = 5.0
        assert_eq!(state.center_cell_x, 5.0);
        
        // coord Y = floor(1000 / 25600) = 0
        // center cell y = (1000 - 0) / 200 = 5.0
        assert_eq!(state.center_cell_y, 5.0);
    }
    
    // Test update
    m2_py_minimap_update(26600.0, 1000.0); // Should be a no-op
    {
        let state = get_minimap_controls_state().read().unwrap();
        assert_eq!(state.center_x, 26600.0);
    }
    
    m2_py_minimap_update(51200.0, 25600.0);
    {
        let state = get_minimap_controls_state().read().unwrap();
        assert_eq!(state.center_x, 51200.0);
        assert_eq!(state.center_y, 25600.0);
        
        // coord X = 2
        // cell x = (51200 - 2 * 25600) / 200 = 0.0
        assert_eq!(state.center_cell_x, 0.0);
        
        // coord Y = 1
        // cell y = (25600 - 1 * 25600) / 200 = 0.0
        assert_eq!(state.center_cell_y, 0.0);
    }
}
