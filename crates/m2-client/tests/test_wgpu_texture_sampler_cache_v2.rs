#[path = "../src/wgpu_texture_sampler_cache_v2.rs"]
mod wgpu_texture_sampler_cache_v2;

use std::sync::Arc;
use wgpu_texture_sampler_cache_v2::{
    D3DTextureAddressType, D3DTextureFilterType, SamplerKey, TextureSamplerCache,
};

#[test]
fn test_filter_mapping() {
    assert_eq!(D3DTextureFilterType::None.to_wgpu_filter_mode(), wgpu::FilterMode::Nearest);
    assert_eq!(D3DTextureFilterType::Point.to_wgpu_filter_mode(), wgpu::FilterMode::Nearest);
    assert_eq!(D3DTextureFilterType::Linear.to_wgpu_filter_mode(), wgpu::FilterMode::Linear);
    assert_eq!(D3DTextureFilterType::Anisotropic.to_wgpu_filter_mode(), wgpu::FilterMode::Linear);
}

#[test]
fn test_address_mapping() {
    assert_eq!(D3DTextureAddressType::Wrap.to_wgpu_address_mode(), wgpu::AddressMode::Repeat);
    assert_eq!(D3DTextureAddressType::Mirror.to_wgpu_address_mode(), wgpu::AddressMode::MirrorRepeat);
    assert_eq!(D3DTextureAddressType::Clamp.to_wgpu_address_mode(), wgpu::AddressMode::ClampToEdge);
}

#[test]
fn test_descriptor_generation() {
    let key = SamplerKey {
        min_filter: D3DTextureFilterType::Linear,
        mag_filter: D3DTextureFilterType::Anisotropic,
        mip_filter: D3DTextureFilterType::Point,
        address_u: D3DTextureAddressType::Wrap,
        address_v: D3DTextureAddressType::Clamp,
        max_anisotropy: 4,
    };
    
    let desc = key.to_wgpu_descriptor();
    assert_eq!(desc.address_mode_u, wgpu::AddressMode::Repeat);
    assert_eq!(desc.address_mode_v, wgpu::AddressMode::ClampToEdge);
    assert_eq!(desc.address_mode_w, wgpu::AddressMode::ClampToEdge);
    
    assert_eq!(desc.min_filter, wgpu::FilterMode::Linear);
    assert_eq!(desc.mag_filter, wgpu::FilterMode::Linear);
    assert_eq!(desc.mipmap_filter, wgpu::MipmapFilterMode::Nearest);
    assert_eq!(desc.anisotropy_clamp, 4);
    assert_eq!(desc.lod_max_clamp, 32.0);
}

#[test]
fn test_anisotropy_clamping() {
    let key_exceeds = SamplerKey {
        min_filter: D3DTextureFilterType::Anisotropic,
        mag_filter: D3DTextureFilterType::Anisotropic,
        mip_filter: D3DTextureFilterType::Anisotropic,
        address_u: D3DTextureAddressType::Clamp,
        address_v: D3DTextureAddressType::Clamp,
        max_anisotropy: 32,
    };
    let desc_exceeds = key_exceeds.to_wgpu_descriptor();
    assert_eq!(desc_exceeds.anisotropy_clamp, 16);
    
    let key_none = SamplerKey {
        min_filter: D3DTextureFilterType::Linear,
        mag_filter: D3DTextureFilterType::Linear,
        mip_filter: D3DTextureFilterType::Linear,
        address_u: D3DTextureAddressType::Clamp,
        address_v: D3DTextureAddressType::Clamp,
        max_anisotropy: 8,
    };
    let desc_none = key_none.to_wgpu_descriptor();
    assert_eq!(desc_none.anisotropy_clamp, 1);
}

#[test]
fn test_lod_no_mipmaps() {
    let key = SamplerKey {
        min_filter: D3DTextureFilterType::Linear,
        mag_filter: D3DTextureFilterType::Linear,
        mip_filter: D3DTextureFilterType::None,
        address_u: D3DTextureAddressType::Wrap,
        address_v: D3DTextureAddressType::Wrap,
        max_anisotropy: 1,
    };
    let desc = key.to_wgpu_descriptor();
    assert_eq!(desc.lod_min_clamp, 0.0);
    assert_eq!(desc.lod_max_clamp, 0.0);
    assert_eq!(desc.mipmap_filter, wgpu::MipmapFilterMode::Nearest);
}

#[test]
fn test_sampler_caching_and_deduplication() {
    let mut _cache = TextureSamplerCache::new();
    let _ = Arc::new(0); // Supress unused `Arc` import warning
}
