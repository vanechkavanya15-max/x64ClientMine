use m2_client::character_delete_flow::{
    TPacketCGPlayerDelete,
    TPacketGCPlayerDeleteSuccess,
    TPacketGCPlayerDeleteWrongSocialId,
    HEADER_CG_CHARACTER_DELETE,
    HEADER_GC_CHARACTER_DELETE_SUCCESS,
    HEADER_GC_CHARACTER_DELETE_WRONG_SOCIAL_ID,
    process_character_delete_response,
    handle_pin_input,
    handle_delete_modal_click,
    create_character_delete_modal,
    open_character_delete_modal,
    DELETE_MODAL_PIN,
    DELETE_MODAL_SLOT,
};
use m2_client::ui_bridge_ffi::{GLOBAL_CHARACTERS, CharacterSlot, SELECTED_CHAR_SLOT};
use m2_client::ui_window_mgr::WindowManager;

#[test]
fn test_packet_sizes() {
    assert_eq!(std::mem::size_of::<TPacketCGPlayerDelete>(), 13);
    assert_eq!(std::mem::size_of::<TPacketGCPlayerDeleteSuccess>(), 5);
    assert_eq!(std::mem::size_of::<TPacketGCPlayerDeleteWrongSocialId>(), 4);
}

#[test]
fn test_delete_code_validation() {
    let packet = TPacketCGPlayerDelete::new(2, "1234567");
    
    let h = packet.header;
    assert_eq!(h, HEADER_CG_CHARACTER_DELETE);
    let l = packet.length;
    assert_eq!(l, 13);
    let i = packet.index;
    assert_eq!(i, 2);
    
    let mut expected_pin = [0u8; 8];
    expected_pin[..7].copy_from_slice(b"1234567");
    let c = packet.private_code;
    assert_eq!(c, expected_pin);
}

#[test]
fn test_delete_success() {
    // Populate the global characters array
    {
        let mut chars = GLOBAL_CHARACTERS.lock().unwrap();
        chars.clear();
        chars.push(CharacterSlot { name: "Char1".to_string(), ..Default::default() });
        chars.push(CharacterSlot { name: "Char2".to_string(), ..Default::default() });
        chars.push(CharacterSlot { name: "Char3".to_string(), ..Default::default() });
    }
    {
        let mut selected = SELECTED_CHAR_SLOT.lock().unwrap();
        *selected = 1;
    }
    
    let success_packet = TPacketGCPlayerDeleteSuccess {
        header: HEADER_GC_CHARACTER_DELETE_SUCCESS,
        length: 5,
        account_index: 1,
    };
    
    let payload = unsafe {
        std::slice::from_raw_parts(
            (&success_packet as *const TPacketGCPlayerDeleteSuccess) as *const u8,
            std::mem::size_of::<TPacketGCPlayerDeleteSuccess>(),
        )
    };
    
    assert!(process_character_delete_response(HEADER_GC_CHARACTER_DELETE_SUCCESS, payload).is_ok());
    
    // Check if slot 1 was cleared
    {
        let chars = GLOBAL_CHARACTERS.lock().unwrap();
        assert_eq!(chars[0].name, "Char1");
        assert_eq!(chars[1].name, ""); // Should be cleared to default
        assert_eq!(chars[2].name, "Char3");
    }
}

#[test]
fn test_delete_wrong_id() {
    let wrong_id_packet = TPacketGCPlayerDeleteWrongSocialId {
        header: HEADER_GC_CHARACTER_DELETE_WRONG_SOCIAL_ID,
        length: 4,
    };
    
    let payload = unsafe {
        std::slice::from_raw_parts(
            (&wrong_id_packet as *const TPacketGCPlayerDeleteWrongSocialId) as *const u8,
            std::mem::size_of::<TPacketGCPlayerDeleteWrongSocialId>(),
        )
    };
    
    assert!(process_character_delete_response(HEADER_GC_CHARACTER_DELETE_WRONG_SOCIAL_ID, payload).is_ok());
}

#[test]
fn test_ui_modal_interactions() {
    let mut wm = WindowManager::new();
    create_character_delete_modal(&mut wm, 800, 600);
    open_character_delete_modal(&mut wm, 2);
    
    assert_eq!(*DELETE_MODAL_SLOT.lock().unwrap(), 2);
    
    handle_pin_input(&mut wm, '1');
    handle_pin_input(&mut wm, '2');
    handle_pin_input(&mut wm, '3');
    assert_eq!(*DELETE_MODAL_PIN.lock().unwrap(), "123");
    
    handle_pin_input(&mut wm, '\x08'); // backspace
    assert_eq!(*DELETE_MODAL_PIN.lock().unwrap(), "12");
    
    handle_pin_input(&mut wm, '3');
    handle_pin_input(&mut wm, '4');
    handle_pin_input(&mut wm, '5');
    handle_pin_input(&mut wm, '6');
    handle_pin_input(&mut wm, '7');
    assert_eq!(*DELETE_MODAL_PIN.lock().unwrap(), "1234567");
    
    // Click OK with valid PIN
    let handled = handle_delete_modal_click("DeleteCharacterConfirmButton", &mut wm);
    assert!(handled);
    
    // Test cancel
    open_character_delete_modal(&mut wm, 1);
    let handled = handle_delete_modal_click("DeleteCharacterCancelButton", &mut wm);
    assert!(handled);
}
