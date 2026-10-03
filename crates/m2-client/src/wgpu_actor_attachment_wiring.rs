use glam::Mat4;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActorPartIndex {
    Main = 0,
    Weapon = 1,
    Head = 2,
    WeaponLeft = 3,
    Hair = 4,
    MaxNum = 5,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EffectLifeType {
    Normal = 0,
    Infinite = 1,
    WithMotion = 2,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttachingDataType {
    None = 0,
    CollisionData = 1,
    Effect = 2,
    Object = 3,
    MaxNum = 4,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollisionType {
    None = 0,
    Body = 1,
    Attacking = 2,
    Defending = 3,
    Splash = 4,
}

#[derive(Debug, Clone)]
pub struct AttachingEffect {
    pub effect_index: u32,
    pub bone_index: i32,
    pub model_index: u32,
    pub translation_matrix: Mat4,
    pub is_attaching: bool,
    pub life_type: EffectLifeType,
    pub end_time: u32,
}

impl AttachingEffect {
    pub fn new(
        effect_index: u32,
        bone_index: i32,
        model_index: u32,
        translation_matrix: Mat4,
        is_attaching: bool,
        life_type: EffectLifeType,
        end_time: u32,
    ) -> Self {
        Self {
            effect_index,
            bone_index,
            model_index,
            translation_matrix,
            is_attaching,
            life_type,
            end_time,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CollisionPointInstance {
    pub collision_type: CollisionType,
    // Add point instance specific data when needed
}

pub struct WeaponTraceInstance {
    pub texture_name: Option<String>,
    pub use_alpha: bool,
}

impl WeaponTraceInstance {
    pub fn new() -> Self {
        Self {
            texture_name: None,
            use_alpha: false,
        }
    }

    pub fn set_texture(&mut self, texture_name: &str) {
        self.texture_name = Some(texture_name.to_string());
        self.use_alpha = false;
    }

    pub fn use_texture(&mut self) {
        self.use_alpha = false;
    }

    pub fn use_alpha(&mut self) {
        self.use_alpha = true;
    }
}

pub struct ActorAttachmentWiring {
    pub attaching_effects: Vec<AttachingEffect>,
    pub weapon_traces: Vec<WeaponTraceInstance>,
    pub defending_point_instances: Vec<CollisionPointInstance>,
    pub body_point_instances: Vec<CollisionPointInstance>,
    // Removed attacking point instances logic since it's commented out in C++ (m_AttackingPointInstanceList)
    pub is_effect_initialized: bool,
}

impl ActorAttachmentWiring {
    pub fn new() -> Self {
        Self {
            attaching_effects: Vec::new(),
            weapon_traces: Vec::new(),
            defending_point_instances: Vec::new(),
            body_point_instances: Vec::new(),
            is_effect_initialized: false,
        }
    }

    pub fn calculate_effect_render_matrix(
        &self,
        effect: &AttachingEffect,
        world_matrix: Mat4,
        bone_matrix: Option<Mat4>,
    ) -> Mat4 {
        if !effect.is_attaching {
            return Mat4::IDENTITY;
        }

        if effect.bone_index == -1 {
            // matTransform = it->matTranslation;
            // matTransform *= m_worldMatrix;
            // Note: In D3D math (row-major), M = Translation * World means M = Translation * World.
            // In glam (column-major), this would typically be World * Translation if translating vectors.
            // However, C++ code shows: matTransform = it->matTranslation * m_worldMatrix.
            world_matrix * effect.translation_matrix
        } else {
            if let Some(bone_mat) = bone_matrix {
                // matTransform = *pBoneMat;
                // matTransform *= it->matTranslation;
                // matTransform *= m_worldMatrix;
                world_matrix * effect.translation_matrix * bone_mat
            } else {
                Mat4::IDENTITY // Error handling fall-back
            }
        }
    }

    pub fn calculate_part_render_matrix(&self, parent_matrix: Mat4, socket_matrix: Mat4) -> Mat4 {
        // Equivalent to AttachModelInstance
        parent_matrix * socket_matrix
    }

    pub fn get_attaching_bone_name(part_index: ActorPartIndex) -> Option<&'static str> {
        match part_index {
            ActorPartIndex::Weapon => Some("equip_right"),
            ActorPartIndex::WeaponLeft => Some("equip_left"),
            ActorPartIndex::Head => Some("Bip01 Head"),
            // Spine2 and others can be added as needed
            _ => None,
        }
    }

    pub fn add_collision_point(&mut self, instance: CollisionPointInstance) {
        match instance.collision_type {
            CollisionType::Attacking => {
                // In C++, this is explicitly skipped/commented out: //m_AttackingPointInstanceList.push_back(PointInstance);
            }
            CollisionType::Defending => {
                self.defending_point_instances.push(instance);
            }
            CollisionType::Body => {
                self.body_point_instances.push(instance);
            }
            _ => {}
        }
    }

    pub fn hide_all_attaching_effects_for_eunhyeong(&self, get_effect_file_name: impl Fn(u32) -> Option<String>) -> Vec<u32> {
        let mut effects_to_hide = Vec::new();

        for effect in &self.attaching_effects {
            let mut is_eunh = false;

            if let Some(file_name) = get_effect_file_name(effect.effect_index) {
                let mut fname_lower = file_name;
                fname_lower = fname_lower.replace("\\", "/").to_lowercase();

                if fname_lower.contains("eunhyeongbeop") {
                    is_eunh = true;
                }
            }

            if !is_eunh {
                effects_to_hide.push(effect.effect_index);
            }
        }

        effects_to_hide
    }

    pub fn clear_attaching_effects(&mut self) {
        self.is_effect_initialized = false;
        self.attaching_effects.clear();
    }
}
