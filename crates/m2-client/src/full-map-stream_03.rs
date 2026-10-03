#[path = "terrain_attr_tile_loader.rs"]
pub mod terrain_attr_tile_loader;

pub use terrain_attr_tile_loader::{
    TerrainAttrTileLoader, TerrainAttrTile, FLAG_BLOCKED, FLAG_CANT_BUILD, FLAG_SAFE_ZONE,
    FLAG_WATER, TILE_MAGIC, TILE_SIZE,
};
