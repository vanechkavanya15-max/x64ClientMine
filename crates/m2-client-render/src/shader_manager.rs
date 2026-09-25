//! Modul shader_manager dla skrzynki m2-client-render
//! Katalog i zarzadca shaderow WGSL / HLSL
//! Kompilacja i rejestr programow cieniujacych dla terenu, modeli 3D i interfejsu

use std::collections::HashMap;

/// Typ uzywanego shadera.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShaderType {
    Wgsl,
    Hlsl,
}

/// Kategoria shadera.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShaderCategory {
    Terrain,
    Model3D,
    Interface,
}

/// Blad podczas operacji na shaderach.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShaderError {
    AlreadyExists,
    NotFound,
    CompilationFailed,
    EmptySource,
}

/// Reprezentacja programu cieniujacego w systemie.
#[derive(Debug, Clone)]
pub struct ShaderProgram {
    pub name: String,
    pub source: String,
    pub shader_type: ShaderType,
    pub category: ShaderCategory,
    pub is_compiled: bool,
}

impl ShaderProgram {
    pub fn new(name: String, source: String, shader_type: ShaderType, category: ShaderCategory) -> Self {
        Self {
            name,
            source,
            shader_type,
            category,
            is_compiled: false,
        }
    }
}

/// Glowna struktura implementacyjna zarzadzajaca shaderami.
#[derive(Debug, Clone, Default)]
pub struct ShaderManager {
    pub initialized: bool,
    shaders: HashMap<String, ShaderProgram>,
}

impl ShaderManager {
    /// Tworzy nowa instancje ShaderManager.
    pub fn new() -> Self {
        Self {
            initialized: true,
            shaders: HashMap::new(),
        }
    }

    /// Sprawdza czy zarzadca jest gotowy do uzycia.
    pub fn is_ready(&self) -> bool {
        self.initialized
    }

    /// Rejestruje nowy program cieniujacy w systemie.
    /// Zwraca blad ShaderError::AlreadyExists jesli shader o takiej nazwie juz istnieje.
    /// Zwraca blad ShaderError::EmptySource jesli kod zrodlowy jest pusty.
    pub fn register_shader(
        &mut self,
        name: &str,
        source: &str,
        shader_type: ShaderType,
        category: ShaderCategory,
    ) -> Result<(), ShaderError> {
        if self.shaders.contains_key(name) {
            return Err(ShaderError::AlreadyExists);
        }
        
        let trimmed_source = source.trim();
        if trimmed_source.is_empty() {
            return Err(ShaderError::EmptySource);
        }

        let program = ShaderProgram::new(name.to_string(), trimmed_source.to_string(), shader_type, category);
        self.shaders.insert(name.to_string(), program);
        Ok(())
    }

    /// Kompiluje zarejestrowany shader na podstawie podanej nazwy.
    /// Ta metoda jedynie symuluje kompilacje w podsystemie renderowania, sprawdzajac obecnosc kodu i wewnetrzna spojnosc.
    pub fn compile_shader(&mut self, name: &str) -> Result<(), ShaderError> {
        let shader = self.shaders.get_mut(name).ok_or(ShaderError::NotFound)?;
        
        // Symulacja kompilacji: shader musi posiadac zrodlo (co juz zabezpieczamy przy rejestracji)
        // Zakladamy, ze puste lub specjalne testowe stringi moga wywolac blad kompilacji
        if shader.source == "invalid_syntax" {
            return Err(ShaderError::CompilationFailed);
        }
        
        shader.is_compiled = true;
        Ok(())
    }

    /// Pobiera shader wg nazwy.
    pub fn get_shader(&self, name: &str) -> Option<&ShaderProgram> {
        self.shaders.get(name)
    }

    /// Pobiera liste wszystkich shaderow danej kategorii.
    pub fn get_shaders_by_category(&self, category: ShaderCategory) -> Vec<&ShaderProgram> {
        self.shaders.values().filter(|s| s.category == category).collect()
    }

    /// Usuwa shader wg nazwy.
    pub fn remove_shader(&mut self, name: &str) -> Result<(), ShaderError> {
        if self.shaders.remove(name).is_some() {
            Ok(())
        } else {
            Err(ShaderError::NotFound)
        }
    }

    /// Cysci wszystkie zarejestrowane shadery.
    pub fn clear(&mut self) {
        self.shaders.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skeleton_shader_manager() {
        let obj = ShaderManager::new();
        assert!(obj.is_ready());
    }

    #[test]
    fn test_register_and_get() {
        let mut manager = ShaderManager::new();
        assert!(manager.register_shader("test_shader", "fn main() {}", ShaderType::Wgsl, ShaderCategory::Model3D).is_ok());
        
        let shader = manager.get_shader("test_shader");
        assert!(shader.is_some());
        let shader = shader.unwrap();
        assert_eq!(shader.name, "test_shader");
        assert_eq!(shader.source, "fn main() {}");
        assert_eq!(shader.shader_type, ShaderType::Wgsl);
        assert_eq!(shader.category, ShaderCategory::Model3D);
        assert!(!shader.is_compiled);
    }
}
