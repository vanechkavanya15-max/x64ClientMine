use crate::ffi_guard::safe_ffi_call;
use std::slice;

// Struktura pakietu GCDamageInfo odpowiadajaca TPacketGCDamage w C++.
// W zwiazku z `#pragma pack(1)` wszystkie pola sa upakowane jedno po drugim bez paddingu.
#[repr(C, packed)]
#[derive(Debug, PartialEq)]
pub struct TPacketGCDamage {
    pub header: u8,
    pub dw_victim_vid: u32,
    pub b_flag: u8,
    pub l_damage: i32,
}

impl TPacketGCDamage {
    // Rozmiar struktury wynikajacy z: 1 (header) + 4 (vid) + 1 (flag) + 4 (damage) = 10 bajtow.
    pub const PACKET_SIZE: usize = 10;

    /// Deserializuje pakiet z bufora bajtow.
    pub fn from_bytes(buf: &[u8]) -> Result<Self, String> {
        // Zabezpieczenie przed za krotkim buforem. Zwracamy Err, zeby uniknac paniki (np. przy uzyciu indeksowania na out of bounds).
        if buf.len() < Self::PACKET_SIZE {
            return Err(format!(
                "Bufor jest za krotki. Wymagane {}, otrzymano {}",
                Self::PACKET_SIZE,
                buf.len()
            ));
        }

        // Odczyt wartosci uzywajac little-endian (typowy dla x86, architektury Metin2).
        let header = buf[0];
        
        // Zczytanie 4 bajtow dla dw_victim_vid i przekonwertowanie na u32.
        let mut vid_bytes = [0u8; 4];
        vid_bytes.copy_from_slice(&buf[1..5]);
        let dw_victim_vid = u32::from_le_bytes(vid_bytes);

        // Odczyt flagi (pojedynczy bajt).
        let b_flag = buf[5];

        // Zczytanie 4 bajtow dla l_damage i przekonwertowanie na i32.
        let mut damage_bytes = [0u8; 4];
        damage_bytes.copy_from_slice(&buf[6..10]);
        let l_damage = i32::from_le_bytes(damage_bytes);

        Ok(Self {
            header,
            dw_victim_vid,
            b_flag,
            l_damage,
        })
    }
}

/// Funkcja C-FFI do parsowania pakietu HEADER_GC_DAMAGE.
///
/// # Safety
/// Funkcja alokuje pamiec dla struktury `TPacketGCDamage`. Pamiec musi zostac zwolniona za pomoca `m2_net_free_damage`.
#[no_mangle]
pub unsafe extern "C" fn m2_net_parse_damage(
    buf: *const u8,
    len: usize,
    out_packet: *mut *mut TPacketGCDamage,
) -> bool {
    safe_ffi_call(false, || {
        if buf.is_null() || out_packet.is_null() {
            return false;
        }

        let slice = unsafe { slice::from_raw_parts(buf, len) };
        
        match TPacketGCDamage::from_bytes(slice) {
            Ok(packet) => {
                let boxed_packet = Box::new(packet);
                unsafe {
                    *out_packet = Box::into_raw(boxed_packet);
                }
                true
            }
            Err(e) => {
                eprintln!("[m2_client_bridge] Blad parsowania pakietu HEADER_GC_DAMAGE: {}", e);
                false
            }
        }
    })
}

/// Funkcja C-FFI do zwalniania pamieci po strukturze `TPacketGCDamage`.
///
/// # Safety
/// Wskaznik musi pochodzic z funkcji `m2_net_parse_damage` i byc prawidlowy.
#[no_mangle]
pub unsafe extern "C" fn m2_net_free_damage(packet: *mut TPacketGCDamage) {
    safe_ffi_call((), || {
        if !packet.is_null() {
            unsafe {
                drop(Box::from_raw(packet));
            }
        }
    });
}
