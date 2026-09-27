// d3d9_state_pipeline.rs
// Implements the D3D9 StateManager functionality from EterLib/StateManager.cpp

pub const D3DRS_ZENABLE: u32 = 7;
pub const D3DRS_ZWRITEENABLE: u32 = 14;
pub const D3DRS_SRCBLEND: u32 = 19;
pub const D3DRS_DESTBLEND: u32 = 20;
pub const D3DRS_CULLMODE: u32 = 22;
pub const D3DRS_ALPHABLENDENABLE: u32 = 27;
pub const D3DRS_FOGENABLE: u32 = 28;

#[derive(Debug, Clone)]
pub struct RenderStateCache {
    cache: [u32; 256],
}

impl Default for RenderStateCache {
    fn default() -> Self {
        Self {
            cache: [0; 256],
        }
    }
}

#[derive(Debug, Clone)]
pub struct SamplerStateCache {
    cache: [[u32; 16]; 8],
}

impl Default for SamplerStateCache {
    fn default() -> Self {
        Self {
            cache: [[0; 16]; 8],
        }
    }
}

#[derive(Debug, Clone)]
pub struct TextureStageStateCache {
    cache: [[u32; 32]; 8],
}

impl Default for TextureStageStateCache {
    fn default() -> Self {
        Self {
            cache: [[0; 32]; 8],
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct StateManager {
    render_states: RenderStateCache,
    sampler_states: SamplerStateCache,
    texture_stage_states: TextureStageStateCache,
}

impl StateManager {
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets a render state, returning true if the state changed.
    pub fn set_render_state(&mut self, state: u32, value: u32) -> bool {
        if state >= 256 {
            return false;
        }

        if self.render_states.cache[state as usize] != value {
            self.render_states.cache[state as usize] = value;
            true
        } else {
            false
        }
    }

    /// Sets a sampler state, returning true if the state changed.
    pub fn set_sampler_state(&mut self, stage: u32, state: u32, value: u32) -> bool {
        if stage >= 8 || state >= 16 {
            return false;
        }

        if self.sampler_states.cache[stage as usize][state as usize] != value {
            self.sampler_states.cache[stage as usize][state as usize] = value;
            true
        } else {
            false
        }
    }

    /// Sets a texture stage state, returning true if the state changed.
    pub fn set_texture_stage_state(&mut self, stage: u32, state: u32, value: u32) -> bool {
        if stage >= 8 || state >= 32 {
            return false;
        }

        if self.texture_stage_states.cache[stage as usize][state as usize] != value {
            self.texture_stage_states.cache[stage as usize][state as usize] = value;
            true
        } else {
            false
        }
    }
}
