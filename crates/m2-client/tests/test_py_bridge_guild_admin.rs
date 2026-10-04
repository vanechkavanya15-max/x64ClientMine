#[path = "../src/recv_guild_logic_v2.rs"]
mod recv_guild_logic_v2;

#[path = "../src/py_bridge_guild_management.rs"]
mod py_bridge_guild_management;

use py_bridge_guild_management::{
    m2_py_guild_get_guild_money,
    m2_py_guild_get_guild_experience,
    m2_py_guild_get_guild_level,
    m2_py_guild_get_grade_name,
    m2_py_guild_get_grade_authority,
    m2_py_guild_get_guild_board_comment_data,
};
use recv_guild_logic_v2::{get_guild_state, test_clear_guild_state, GuildGradeData, GuildCommentData};

#[test]
fn test_get_guild_money_exp_level() {
    test_clear_guild_state();

    // Verify it correctly handles disabled state (0)
    assert_eq!(m2_py_guild_get_guild_money(), 0);
    assert_eq!(m2_py_guild_get_guild_experience(), 0);
    assert_eq!(m2_py_guild_get_guild_level(), 0);

    {
        let mut state = get_guild_state().lock().unwrap();
        state.is_enabled = true;
        state.gold = 100500;
        state.exp = 4500;
        state.level = 15;
    }

    assert_eq!(m2_py_guild_get_guild_money(), 100500);
    assert_eq!(m2_py_guild_get_guild_experience(), 4500);
    assert_eq!(m2_py_guild_get_guild_level(), 15);
}

#[test]
fn test_get_grade_name_and_authority() {
    test_clear_guild_state();
    
    // Test disabled state
    let mut name_buf = [0u8; 64];
    assert!(!m2_py_guild_get_grade_name(1, name_buf.as_mut_ptr(), name_buf.len()));
    assert_eq!(m2_py_guild_get_grade_authority(1), 0);
    
    {
        let mut state = get_guild_state().lock().unwrap();
        state.is_enabled = true;
        
        state.grades.insert(1, GuildGradeData {
            auth_flag: 15,
            name: "Guild Master".to_string(),
        });
        
        state.grades.insert(5, GuildGradeData {
            auth_flag: 2,
            name: "Recruit".to_string(),
        });
    }
    
    let mut name_buf = [0u8; 64];
    let res = m2_py_guild_get_grade_name(1, name_buf.as_mut_ptr(), name_buf.len());
    assert!(res);
    
    // Find null terminator
    let null_idx = name_buf.iter().position(|&b| b == 0).unwrap();
    let name_str = std::str::from_utf8(&name_buf[..null_idx]).unwrap();
    assert_eq!(name_str, "Guild Master");
    assert_eq!(m2_py_guild_get_grade_authority(1), 15);
    
    // Test a short buffer
    let mut short_buf = [0u8; 4]; // can hold 3 chars + null
    let res = m2_py_guild_get_grade_name(5, short_buf.as_mut_ptr(), short_buf.len());
    assert!(res);
    let null_idx = short_buf.iter().position(|&b| b == 0).unwrap();
    let name_str = std::str::from_utf8(&short_buf[..null_idx]).unwrap();
    assert_eq!(name_str, "Rec");
    assert_eq!(m2_py_guild_get_grade_authority(5), 2);
    
    // Test missing
    let mut missing_buf = [0u8; 64];
    assert!(!m2_py_guild_get_grade_name(10, missing_buf.as_mut_ptr(), missing_buf.len()));
    assert_eq!(m2_py_guild_get_grade_authority(10), 0);
}

#[test]
fn test_get_guild_board_comment_data() {
    test_clear_guild_state();
    
    let mut id: u32 = 0;
    let mut name_buf = [0u8; 64];
    let mut comment_buf = [0u8; 128];
    
    // Disabled
    assert!(!m2_py_guild_get_guild_board_comment_data(
        0, &mut id, name_buf.as_mut_ptr(), name_buf.len(), comment_buf.as_mut_ptr(), comment_buf.len()
    ));

    {
        let mut state = get_guild_state().lock().unwrap();
        state.is_enabled = true;
        state.comments.push(GuildCommentData {
            comment_id: 100,
            name: "Player1".to_string(),
            comment: "Hello World".to_string(),
        });
    }

    assert!(m2_py_guild_get_guild_board_comment_data(
        0, &mut id, name_buf.as_mut_ptr(), name_buf.len(), comment_buf.as_mut_ptr(), comment_buf.len()
    ));

    assert_eq!(id, 100);
    
    let null_idx_name = name_buf.iter().position(|&b| b == 0).unwrap();
    assert_eq!(std::str::from_utf8(&name_buf[..null_idx_name]).unwrap(), "Player1");

    let null_idx_comment = comment_buf.iter().position(|&b| b == 0).unwrap();
    assert_eq!(std::str::from_utf8(&comment_buf[..null_idx_comment]).unwrap(), "Hello World");

    // Out of bounds
    assert!(!m2_py_guild_get_guild_board_comment_data(
        1, &mut id, name_buf.as_mut_ptr(), name_buf.len(), comment_buf.as_mut_ptr(), comment_buf.len()
    ));
}
