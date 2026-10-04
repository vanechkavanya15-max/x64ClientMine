// msm_equipment_registry.rs - Dynamiczne zarzadzanie modelami zbroi, broni i wlosow na podstawie plikow .msm
use std::collections::HashMap;
use std::ffi::{c_char, CStr};
use std::sync::{Arc, Mutex, OnceLock};

use crate::monster_script_msm_parser::{MonsterScript, ScriptParser, ShapeData, HairData};

/// Centralny rejestr konfiguracji ras i czesci ciala (.msm)
#[derive(Default)]
pub struct MsmEquipmentRegistry {
    /// Mapowanie rasy (0=Wojownik M, 1=Ninja K, 2=Sura M, 3=Szaman K, itd.) na sparsowany skrypt .msm
    race_scripts: HashMap<u32, MonsterScript>,
}

static MSM_REGISTRY: OnceLock<Mutex<MsmEquipmentRegistry>> = OnceLock::new();

pub fn get_msm_registry() -> &'static Mutex<MsmEquipmentRegistry> {
    MSM_REGISTRY.get_or_init(|| Mutex::new(MsmEquipmentRegistry::default()))
}

impl MsmEquipmentRegistry {
    /// Rejestruje lub parsuje zawartosc pliku .msm dla danej rasy
    pub fn register_msm_content(&mut self, race: u32, content: &str) -> Result<(), String> {
        let script = ScriptParser::parse(content)?;
        self.race_scripts.insert(race, script);
        Ok(())
    }

    /// Laduje plik .msm z VFS lub z dysku lokalnego
    pub fn load_race_msm_from_vfs(&mut self, race: u32, msm_path: &str) -> Result<(), String> {
        let norm_path = msm_path.to_lowercase().replace('\\', "/");
        
        let content_opt = if let Some(bytes) = crate::texture_vfs_pipeline::read_vfs_bytes(&norm_path) {
            String::from_utf8(bytes).ok()
        } else if let Ok(content) = std::fs::read_to_string(&norm_path) {
            Some(content)
        } else {
            // Proba w folderach pc
            let fallback_paths = [
                format!("d:/ymir work/{}", norm_path),
                format!("pc/warrior/{}", norm_path),
                format!("pc/assassin/{}", norm_path),
                format!("pc/sura/{}", norm_path),
                format!("pc/shaman/{}", norm_path),
            ];
            let mut found = None;
            for p in &fallback_paths {
                if let Some(bytes) = crate::texture_vfs_pipeline::read_vfs_bytes(p) {
                    if let Ok(s) = String::from_utf8(bytes) {
                        found = Some(s);
                        break;
                    }
                }
            }
            found
        };

        if let Some(content) = content_opt {
            self.register_msm_content(race, &content)
        } else {
            Err(format!("Nie mozna odnalezc pliku MSM: {}", msm_path))
        }
    }

    /// Wyszukuje dane ksztaltu zbroi (ShapeData) dla zadanego shape_index
    pub fn get_armor_shape(&self, race: u32, shape_index: u32) -> Option<(String, String)> {
        let script = self.race_scripts.get(&race)?;
        for shape in &script.shape_data {
            if shape.shape_index == shape_index {
                let model = shape.model_file_name.clone();
                let skin = shape.skins.first().map(|s| s.dst_file_name.clone()).unwrap_or_default();
                return Some((model, skin));
            }
        }
        None
    }

    /// Wyszukuje dane fryzury (HairData) dla zadanego hair_index
    pub fn get_hair_data(&self, race: u32, hair_index: u32) -> Option<(String, String)> {
        let script = self.race_scripts.get(&race)?;
        for hair in &script.hair_data {
            if hair.hair_index == hair_index {
                let model = hair.model_file_name.clone();
                let skin = hair.skins.first().map(|s| s.dst_file_name.clone()).unwrap_or_default();
                return Some((model, skin));
            }
        }
        None
    }

    /// Zwraca sciezke do bazowego modelu postaci dla rasy
    pub fn get_base_model(&self, race: u32) -> Option<String> {
        let script = self.race_scripts.get(&race)?;
        if !script.base_model_file_name.is_empty() {
            Some(script.base_model_file_name.clone())
        } else {
            None
        }
    }
}

// ============================================================================
// FFI C-Bridge do wywolan z Pythona / C++
// ============================================================================

#[no_mangle]
pub extern "C" fn m2_msm_load_race_file(race: u32, path: *const c_char) -> bool {
    if path.is_null() { return false; }
    let c_str = unsafe { CStr::from_ptr(path) };
    let path_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return false,
    };

    if let Ok(mut reg) = get_msm_registry().lock() {
        reg.load_race_msm_from_vfs(race, path_str).is_ok()
    } else {
        false
    }
}

#[no_mangle]
pub extern "C" fn m2_msm_get_armor_skin_path(
    race: u32,
    shape_index: u32,
    out_buf: *mut c_char,
    max_len: usize,
) -> bool {
    if out_buf.is_null() || max_len == 0 { return false; }
    if let Ok(reg) = get_msm_registry().lock() {
        if let Some((_, skin)) = reg.get_armor_shape(race, shape_index) {
            let bytes = skin.as_bytes();
            let copy_len = bytes.len().min(max_len - 1);
            unsafe {
                std::ptr::copy_nonoverlapping(bytes.as_ptr(), out_buf as *mut u8, copy_len);
                *out_buf.add(copy_len) = 0;
            }
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_WARRIOR_MSM: &str = r#"
ScriptType            RaceDataScript

BaseModelFileName     "d:/ymir work/pc/warrior/warrior_novice.GR2"

Group ShapeData
{
    PathName	"d:/ymir work/pc/warrior/"
    ShapeDataCount			2
    Group ShapeData00
    {
        ShapeIndex			0
        Model				"warrior_novice.GR2"
        SourceSkin			"warrior_novice_red.dds"
        TargetSkin			"warrior_novice_red.dds"
    }
    Group ShapeData01
    {
        ShapeIndex			1
        Model				"warrior_novice.GR2"
        SourceSkin			"warrior_novice_red.dds"
        TargetSkin			"warrior_novice_blue.dds"
    }
}

Group HairData
{
    PathName	"d:/ymir work/pc/warrior/hair/"
    HairDataCount			1
    Group HairData00
    {
        HairIndex			1001
        Model				"hair_01.GR2"
        SourceSkin			"hair_01.dds"
        TargetSkin			"hair_01_black.dds"
    }
}
"#;

    #[test]
    fn test_msm_equipment_registry_parse_and_lookup() {
        let mut reg = MsmEquipmentRegistry::default();
        let res = reg.register_msm_content(0, SAMPLE_WARRIOR_MSM);
        assert!(res.is_ok(), "Parsowanie MSM powinno zakonczyc sie sukcesem");

        // Weryfikacja bazowego modelu
        assert_eq!(
            reg.get_base_model(0),
            Some("d:/ymir work/pc/warrior/warrior_novice.GR2".to_string())
        );

        // Weryfikacja ShapeIndex 0 (domyslny novice red)
        let shape0 = reg.get_armor_shape(0, 0);
        assert!(shape0.is_some());
        let (m0, s0) = shape0.unwrap();
        assert!(m0.contains("warrior_novice.GR2"));
        assert!(s0.contains("warrior_novice_red.dds"));

        // Weryfikacja ShapeIndex 1 (novice blue)
        let shape1 = reg.get_armor_shape(0, 1);
        assert!(shape1.is_some());
        let (m1, s1) = shape1.unwrap();
        assert!(m1.contains("warrior_novice.GR2"));
        assert!(s1.contains("warrior_novice_blue.dds"));

        // Weryfikacja HairData
        let hair = reg.get_hair_data(0, 1001);
        assert!(hair.is_some());
        let (hm, hs) = hair.unwrap();
        assert!(hm.contains("hair_01.GR2"));
        assert!(hs.contains("hair_01_black.dds"));
    }
}
