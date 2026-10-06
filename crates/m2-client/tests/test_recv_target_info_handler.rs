#[path = "../src/recv_target_info_handler.rs"]
mod recv_target_info_handler;

use recv_target_info_handler::*;

#[test]
fn test_packet_sizes() {
    assert_eq!(std::mem::size_of::<TPacketGCTarget>(), 2 + 2 + 4 + 1);
    assert_eq!(std::mem::size_of::<TPacketGCTargetCreate>(), 2 + 2 + 4 + 33);
    assert_eq!(std::mem::size_of::<TPacketGCTargetCreateNew>(), 2 + 2 + 4 + 33 + 4 + 1);
    assert_eq!(std::mem::size_of::<TPacketGCTargetUpdate>(), 2 + 2 + 4 + 4 + 4);
    assert_eq!(std::mem::size_of::<TPacketGCTargetDelete>(), 2 + 2 + 4);
}

#[test]
fn test_handle_target_packet() {
    let packet = TPacketGCTarget {
        header: 0,
        length: 9,
        dwVID: 12345,
        bHPPercent: 80,
    };

    // Test dead target
    let result = handle_target_packet(&packet, true, false, false);
    assert_eq!(
        result,
        TargetPacketResult {
            action: TargetBoardAction::CloseTargetBoard
        }
    );

    // Test PC/Building target
    let result = handle_target_packet(&packet, false, true, false);
    assert_eq!(
        result,
        TargetPacketResult {
            action: TargetBoardAction::CloseTargetBoardIfDifferent(12345)
        }
    );

    // Test normal target with viewable HP
    let result = handle_target_packet(&packet, false, false, true);
    assert_eq!(
        result,
        TargetPacketResult {
            action: TargetBoardAction::SetHPTargetBoard {
                vid: 12345,
                hp_percent: 80
            }
        }
    );

    // Test normal target with hidden HP
    let result = handle_target_packet(&packet, false, false, false);
    assert_eq!(
        result,
        TargetPacketResult {
            action: TargetBoardAction::CloseTargetBoard
        }
    );
}

#[test]
fn test_handle_target_create_packet() {
    let mut szTargetName = [0u8; 33];
    szTargetName[0] = b'T';
    szTargetName[1] = b'e';
    szTargetName[2] = b's';
    szTargetName[3] = b't';
    let packet = TPacketGCTargetCreate {
        header: 0,
        length: 41,
        lID: 100,
        szTargetName,
    };

    let result = handle_target_create_packet(&packet);
    assert_eq!(
        result,
        TargetCreateResult {
            lID: 100,
            szTargetName,
            open_atlas_window: true,
        }
    );
}

#[test]
fn test_handle_target_create_packet_new() {
    let mut szTargetName = [0u8; 33];
    szTargetName[0] = b'N';
    szTargetName[1] = b'P';
    szTargetName[2] = b'C';

    // Test Location Type
    let packet_loc = TPacketGCTargetCreateNew {
        header: 0,
        length: 46,
        lID: 200,
        szTargetName,
        dwVID: 0,
        byType: CREATE_TARGET_TYPE_LOCATION,
    };
    let result = handle_target_create_packet_new(&packet_loc);
    assert_eq!(
        result,
        TargetCreateNewResult {
            action: TargetCreateNewAction::Location {
                lID: 200,
                szTargetName,
            },
            open_atlas_window: true,
        }
    );

    // Test Character Type
    let packet_char = TPacketGCTargetCreateNew {
        header: 0,
        length: 46,
        lID: 201,
        szTargetName,
        dwVID: 9999,
        byType: CREATE_TARGET_TYPE_CHARACTER,
    };
    let result = handle_target_create_packet_new(&packet_char);
    assert_eq!(
        result,
        TargetCreateNewResult {
            action: TargetCreateNewAction::Character {
                lID: 201,
                szTargetName,
                dwVID: 9999,
                create_target_effect: true,
            },
            open_atlas_window: true,
        }
    );
}

#[test]
fn test_handle_target_update_packet() {
    let packet = TPacketGCTargetUpdate {
        header: 0,
        length: 16,
        lID: 300,
        lX: 5000,
        lY: 6000,
    };

    let result = handle_target_update_packet(&packet);
    assert_eq!(
        result,
        TargetUpdateResult {
            lID: 300,
            lX: 5000,
            lY: 6000,
            update_target: true,
            create_target_effect: true,
        }
    );
}

#[test]
fn test_handle_target_delete_packet() {
    let packet = TPacketGCTargetDelete {
        header: 0,
        length: 8,
        lID: 400,
    };

    let result = handle_target_delete_packet(&packet);
    assert_eq!(
        result,
        TargetDeleteResult {
            lID: 400,
            delete_target: true,
            delete_target_effect: true,
        }
    );
}
