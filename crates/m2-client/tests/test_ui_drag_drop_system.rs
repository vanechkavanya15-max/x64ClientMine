use m2_client::ui_drag_drop_system::get_drag_drop_system;
use m2_client::inventory_grid_v2::WindowType;
use m2_client::belt_inventory_gate_v2::ITEM_TYPE_POTION;
use m2_client::send_item_storage_packets::{HEADER_CG_ITEM_MOVE, TPacketCGItemMove, HEADER_CG_ITEM_DROP, TPacketCGItemDrop};
use std::mem::size_of;
use std::ptr;

#[test]
fn test_drag_drop_state_transitions() {
    let sys_mutex = get_drag_drop_system();
    
    // Clear state
    {
        let mut sys = sys_mutex.lock().unwrap();
        sys.cancel_drag();
        assert!(!sys.state.is_dragging);
    }
    
    // Start drag
    {
        let mut sys = sys_mutex.lock().unwrap();
        sys.on_drag_start(1, 10, 50201, 200, 1, 1);
        assert!(sys.state.is_dragging);
        assert_eq!(sys.state.dragged_window_type, 1);
        assert_eq!(sys.state.dragged_cell, 10);
        assert_eq!(sys.state.dragged_vnum, 50201);
        assert_eq!(sys.state.dragged_count, 200);
    }
    
    // Cancel drag via ESC
    {
        let mut sys = sys_mutex.lock().unwrap();
        let handled = sys.handle_key_press(27);
        assert!(handled);
        assert!(!sys.state.is_dragging);
    }

    // Start drag
    {
        let mut sys = sys_mutex.lock().unwrap();
        sys.on_drag_start(1, 10, 50201, 200, 1, 1);
    }

    // Cancel drag via Right Click
    {
        let mut sys = sys_mutex.lock().unwrap();
        let handled = sys.handle_mouse_click(1);
        assert!(handled);
        assert!(!sys.state.is_dragging);
    }
}

#[test]
fn test_on_drop_on_inventory() {
    let sys_mutex = get_drag_drop_system();
    
    // Clear state
    {
        let mut sys = sys_mutex.lock().unwrap();
        sys.cancel_drag();
    }
    
    // Try to drop without dragging
    {
        let mut sys = sys_mutex.lock().unwrap();
        let mut buf = [0u8; 1024];
        let written = sys.on_drop_on_inventory(1, 15, buf.as_mut_ptr());
        assert_eq!(written, 0);
    }
    
    // Start drag
    {
        let mut sys = sys_mutex.lock().unwrap();
        sys.on_drag_start(1, 10, 50201, 200, 1, 1);
    }
    
    // Drop
    {
        let mut sys = sys_mutex.lock().unwrap();
        let mut buf = [0u8; 1024];
        let written = sys.on_drop_on_inventory(1, 15, buf.as_mut_ptr());
        
        assert_eq!(written, size_of::<TPacketCGItemMove>());
        
        let packet: TPacketCGItemMove = unsafe { std::ptr::read_unaligned(buf.as_ptr() as *const _) };
        let header = { packet.header };
        let pos_window_type = { packet.pos.window_type };
        let pos_cell = unsafe { ptr::addr_of!(packet.pos.cell).read_unaligned() };
        let change_pos_window_type = { packet.change_pos.window_type };
        let change_pos_cell = unsafe { ptr::addr_of!(packet.change_pos.cell).read_unaligned() };
        let num = { packet.num };
        
        assert_eq!(header, HEADER_CG_ITEM_MOVE);
        assert_eq!(pos_window_type, 1);
        assert_eq!(pos_cell, 10);
        assert_eq!(change_pos_window_type, 1);
        assert_eq!(change_pos_cell, 15);
        assert_eq!(num, 200);
        
        // State should be reset
        assert!(!sys.state.is_dragging);
    }
}

#[test]
fn test_on_drop_on_belt() {
    let sys_mutex = get_drag_drop_system();
    
    // Start drag
    {
        let mut sys = sys_mutex.lock().unwrap();
        sys.on_drag_start(1, 5, 27001, 100, 1, 1);
    }
    
    // Drop on belt (allowable item type - POTION)
    {
        let mut sys = sys_mutex.lock().unwrap();
        let mut buf = [0u8; 1024];
        let written = sys.on_drop_on_belt(0, ITEM_TYPE_POTION, 1, buf.as_mut_ptr());
        
        assert_eq!(written, size_of::<TPacketCGItemMove>());
        
        let packet: TPacketCGItemMove = unsafe { std::ptr::read_unaligned(buf.as_ptr() as *const _) };
        let header = { packet.header };
        let pos_window_type = { packet.pos.window_type };
        let pos_cell = unsafe { ptr::addr_of!(packet.pos.cell).read_unaligned() };
        let change_pos_window_type = { packet.change_pos.window_type };
        let change_pos_cell = unsafe { ptr::addr_of!(packet.change_pos.cell).read_unaligned() };
        let num = { packet.num };
        
        assert_eq!(header, HEADER_CG_ITEM_MOVE);
        assert_eq!(pos_window_type, 1);
        assert_eq!(pos_cell, 5);
        assert_eq!(change_pos_window_type, WindowType::BELT_INVENTORY as u8);
        assert_eq!(change_pos_cell, 0);
        assert_eq!(num, 100);
        
        assert!(!sys.state.is_dragging);
    }
    
    // Test unallowable item type
    {
        let mut sys = sys_mutex.lock().unwrap();
        sys.on_drag_start(1, 5, 10, 1, 1, 1); // e.g. sword
        
        let mut buf = [0u8; 1024];
        // 1 = WEAPON
        let written = sys.on_drop_on_belt(0, 1, 1, buf.as_mut_ptr());
        
        assert_eq!(written, 0); // Not written because belt can't store weapons
        assert!(!sys.state.is_dragging); // Should cancel drag
    }
}

#[test]
fn test_on_drop_on_exchange() {
    let sys_mutex = get_drag_drop_system();
    
    // Start drag
    {
        let mut sys = sys_mutex.lock().unwrap();
        sys.on_drag_start(1, 20, 11299, 1, 1, 2);
    }
    
    // Drop on exchange
    {
        let mut sys = sys_mutex.lock().unwrap();
        let mut buf = [0u8; 1024];
        let written = sys.on_drop_on_exchange(0, buf.as_mut_ptr()); // pos 0 in exchange window
        
        assert!(written > 0);
        assert!(!sys.state.is_dragging);
    }
}

#[test]
fn test_on_drop_on_quickslot() {
    let sys_mutex = get_drag_drop_system();
    
    // Start drag
    {
        let mut sys = sys_mutex.lock().unwrap();
        sys.on_drag_start(1, 30, 27002, 200, 1, 1);
    }
    
    // Drop on quickslot
    {
        let mut sys = sys_mutex.lock().unwrap();
        let success = sys.on_drop_on_quickslot(0); // QuickSlot index 0
        
        assert!(success);
        assert!(!sys.state.is_dragging);
    }
}

#[test]
fn test_on_drop_on_ground() {
    let sys_mutex = get_drag_drop_system();
    
    // Start drag
    {
        let mut sys = sys_mutex.lock().unwrap();
        sys.on_drag_start(1, 40, 19, 1, 1, 1);
    }
    
    // Try drop on ground
    {
        let mut sys = sys_mutex.lock().unwrap();
        let success = sys.try_drop_on_ground();
        assert!(success);
        assert!(sys.state.is_dragging); // still dragging, waiting for confirmation
        assert!(sys.state.is_drop_dialog_open);
    }

    // Confirm drop on ground
    {
        let mut sys = sys_mutex.lock().unwrap();
        let mut buf = [0u8; 1024];
        let written = sys.confirm_drop_on_ground(0, buf.as_mut_ptr()); // gold = 0
        
        assert_eq!(written, size_of::<TPacketCGItemDrop>());
        
        let packet: TPacketCGItemDrop = unsafe { std::ptr::read_unaligned(buf.as_ptr() as *const _) };
        let header = { packet.header };
        let pos_window_type = { packet.pos.window_type };
        let pos_cell = unsafe { ptr::addr_of!(packet.pos.cell).read_unaligned() };
        let gold = unsafe { ptr::addr_of!(packet.gold).read_unaligned() };
        let count = { packet.count };
        
        assert_eq!(header, HEADER_CG_ITEM_DROP);
        assert_eq!(pos_window_type, 1);
        assert_eq!(pos_cell, 40);
        assert_eq!(gold, 0);
        assert_eq!(count, 1);
        
        assert!(!sys.state.is_dragging);
        assert!(!sys.state.is_drop_dialog_open);
    }
}
