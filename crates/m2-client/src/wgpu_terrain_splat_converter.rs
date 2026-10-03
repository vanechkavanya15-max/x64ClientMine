use std::sync::Arc;

#[cfg(test)]
use m2_client::wgpu_terrain_splatting::TerrainSplattingRenderer;
#[cfg(not(test))]
use crate::wgpu_terrain_splatting::TerrainSplattingRenderer;

#[cfg(test)]
use m2_client::texture_set_parser::TextureSet;
#[cfg(not(test))]
use crate::texture_set_parser::TextureSet;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TerrainSplatConverterConfig {
    pub splat_alpha_x_size: u32,
    pub splat_alpha_y_size: u32,
    pub max_splat_limit: u32,
}

impl Default for TerrainSplatConverterConfig {
    fn default() -> Self {
        Self {
            splat_alpha_x_size: 258,
            splat_alpha_y_size: 258,
            max_splat_limit: 50000,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum SplatConverterError {
    InvalidAlphaSize,
    InvalidTextureCount,
    MissingDevice,
    NotInitialized,
}

impl std::fmt::Display for SplatConverterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidAlphaSize => write!(f, "Invalid alpha mask size"),
            Self::InvalidTextureCount => write!(f, "Invalid texture count or size"),
            Self::MissingDevice => write!(f, "WGPU device is missing"),
            Self::NotInitialized => write!(f, "Splat converter not properly initialized"),
        }
    }
}

pub fn pack_alpha_maps_to_rgba(
    mask0: &[u8],
    mask1: &[u8],
    mask2: &[u8],
    mask3: &[u8],
    width: usize,
    height: usize,
) -> Result<Vec<u8>, SplatConverterError> {
    let expected_len = width * height;
    if mask0.len() != expected_len || mask1.len() != expected_len ||
       mask2.len() != expected_len || mask3.len() != expected_len {
        return Err(SplatConverterError::InvalidAlphaSize);
    }
    
    let mut packed = Vec::with_capacity(expected_len * 4);
    for i in 0..expected_len {
        packed.push(mask0[i]); // R
        packed.push(mask1[i]); // G
        packed.push(mask2[i]); // B
        packed.push(mask3[i]); // A
    }
    Ok(packed)
}

pub struct WgpuTerrainSplatConverter {
    pub config: TerrainSplatConverterConfig,
    pub rendered_splat_num: u32,
    pub rendered_splat_num_sq_sum: u32,
    pub splat_alpha_texture: Option<wgpu::Texture>,
    pub splat_alpha_view: Option<wgpu::TextureView>,
    pub splat_sampler: Option<wgpu::Sampler>,
    pub terrain_textures: [Option<wgpu::Texture>; 4],
    pub terrain_texture_views: [Option<wgpu::TextureView>; 4],
    pub tex_sampler: Option<wgpu::Sampler>,
    pub shadow_map_texture: Option<wgpu::Texture>,
    pub shadow_map_view: Option<wgpu::TextureView>,
}

impl WgpuTerrainSplatConverter {
    pub fn new(config: TerrainSplatConverterConfig) -> Self {
        Self {
            config,
            rendered_splat_num: 0,
            rendered_splat_num_sq_sum: 0,
            splat_alpha_texture: None,
            splat_alpha_view: None,
            splat_sampler: None,
            terrain_textures: Default::default(),
            terrain_texture_views: Default::default(),
            tex_sampler: None,
            shadow_map_texture: None,
            shadow_map_view: None,
        }
    }

    pub fn set_splat_limit(&mut self, limit: u32) {
        self.config.max_splat_limit = limit;
    }

    pub fn get_rendered_splat_num(&self, patch_count: u32) -> (u32, f32) {
        let ratio = if patch_count > 0 {
            self.rendered_splat_num_sq_sum as f32 / patch_count as f32
        } else {
            0.0
        };
        (self.rendered_splat_num, ratio)
    }

    pub fn create_samplers(&mut self, device: &wgpu::Device) {
        self.splat_sampler = Some(device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Splat Alpha Sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::MipmapFilterMode::Linear,
            ..Default::default()
        }));

        self.tex_sampler = Some(device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Terrain Texture Sampler"),
            address_mode_u: wgpu::AddressMode::Repeat,
            address_mode_v: wgpu::AddressMode::Repeat,
            address_mode_w: wgpu::AddressMode::Repeat,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::MipmapFilterMode::Linear,
            ..Default::default()
        }));
    }

    pub fn load_alpha_mask(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        alpha_data: &[u8],
    ) -> Result<(), SplatConverterError> {
        let expected_size = (self.config.splat_alpha_x_size * self.config.splat_alpha_y_size * 4) as usize;
        if alpha_data.len() != expected_size {
            return Err(SplatConverterError::InvalidAlphaSize);
        }

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Splat Alpha Texture"),
            size: wgpu::Extent3d {
                width: self.config.splat_alpha_x_size,
                height: self.config.splat_alpha_y_size,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            alpha_data,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(self.config.splat_alpha_x_size * 4),
                rows_per_image: Some(self.config.splat_alpha_y_size),
            },
            wgpu::Extent3d {
                width: self.config.splat_alpha_x_size,
                height: self.config.splat_alpha_y_size,
                depth_or_array_layers: 1,
            },
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        self.splat_alpha_texture = Some(texture);
        self.splat_alpha_view = Some(view);

        Ok(())
    }

    pub fn load_terrain_textures(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        texture_data_list: &[&[u8]; 4],
        width: u32,
        height: u32,
    ) -> Result<(), SplatConverterError> {
        let mut idx = 0;
        for data in texture_data_list.iter() {
            let expected_size = (width * height * 4) as usize;
            if data.len() != expected_size {
                return Err(SplatConverterError::InvalidTextureCount);
            }

            let texture = device.create_texture(&wgpu::TextureDescriptor {
                label: Some(&format!("Terrain Splat Texture {}", idx)),
                size: wgpu::Extent3d {
                    width,
                    height,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                view_formats: &[],
            });

            queue.write_texture(
                wgpu::TexelCopyTextureInfo {
                    texture: &texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO,
                    aspect: wgpu::TextureAspect::All,
                },
                data,
                wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(width * 4),
                    rows_per_image: Some(height),
                },
                wgpu::Extent3d {
                    width,
                    height,
                    depth_or_array_layers: 1,
                },
            );

            let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
            self.terrain_textures[idx] = Some(texture);
            self.terrain_texture_views[idx] = Some(view);
            
            idx += 1;
        }

        Ok(())
    }

    pub fn load_shadow_map(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        shadow_data: &[u8],
        width: u32,
        height: u32,
    ) -> Result<(), SplatConverterError> {
        let expected_size = (width * height * 4) as usize;
        if shadow_data.len() != expected_size {
            return Err(SplatConverterError::InvalidAlphaSize);
        }

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Shadow Map Texture"),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            shadow_data,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(width * 4),
                rows_per_image: Some(height),
            },
            wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        self.shadow_map_texture = Some(texture);
        self.shadow_map_view = Some(view);

        Ok(())
    }
    
    pub fn prepare_renderer(
        &self,
        device: &wgpu::Device,
        renderer: &mut TerrainSplattingRenderer,
        texture_set: TextureSet,
    ) -> Result<(), SplatConverterError> {
        let alpha_view = self.splat_alpha_view.as_ref().ok_or(SplatConverterError::NotInitialized)?;
        let splat_sampler = self.splat_sampler.as_ref().ok_or(SplatConverterError::NotInitialized)?;
        let shadow_map_view = self.shadow_map_view.as_ref().ok_or(SplatConverterError::NotInitialized)?;
        
        let t0 = self.terrain_texture_views[0].as_ref().ok_or(SplatConverterError::NotInitialized)?;
        let t1 = self.terrain_texture_views[1].as_ref().ok_or(SplatConverterError::NotInitialized)?;
        let t2 = self.terrain_texture_views[2].as_ref().ok_or(SplatConverterError::NotInitialized)?;
        let t3 = self.terrain_texture_views[3].as_ref().ok_or(SplatConverterError::NotInitialized)?;
        let tex_sampler = self.tex_sampler.as_ref().ok_or(SplatConverterError::NotInitialized)?;

        renderer.load_texture_set(
            device,
            texture_set,
            alpha_view,
            splat_sampler,
            shadow_map_view,
            [t0, t1, t2, t3],
            tex_sampler,
        ).map_err(|_| SplatConverterError::NotInitialized)?;

        Ok(())
    }

    pub fn draw_terrain_splat<'a>(
        &mut self,
        renderer: &'a TerrainSplattingRenderer,
        render_pass: &mut wgpu::RenderPass<'a>,
        vertex_buffer: &'a wgpu::Buffer,
        index_buffer: &'a wgpu::Buffer,
        index_count: u32,
    ) -> Result<(), SplatConverterError> {
        if self.rendered_splat_num >= self.config.max_splat_limit {
            return Ok(());
        }

        renderer.draw(render_pass, vertex_buffer, index_buffer, index_count)
            .map_err(|_| SplatConverterError::NotInitialized)?;

        self.rendered_splat_num += 1;
        self.rendered_splat_num_sq_sum += 1;

        Ok(())
    }
}
