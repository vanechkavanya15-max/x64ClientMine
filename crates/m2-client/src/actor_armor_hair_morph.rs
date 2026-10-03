#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GrannyPwnt3432Vertex {
    pub position: [f32; 3],
    pub bone_weights: [u8; 4],
    pub bone_indices: [u8; 4],
    pub normal: [f32; 3],
    pub uv: [f32; 2],
}

#[derive(Debug, Clone, PartialEq)]
pub struct Mesh {
    pub vertices: Vec<GrannyPwnt3432Vertex>,
    pub indices: Vec<u16>,
}

#[derive(Debug, Clone, Default)]
pub struct ActorMorphState {
    pub armor_mesh: Option<Mesh>,
    pub hair_mesh: Option<Mesh>,
}

impl ActorMorphState {
    pub fn new() -> Self {
        Self {
            armor_mesh: None,
            hair_mesh: None,
        }
    }

    pub fn set_armor(&mut self, mesh: Mesh) -> Result<(), &'static str> {
        self.validate_mesh(&mesh)?;
        self.armor_mesh = Some(mesh);
        Ok(())
    }

    pub fn set_hair(&mut self, mesh: Mesh) -> Result<(), &'static str> {
        self.validate_mesh(&mesh)?;
        self.hair_mesh = Some(mesh);
        Ok(())
    }

    pub fn remove_armor(&mut self) {
        self.armor_mesh = None;
    }

    pub fn remove_hair(&mut self) {
        self.hair_mesh = None;
    }

    fn validate_mesh(&self, mesh: &Mesh) -> Result<(), &'static str> {
        for vertex in &mesh.vertices {
            // Because GrannyPwnt3432Vertex is #[repr(C, packed)], we need to copy fields
            // before using them to avoid unaligned reference errors.
            let weights = { vertex.bone_weights };
            let indices = { vertex.bone_indices };
            
            let mut sum_f = 0.0;
            for i in 0..4 {
                sum_f += (weights[i] as f32) / 255.0;
                if indices[i] >= 128 {
                    return Err("Bone index exceeds maximum allowed (127)");
                }
            }
            
            if (sum_f - 1.0).abs() > 0.02 {
                return Err("Bone weights do not sum to 1.0");
            }
        }
        Ok(())
    }
}
