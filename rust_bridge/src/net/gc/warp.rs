use crate::ffi_guard::safe_ffi_call;

#[repr(C, packed)]
#[derive(Debug, PartialEq, Eq)]
pub struct TPacketGCWarp {
    pub header: u8,
    pub l_x: i32,
    pub l_y: i32,
    pub l_addr: u32,
    pub w_port: u16,
}

impl TPacketGCWarp {
    /// Oczekiwany rozmiar pakietu wynikajacy ze struktury C++ i #pragma pack(1)
    pub const PACKET_SIZE: usize = 15;

    /// Deserializacja pakietu z bufora bajtow
    pub fn from_bytes(buf: &[u8]) -> Result<Self, String> {
        if buf.len() < Self::PACKET_SIZE {
            return Err(format!("Bufor jest za krotki. Oczekiwano {}, otrzymano {}", Self::PACKET_SIZE, buf.len()));
        }

        let header = buf[0];
        
        let mut l_x_bytes = [0u8; 4];
        l_x_bytes.copy_from_slice(&buf[1..5]);
        let l_x = i32::from_le_bytes(l_x_bytes);

        let mut l_y_bytes = [0u8; 4];
        l_y_bytes.copy_from_slice(&buf[5..9]);
        let l_y = i32::from_le_bytes(l_y_bytes);

        let mut l_addr_bytes = [0u8; 4];
        l_addr_bytes.copy_from_slice(&buf[9..13]);
        let l_addr = u32::from_le_bytes(l_addr_bytes);

        let mut w_port_bytes = [0u8; 2];
        w_port_bytes.copy_from_slice(&buf[13..15]);
        let w_port = u16::from_le_bytes(w_port_bytes);

        Ok(TPacketGCWarp {
            header,
            l_x,
            l_y,
            l_addr,
            w_port,
        })
    }
}

/// Funkcja C-FFI sluzaca do parsowania pakietu warp z surowego bufora bajtow.
///
/// Jesli parsowanie sie powiedzie, alokuje nowa strukture na stercie i zapisuje jej
/// adres w out_packet. Zwraca true w przypadku sukcesu i false w razie bledu (np. zbyt krotki bufor).
/// Wymaga uzycia m2_net_free_warp do poprawnego zwolnienia zaalokowanej pamieci.
#[no_mangle]
pub unsafe extern "C" fn m2_net_parse_warp(
    buf: *const u8,
    len: usize,
    out_packet: *mut *mut TPacketGCWarp,
) -> bool {
    safe_ffi_call(false, || {
        if buf.is_null() || out_packet.is_null() {
            return false;
        }
        
        let slice = std::slice::from_raw_parts(buf, len);
        
        match TPacketGCWarp::from_bytes(slice) {
            Ok(packet) => {
                let boxed_packet = Box::new(packet);
                *out_packet = Box::into_raw(boxed_packet);
                true
            }
            Err(_) => false,
        }
    })
}

/// Zwalnia pamiec zajmowana przez strukture TPacketGCWarp zaalokowana z uzyciem m2_net_parse_warp.
#[no_mangle]
pub unsafe extern "C" fn m2_net_free_warp(packet: *mut TPacketGCWarp) {
    safe_ffi_call((), || {
        if !packet.is_null() {
            let _ = Box::from_raw(packet);
        }
    });
}
