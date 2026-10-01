#[cfg(test)]
mod tests {
    use std::mem::size_of;
    use m2_client::send_item_storage_packets::*;

    #[test]
    fn test_packet_sizes_and_offsets() {
        // TPacketCGItemUse
        assert_eq!(size_of::<TItemPos>(), 3, "TItemPos size must be exactly 3 bytes");
        assert_eq!(
            size_of::<TPacketCGItemUse>(),
            7,
            "TPacketCGItemUse size must be exactly 7 bytes"
        );

        // TPacketCGItemMove
        assert_eq!(
            size_of::<TPacketCGItemMove>(),
            11,
            "TPacketCGItemMove size must be exactly 11 bytes"
        );

        // TPacketCGItemDrop
        assert_eq!(
            size_of::<TPacketCGItemDrop>(),
            12,
            "TPacketCGItemDrop size must be exactly 12 bytes"
        );

        // TPacketCGItemPickUp
        assert_eq!(
            size_of::<TPacketCGItemPickUp>(),
            8,
            "TPacketCGItemPickUp size must be exactly 8 bytes"
        );

        // TPacketCGSafeboxCheckout
        assert_eq!(
            size_of::<TPacketCGSafeboxCheckout>(),
            8,
            "TPacketCGSafeboxCheckout size must be exactly 8 bytes"
        );

        // TPacketCGSafeboxCheckin
        assert_eq!(
            size_of::<TPacketCGSafeboxCheckin>(),
            8,
            "TPacketCGSafeboxCheckin size must be exactly 8 bytes"
        );
    }

    #[test]
    fn test_m2_net_send_use_item_v2() {
        let mut buf = [0u8; 32];
        let size = unsafe { m2_net_send_use_item_v2(1, 42, buf.as_mut_ptr()) };

        assert_eq!(size, 7);

        // Header (2 bytes, 0x0501)
        assert_eq!(buf[0], 0x01);
        assert_eq!(buf[1], 0x05);

        // Length (2 bytes, 7)
        assert_eq!(buf[2], 7);
        assert_eq!(buf[3], 0);

        // TItemPos (3 bytes: window_type=1, cell=42)
        assert_eq!(buf[4], 1); // window_type
        assert_eq!(buf[5], 42); // cell lower byte
        assert_eq!(buf[6], 0); // cell upper byte
    }

    #[test]
    fn test_m2_net_send_item_move_v2() {
        let mut buf = [0u8; 32];
        let size = unsafe { m2_net_send_item_move_v2(1, 10, 2, 20, 5, buf.as_mut_ptr()) };

        assert_eq!(size, 11);

        // Header (2 bytes, 0x0503)
        assert_eq!(buf[0], 0x03);
        assert_eq!(buf[1], 0x05);

        // Length (2 bytes, 11)
        assert_eq!(buf[2], 11);
        assert_eq!(buf[3], 0);

        // pos: TItemPos (3 bytes: window_type=1, cell=10)
        assert_eq!(buf[4], 1);
        assert_eq!(buf[5], 10);
        assert_eq!(buf[6], 0);

        // change_pos: TItemPos (3 bytes: window_type=2, cell=20)
        assert_eq!(buf[7], 2);
        assert_eq!(buf[8], 20);
        assert_eq!(buf[9], 0);

        // num (1 byte: 5)
        assert_eq!(buf[10], 5);
    }

    #[test]
    fn test_m2_net_send_item_drop_v2() {
        let mut buf = [0u8; 32];
        let size = unsafe { m2_net_send_item_drop_v2(1, 15, 1000, 2, buf.as_mut_ptr()) };

        assert_eq!(size, 12);

        // Header (2 bytes, 0x0504)
        assert_eq!(buf[0], 0x04);
        assert_eq!(buf[1], 0x05);

        // Length (2 bytes, 12)
        assert_eq!(buf[2], 12);
        assert_eq!(buf[3], 0);

        // pos: TItemPos (3 bytes: window_type=1, cell=15)
        assert_eq!(buf[4], 1);
        assert_eq!(buf[5], 15);
        assert_eq!(buf[6], 0);

        // gold (4 bytes: 1000 = 0x03E8)
        assert_eq!(buf[7], 0xE8);
        assert_eq!(buf[8], 0x03);
        assert_eq!(buf[9], 0x00);
        assert_eq!(buf[10], 0x00);

        // count (1 byte: 2)
        assert_eq!(buf[11], 2);
    }

    #[test]
    fn test_m2_net_send_pickup_v2() {
        let mut buf = [0u8; 32];
        let size = unsafe { m2_net_send_pickup_v2(123456, buf.as_mut_ptr()) };

        assert_eq!(size, 8);

        // Header (2 bytes, 0x0505)
        assert_eq!(buf[0], 0x05);
        assert_eq!(buf[1], 0x05);

        // Length (2 bytes, 8)
        assert_eq!(buf[2], 8);
        assert_eq!(buf[3], 0);

        // vid (4 bytes: 123456 = 0x01E240)
        assert_eq!(buf[4], 0x40);
        assert_eq!(buf[5], 0xE2);
        assert_eq!(buf[6], 0x01);
        assert_eq!(buf[7], 0x00);
    }

    #[test]
    fn test_m2_net_send_safebox_checkout() {
        let mut buf = [0u8; 32];
        let size = unsafe { m2_net_send_safebox_checkout(5, 1, 25, buf.as_mut_ptr()) };

        assert_eq!(size, 8);

        // Header (2 bytes, 0x0821)
        assert_eq!(buf[0], 0x21);
        assert_eq!(buf[1], 0x08);

        // Length (2 bytes, 8)
        assert_eq!(buf[2], 8);
        assert_eq!(buf[3], 0);

        // bSafePos (1 byte: 5)
        assert_eq!(buf[4], 5);

        // ItemPos: TItemPos (3 bytes: window_type=1, cell=25)
        assert_eq!(buf[5], 1);
        assert_eq!(buf[6], 25);
        assert_eq!(buf[7], 0);
    }

    #[test]
    fn test_m2_net_send_safebox_checkin() {
        let mut buf = [0u8; 32];
        let size = unsafe { m2_net_send_safebox_checkin(6, 2, 30, buf.as_mut_ptr()) };

        assert_eq!(size, 8);

        // Header (2 bytes, 0x0820)
        assert_eq!(buf[0], 0x20);
        assert_eq!(buf[1], 0x08);

        // Length (2 bytes, 8)
        assert_eq!(buf[2], 8);
        assert_eq!(buf[3], 0);

        // bSafePos (1 byte: 6)
        assert_eq!(buf[4], 6);

        // ItemPos: TItemPos (3 bytes: window_type=2, cell=30)
        assert_eq!(buf[5], 2);
        assert_eq!(buf[6], 30);
        assert_eq!(buf[7], 0);
    }

    #[test]
    fn test_null_buffer() {
        assert_eq!(unsafe { m2_net_send_use_item_v2(1, 1, std::ptr::null_mut()) }, 0);
        assert_eq!(unsafe { m2_net_send_item_move_v2(1, 1, 1, 1, 1, std::ptr::null_mut()) }, 0);
        assert_eq!(unsafe { m2_net_send_item_drop_v2(1, 1, 1, 1, std::ptr::null_mut()) }, 0);
        assert_eq!(unsafe { m2_net_send_pickup_v2(1, std::ptr::null_mut()) }, 0);
        assert_eq!(unsafe { m2_net_send_safebox_checkout(1, 1, 1, std::ptr::null_mut()) }, 0);
        assert_eq!(unsafe { m2_net_send_safebox_checkin(1, 1, 1, std::ptr::null_mut()) }, 0);
    }
}