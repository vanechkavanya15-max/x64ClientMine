use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActiveBuff {
    pub affect_id: u32,
    pub duration_ms: u32,
    pub elapsed_ms: u32,
    pub value1: i32,
    pub value2: i32,
}

#[derive(Debug, Default)]
pub struct BuffContainer {
    pub active_buffs: HashMap<u32, ActiveBuff>,
}

impl BuffContainer {
    pub fn new() -> Self {
        Self {
            active_buffs: HashMap::new(),
        }
    }

    pub fn apply_buff(&mut self, affect_id: u32, duration_ms: u32, value1: i32, value2: i32) {
        let buff = ActiveBuff {
            affect_id,
            duration_ms,
            elapsed_ms: 0,
            value1,
            value2,
        };
        self.active_buffs.insert(affect_id, buff);
    }

    /// Aktualizuje czas trwania aktywnych buffow i zwraca te, ktorych czas trwania minal.
    /// Zwracane buffy sa usuwane z kontenera.
    pub fn update(&mut self, delta_time_ms: u32) -> Vec<ActiveBuff> {
        let mut expired_buffs = Vec::new();
        let mut expired_keys = Vec::new();

        for (affect_id, buff) in self.active_buffs.iter_mut() {
            buff.elapsed_ms = buff.elapsed_ms.saturating_add(delta_time_ms);
            if buff.elapsed_ms >= buff.duration_ms {
                expired_keys.push(*affect_id);
            }
        }

        for affect_id in expired_keys {
            if let Some(buff) = self.active_buffs.remove(&affect_id) {
                expired_buffs.push(buff);
            }
        }

        expired_buffs
    }
    
    pub fn get_buff(&self, affect_id: u32) -> Option<&ActiveBuff> {
        self.active_buffs.get(&affect_id)
    }

    pub fn clear(&mut self) {
        self.active_buffs.clear();
    }
}
