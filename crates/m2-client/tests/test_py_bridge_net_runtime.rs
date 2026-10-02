use std::ffi::CString;
use std::io::Read;
use std::net::TcpListener;
use std::sync::{Mutex, OnceLock};
use std::thread;
use std::time::Duration;

use m2_client::py_bridge_net_runtime::*;
use m2_client::ui_bridge_ffi::ACTIVE_GAME_SOCKET;
use m2_client::send_chat_packets_v2::{TPacketCGChat, TPacketCGWhisper};
use m2_client::send_combat_packets_v2::{TPacketCGAttack, TPacketCGUseSkill};
use m2_client::send_item_packets_v2::{
    TPacketCGItemDrop, TPacketCGItemDrop2, TPacketCGItemMove, TPacketCGItemUse,
};

static TEST_MUTEX: OnceLock<Mutex<()>> = OnceLock::new();

fn run_with_lock<F: FnOnce()>(f: F) {
    let _guard = TEST_MUTEX.get_or_init(|| Mutex::new(())).lock().unwrap();
    // Clean up active socket
    if let Ok(mut guard) = ACTIVE_GAME_SOCKET.lock() {
        *guard = None;
    }
    f();
}

fn setup_test_server() -> (u16, thread::JoinHandle<Vec<u8>>) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();

    let handle = thread::spawn(move || {
        let mut buf = Vec::new();
        if let Ok((mut stream, _)) = listener.accept() {
            // Read incoming data
            stream.set_read_timeout(Some(Duration::from_millis(500))).unwrap();
            let mut temp_buf = [0u8; 1024];
            while let Ok(n) = stream.read(&mut temp_buf) {
                if n == 0 {
                    break;
                }
                buf.extend_from_slice(&temp_buf[..n]);
            }
        }
        buf
    });

    (port, handle)
}

#[test]
fn test_net_connect_and_disconnect() {
    run_with_lock(|| {
        let (port, handle) = setup_test_server();
        let ip = CString::new("127.0.0.1").unwrap();

        // Connect
        let connected = m2_net_runtime_Connect(ip.as_ptr(), port);
        assert!(connected);
        
        {
            let guard = ACTIVE_GAME_SOCKET.lock().unwrap();
            assert!(guard.is_some());
        }

        // Disconnect
        m2_net_runtime_Disconnect();
        
        {
            let guard = ACTIVE_GAME_SOCKET.lock().unwrap();
            assert!(guard.is_none());
        }
        
        // Wait for thread to finish
        let _ = handle.join();
    });
}

#[test]
fn test_send_chat_packet() {
    run_with_lock(|| {
        let (port, handle) = setup_test_server();
        let ip = CString::new("127.0.0.1").unwrap();
        assert!(m2_net_runtime_Connect(ip.as_ptr(), port));

        let msg = CString::new("hello").unwrap();
        let chat_type = 1;
        
        assert!(m2_net_runtime_SendChatPacket(msg.as_ptr(), chat_type));

        m2_net_runtime_Disconnect();
        let data = handle.join().unwrap();
        
        assert!(!data.is_empty(), "Expected some data to be sent for chat packet");
        assert_eq!(data.len(), std::mem::size_of::<TPacketCGChat>() + 6);
    });
}

#[test]
fn test_send_whisper_packet() {
    run_with_lock(|| {
        let (port, handle) = setup_test_server();
        let ip = CString::new("127.0.0.1").unwrap();
        assert!(m2_net_runtime_Connect(ip.as_ptr(), port));

        let name = CString::new("player").unwrap();
        let msg = CString::new("psst").unwrap();
        
        assert!(m2_net_runtime_SendWhisperPacket(name.as_ptr(), msg.as_ptr()));

        m2_net_runtime_Disconnect();
        let data = handle.join().unwrap();
        
        assert!(!data.is_empty(), "Expected some data to be sent for whisper packet");
        assert_eq!(data.len(), std::mem::size_of::<TPacketCGWhisper>() + 5);
    });
}

#[test]
fn test_send_item_use_packet() {
    run_with_lock(|| {
        let (port, handle) = setup_test_server();
        let ip = CString::new("127.0.0.1").unwrap();
        assert!(m2_net_runtime_Connect(ip.as_ptr(), port));

        assert!(m2_net_runtime_SendItemUsePacket(1, 10));

        m2_net_runtime_Disconnect();
        let data = handle.join().unwrap();
        
        assert_eq!(data.len(), std::mem::size_of::<TPacketCGItemUse>());
    });
}

#[test]
fn test_send_item_move_packet() {
    run_with_lock(|| {
        let (port, handle) = setup_test_server();
        let ip = CString::new("127.0.0.1").unwrap();
        assert!(m2_net_runtime_Connect(ip.as_ptr(), port));

        assert!(m2_net_runtime_SendItemMovePacket(1, 10, 1, 11, 5));

        m2_net_runtime_Disconnect();
        let data = handle.join().unwrap();
        
        assert_eq!(data.len(), std::mem::size_of::<TPacketCGItemMove>());
    });
}

#[test]
fn test_send_item_drop_packet() {
    run_with_lock(|| {
        let (port, handle) = setup_test_server();
        let ip = CString::new("127.0.0.1").unwrap();
        assert!(m2_net_runtime_Connect(ip.as_ptr(), port));

        assert!(m2_net_runtime_SendItemDropPacket(1, 10, 2));

        m2_net_runtime_Disconnect();
        let data = handle.join().unwrap();
        
        assert_eq!(data.len(), std::mem::size_of::<TPacketCGItemDrop2>());
    });
}

#[test]
fn test_send_item_destroy_packet() {
    run_with_lock(|| {
        let (port, handle) = setup_test_server();
        let ip = CString::new("127.0.0.1").unwrap();
        assert!(m2_net_runtime_Connect(ip.as_ptr(), port));

        assert!(m2_net_runtime_SendItemDestroyPacket(1, 10));

        m2_net_runtime_Disconnect();
        let data = handle.join().unwrap();
        
        assert_eq!(data.len(), std::mem::size_of::<TPacketCGItemDrop>());
    });
}

#[test]
fn test_send_attack_packet() {
    run_with_lock(|| {
        let (port, handle) = setup_test_server();
        let ip = CString::new("127.0.0.1").unwrap();
        assert!(m2_net_runtime_Connect(ip.as_ptr(), port));

        assert!(m2_net_runtime_SendAttackPacket(0, 12345));

        m2_net_runtime_Disconnect();
        let data = handle.join().unwrap();
        
        assert_eq!(data.len(), std::mem::size_of::<TPacketCGAttack>());
    });
}

#[test]
fn test_send_enter_game_packet() {
    run_with_lock(|| {
        let (port, handle) = setup_test_server();
        let ip = CString::new("127.0.0.1").unwrap();
        assert!(m2_net_runtime_Connect(ip.as_ptr(), port));

        assert!(m2_net_runtime_SendEnterGame());

        m2_net_runtime_Disconnect();
        let data = handle.join().unwrap();
        
        assert_eq!(data.len(), 1); // 0x04 length is 1 byte
    });
}
