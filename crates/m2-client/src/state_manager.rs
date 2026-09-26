use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlendMode {
    Opaque,
    AlphaBlend,
    Additive,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DepthTest {
    None,
    ReadOnly,
    ReadWrite,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CullMode {
    None,
    Front,
    Back,
}

// Stale D3DRENDERSTATETYPE z C++ StateManager.h
#[allow(dead_code)]
pub const D3DRS_ZENABLE: u32 = 7;
#[allow(dead_code)]
pub const D3DRS_FILLMODE: u32 = 8;
#[allow(dead_code)]
pub const D3DRS_ZWRITEENABLE: u32 = 14;
#[allow(dead_code)]
pub const D3DRS_ALPHATESTENABLE: u32 = 15;
#[allow(dead_code)]
pub const D3DRS_SRCBLEND: u32 = 19;
#[allow(dead_code)]
pub const D3DRS_DESTBLEND: u32 = 20;
#[allow(dead_code)]
pub const D3DRS_CULLMODE: u32 = 22;
#[allow(dead_code)]
pub const D3DRS_ALPHABLENDENABLE: u32 = 27;
#[allow(dead_code)]
pub const D3DRS_FOGENABLE: u32 = 28;
#[allow(dead_code)]
pub const D3DRS_LIGHTING: u32 = 137;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StateManager {
    blend_mode: BlendMode,
    depth_test: DepthTest,
    cull_mode: CullMode,
    wireframe: bool,
    // Pelny rejestr 256 stanow renderowania (jak w C++ CStateManager)
    render_states: [u32; 256],
    // Stosy stanow dla SaveRenderState / RestoreRenderState
    state_stacks: HashMap<u32, Vec<u32>>,
}

impl Default for StateManager {
    fn default() -> Self {
        Self::new()
    }
}

impl StateManager {
    pub fn new() -> Self {
        let mut sm = Self {
            blend_mode: BlendMode::Opaque,
            depth_test: DepthTest::ReadWrite,
            cull_mode: CullMode::Back,
            wireframe: false,
            render_states: [0; 256],
            state_stacks: HashMap::new(),
        };
        sm.reset_to_defaults();
        sm
    }

    pub fn set_blend_mode(&mut self, mode: BlendMode) -> bool {
        if self.blend_mode == mode {
            false
        } else {
            self.blend_mode = mode;
            true
        }
    }

    pub fn set_depth_test(&mut self, mode: DepthTest) -> bool {
        if self.depth_test == mode {
            false
        } else {
            self.depth_test = mode;
            true
        }
    }

    pub fn set_cull_mode(&mut self, mode: CullMode) -> bool {
        if self.cull_mode == mode {
            false
        } else {
            self.cull_mode = mode;
            true
        }
    }

    pub fn set_wireframe(&mut self, enabled: bool) -> bool {
        if self.wireframe == enabled {
            false
        } else {
            self.wireframe = enabled;
            true
        }
    }

    /// Ustawia stan renderowania z filtrowaniem redundancji (C++ CStateManager::SetRenderState).
    /// Zwraca true, jesli stan ulegl zmianie (wyslano do GPU), false jesli byl juz ustawiony.
    pub fn set_render_state(&mut self, state: u32, value: u32) -> bool {
        let idx = (state as usize) % 256;
        if self.render_states[idx] == value {
            return false;
        }
        self.render_states[idx] = value;
        true
    }

    /// Pobiera biezaca wartosc stanu renderowania
    pub fn get_render_state(&self, state: u32) -> u32 {
        let idx = (state as usize) % 256;
        self.render_states[idx]
    }

    /// Zapisuje biezacy stan na stosie danego rejestru (C++ CStateManager::SaveRenderState).
    pub fn save_render_state(&mut self, state: u32) {
        let current = self.get_render_state(state);
        self.state_stacks.entry(state).or_default().push(current);
    }

    /// Przywraca poprzedni stan ze stosu (C++ CStateManager::RestoreRenderState).
    pub fn restore_render_state(&mut self, state: u32) -> Option<u32> {
        if let Some(stack) = self.state_stacks.get_mut(&state) {
            if let Some(prev) = stack.pop() {
                self.set_render_state(state, prev);
                return Some(prev);
            }
        }
        None
    }

    pub fn reset_to_defaults(&mut self) {
        self.blend_mode = BlendMode::Opaque;
        self.depth_test = DepthTest::ReadWrite;
        self.cull_mode = CullMode::Back;
        self.wireframe = false;
        self.render_states = [0; 256];
        self.render_states[D3DRS_ZENABLE as usize] = 1;
        self.render_states[D3DRS_ZWRITEENABLE as usize] = 1;
        self.render_states[D3DRS_CULLMODE as usize] = 3; // D3DCULL_CCW
        self.render_states[D3DRS_ALPHABLENDENABLE as usize] = 0;
        self.state_stacks.clear();
    }

    pub fn get_blend_mode(&self) -> BlendMode {
        self.blend_mode
    }

    pub fn get_depth_test(&self) -> DepthTest {
        self.depth_test
    }

    pub fn get_cull_mode(&self) -> CullMode {
        self.cull_mode
    }

    pub fn is_wireframe(&self) -> bool {
        self.wireframe
    }
}

