use std::sync::Mutex;
use std::f32::consts::PI;

/// Reprezentuje poziom szczegółowości drzewa (Level Of Detail).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpeedTreeLodLevel {
    Mesh3D,
    CrossBillboard,
    SingleBillboard,
    Culled,
}

/// Konfiguracja progów odległości dla poszczególnych poziomów LOD.
#[derive(Debug, Clone)]
pub struct SpeedTreeLodConfig {
    pub mesh_to_cross_dist: f32,
    pub cross_to_single_dist: f32,
    pub single_to_culled_dist: f32,
    pub alpha_dither_overlap: f32,
}

impl Default for SpeedTreeLodConfig {
    fn default() -> Self {
        Self {
            mesh_to_cross_dist: 1000.0,
            cross_to_single_dist: 3000.0,
            single_to_culled_dist: 10000.0,
            alpha_dither_overlap: 200.0,
        }
    }
}

/// Oscylator kołysania gałęzi i liści na wietrze (Wind Sway Oscillation).
#[derive(Debug, Clone)]
pub struct SpeedTreeOscillator {
    wind_strength: f32,
    time: f32,
}

impl SpeedTreeOscillator {
    pub fn new() -> Self {
        Self {
            wind_strength: 0.5,
            time: 0.0,
        }
    }

    /// Ustawia siłę wiatru.
    pub fn set_wind_strength(&mut self, strength: f32) {
        self.wind_strength = strength;
    }

    /// Przesuwa czas w przód (advance).
    pub fn advance_time(&mut self, delta_time: f32) {
        self.time += delta_time;
    }

    /// Oblicza odchylenie wierzchołka w zależności od jego wysokości i siły wiatru.
    /// Zwraca kąt w radianach.
    pub fn calculate_deflection(&self, vertex_height: f32, tree_height: f32) -> f32 {
        if tree_height <= 0.0 || vertex_height <= 0.0 {
            return 0.0;
        }

        let height_ratio = (vertex_height / tree_height).clamp(0.0, 1.0);
        // Oscylacja bazowana na sinusoidzie: sin(czas * stała)
        // Im wyżej, tym większe wychylenie. Siła wiatru potęguje efekt.
        let base_oscillation = (self.time * 2.0 * PI * 0.5).sin(); 
        
        let deflection = base_oscillation * self.wind_strength * height_ratio * 0.1; // 0.1 to max odchylenie przy sile=1.0 i max wysokości
        
        deflection
    }
}

/// Instance of a SpeedTree object in the world.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SpeedTreeInstance {
    pub id: u32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub tree_height: f32,
}

/// Result of evaluating LOD for a specific tree instance.
#[derive(Debug, Clone, PartialEq)]
pub struct SpeedTreeLodResult {
    pub lod_level: SpeedTreeLodLevel,
    /// Alpha value for dither cross-fade (0.0 to 1.0).
    /// Used for smooth transitions between Mesh3D -> CrossBillboard -> SingleBillboard.
    pub alpha: f32,
}

/// Simple frustum culling representation for filtering instances.
#[derive(Debug, Clone)]
pub struct ViewFrustum {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
}

impl ViewFrustum {
    pub fn contains_point(&self, px: f32, py: f32) -> bool {
        let dx = px - self.x;
        let dy = py - self.y;
        (dx * dx + dy * dy) <= (self.radius * self.radius)
    }
}

/// Manager handling instances, LOD logic, cross-fade and frustum culling.
pub struct SpeedTreeManager {
    instances: Vec<SpeedTreeInstance>,
    lod_config: SpeedTreeLodConfig,
    oscillator: SpeedTreeOscillator,
}

impl SpeedTreeManager {
    pub fn new() -> Self {
        Self {
            instances: Vec::new(),
            lod_config: SpeedTreeLodConfig::default(),
            oscillator: SpeedTreeOscillator::new(),
        }
    }

    pub fn add_instance(&mut self, instance: SpeedTreeInstance) {
        self.instances.push(instance);
    }

    pub fn clear_instances(&mut self) {
        self.instances.clear();
    }

    pub fn get_oscillator_mut(&mut self) -> &mut SpeedTreeOscillator {
        &mut self.oscillator
    }

    pub fn evaluate_lod(&self, distance: f32) -> SpeedTreeLodResult {
        if distance < self.lod_config.mesh_to_cross_dist {
            // Check if within dither range towards CrossBillboard
            let overlap_start = self.lod_config.mesh_to_cross_dist - self.lod_config.alpha_dither_overlap;
            let mut alpha = 1.0;
            if distance > overlap_start {
                alpha = 1.0 - ((distance - overlap_start) / self.lod_config.alpha_dither_overlap).clamp(0.0, 1.0);
            }
            SpeedTreeLodResult {
                lod_level: SpeedTreeLodLevel::Mesh3D,
                alpha,
            }
        } else if distance < self.lod_config.cross_to_single_dist {
            let overlap_start = self.lod_config.cross_to_single_dist - self.lod_config.alpha_dither_overlap;
            let mut alpha = 1.0;
            if distance > overlap_start {
                alpha = 1.0 - ((distance - overlap_start) / self.lod_config.alpha_dither_overlap).clamp(0.0, 1.0);
            }
            SpeedTreeLodResult {
                lod_level: SpeedTreeLodLevel::CrossBillboard,
                alpha,
            }
        } else if distance < self.lod_config.single_to_culled_dist {
            SpeedTreeLodResult {
                lod_level: SpeedTreeLodLevel::SingleBillboard,
                alpha: 1.0,
            }
        } else {
            SpeedTreeLodResult {
                lod_level: SpeedTreeLodLevel::Culled,
                alpha: 0.0,
            }
        }
    }

    pub fn get_visible_instances(&self, frustum: &ViewFrustum) -> Vec<SpeedTreeInstance> {
        self.instances
            .iter()
            .filter(|inst| frustum.contains_point(inst.x, inst.y))
            .cloned()
            .collect()
    }
}

// C-FFI wrapper functions for C++ integration

use std::sync::OnceLock;

static SPEEDTREE_MANAGER: OnceLock<Mutex<SpeedTreeManager>> = OnceLock::new();

fn get_manager() -> &'static Mutex<SpeedTreeManager> {
    SPEEDTREE_MANAGER.get_or_init(|| Mutex::new(SpeedTreeManager::new()))
}

#[no_mangle]
pub extern "C" fn m2_speedtree_add_instance(id: u32, x: f32, y: f32, z: f32, tree_height: f32) {
    if let Ok(mut mgr) = get_manager().lock() {
        mgr.add_instance(SpeedTreeInstance {
            id,
            x,
            y,
            z,
            tree_height,
        });
    }
}

#[no_mangle]
pub extern "C" fn m2_speedtree_clear_instances() {
    if let Ok(mut mgr) = get_manager().lock() {
        mgr.clear_instances();
    }
}

#[no_mangle]
pub extern "C" fn m2_speedtree_set_wind_strength(strength: f32) {
    if let Ok(mut mgr) = get_manager().lock() {
        mgr.get_oscillator_mut().set_wind_strength(strength);
    }
}

#[no_mangle]
pub extern "C" fn m2_speedtree_advance_time(delta_time: f32) {
    if let Ok(mut mgr) = get_manager().lock() {
        mgr.get_oscillator_mut().advance_time(delta_time);
    }
}

#[no_mangle]
pub extern "C" fn m2_speedtree_get_deflection(vertex_height: f32, tree_height: f32) -> f32 {
    if let Ok(mgr) = get_manager().lock() {
        mgr.oscillator.calculate_deflection(vertex_height, tree_height)
    } else {
        0.0
    }
}

#[no_mangle]
pub extern "C" fn m2_speedtree_evaluate_lod(distance: f32, out_alpha: *mut f32) -> i32 {
    if let Ok(mgr) = get_manager().lock() {
        let res = mgr.evaluate_lod(distance);
        if !out_alpha.is_null() {
            unsafe {
                *out_alpha = res.alpha;
            }
        }
        match res.lod_level {
            SpeedTreeLodLevel::Mesh3D => 0,
            SpeedTreeLodLevel::CrossBillboard => 1,
            SpeedTreeLodLevel::SingleBillboard => 2,
            SpeedTreeLodLevel::Culled => 3,
        }
    } else {
        3 // Culled default on error
    }
}

/// FFI function for frustum culling. Retrieves the number of visible instances
/// and fills the provided array if enough space is provided.
/// Returns the actual number of visible instances.
#[no_mangle]
pub extern "C" fn m2_speedtree_get_visible_instances(
    fx: f32,
    fy: f32,
    fradius: f32,
    out_instances: *mut SpeedTreeInstance,
    max_count: usize,
) -> usize {
    if let Ok(mgr) = get_manager().lock() {
        let frustum = ViewFrustum {
            x: fx,
            y: fy,
            radius: fradius,
        };
        let visible = mgr.get_visible_instances(&frustum);
        let count = visible.len();
        
        if !out_instances.is_null() && max_count > 0 {
            let write_count = std::cmp::min(count, max_count);
            for i in 0..write_count {
                unsafe {
                    let ptr = out_instances.add(i);
                    *ptr = visible[i];
                }
            }
        }
        
        count
    } else {
        0
    }
}
