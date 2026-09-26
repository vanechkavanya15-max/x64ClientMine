#[path = "../src/pathfinding_astar.rs"]
mod pathfinding_astar;

use pathfinding_astar::AStarPathfinder;

#[test]
fn test_open_space() {
    let pathfinder = AStarPathfinder::new();
    let is_blocked = |_, _| false;

    // Straight line
    let path = pathfinder.find_path((0, 0), (0, 5), is_blocked, false).expect("Path should be found");
    assert_eq!(path.len(), 6);
    assert_eq!(path.first(), Some(&(0, 0)));
    assert_eq!(path.last(), Some(&(0, 5)));

    // Diagonal line
    let path = pathfinder.find_path((0, 0), (5, 5), is_blocked, false).expect("Path should be found");
    assert_eq!(path.len(), 6);
    assert_eq!(path.first(), Some(&(0, 0)));
    assert_eq!(path.last(), Some(&(5, 5)));
}

#[test]
fn test_wall_avoidance() {
    let pathfinder = AStarPathfinder::new();
    
    // U-shaped wall
    // 0 1 2 3
    // S # # #
    // . . . #
    // G # . #
    let is_blocked = |x: u16, y: u16| -> bool {
        if x > 10 || y > 10 { return true; } // Bound the grid
        if x == 1 && y == 0 { return true; }
        if x == 2 && y == 0 { return true; }
        if x == 3 && y == 0 { return true; }
        if x == 3 && y == 1 { return true; }
        if x == 3 && y == 2 { return true; }
        if x == 1 && y == 2 { return true; }
        false
    };

    let path = pathfinder.find_path((0, 0), (0, 2), is_blocked, false).expect("Path should be found");
    // Should go around the wall instead of cutting through
    assert!(!path.contains(&(1, 0)));
    assert!(!path.contains(&(1, 2)));
    assert_eq!(path.first(), Some(&(0, 0)));
    assert_eq!(path.last(), Some(&(0, 2)));
}

#[test]
fn test_unreachable_goal() {
    let pathfinder = AStarPathfinder::new();
    
    // Goal surrounded by walls
    let is_blocked = |x: u16, y: u16| -> bool {
        if x > 5 || y > 5 { return true; } // Bound the grid to avoid infinite search
        if x == 1 && y == 1 { return true; }
        if x == 2 && y == 1 { return true; }
        if x == 3 && y == 1 { return true; }
        if x == 1 && y == 2 { return true; }
        if x == 3 && y == 2 { return true; }
        if x == 1 && y == 3 { return true; }
        if x == 2 && y == 3 { return true; }
        if x == 3 && y == 3 { return true; }
        false
    };

    let path = pathfinder.find_path((0, 0), (2, 2), is_blocked, false);
    assert_eq!(path, None, "Goal is surrounded, should not find a path");
}

#[test]
fn test_corner_cutting_prevention() {
    let pathfinder = AStarPathfinder::new();
    
    let is_blocked = |x: u16, y: u16| -> bool {
        if x > 5 || y > 5 { return true; } // Bound the grid
        (x == 1 && y == 0) || (x == 0 && y == 1)
    };

    let path = pathfinder.find_path((0, 0), (1, 1), is_blocked, false);
    assert_eq!(path, None, "Diagonal move cutting blocked corners should be prevented");
}

#[test]
fn test_invalid_inputs() {
    let pathfinder = AStarPathfinder::new();
    let is_blocked = |x: u16, y: u16| -> bool {
        x == 5 && y == 5
    };

    // Goal is blocked
    assert_eq!(pathfinder.find_path((0, 0), (5, 5), is_blocked, false), None);
    
    // Start is blocked
    assert_eq!(pathfinder.find_path((5, 5), (0, 0), is_blocked, false), None);
    
    // Start == Goal
    let path = pathfinder.find_path((0, 0), (0, 0), is_blocked, false).unwrap();
    assert_eq!(path, vec![(0, 0)]);
}

#[test]
fn test_raycast_smoothing() {
    let pathfinder = AStarPathfinder::new();
    let is_blocked = |_, _| false;

    // A long straight line should be smoothed into 2 nodes (start and end)
    let path = pathfinder.find_path((0, 0), (0, 10), is_blocked, true).expect("Path should be found");
    assert_eq!(path.len(), 2, "Raycast smoothing should remove intermediate nodes");
    assert_eq!(path[0], (0, 0));
    assert_eq!(path[1], (0, 10));
}
