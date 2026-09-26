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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StateManager {
    blend_mode: BlendMode,
    depth_test: DepthTest,
    cull_mode: CullMode,
    wireframe: bool,
}

impl Default for StateManager {
    fn default() -> Self {
        Self::new()
    }
}

impl StateManager {
    pub fn new() -> Self {
        Self {
            blend_mode: BlendMode::Opaque,
            depth_test: DepthTest::ReadWrite,
            cull_mode: CullMode::Back,
            wireframe: false,
        }
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

    pub fn reset_to_defaults(&mut self) {
        self.blend_mode = BlendMode::Opaque;
        self.depth_test = DepthTest::ReadWrite;
        self.cull_mode = CullMode::Back;
        self.wireframe = false;
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
