use m2_client::wgpu_actor_render_pass::{FfiActorRenderInfo, m2_actor_pass_get_info};

#[test]
fn test_ffi_actor_pass_get_info_defaults() {
    let info = m2_actor_pass_get_info();
    assert_eq!(info.total_drawn, 0);
    assert_eq!(info.cull_count, 0);
}

#[test]
fn test_ffi_actor_render_info_layout() {
    use std::mem;
    assert_eq!(mem::size_of::<FfiActorRenderInfo>(), 8, "C-ABI struct size mismatch");
    assert_eq!(mem::align_of::<FfiActorRenderInfo>(), 4, "C-ABI struct alignment mismatch");
}

#[test]
fn test_ffi_actor_render_info_equality() {
    let info1 = FfiActorRenderInfo { total_drawn: 5, cull_count: 2 };
    let info2 = FfiActorRenderInfo { total_drawn: 5, cull_count: 2 };
    let info3 = FfiActorRenderInfo { total_drawn: 10, cull_count: 0 };
    assert_eq!(info1, info2);
    assert_ne!(info1, info3);
}

#[test]
fn test_ffi_actor_render_info_clone() {
    let info1 = FfiActorRenderInfo { total_drawn: 5, cull_count: 2 };
    let info2 = info1.clone();
    assert_eq!(info1, info2);
}

#[test]
fn test_ffi_actor_render_info_copy() {
    let info1 = FfiActorRenderInfo { total_drawn: 5, cull_count: 2 };
    let info2 = info1; // should copy
    assert_eq!(info1.total_drawn, info2.total_drawn);
}

#[test]
fn test_ffi_actor_render_info_debug() {
    let info1 = FfiActorRenderInfo { total_drawn: 5, cull_count: 2 };
    let debug_str = format!("{:?}", info1);
    assert!(debug_str.contains("FfiActorRenderInfo"));
    assert!(debug_str.contains("total_drawn: 5"));
    assert!(debug_str.contains("cull_count: 2"));
}
