/// Modul 12: 3D Scene Pipeline & Camera Pass Integration dla podgladu aktora
/// Zgodnie z wymaganiami w tym pliku moze byc zaimplementowany glowny punkt eksportujacy.
/// Logika uzywa `wgpu_rendertarget_actor`.

#[path = "wgpu_rendertarget_actor.rs"]
pub mod wgpu_rendertarget_actor;

pub use wgpu_rendertarget_actor::ActorPreviewRenderer;
