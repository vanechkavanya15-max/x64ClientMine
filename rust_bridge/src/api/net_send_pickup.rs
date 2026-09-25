use crate::ffi_guard::safe_ffi_call;
use crate::net::cg::item_pickup::TPacketCGItemPickup;

/// FFI: Wysyla pakiet zadania podniesienia przedmiotu o podanym VID.
#[no_mangle]
pub unsafe extern "C" fn m2_net_send_pickup(vid: u32) -> bool {
    safe_ffi_call(false, || {
        // Konstrukcja pakietu
        let packet = TPacketCGItemPickup::new(vid);
        
        // Konwersja do bajtow, aby udowodnic serializacje (brak alokacji na stercie)
        let _bytes = packet.to_bytes();
        
        // Mock logiki sieciowej Rusta dla Fazy A.
        true
    })
}
