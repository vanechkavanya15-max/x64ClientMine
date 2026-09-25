use m2_client_render::render_ffi::{
    RenderFfi, m2_render_begin_frame, m2_render_destroy, m2_render_end_frame, m2_render_init,
};

#[test]
fn test_integration_render_ffi() {
    let instance = RenderFfi::new();
    assert!(instance.is_ready());
}

#[test]
fn test_ffi_lifecycle() {
    // 1. Inicjalizacja
    let ctx = m2_render_init();
    assert!(!ctx.is_null(), "Wskaznik kontekstu nie powinien byc null");

    // 2. Rozpoczecie klatki (powinno sie udac)
    let begin_success = m2_render_begin_frame(ctx);
    assert!(begin_success, "Rozpoczecie klatki powinno zwrocic true");

    // Proba ponownego rozpoczecia bez zakonczenia (powinna zwrocic false)
    let double_begin = m2_render_begin_frame(ctx);
    assert!(!double_begin, "Podwojne rozpoczecie klatki powinno zwrocic false");

    // 3. Zakonczenie klatki (powinno sie udac)
    let end_success = m2_render_end_frame(ctx);
    assert!(end_success, "Zakonczenie klatki powinno zwrocic true");

    // Proba ponownego zakonczenia bez rozpoczecia (powinna zwrocic false)
    let double_end = m2_render_end_frame(ctx);
    assert!(!double_end, "Podwojne zakonczenie klatki powinno zwrocic false");

    // 4. Zniszczenie obiektu
    m2_render_destroy(ctx);
}

#[test]
fn test_ffi_null_pointers() {
    // Testowanie wywolan FFI ze wskaznikiem null (bezpieczenstwo pamieci)
    let null_ptr: *mut RenderFfi = std::ptr::null_mut();

    let begin_success = m2_render_begin_frame(null_ptr);
    assert!(!begin_success, "Przekazanie null do begin_frame powinno zwrocic false");

    let end_success = m2_render_end_frame(null_ptr);
    assert!(!end_success, "Przekazanie null do end_frame powinno zwrocic false");

    // Wywolanie z null nie powinno spowodowac paniki programu
    m2_render_destroy(null_ptr);
}
