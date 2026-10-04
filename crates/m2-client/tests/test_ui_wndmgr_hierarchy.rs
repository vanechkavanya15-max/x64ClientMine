#[path = "../src/ui_wndmgr_hierarchy.rs"]
mod ui_wndmgr_hierarchy;

use ui_wndmgr_hierarchy::{WndMgrHierarchy, FLAG_FLOAT};

#[test]
fn test_hierarchy_parent_child() {
    let mut hierarchy = WndMgrHierarchy::new();

    hierarchy.create_window(1); // Root
    hierarchy.create_window(2); // Child 1
    hierarchy.create_window(3); // Child 2
    hierarchy.create_window(4); // Grandchild 1

    hierarchy.set_parent(2, Some(1));
    hierarchy.set_parent(3, Some(1));
    hierarchy.set_parent(4, Some(2));

    // Test get_child_count
    assert_eq!(hierarchy.get_child_count(1), Some(2));
    assert_eq!(hierarchy.get_child_count(2), Some(1));
    assert_eq!(hierarchy.get_child_count(3), Some(0));

    // Test get_child
    let children_of_1 = vec![
        hierarchy.get_child(1, 0).unwrap(),
        hierarchy.get_child(1, 1).unwrap(),
    ];
    assert!(children_of_1.contains(&2));
    assert!(children_of_1.contains(&3));

    assert_eq!(hierarchy.get_child(2, 0), Some(4));
    
    // Reparent 4 to 3
    hierarchy.set_parent(4, Some(3));
    assert_eq!(hierarchy.get_child_count(2), Some(0));
    assert_eq!(hierarchy.get_child_count(3), Some(1));
    assert_eq!(hierarchy.get_child(3, 0), Some(4));
    
    // Unparent 4
    hierarchy.set_parent(4, None);
    assert_eq!(hierarchy.get_child_count(3), Some(0));
    assert_eq!(hierarchy.windows.get(&4).unwrap().parent_id, None);
}

#[test]
fn test_hierarchy_z_order() {
    let mut hierarchy = WndMgrHierarchy::new();

    hierarchy.create_window(1); // Root
    
    hierarchy.create_window(2); // Child 1
    hierarchy.create_window(3); // Child 2
    hierarchy.create_window(4); // Child 3 (with FLAG_FLOAT)
    
    hierarchy.set_parent(2, Some(1));
    hierarchy.set_parent(3, Some(1));
    hierarchy.set_parent(4, Some(1));
    
    // Set FLAG_FLOAT on child 3
    if let Some(w) = hierarchy.windows.get_mut(&4) {
        w.set_flag(FLAG_FLOAT, true);
    }
    
    // Check initial child order (should be 2, 3, 4)
    let root_children = hierarchy.windows.get(&1).unwrap().children.clone();
    assert_eq!(root_children, vec![2, 3, 4]);

    // Let's set FLAG_FLOAT on 2 and set_top it
    if let Some(w) = hierarchy.windows.get_mut(&2) {
        w.set_flag(FLAG_FLOAT, true);
    }
    hierarchy.set_top(2);
    let root_children = hierarchy.windows.get(&1).unwrap().children.clone();
    assert_eq!(root_children, vec![3, 4, 2]);

    // Now let's try bring_to_front on a grandchild
    hierarchy.create_window(5);
    hierarchy.set_parent(5, Some(3));
    
    // It shouldn't move 3 unless 3 has FLAG_FLOAT
    if let Some(w) = hierarchy.windows.get_mut(&3) {
        w.set_flag(FLAG_FLOAT, true);
    }
    if let Some(w) = hierarchy.windows.get_mut(&5) {
        w.set_flag(FLAG_FLOAT, true);
    }
    
    // bring_to_front on 5 should push 5 to top of 3, and 3 to top of 1
    hierarchy.bring_to_front(5);
    
    let root_children = hierarchy.windows.get(&1).unwrap().children.clone();
    assert_eq!(root_children, vec![4, 2, 3]); // 3 moved to the end
}

#[test]
fn test_hierarchy_visibility() {
    let mut hierarchy = WndMgrHierarchy::new();

    hierarchy.create_window(1); // Root
    hierarchy.create_window(2); // Child of 1
    hierarchy.create_window(3); // Child of 2
    hierarchy.create_window(4); // Child of 3
    
    hierarchy.set_parent(2, Some(1));
    hierarchy.set_parent(3, Some(2));
    hierarchy.set_parent(4, Some(3));
    
    // Initially, everything is hidden
    assert_eq!(hierarchy.windows.get(&1).unwrap().is_rendering, false);
    assert_eq!(hierarchy.windows.get(&4).unwrap().is_rendering, false);
    
    // Show root
    hierarchy.show(1);
    assert_eq!(hierarchy.windows.get(&1).unwrap().is_rendering, true);
    
    // 2, 3, 4 are still hidden
    assert_eq!(hierarchy.windows.get(&2).unwrap().is_rendering, false);
    
    // Show 2, 3, 4
    hierarchy.show(2);
    hierarchy.show(3);
    hierarchy.show(4);
    assert_eq!(hierarchy.windows.get(&4).unwrap().is_rendering, true);
    
    // Hide 2. This should recursively update is_rendering to false for 3 and 4
    hierarchy.hide(2);
    
    // 2 is explicitly hidden, so it shouldn't render
    assert_eq!(hierarchy.windows.get(&2).unwrap().is_rendering, false);
    // 3 and 4 are "show" = true, but their parent is hidden, so they shouldn't render
    assert_eq!(hierarchy.windows.get(&3).unwrap().is_rendering, false);
    assert_eq!(hierarchy.windows.get(&3).unwrap().is_show, true); // Still explicitly shown
    assert_eq!(hierarchy.windows.get(&4).unwrap().is_rendering, false);
    
    // Show 2 again
    hierarchy.show(2);
    assert_eq!(hierarchy.windows.get(&2).unwrap().is_rendering, true);
    assert_eq!(hierarchy.windows.get(&3).unwrap().is_rendering, true);
    assert_eq!(hierarchy.windows.get(&4).unwrap().is_rendering, true);
}
