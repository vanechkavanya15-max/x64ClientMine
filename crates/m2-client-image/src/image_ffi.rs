//! Modul image_ffi dla skrzynki m2-client-image
//! Mostek C-FFI dla dekodowania tekstur
//! Eksporty C-FFI m2_image_decode_dxt, m2_image_decode_tga, m2_image_free

use std::panic::catch_unwind;
use std::ptr;

/// Struktura przechowujaca zdekodowany obraz dla C-FFI
#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct M2Image {
    pub width: u32,
    pub height: u32,
    pub data: *mut u8,
    pub data_len: usize,
}

/// Dekoduje obraz w formacie DXT
/// 
/// # Bezpieczenstwo
/// Funkcja jest bezpieczna dla wywolan z C, o ile wskaznik `input` jest poprawny lub null.
#[no_mangle]
pub unsafe extern "C" fn m2_image_decode_dxt(
    input: *const u8,
    input_len: usize,
    width: u32,
    height: u32,
) -> *mut M2Image {
    let result = catch_unwind(|| {
        if input.is_null() || input_len == 0 || width == 0 || height == 0 {
            return ptr::null_mut();
        }
        
        // Logika biznesowa: symulacja dekodowania
        let data_len = (width as usize) * (height as usize) * 4;
        let data_box = vec![0u8; data_len].into_boxed_slice();
        let data_ptr = Box::into_raw(data_box) as *mut u8;
        
        let img = Box::new(M2Image {
            width,
            height,
            data: data_ptr,
            data_len,
        });
        
        Box::into_raw(img)
    });
    
    result.unwrap_or(ptr::null_mut())
}

/// Dekoduje obraz w formacie TGA
/// 
/// # Bezpieczenstwo
/// Funkcja jest bezpieczna dla wywolan z C, o ile wskaznik `input` jest poprawny lub null.
#[no_mangle]
pub unsafe extern "C" fn m2_image_decode_tga(
    input: *const u8,
    input_len: usize,
) -> *mut M2Image {
    let result = catch_unwind(|| {
        if input.is_null() || input_len == 0 {
            return ptr::null_mut();
        }
        
        // Logika biznesowa: symulacja dekodowania TGA
        let width = 1;
        let height = 1;
        let data_len = 4;
        let data_box = vec![0u8; data_len].into_boxed_slice();
        let data_ptr = Box::into_raw(data_box) as *mut u8;
        
        let img = Box::new(M2Image {
            width,
            height,
            data: data_ptr,
            data_len,
        });
        
        Box::into_raw(img)
    });
    
    result.unwrap_or(ptr::null_mut())
}

/// Zwalnia z pamieci strukture obrazu i jej dane
/// 
/// # Bezpieczenstwo
/// Funkcja jest bezpieczna dla wywolan z C, o ile wskaznik `img` zostal zwrocony przez
/// funkcje dekodujace, lub jest null.
#[no_mangle]
pub unsafe extern "C" fn m2_image_free(img: *mut M2Image) {
    let _ = catch_unwind(|| {
        if !img.is_null() {
            let boxed_img = Box::from_raw(img);
            if !boxed_img.data.is_null() && boxed_img.data_len > 0 {
                let slice_ptr = ptr::slice_from_raw_parts_mut(boxed_img.data, boxed_img.data_len);
                let _ = Box::from_raw(slice_ptr);
            }
        }
    });
}
