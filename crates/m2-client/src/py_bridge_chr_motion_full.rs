//! Python Bridge Character Motion Full
//!
//! Zgodnosc z CPythonCharacterManager (chrSetLoopMotion, chrBlendLoopMotion,
//! chrPushOnceMotion, chrSetRotation, chrGetPixelPosition, chrSetArmor,
//! chrSetWeapon, chrSetHair).

#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_variables)]

use std::collections::HashMap;
use std::sync::{Mutex, Arc, OnceLock};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TPixelPosition {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Default for TPixelPosition {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CInstanceBase {
    pub vid: u32,
    pub loop_motion: u16,
    pub loop_motion_blend: f32,
    pub once_motion: u16,
    pub once_motion_blend: f32,
    pub rotation: f32,
    pub pixel_position: TPixelPosition,
    pub armor: u32,
    pub weapon: u32,
    pub hair: u32,
    pub has_bounding_sphere: bool,
}

impl CInstanceBase {
    pub fn new(vid: u32) -> Self {
        Self {
            vid,
            loop_motion: 0,
            loop_motion_blend: 0.0,
            once_motion: 0,
            once_motion_blend: 0.1, // domyslny blend z C++
            rotation: 0.0,
            pixel_position: TPixelPosition::default(),
            armor: 0,
            weapon: 0,
            hair: 0,
            has_bounding_sphere: false,
        }
    }

    pub fn set_loop_motion(&mut self, motion: u16, blend_time: f32) {
        self.loop_motion = motion;
        self.loop_motion_blend = blend_time;
    }

    pub fn push_once_motion(&mut self, motion: u16, blend_time: f32) {
        self.once_motion = motion;
        self.once_motion_blend = blend_time;
    }

    pub fn set_rotation(&mut self, mut rot: f32) {
        rot = (rot + 180.0) % 360.0;
        if rot < 0.0 {
            rot += 360.0; // fix dla float modulo w Ruscis
        }
        self.rotation = rot;
    }

    pub fn new_get_pixel_position(&self, pos: &mut TPixelPosition) {
        *pos = self.pixel_position;
    }

    pub fn set_armor(&mut self, form: u32) {
        self.armor = form;
    }

    pub fn register_bounding_sphere(&mut self) {
        self.has_bounding_sphere = true;
    }

    pub fn set_weapon(&mut self, form: u32) {
        self.weapon = form;
    }

    pub fn set_hair(&mut self, race: u32) {
        self.hair = race;
    }
}

pub struct CPythonCharacterManager {
    instances: HashMap<u32, Arc<Mutex<CInstanceBase>>>,
    selected_vid: Option<u32>,
}

impl CPythonCharacterManager {
    pub fn new() -> Self {
        Self {
            instances: HashMap::new(),
            selected_vid: None,
        }
    }

    pub fn get_instance_ptr(&self, vid: u32) -> Option<Arc<Mutex<CInstanceBase>>> {
        self.instances.get(&vid).cloned()
    }

    pub fn get_selected_instance_ptr(&self) -> Option<Arc<Mutex<CInstanceBase>>> {
        if let Some(vid) = self.selected_vid {
            self.get_instance_ptr(vid)
        } else {
            None
        }
    }

    // Metody testowe do zarzadzania instancjami (symulacja C++)
    pub fn register_instance(&mut self, vid: u32) -> Arc<Mutex<CInstanceBase>> {
        let inst = Arc::new(Mutex::new(CInstanceBase::new(vid)));
        self.instances.insert(vid, inst.clone());
        inst
    }

    pub fn select_instance(&mut self, vid: u32) {
        self.selected_vid = Some(vid);
    }
}

static CHR_MANAGER: OnceLock<Mutex<CPythonCharacterManager>> = OnceLock::new();

pub fn get_chr_manager() -> &'static Mutex<CPythonCharacterManager> {
    CHR_MANAGER.get_or_init(|| Mutex::new(CPythonCharacterManager::new()))
}

// Funkcje dostepowe z Python API (odpowiadaja PyObject * w C++)

pub fn m2_py_chr_set_loop_motion(motion_index: i32) -> Result<(), &'static str> {
    let mgr = get_chr_manager().lock().unwrap();
    if let Some(inst_mutex) = mgr.get_selected_instance_ptr() {
        let mut inst = inst_mutex.lock().unwrap();
        inst.set_loop_motion(motion_index as u16, 0.0);
    }
    Ok(())
}

pub fn set_blend_loop_motion(motion_index: i32, blend_time: f32) -> Result<(), &'static str> {
    let mgr = get_chr_manager().lock().unwrap();
    if let Some(inst_mutex) = mgr.get_selected_instance_ptr() {
        let mut inst = inst_mutex.lock().unwrap();
        inst.set_loop_motion(motion_index as u16, blend_time);
    }
    Ok(())
}

pub fn push_once_motion(motion_index: i32, blend_time: Option<f32>) -> Result<(), &'static str> {
    let blend = blend_time.unwrap_or(0.1);
    let mgr = get_chr_manager().lock().unwrap();
    if let Some(inst_mutex) = mgr.get_selected_instance_ptr() {
        let mut inst = inst_mutex.lock().unwrap();
        inst.push_once_motion(motion_index as u16, blend);
    }
    Ok(())
}

pub fn set_direction(rotation: f32) -> Result<(), &'static str> {
    let mgr = get_chr_manager().lock().unwrap();
    if let Some(inst_mutex) = mgr.get_selected_instance_ptr() {
        let mut inst = inst_mutex.lock().unwrap();
        inst.set_rotation(rotation);
    }
    Ok(())
}

pub fn get_pixel_position(virtual_id: i32) -> Result<(f32, f32, f32), &'static str> {
    let mgr = get_chr_manager().lock().unwrap();
    if let Some(inst_mutex) = mgr.get_instance_ptr(virtual_id as u32) {
        let inst = inst_mutex.lock().unwrap();
        let mut pos = TPixelPosition::default();
        inst.new_get_pixel_position(&mut pos);
        Ok((pos.x, pos.y, pos.z))
    } else {
        Err("Instance not found")
    }
}

pub fn set_armor(form: i32) -> Result<(), &'static str> {
    let mgr = get_chr_manager().lock().unwrap();
    if let Some(inst_mutex) = mgr.get_selected_instance_ptr() {
        let mut inst = inst_mutex.lock().unwrap();
        inst.set_armor(form as u32);
        inst.register_bounding_sphere();
    }
    Ok(())
}

pub fn set_weapon(form: i32) -> Result<(), &'static str> {
    let mgr = get_chr_manager().lock().unwrap();
    if let Some(inst_mutex) = mgr.get_selected_instance_ptr() {
        let mut inst = inst_mutex.lock().unwrap();
        inst.set_weapon(form as u32);
    }
    Ok(())
}

pub fn set_hair(race: i32) -> Result<(), &'static str> {
    let mgr = get_chr_manager().lock().unwrap();
    if let Some(inst_mutex) = mgr.get_selected_instance_ptr() {
        let mut inst = inst_mutex.lock().unwrap();
        inst.set_hair(race as u32);
    }
    Ok(())
}
