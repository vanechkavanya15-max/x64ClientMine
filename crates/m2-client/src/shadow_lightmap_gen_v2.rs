use glam::Vec3;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Mutex;
use std::collections::HashMap;

// 1:1 transcription of C++ MapOutdoorQuadtree / TerrainQuadtreeNode

#[derive(Debug)]
pub struct TerrainQuadtreeNode {
    pub x0: i32,
    pub y0: i32,
    pub x1: i32,
    pub y1: i32,
    pub nw_node: Option<Box<TerrainQuadtreeNode>>,
    pub ne_node: Option<Box<TerrainQuadtreeNode>>,
    pub sw_node: Option<Box<TerrainQuadtreeNode>>,
    pub se_node: Option<Box<TerrainQuadtreeNode>>,
    pub size: i32,
    pub patch_num: i32,
    pub center: Vec3,
    pub radius: f32,
    pub by_lod_level: u8,
}

impl TerrainQuadtreeNode {
    pub fn new(x0: i32, y0: i32, x1: i32, y1: i32, patch_count: i32) -> Option<Self> {
        let xsize = x1 - x0 + 1;
        let ysize = y1 - y0 + 1;
        
        if xsize == 0 || ysize == 0 {
            return None;
        }

        let size = if ysize > xsize { ysize } else { xsize };
        let patch_num = y0 * patch_count + x0;

        Some(Self {
            x0,
            y0,
            x1,
            y1,
            nw_node: None,
            ne_node: None,
            sw_node: None,
            se_node: None,
            size,
            patch_num,
            center: Vec3::ZERO,
            radius: 0.0,
            by_lod_level: 0,
        })
    }
}

pub struct MapOutdoorQuadtree {
    pub patch_count: i32,
    pub root_node: Option<Box<TerrainQuadtreeNode>>,
}

impl MapOutdoorQuadtree {
    pub fn new() -> Self {
        Self {
            patch_count: 0,
            root_node: None,
        }
    }

    pub fn build_quad_tree(&mut self) {
        self.free_quad_tree();

        if self.patch_count == 0 {
            // TraceError: m_wPatchCount is zero
            return;
        }

        if let Some(mut root) = TerrainQuadtreeNode::new(0, 0, self.patch_count - 1, self.patch_count - 1, self.patch_count) {
            if root.size > 1 {
                self.subdivide_node(&mut root);
            }
            self.root_node = Some(Box::new(root));
        }
    }

    fn subdivide_node(&self, node: &mut TerrainQuadtreeNode) {
        let nw_size = node.size / 2;

        node.nw_node = TerrainQuadtreeNode::new(node.x0, node.y0, node.x0 + nw_size - 1, node.y0 + nw_size - 1, self.patch_count).map(Box::new);
        node.ne_node = TerrainQuadtreeNode::new(node.x0 + nw_size, node.y0, node.x1, node.y0 + nw_size - 1, self.patch_count).map(Box::new);
        node.sw_node = TerrainQuadtreeNode::new(node.x0, node.y0 + nw_size, node.x0 + nw_size - 1, node.y1, self.patch_count).map(Box::new);
        node.se_node = TerrainQuadtreeNode::new(node.x0 + nw_size, node.y0 + nw_size, node.x1, node.y1, self.patch_count).map(Box::new);

        if let Some(ref mut nw) = node.nw_node {
            if nw.size > 1 {
                self.subdivide_node(nw);
            }
        }
        if let Some(ref mut ne) = node.ne_node {
            if ne.size > 1 {
                self.subdivide_node(ne);
            }
        }
        if let Some(ref mut sw) = node.sw_node {
            if sw.size > 1 {
                self.subdivide_node(sw);
            }
        }
        if let Some(ref mut se) = node.se_node {
            if se.size > 1 {
                self.subdivide_node(se);
            }
        }
    }

    pub fn free_quad_tree(&mut self) {
        self.root_node = None;
    }
}

// Architectural Requirement: Generate mountain shadows
pub struct ShadowLightmapGen {
    pub sun_direction: Vec3,
}

impl ShadowLightmapGen {
    pub fn new(sun_direction: Vec3) -> Self {
        Self {
            sun_direction: sun_direction.normalize_or_zero(),
        }
    }

    // Shadow raycast algorithm
    pub fn compute_shadow_factor(&self, x: f32, y: f32, z: f32, get_height: &impl Fn(f32, f32) -> f32) -> f32 {
        if self.sun_direction == Vec3::ZERO || self.sun_direction.z <= 0.0 {
            // Sun is below horizon or invalid
            return 1.0;
        }

        let mut current_pos = Vec3::new(x, y, z);
        let step_size = 50.0;
        let max_distance = 15000.0; // Distance to raymarch
        let mut traveled = 0.0;
        
        // Reverse sun direction to march TOWARDS the sun
        let ray_dir = self.sun_direction; 

        while traveled < max_distance {
            current_pos += ray_dir * step_size;
            traveled += step_size;

            let terrain_height = get_height(current_pos.x, current_pos.y);
            
            if terrain_height > current_pos.z {
                // Hit terrain, so we are in shadow
                return 0.0;
            }
        }

        // Ray reached the sun without hitting terrain
        1.0
    }
}

// Simple terrain height mock for the global FFI, ideally we'd tie this into the real height map
// but since the requirement forces us to implement the FFI matching standard signature
// we provide a configurable singleton logic.
static SUN_DIR_X: AtomicU32 = AtomicU32::new(0x3f800000); // 1.0f32
static SUN_DIR_Y: AtomicU32 = AtomicU32::new(0x3f800000);
static SUN_DIR_Z: AtomicU32 = AtomicU32::new(0x3f800000);

static mut HEIGHT_MAP: Option<Mutex<HashMap<(i32, i32), f32>>> = None;
static INIT: std::sync::Once = std::sync::Once::new();

#[allow(static_mut_refs)]
fn get_height_map() -> &'static Mutex<HashMap<(i32, i32), f32>> {
    unsafe {
        INIT.call_once(|| {
            HEIGHT_MAP = Some(Mutex::new(HashMap::new()));
        });
        HEIGHT_MAP.as_ref().unwrap()
    }
}

pub fn set_sun_direction_for_ffi(dir: Vec3) {
    SUN_DIR_X.store(f32::to_bits(dir.x), Ordering::SeqCst);
    SUN_DIR_Y.store(f32::to_bits(dir.y), Ordering::SeqCst);
    SUN_DIR_Z.store(f32::to_bits(dir.z), Ordering::SeqCst);
}

pub fn set_terrain_height_for_ffi(x: i32, y: i32, h: f32) {
    if let Ok(mut map) = get_height_map().lock() {
        map.insert((x, y), h);
    }
}

pub fn clear_terrain_height_for_ffi() {
    if let Ok(mut map) = get_height_map().lock() {
        map.clear();
    }
}

#[no_mangle]
pub extern "C" fn m2_terrain_is_in_shadow(x: f32, y: f32, z: f32) -> bool {
    let dx = f32::from_bits(SUN_DIR_X.load(Ordering::SeqCst));
    let dy = f32::from_bits(SUN_DIR_Y.load(Ordering::SeqCst));
    let dz = f32::from_bits(SUN_DIR_Z.load(Ordering::SeqCst));
    
    let gen = ShadowLightmapGen::new(Vec3::new(dx, dy, dz));
    
    let height_getter = |px: f32, py: f32| -> f32 {
        // Grid size of 200 units (Metin2 scale)
        let cx = (px / 200.0).round() as i32;
        let cy = (py / 200.0).round() as i32;
        if let Ok(map) = get_height_map().lock() {
            *map.get(&(cx, cy)).unwrap_or(&0.0)
        } else {
            0.0
        }
    };

    let factor = gen.compute_shadow_factor(x, y, z, &height_getter);
    factor < 0.5 // 0.0 == shadow
}
