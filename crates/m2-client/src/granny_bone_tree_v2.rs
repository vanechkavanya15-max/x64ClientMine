use std::ffi::CStr;
use std::os::raw::c_char;
use std::sync::RwLock;
use std::collections::HashMap;

// 4x4 Matrix (Row-major matching D3DXMATRIX for DirectX 8/9 in Metin2)
pub type Matrix4x4 = [f32; 16];

pub fn identity_matrix() -> Matrix4x4 {
    [
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
    ]
}

// Row-major matrix multiplication: out = a * b (DirectX order)
// In Metin2 (DirectX 8): D3DXMatrixMultiply(&matWorld, &matLocal, &matParent)
// which means matWorld = matLocal * matParent in D3DX math notation
pub fn multiply_matrix(a: &Matrix4x4, b: &Matrix4x4) -> Matrix4x4 {
    let mut out = [0.0; 16];
    for row in 0..4 {
        for col in 0..4 {
            out[row * 4 + col] = 
                a[row * 4 + 0] * b[0 * 4 + col] +
                a[row * 4 + 1] * b[1 * 4 + col] +
                a[row * 4 + 2] * b[2 * 4 + col] +
                a[row * 4 + 3] * b[3 * 4 + col];
        }
    }
    out
}

#[derive(Debug, Clone)]
pub struct Bone {
    pub id: i32,
    pub name: String,
    pub parent_id: i32,
    pub local_matrix: Matrix4x4,
    pub global_matrix: Matrix4x4,
}

impl Bone {
    pub fn new(id: i32, name: &str, parent_id: i32, local_matrix: Matrix4x4) -> Self {
        Self {
            id,
            name: name.to_string(),
            parent_id,
            local_matrix,
            global_matrix: identity_matrix(), // will be calculated in update_skeleton
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct BoneTree {
    pub bones: Vec<Bone>,
}

impl BoneTree {
    pub fn new() -> Self {
        Self {
            bones: Vec::new(),
        }
    }

    pub fn add_bone(&mut self, bone: Bone) {
        self.bones.push(bone);
    }

    pub fn find_bone_by_name(&self, name: &str) -> Option<&Bone> {
        self.bones.iter().find(|b| b.name == name)
    }

    pub fn find_bone_index_by_id(&self, id: i32) -> Option<usize> {
        self.bones.iter().position(|b| b.id == id)
    }

    // Updates all global matrices based on local_matrix * parent's global_matrix
    pub fn update_skeleton(&mut self) {
        // Iterate through all bones. This assumes bones are ordered such that parents come before children.
        // If not guaranteed, we would need to build a topological sort, but in Granny formats, 
        // they usually are strictly ordered (parent < child). We will support arbitrary order by
        // calculating iteratively.
        
        let mut calculated_globals: HashMap<i32, Matrix4x4> = HashMap::new();
        let mut pending_bones: Vec<usize> = (0..self.bones.len()).collect();
        let mut progress = true;

        while !pending_bones.is_empty() && progress {
            progress = false;
            let mut next_pending = Vec::new();

            for &idx in &pending_bones {
                let bone = &self.bones[idx];
                
                if bone.parent_id == -1 {
                    // Root bone
                    let global = bone.local_matrix;
                    calculated_globals.insert(bone.id, global);
                    self.bones[idx].global_matrix = global;
                    progress = true;
                } else if let Some(parent_global) = calculated_globals.get(&bone.parent_id) {
                    // We have parent's global matrix
                    // Direct3D: matWorld = matLocal * matParent
                    let global = multiply_matrix(&bone.local_matrix, parent_global);
                    calculated_globals.insert(bone.id, global);
                    self.bones[idx].global_matrix = global;
                    progress = true;
                } else {
                    next_pending.push(idx);
                }
            }
            pending_bones = next_pending;
        }
        
        // If there's a cycle or missing parent, pending_bones might not be empty,
        // but we've done our best.
    }
}

use std::sync::OnceLock;

pub static GLOBAL_BONE_TREE: OnceLock<RwLock<BoneTree>> = OnceLock::new();

pub fn get_global_bone_tree() -> &'static RwLock<BoneTree> {
    GLOBAL_BONE_TREE.get_or_init(|| RwLock::new(BoneTree::new()))
}

// FFI Export
#[no_mangle]
pub extern "C" fn m2_bone_get_global_matrix(bone_name: *const c_char, out_mat: *mut f32) -> bool {
    if bone_name.is_null() || out_mat.is_null() {
        return false;
    }

    let name_str = match unsafe { CStr::from_ptr(bone_name) }.to_str() {
        Ok(s) => s,
        Err(_) => return false,
    };

    let tree_lock = match get_global_bone_tree().read() {
        Ok(lock) => lock,
        Err(_) => return false,
    };

    if let Some(bone) = tree_lock.find_bone_by_name(name_str) {
        unsafe {
            for i in 0..16 {
                *out_mat.add(i) = bone.global_matrix[i];
            }
        }
        return true;
    }

    false
}

#[derive(Debug, Default)]
pub struct ActorBoneManager {
    pub actors: HashMap<u32, BoneTree>,
}

pub static GLOBAL_ACTOR_BONE_MANAGER: OnceLock<RwLock<ActorBoneManager>> = OnceLock::new();

pub fn get_actor_bone_manager() -> &'static RwLock<ActorBoneManager> {
    GLOBAL_ACTOR_BONE_MANAGER.get_or_init(|| RwLock::new(ActorBoneManager::default()))
}

#[no_mangle]
pub extern "C" fn m2_actor_bone_remove(vid: u32) {
    if let Ok(mut mgr) = get_actor_bone_manager().write() {
        mgr.actors.remove(&vid);
    }
}

#[no_mangle]
pub extern "C" fn m2_actor_bone_get_global_matrix(vid: u32, bone_name: *const c_char, out_mat: *mut f32) -> bool {
    if bone_name.is_null() || out_mat.is_null() {
        return false;
    }
    let name_str = match unsafe { CStr::from_ptr(bone_name) }.to_str() {
        Ok(s) => s,
        Err(_) => return false,
    };
    if let Ok(mgr) = get_actor_bone_manager().read() {
        if let Some(tree) = mgr.actors.get(&vid) {
            if let Some(bone) = tree.find_bone_by_name(name_str) {
                unsafe {
                    for i in 0..16 {
                        *out_mat.add(i) = bone.global_matrix[i];
                    }
                }
                return true;
            }
        }
    }
    m2_bone_get_global_matrix(bone_name, out_mat)
}
