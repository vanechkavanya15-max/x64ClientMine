 //! Wzorzec referencyjny C++: `source/EffectLib/EffectManager.cpp oraz source/EffectLib/EffectInstance.cpp`
 //! Opis: Odwzorowanie CEffectManager: rejestr aktywnych efektow w swiecie 3D (uderzenia miecza, rozblyski skilli, dym, swiatla), czas zycia czasteczek i aktualizacja klatek.
 
use std::collections::BTreeMap;

// A mock implementation of GetCaseCRC32 for case-insensitive hashing
fn get_case_crc32(s: &str) -> u32 {
    let lower = s.to_ascii_lowercase();
    let mut crc = 0xFFFFFFFFu32;
    for &b in lower.as_bytes() {
        crc ^= b as u32;
        for _ in 0..8 {
            if crc & 1 != 0 {
                crc = (crc >> 1) ^ 0xEDB88320;
            } else {
                crc >>= 1;
            }
        }
    }
    !crc
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EffectType {
    None = 0,
    Particle = 1,
    AnimationTexture = 2,
    Mesh = 3,
    SimpleLight = 4,
}

#[derive(Debug, Clone, Default)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Clone, Default)]
pub struct Matrix {
    pub m: [[f32; 4]; 4],
}

impl Matrix {
    pub fn identity() -> Self {
        Self {
            m: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }
}

#[derive(Debug, Clone)]
pub struct EffectData {
    pub id: u32,
    pub file_name: String,
    pub bounding_sphere_radius: f32,
    pub bounding_sphere_position: Vector3,
}

#[derive(Debug, Clone)]
pub struct EffectInstance {
    pub id: u32,
    pub data_id: u32,
    pub is_alive: bool,
    pub is_active: bool,
    pub is_visible: bool,
    pub frame: u32,
    pub global_matrix: Matrix,
    pub position: Vector3,
    pub rotation: Vector3,
    pub bounding_sphere_radius: f32,
    pub bounding_sphere_position: Vector3,
    pub last_time: f32,
    pub particle_instances: Vec<ParticleInstance>,
    pub mesh_instances: Vec<EffectMeshInstance>,
    pub light_instances: Vec<SimpleLightInstance>,
}

impl EffectInstance {
    pub fn new(id: u32, data_id: u32) -> Self {
        Self {
            id,
            data_id,
            is_alive: true,
            is_active: true,
            is_visible: true,
            frame: 0,
            global_matrix: Matrix::identity(),
            position: Vector3::default(),
            rotation: Vector3::default(),
            bounding_sphere_radius: 0.0,
            bounding_sphere_position: Vector3::default(),
            last_time: 0.0,
            particle_instances: Vec::new(),
            mesh_instances: Vec::new(),
            light_instances: Vec::new(),
        }
    }

    pub fn get_type(&self) -> u32 {
        0x1cfa97c6 // EFFECT_OBJECT
    }

    pub fn set_position(&mut self, pos: Vector3) {
        self.position = pos;
        self.global_matrix.m[3][0] = self.position.x;
        self.global_matrix.m[3][1] = self.position.y;
        self.global_matrix.m[3][2] = self.position.z;
    }

    pub fn set_rotation(&mut self, rot: Vector3) {
        self.rotation = rot;
    }

    pub fn set_global_matrix(&mut self, matrix: Matrix) {
        self.global_matrix = matrix;
    }

    pub fn show(&mut self) {
        self.is_visible = true;
    }

    pub fn hide(&mut self) {
        self.is_visible = false;
    }

    pub fn update(&mut self) {
        if self.is_active {
            // Real implementation updates children here, for now it relies on dummy structures
            for p in &mut self.particle_instances {
                p.active = self.is_alive;
            }
            for m in &mut self.mesh_instances {
                m.active = self.is_alive;
            }
            for l in &mut self.light_instances {
                l.active = self.is_alive;
            }
            self.frame += 1;
            // Simple mock lifecycle: effects die after 100 frames for tests
            if self.frame > 100 {
                self.is_alive = false;
            }
        }
    }
    
    pub fn render(&self) {
        // Mock render
    }
    
    pub fn less_render_order(&self, other: &EffectInstance) -> bool {
        self.id < other.id
    }
}

 #[derive(Debug, Clone, Default)]
 pub struct EffectManager {
     pub initialized: bool,
    pub effect_data_map: BTreeMap<u32, EffectData>,
    pub effect_instance_map: BTreeMap<u32, EffectInstance>,
    pub effect_cache_map: BTreeMap<u32, EffectInstance>,
    pub selected_instance: Option<u32>,
    pub disable_sort_rendering: bool,
    next_empty_index: u32,
 }
 
 impl EffectManager {
     pub fn new() -> Self {
        let mut mgr = Self::default();
        mgr.initialized = true;
        mgr.next_empty_index = 1;
        mgr
     }
 
     pub fn is_ready(&self) -> bool {
         self.initialized
     }

    pub fn destroy(&mut self) {
        self.delete_all_instances();
        self.effect_cache_map.clear();
        self.effect_data_map.clear();
        self.selected_instance = None;
        self.disable_sort_rendering = false;
    }

    pub fn update(&mut self) {
        let mut to_remove = Vec::new();
        
        for (id, instance) in self.effect_instance_map.iter_mut() {
            instance.update();
            if !instance.is_alive {
                to_remove.push(*id);
            }
        }

        for id in to_remove {
            self.effect_instance_map.remove(&id);
        }
    }

    pub fn render(&self) {
        if self.disable_sort_rendering {
            for instance in self.effect_instance_map.values() {
                instance.render();
            }
        } else {
            let mut instances: Vec<&EffectInstance> = self.effect_instance_map.values().collect();
            instances.sort_by(|a, b| {
                if a.less_render_order(b) {
                    std::cmp::Ordering::Less
                } else if b.less_render_order(a) {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Equal
                }
            });

            for instance in instances {
                instance.render();
            }
        }
    }

    pub fn register_effect(&mut self, file_name: &str, is_exist_delete: bool, is_need_cache: bool) -> bool {
        let crc = get_case_crc32(file_name);

        if let Some(_) = self.effect_data_map.get(&crc) {
            if is_exist_delete {
                self.effect_data_map.remove(&crc);
            } else {
                return true; // Already exists
            }
        }

        let data = EffectData {
            id: crc,
            file_name: file_name.to_string(),
            bounding_sphere_radius: 0.0,
            bounding_sphere_position: Vector3::default(),
        };

        self.effect_data_map.insert(crc, data);

        if is_need_cache {
            if !self.effect_cache_map.contains_key(&crc) {
                let cache_instance = EffectInstance::new(0, crc);
                self.effect_cache_map.insert(crc, cache_instance);
            }
        }

        true
    }

    pub fn register_effect2(&mut self, file_name: &str, is_need_cache: bool) -> (bool, u32) {
        let crc = get_case_crc32(file_name);
        let result = self.register_effect(file_name, false, is_need_cache);
        (result, crc)
    }

    pub fn get_empty_index(&mut self) -> u32 {
        let mut index = self.next_empty_index;
        if index > 2100000000 {
            index = 1;
        }

        while self.effect_instance_map.contains_key(&index) {
            index += 1;
            if index > 2100000000 {
                index = 1;
            }
        }

        self.next_empty_index = index + 1;
        index
    }

    pub fn create_effect_by_id(&mut self, id: u32, pos: Vector3, rot: Vector3) -> Option<u32> {
        let instance_index = self.get_empty_index();
        if self.create_effect_instance(instance_index, id) {
            self.select_effect_instance(instance_index);
            self.set_effect_instance_position(pos);
            self.set_effect_instance_rotation(rot);
            Some(instance_index)
        } else {
            None
        }
    }

    pub fn create_effect_by_name(&mut self, file_name: &str, pos: Vector3, rot: Vector3) -> Option<u32> {
        let crc = get_case_crc32(file_name);
        self.create_effect_by_id(crc, pos, rot)
    }

    pub fn create_effect_instance(&mut self, instance_index: u32, id: u32) -> bool {
        if id == 0 {
            return false;
        }

        if !self.effect_data_map.contains_key(&id) {
            return false;
        }

        let instance = EffectInstance::new(instance_index, id);
        self.effect_instance_map.insert(instance_index, instance);
        true
    }

    pub fn create_unsafe_effect_instance(&self, data_id: u32) -> Option<EffectInstance> {
        if !self.effect_data_map.contains_key(&data_id) {
            return None;
        }

        Some(EffectInstance::new(0, data_id))
    }

    pub fn destroy_effect_instance(&mut self, instance_index: u32) -> bool {
        self.effect_instance_map.remove(&instance_index).is_some()
    }

    pub fn deactive_effect_instance(&mut self, instance_index: u32) {
        if let Some(instance) = self.effect_instance_map.get_mut(&instance_index) {
            instance.is_active = false;
        }
    }

    pub fn select_effect_instance(&mut self, instance_index: u32) -> bool {
        if self.effect_instance_map.contains_key(&instance_index) {
            self.selected_instance = Some(instance_index);
            true
        } else {
            self.selected_instance = None;
            false
        }
    }

    pub fn set_effect_instance_position(&mut self, pos: Vector3) {
        if let Some(selected) = self.selected_instance {
            if let Some(instance) = self.effect_instance_map.get_mut(&selected) {
                instance.set_position(pos);
            }
        }
    }

    pub fn set_effect_instance_rotation(&mut self, rot: Vector3) {
        if let Some(selected) = self.selected_instance {
            if let Some(instance) = self.effect_instance_map.get_mut(&selected) {
                instance.set_rotation(rot);
            }
        }
    }

    pub fn set_effect_instance_global_matrix(&mut self, matrix: Matrix) {
        if let Some(selected) = self.selected_instance {
            if let Some(instance) = self.effect_instance_map.get_mut(&selected) {
                instance.set_global_matrix(matrix);
            }
        }
    }

    pub fn show_effect(&mut self) {
        if let Some(selected) = self.selected_instance {
            if let Some(instance) = self.effect_instance_map.get_mut(&selected) {
                instance.show();
            }
        }
    }

    pub fn hide_effect(&mut self) {
        if let Some(selected) = self.selected_instance {
            if let Some(instance) = self.effect_instance_map.get_mut(&selected) {
                instance.hide();
            }
        }
    }

    pub fn delete_all_instances(&mut self) {
        self.effect_instance_map.clear();
        self.selected_instance = None;
    }
}

#[derive(Debug, Clone)]
pub struct ParticleInstance {
    pub active: bool,
}

#[derive(Debug, Clone)]
pub struct EffectMeshInstance {
    pub active: bool,
}

#[derive(Debug, Clone)]
pub struct SimpleLightInstance {
    pub active: bool,
}
