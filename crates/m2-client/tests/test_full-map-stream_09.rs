#[path = "../src/minimap_navigation.rs"]
mod minimap_navigation;

#[path = "../src/minimap_tile_compositor.rs"]
mod minimap_tile_compositor;

use minimap_navigation::MinimapNavigation;
use minimap_tile_compositor::{MinimapCompositor, MarkerType};

#[test]
fn test_draw_tile_alpha_blending() {
    let mut compositor = MinimapCompositor::new(10, 10);
    // Draw base red tile
    let mut red_tile = vec![0; 4 * 4 * 4];
    for i in 0..16 {
        red_tile[i * 4] = 255;
        red_tile[i * 4 + 3] = 255;
    }
    compositor.draw_tile(0, 0, 4, 4, &red_tile);
    
    // Draw 50% transparent blue tile overlapping
    let mut blue_tile = vec![0; 4 * 4 * 4];
    for i in 0..16 {
        blue_tile[i * 4 + 2] = 255;
        blue_tile[i * 4 + 3] = 127;
    }
    compositor.draw_tile(2, 2, 4, 4, &blue_tile);
    
    // Check non-overlapping red part
    let idx = 0;
    assert_eq!(compositor.buffer[idx], 255);
    assert_eq!(compositor.buffer[idx + 1], 0);
    assert_eq!(compositor.buffer[idx + 2], 0);
    assert_eq!(compositor.buffer[idx + 3], 255);
    
    // Check overlapping part
    let idx2 = ((2 * 10 + 2) * 4) as usize;
    assert!(compositor.buffer[idx2] > 0 && compositor.buffer[idx2] < 255); // Red gets reduced
    assert_eq!(compositor.buffer[idx2 + 1], 0); // Green is 0
    assert!(compositor.buffer[idx2 + 2] > 0); // Blue is added
    assert_eq!(compositor.buffer[idx2 + 3], 255); // Alpha goes back up to 255 approximately or close
}

#[test]
fn test_draw_marker_world() {
    let mut compositor = MinimapCompositor::new(256, 256);
    let nav = MinimapNavigation::new(1000.0, 1000.0);
    
    // Will be at 128, 128 (center)
    compositor.draw_marker_world(500.0, 500.0, MarkerType::Player, &nav);
    
    let center_idx = ((128 * 256 + 128) * 4) as usize;
    assert_eq!(compositor.buffer[center_idx], 255); // Red
    assert_eq!(compositor.buffer[center_idx + 1], 0);
    assert_eq!(compositor.buffer[center_idx + 2], 0);
    assert_eq!(compositor.buffer[center_idx + 3], 255);
    
    // Offset part of the 3x3 square
    let offset_idx = ((127 * 256 + 127) * 4) as usize;
    assert_eq!(compositor.buffer[offset_idx], 255); // Red
}

#[test]
fn test_draw_marker_out_of_bounds_clipping() {
    let mut compositor = MinimapCompositor::new(10, 10);
    let nav = MinimapNavigation::new(100.0, 100.0);
    
    // Map position goes outside compositor buffer size
    compositor.draw_marker_world(-500.0, 2000.0, MarkerType::Npc, &nav);
    
    // Shouldn't crash, just clip and no state change
    for val in compositor.buffer.iter() {
        assert_eq!(*val, 0);
    }
}

#[test]
fn test_draw_marker_edge() {
    let mut compositor = MinimapCompositor::new(10, 10);
    compositor.draw_marker(0, 0, MarkerType::Waypoint);
    
    let idx = 0; // (0, 0)
    assert_eq!(compositor.buffer[idx], 0);
    assert_eq!(compositor.buffer[idx + 1], 0);
    assert_eq!(compositor.buffer[idx + 2], 255);
    assert_eq!(compositor.buffer[idx + 3], 255);
    
    // (-1, -1) was clipped
    // (1, 1) should be colored
    let idx2 = ((1 * 10 + 1) * 4) as usize;
    assert_eq!(compositor.buffer[idx2], 0);
    assert_eq!(compositor.buffer[idx2 + 1], 0);
    assert_eq!(compositor.buffer[idx2 + 2], 255);
    assert_eq!(compositor.buffer[idx2 + 3], 255);
}

#[test]
fn test_draw_tile_out_of_bounds() {
    let mut compositor = MinimapCompositor::new(10, 10);
    let mut tile = vec![0; 20 * 20 * 4];
    for i in 0..400 {
        tile[i * 4 + 1] = 255;
        tile[i * 4 + 3] = 255;
    }
    // Draw tile at offset where most of it is clipped
    compositor.draw_tile(-5, -5, 20, 20, &tile);
    
    // Only 0,0 to 9,9 gets drawn. Verify 0,0 is green
    let idx = 0;
    assert_eq!(compositor.buffer[idx], 0);
    assert_eq!(compositor.buffer[idx + 1], 255); // Green
    assert_eq!(compositor.buffer[idx + 2], 0);
    assert_eq!(compositor.buffer[idx + 3], 255);
}
