use glam::Mat4;
use std::mem;

pub const MAX_BONE_MATRICES: usize = 128;

pub struct SkeletalBonePaletteUploader {
    pub bone_matrices: [[[f32; 4]; 4]; MAX_BONE_MATRICES],
}

impl SkeletalBonePaletteUploader {
    pub fn new() -> Self {
        let identity = Mat4::IDENTITY.to_cols_array_2d();
        Self {
            bone_matrices: [identity; MAX_BONE_MATRICES],
        }
    }

    pub fn update_palette(&mut self, matrices: &[Mat4]) {
        let identity = Mat4::IDENTITY.to_cols_array_2d();
        let copy_count = matrices.len().min(MAX_BONE_MATRICES);

        for i in 0..copy_count {
            self.bone_matrices[i] = matrices[i].to_cols_array_2d();
        }

        for i in copy_count..MAX_BONE_MATRICES {
            self.bone_matrices[i] = identity;
        }
    }

    pub fn upload(
        &self,
        queue: &wgpu::Queue,
        buffer: &wgpu::Buffer,
        offset: wgpu::BufferAddress,
    ) {
        let data = bytemuck::cast_slice(&self.bone_matrices);
        queue.write_buffer(buffer, offset, data);
    }
}

impl Default for SkeletalBonePaletteUploader {
    fn default() -> Self {
        Self::new()
    }
}
