// Zgodnie z wymaganiami technicznymi: 
// 1. Zaimplementuj modul w 'crates/m2-client/src/full-3d-pipe_13.rs' lub we wskazanym pliku.
// Modul wgpu_actor_outline_pass.rs zawiera faktyczna implementacje shaderow i logiki wgpu
// zgodnie z architekturą, wiec ten plik posluzy jako pomost/re-export jesli wymagalby tego crate.

#[path = "wgpu_actor_outline_pass.rs"]
pub mod wgpu_actor_outline_pass;

pub use wgpu_actor_outline_pass::*;
