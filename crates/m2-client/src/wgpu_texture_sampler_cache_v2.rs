#![allow(non_snake_case)]
#![allow(dead_code)]

use std::collections::HashMap;
use std::sync::Arc;

/// Represents Direct3D 9 Texture Filter types as seen in Metin2 C++
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum D3DTextureFilterType {
    None = 0,
    Point = 1,
    Linear = 2,
    Anisotropic = 3,
}

impl D3DTextureFilterType {
    pub fn to_wgpu_filter_mode(self) -> wgpu::FilterMode {
        match self {
            D3DTextureFilterType::None | D3DTextureFilterType::Point => wgpu::FilterMode::Nearest,
            D3DTextureFilterType::Linear | D3DTextureFilterType::Anisotropic => {
                wgpu::FilterMode::Linear
            }
        }
    }
}

/// Represents Direct3D 9 Texture Address types as seen in Metin2 C++
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum D3DTextureAddressType {
    Wrap = 1,
    Mirror = 2,
    Clamp = 3,
}

impl D3DTextureAddressType {
    pub fn to_wgpu_address_mode(self) -> wgpu::AddressMode {
        match self {
            D3DTextureAddressType::Wrap => wgpu::AddressMode::Repeat,
            D3DTextureAddressType::Mirror => wgpu::AddressMode::MirrorRepeat,
            D3DTextureAddressType::Clamp => wgpu::AddressMode::ClampToEdge,
        }
    }
}

/// Unique Key representing a Sampler State for the Cache
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SamplerKey {
    pub min_filter: D3DTextureFilterType,
    pub mag_filter: D3DTextureFilterType,
    pub mip_filter: D3DTextureFilterType,
    pub address_u: D3DTextureAddressType,
    pub address_v: D3DTextureAddressType,
    pub max_anisotropy: u16,
}

impl SamplerKey {
    pub fn to_wgpu_descriptor<'a>(&self) -> wgpu::SamplerDescriptor<'a> {
        wgpu::SamplerDescriptor {
            label: Some("M2TextureSampler"),
            address_mode_u: self.address_u.to_wgpu_address_mode(),
            address_mode_v: self.address_v.to_wgpu_address_mode(),
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: self.mag_filter.to_wgpu_filter_mode(),
            min_filter: self.min_filter.to_wgpu_filter_mode(),
            mipmap_filter: match self.mip_filter {
                D3DTextureFilterType::None => wgpu::MipmapFilterMode::Nearest, // If no mipmap, treat as nearest
                D3DTextureFilterType::Point => wgpu::MipmapFilterMode::Nearest,
                D3DTextureFilterType::Linear | D3DTextureFilterType::Anisotropic => {
                    wgpu::MipmapFilterMode::Linear
                }
            },
            lod_min_clamp: 0.0,
            lod_max_clamp: if self.mip_filter == D3DTextureFilterType::None {
                0.0
            } else {
                32.0 // standard max LOD
            },
            compare: None,
            anisotropy_clamp: if self.min_filter == D3DTextureFilterType::Anisotropic 
                || self.mag_filter == D3DTextureFilterType::Anisotropic 
            {
                self.max_anisotropy.max(1).min(16)
            } else {
                1
            },
            border_color: None,
        }
    }
}

pub struct TextureSamplerCache {
    cache: HashMap<SamplerKey, Arc<wgpu::Sampler>>,
}

impl TextureSamplerCache {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    pub fn get_or_create_sampler(
        &mut self,
        device: &wgpu::Device,
        key: SamplerKey,
    ) -> Arc<wgpu::Sampler> {
        if let Some(sampler) = self.cache.get(&key) {
            return Arc::clone(sampler);
        }

        let descriptor = key.to_wgpu_descriptor();
        let sampler = Arc::new(device.create_sampler(&descriptor));
        self.cache.insert(key, Arc::clone(&sampler));
        sampler
    }
}
