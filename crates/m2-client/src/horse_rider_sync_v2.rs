use std::ptr;

/// Eksport FFI: m2_mount_get_rider_matrix
/// Synchronizacja jezdzca z koniem. 
/// Pobiera macierz kosci `saddle` i aplikuje ja jako macierz korzenia `Bip01` jezdzca.
#[no_mangle]
pub unsafe extern "C" fn m2_mount_get_rider_matrix(horse_mat: *const f32, out_rider_mat: *mut f32) -> bool {
    if horse_mat.is_null() || out_rider_mat.is_null() {
        return false;
    }

    // Since in the C++ engine the rider is attached directly to the "saddle" bone,
    // the matrix representing the saddle bone's world transform is directly
    // copied to the rider's root (Bip01) without extra static offsets.
    ptr::copy_nonoverlapping(horse_mat, out_rider_mat, 16);

    true
}
