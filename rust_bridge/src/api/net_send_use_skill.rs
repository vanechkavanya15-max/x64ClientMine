use crate::ffi_guard::safe_ffi_call;
use crate::net::cg::use_skill::TPacketCGUseSkill;

// Funkcja wysylajaca pakiet uzycia umiejetnosci do serwera
// Zabezpieczona przed panika w C++
#[no_mangle]
pub unsafe extern "C" fn m2_net_send_use_skill(skill_vnum: u32, target_vid: u32) -> bool {
    safe_ffi_call(false, || {
        // Tworzymy strukture reprezentujaca uzycie umiejetnosci
        let _packet = TPacketCGUseSkill::new(skill_vnum, target_vid);
        
        // W prawdziwej implementacji tutaj pakiet zostalby przekazany do wyslania przez NetChannel
        // i serializowany za pomoca packet.to_bytes()
        
        true
    })
}
