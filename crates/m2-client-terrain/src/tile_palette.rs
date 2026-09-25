//! Modul tile_palette dla skrzynki m2-client-terrain
//! Paleta tekstur podloza terenu (AreaAmbience, TileDir)
//! Zestaw tekstur trawy, piasku, skaly, sciezki i przypisanie ID warstw

use std::collections::HashMap;

/// Typ terenu reprezentujacy rodzaj podloza
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerrainType {
    /// Trawa
    Grass,
    /// Piasek
    Sand,
    /// Skala
    Rock,
    /// Sciezka
    Path,
}

/// Struktura reprezentujaca pojedyncza teksture w palecie
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TileTexture {
    /// Identyfikator warstwy (0-255)
    pub layer_id: u8,
    /// Sciezka do pliku tekstury
    pub file_path: String,
    /// Typ terenu przypisany do tej tekstury
    pub terrain_type: TerrainType,
}

/// Glowna struktura implementacyjna zarzadzajaca paleta tekstur terenu
#[derive(Debug, Clone, Default)]
pub struct TilePalette {
    /// Flaga inicjalizacji
    pub initialized: bool,
    /// Katalog bazowy dla tekstur
    pub base_directory: String,
    /// Mapa przechowujaca tekstury zaindeksowane wedlug id warstwy
    pub textures: HashMap<u8, TileTexture>,
}

impl TilePalette {
    /// Tworzy nowa, pusta palete tekstur
    pub fn new() -> Self {
        Self {
            initialized: true,
            base_directory: String::new(),
            textures: HashMap::new(),
        }
    }

    /// Sprawdza czy paleta zostala zainicjalizowana
    pub fn is_ready(&self) -> bool {
        self.initialized
    }

    /// Ustawia katalog bazowy dla wszystkich tekstur w palecie
    pub fn set_base_directory(&mut self, directory: &str) {
        self.base_directory = directory.to_string();
    }

    /// Dodaje nowa teksture do palety. Jesli tekstura o podanym id warstwy
    /// juz istnieje, zostaje nadpisana.
    pub fn add_texture(&mut self, layer_id: u8, file_path: &str, terrain_type: TerrainType) {
        let texture = TileTexture {
            layer_id,
            file_path: file_path.to_string(),
            terrain_type,
        };
        self.textures.insert(layer_id, texture);
    }

    /// Zwraca referencje do tekstury o podanym id warstwy, jesli istnieje.
    pub fn get_texture(&self, layer_id: u8) -> Option<&TileTexture> {
        self.textures.get(&layer_id)
    }

    /// Zwraca wektor referencji do tekstur pasujacych do podanego typu terenu.
    pub fn get_textures_by_type(&self, terrain_type: TerrainType) -> Vec<&TileTexture> {
        self.textures
            .values()
            .filter(|t| t.terrain_type == terrain_type)
            .collect()
    }

    /// Usuwa teksture o podanym id warstwy. Zwraca true jesli usunieto, false jesli nie bylo.
    pub fn remove_texture(&mut self, layer_id: u8) -> bool {
        self.textures.remove(&layer_id).is_some()
    }

    /// Zwraca liczbe aktualnie zaladowanych tekstur.
    pub fn texture_count(&self) -> usize {
        self.textures.len()
    }

    /// Czysci palete ze wszystkich tekstur.
    pub fn clear(&mut self) {
        self.textures.clear();
    }
}
