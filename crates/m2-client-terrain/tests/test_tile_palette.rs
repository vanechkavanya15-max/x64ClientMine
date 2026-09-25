use m2_client_terrain::tile_palette::{TerrainType, TilePalette};

#[test]
fn test_integration_tile_palette() {
    let mut palette = TilePalette::new();
    assert!(palette.is_ready());
    assert_eq!(palette.texture_count(), 0);

    // Testowanie ustawiania sciezki bazowej
    palette.set_base_directory("textures/terrain");
    assert_eq!(palette.base_directory, "textures/terrain");

    // Testowanie dodawania tekstur
    palette.add_texture(1, "grass.dds", TerrainType::Grass);
    palette.add_texture(2, "sand.dds", TerrainType::Sand);
    palette.add_texture(3, "rock.dds", TerrainType::Rock);
    palette.add_texture(4, "path.dds", TerrainType::Path);

    assert_eq!(palette.texture_count(), 4);

    // Testowanie pobierania tekstur
    let tex1 = palette.get_texture(1).expect("Powinna byc tekstura o id 1");
    assert_eq!(tex1.file_path, "grass.dds");
    assert_eq!(tex1.terrain_type, TerrainType::Grass);

    assert!(palette.get_texture(99).is_none());

    // Testowanie pobierania po typie
    let grass_textures = palette.get_textures_by_type(TerrainType::Grass);
    assert_eq!(grass_textures.len(), 1);
    assert_eq!(grass_textures[0].layer_id, 1);

    // Testowanie nadpisywania tekstury
    palette.add_texture(1, "grass_high_res.dds", TerrainType::Grass);
    assert_eq!(palette.texture_count(), 4);
    let tex1_updated = palette.get_texture(1).unwrap();
    assert_eq!(tex1_updated.file_path, "grass_high_res.dds");

    // Testowanie usuwania tekstury
    assert!(palette.remove_texture(2));
    assert_eq!(palette.texture_count(), 3);
    assert!(palette.get_texture(2).is_none());
    assert!(!palette.remove_texture(99)); // Nie powinno byc bledu

    // Testowanie czyszczenia palety
    palette.clear();
    assert_eq!(palette.texture_count(), 0);
}
