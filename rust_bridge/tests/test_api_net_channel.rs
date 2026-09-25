use m2_client_bridge::api::net_channel::*;
use std::ptr;

#[test]
fn test_channel_creation() {
    let channel = m2_net_channel_create();
    assert!(!channel.is_null());
    m2_net_channel_destroy(channel);
}

#[test]
fn test_send_and_receive() {
    let channel = m2_net_channel_create();
    assert!(!channel.is_null());

    let payload = b"Hello, NetChannel!";
    
    // Test sending
    let sent = m2_net_channel_send(channel, payload.as_ptr(), payload.len());
    assert!(sent);

    // Test receiving
    let mut out_data: *mut u8 = ptr::null_mut();
    let mut out_len: usize = 0;
    
    let received = m2_net_channel_recv(channel, &mut out_data, &mut out_len);
    assert!(received);
    assert!(!out_data.is_null());
    assert_eq!(out_len, payload.len());

    // Verify received data
    let received_slice = unsafe { std::slice::from_raw_parts(out_data, out_len) };
    assert_eq!(received_slice, payload);

    // Free packet
    m2_net_channel_free_packet(out_data, out_len);

    // Empty channel
    let mut out_data_empty: *mut u8 = ptr::null_mut();
    let mut out_len_empty: usize = 0;
    let received_empty = m2_net_channel_recv(channel, &mut out_data_empty, &mut out_len_empty);
    assert!(!received_empty);

    m2_net_channel_destroy(channel);
}

#[test]
fn test_null_pointer_handling() {
    // Send with null channel
    let payload = b"Test";
    let sent = m2_net_channel_send(ptr::null_mut(), payload.as_ptr(), payload.len());
    assert!(!sent);

    let channel = m2_net_channel_create();

    // Send with null data
    let sent_null_data = m2_net_channel_send(channel, ptr::null(), payload.len());
    assert!(!sent_null_data);

    // Send with zero len
    let sent_zero_len = m2_net_channel_send(channel, payload.as_ptr(), 0);
    assert!(!sent_zero_len);

    // Recv with null arguments
    let mut out_data: *mut u8 = ptr::null_mut();
    let mut out_len: usize = 0;

    let recv_null_ch = m2_net_channel_recv(ptr::null_mut(), &mut out_data, &mut out_len);
    assert!(!recv_null_ch);

    let recv_null_data = m2_net_channel_recv(channel, ptr::null_mut(), &mut out_len);
    assert!(!recv_null_data);

    let recv_null_len = m2_net_channel_recv(channel, &mut out_data, ptr::null_mut());
    assert!(!recv_null_len);

    m2_net_channel_destroy(channel);

    // Destroy with null - should not panic
    m2_net_channel_destroy(ptr::null_mut());

    // Free with null - should not panic
    m2_net_channel_free_packet(ptr::null_mut(), 10);
    m2_net_channel_free_packet(payload.as_ptr() as *mut u8, 0); // zero len shouldn't do anything
}
