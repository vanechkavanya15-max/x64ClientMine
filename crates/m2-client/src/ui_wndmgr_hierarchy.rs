use std::collections::HashMap;

#[allow(dead_code)]
pub const FLAG_MOVABLE: u32 = 1 << 0;
#[allow(dead_code)]
pub const FLAG_LIMIT: u32 = 1 << 1;
#[allow(dead_code)]
pub const FLAG_SNAP: u32 = 1 << 2;
#[allow(dead_code)]
pub const FLAG_DRAGABLE: u32 = 1 << 3;
#[allow(dead_code)]
pub const FLAG_ATTACH: u32 = 1 << 4;
#[allow(dead_code)]
pub const FLAG_RESTRICT_X: u32 = 1 << 5;
#[allow(dead_code)]
pub const FLAG_RESTRICT_Y: u32 = 1 << 6;
#[allow(dead_code)]
pub const FLAG_NOT_CAPTURE: u32 = 1 << 7;
pub const FLAG_FLOAT: u32 = 1 << 8;
#[allow(dead_code)]
pub const FLAG_NOT_PICK: u32 = 1 << 9;
#[allow(dead_code)]
pub const FLAG_IGNORE_SIZE: u32 = 1 << 10;
#[allow(dead_code)]
pub const FLAG_RTL: u32 = 1 << 11;

#[derive(Debug, Clone)]
pub struct WindowNode {
    #[allow(dead_code)]
    pub id: u32,
    pub parent_id: Option<u32>,
    pub children: Vec<u32>,
    pub flags: u32,
    pub is_show: bool,
    pub is_rendering: bool,
}

#[derive(Debug, Default)]
pub struct WndMgrHierarchy {
    pub windows: HashMap<u32, WindowNode>,
}

impl WindowNode {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            parent_id: None,
            children: Vec::new(),
            flags: 0,
            is_show: false,
            is_rendering: false,
        }
    }

    pub fn has_flag(&self, flag: u32) -> bool {
        (self.flags & flag) != 0
    }

    pub fn set_flag(&mut self, flag: u32, value: bool) {
        if value {
            self.flags |= flag;
        } else {
            self.flags &= !flag;
        }
    }
}

impl WndMgrHierarchy {
    pub fn new() -> Self {
        Self {
            windows: HashMap::new(),
        }
    }

    pub fn create_window(&mut self, id: u32) {
        if !self.windows.contains_key(&id) {
            self.windows.insert(id, WindowNode::new(id));
        }
    }

    #[allow(dead_code)]
    pub fn remove_window(&mut self, id: u32) {
        if let Some(parent_id) = self.windows.get(&id).and_then(|w| w.parent_id) {
            if let Some(parent) = self.windows.get_mut(&parent_id) {
                parent.children.retain(|&c| c != id);
            }
        }

        let mut children_to_remove = Vec::new();
        if let Some(window) = self.windows.get(&id) {
            children_to_remove.extend(window.children.iter().copied());
        }
        for child_id in children_to_remove {
            self.set_parent(child_id, None);
        }

        self.windows.remove(&id);
    }

    pub fn set_parent(&mut self, child_id: u32, parent_id: Option<u32>) {
        if !self.windows.contains_key(&child_id) {
            return;
        }
        if let Some(p_id) = parent_id {
            if !self.windows.contains_key(&p_id) {
                return;
            }
        }

        let old_parent_id = self.windows.get(&child_id).unwrap().parent_id;

        if old_parent_id == parent_id {
            return;
        }

        // Remove from old parent
        if let Some(old_p_id) = old_parent_id {
            if let Some(old_parent) = self.windows.get_mut(&old_p_id) {
                old_parent.children.retain(|&id| id != child_id);
            }
        }

        // Set new parent
        if let Some(window) = self.windows.get_mut(&child_id) {
            window.parent_id = parent_id;
        }

        // Add to new parent
        if let Some(p_id) = parent_id {
            if let Some(parent) = self.windows.get_mut(&p_id) {
                if !parent.children.contains(&child_id) {
                    parent.children.push(child_id);
                }
            }
        }

        // After setting parent, we should update visibility (is_rendering)
        self.update_rendering(child_id);
    }

    pub fn get_child_count(&self, id: u32) -> Option<usize> {
        self.windows.get(&id).map(|w| w.children.len())
    }

    pub fn get_child(&self, parent_id: u32, index: usize) -> Option<u32> {
        self.windows
            .get(&parent_id)
            .and_then(|w| w.children.get(index).copied())
    }

    pub fn set_top(&mut self, id: u32) {
        let (parent_id, has_float) = if let Some(window) = self.windows.get(&id) {
            (window.parent_id, window.has_flag(FLAG_FLOAT))
        } else {
            return;
        };

        if !has_float {
            return;
        }

        if let Some(p_id) = parent_id {
            if let Some(parent) = self.windows.get_mut(&p_id) {
                if let Some(pos) = parent.children.iter().position(|&c| c == id) {
                    parent.children.remove(pos);
                    parent.children.push(id);
                }
            }
        }
    }

    pub fn bring_to_front(&mut self, id: u32) {
        if !self.windows.contains_key(&id) {
            return;
        }

        let mut current_id = id;
        while let Some(parent_id) = self.windows.get(&current_id).and_then(|w| w.parent_id) {
            self.set_top(current_id);
            current_id = parent_id;
        }
    }

    pub fn show(&mut self, id: u32) {
        if let Some(window) = self.windows.get_mut(&id) {
            window.is_show = true;
        }
        self.update_rendering(id);
    }

    pub fn hide(&mut self, id: u32) {
        if let Some(window) = self.windows.get_mut(&id) {
            window.is_show = false;
        }
        self.update_rendering(id);
    }

    pub fn update_rendering(&mut self, start_id: u32) {
        // Find if this node itself is visible from its parent
        let is_rendering = self.calculate_is_rendering(start_id);
        self.propagate_rendering(start_id, is_rendering);
    }

    fn calculate_is_rendering(&self, id: u32) -> bool {
        let window = if let Some(w) = self.windows.get(&id) {
            w
        } else {
            return false;
        };

        if !window.is_show {
            return false;
        }

        if let Some(parent_id) = window.parent_id {
            self.calculate_is_rendering(parent_id)
        } else {
            true // Root and shown
        }
    }

    fn propagate_rendering(&mut self, id: u32, is_rendering: bool) {
        let children = if let Some(window) = self.windows.get_mut(&id) {
            window.is_rendering = is_rendering && window.is_show;
            window.children.clone()
        } else {
            return;
        };

        let next_rendering = self.windows.get(&id).map(|w| w.is_rendering).unwrap_or(false);

        for child_id in children {
            self.propagate_rendering(child_id, next_rendering);
        }
    }
}
