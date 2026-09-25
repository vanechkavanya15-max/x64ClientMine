use m2_client::app_cursor::{AppCursor, CursorMode, CursorShape};

#[test]
fn test_initialization() {
    let mut cursor = AppCursor::new();
    assert!(cursor.is_ready());
    assert_eq!(cursor.get_cursor_mode(), CursorMode::Hardware);
    assert_eq!(cursor.get_cursor_visible(), true);
    assert_eq!(cursor.get_liar_cursor_on(), false);
    
    // Create cursors
    assert!(cursor.create_cursors());
    assert!(cursor.is_ready());
    assert_eq!(cursor.get_cursor_visible(), true);
    assert_eq!(cursor.get_liar_cursor_on(), false);
}

#[test]
fn test_destroy_cursors() {
    let mut cursor = AppCursor::new();
    cursor.create_cursors();
    assert!(cursor.is_ready());
    
    cursor.destroy_cursors();
    assert!(!cursor.is_ready());
}

#[test]
fn test_set_cursor_mode() {
    let mut cursor = AppCursor::new();
    
    cursor.set_cursor_mode(CursorMode::Software);
    assert_eq!(cursor.get_cursor_mode(), CursorMode::Software);
    
    cursor.set_cursor_mode(CursorMode::Hardware);
    assert_eq!(cursor.get_cursor_mode(), CursorMode::Hardware);
}

#[test]
fn test_set_cursor_visible() {
    let mut cursor = AppCursor::new();
    
    cursor.set_cursor_visible(false, true);
    assert_eq!(cursor.get_cursor_visible(), false);
    assert_eq!(cursor.get_liar_cursor_on(), true);
}

#[test]
fn test_continuous_cursor_logic() {
    let mut cursor = AppCursor::new();
    cursor.create_cursors(); // Need this to populate the map for Hardware mode
    
    // Initially normal
    assert_eq!(cursor.cursor_num, CursorShape::Normal);
    assert_eq!(cursor.continuous_cursor_num, CursorShape::Normal);
    
    // Set to Attack (continuous)
    assert!(cursor.set_cursor_num(CursorShape::Attack));
    assert_eq!(cursor.cursor_num, CursorShape::Attack);
    assert_eq!(cursor.continuous_cursor_num, CursorShape::Normal); // Wait, if setting Attack, previous was Normal (continuous). So continuous_cursor_num becomes Normal.
    
    // Set to Door (not continuous)
    assert!(cursor.set_cursor_num(CursorShape::Door));
    assert_eq!(cursor.cursor_num, CursorShape::Door);
    assert_eq!(cursor.continuous_cursor_num, CursorShape::Attack); // previous was Attack, which is continuous. So continuous becomes Attack.
    
    // Set to Normal. Since current (Door) is not continuous, it should restore from continuous_cursor_num (Attack)
    assert!(cursor.set_cursor_num(CursorShape::Normal));
    assert_eq!(cursor.cursor_num, CursorShape::Attack); // It overrides Normal with Attack
}

#[test]
fn test_set_cursor_num_missing_handle() {
    let mut cursor = AppCursor::new();
    // Do not call create_cursors, so handles map is empty
    assert_eq!(cursor.get_cursor_mode(), CursorMode::Hardware);
    
    // Should fail to set cursor because handle map is empty
    assert!(!cursor.set_cursor_num(CursorShape::Attack));
}
