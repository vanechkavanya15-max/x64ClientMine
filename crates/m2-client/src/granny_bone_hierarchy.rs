use glam::Mat4;

#[derive(Debug, Clone, Copy)]
pub struct Bone {
    pub parent_index: Option<usize>,
    pub local_transform: Mat4,
}

#[derive(Debug, Clone)]
pub struct Skeleton {
    pub bones: [Bone; 64],
    pub bone_count: usize,
}

impl Skeleton {
    pub fn new() -> Self {
        Self {
            bones: [Bone {
                parent_index: None,
                local_transform: Mat4::IDENTITY,
            }; 64],
            bone_count: 0,
        }
    }

    pub fn add_bone(&mut self, parent_index: Option<usize>, local_transform: Mat4) -> Result<usize, &'static str> {
        if self.bone_count >= 64 {
            return Err("Max 64 bones allowed");
        }
        if let Some(p_idx) = parent_index {
            if p_idx >= self.bone_count {
                return Err("Parent index out of bounds");
            }
        }
        let index = self.bone_count;
        self.bones[index] = Bone { parent_index, local_transform };
        self.bone_count += 1;
        Ok(index)
    }

    pub fn calculate_global_transforms(&self) -> Result<[Mat4; 64], &'static str> {
        const NONE: Option<Mat4> = None;
        let mut global_transforms = [NONE; 64];
        let mut visited = [false; 64];

        for i in 0..self.bone_count {
            self.compute_global_transform(i, &mut global_transforms, &mut visited)?;
        }

        let mut result = [Mat4::IDENTITY; 64];
        for i in 0..self.bone_count {
            result[i] = global_transforms[i].unwrap();
        }

        Ok(result)
    }

    fn compute_global_transform(
        &self,
        index: usize,
        global_transforms: &mut [Option<Mat4>; 64],
        visited: &mut [bool; 64],
    ) -> Result<Mat4, &'static str> {
        if let Some(transform) = global_transforms[index] {
            return Ok(transform);
        }

        if visited[index] {
            return Err("Cycle detected in bone hierarchy");
        }
        visited[index] = true;

        let bone = &self.bones[index];
        let global_transform = match bone.parent_index {
            Some(parent_idx) => {
                if parent_idx >= self.bone_count {
                    return Err("Parent index out of bounds");
                }
                let parent_global = self.compute_global_transform(parent_idx, global_transforms, visited)?;
                parent_global * bone.local_transform
            }
            None => bone.local_transform,
        };

        global_transforms[index] = Some(global_transform);
        visited[index] = false;

        Ok(global_transform)
    }
}
