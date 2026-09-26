use std::collections::HashMap;

pub const CHAR_STAGE_VIEW_BOUND: f32 = 20000.0;
pub const FADE_OUT_DURATION_MS: u64 = 500;

#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ActorInstance {
    pub id: u32,
    pub position: Position,
    pub is_fading_out: bool,
    pub fade_out_start_time: Option<u64>,
}

impl ActorInstance {
    pub fn new(id: u32, x: f32, y: f32) -> Self {
        Self {
            id,
            position: Position { x, y },
            is_fading_out: false,
            fade_out_start_time: None,
        }
    }
}

pub struct PythonCharacterManager {
    pub alive_instances: HashMap<u32, ActorInstance>,
}

impl PythonCharacterManager {
    pub fn new() -> Self {
        Self {
            alive_instances: HashMap::new(),
        }
    }

    pub fn add_actor(&mut self, actor: ActorInstance) {
        self.alive_instances.insert(actor.id, actor);
    }

    pub fn update(&mut self, main_pos: &Position, current_time_ms: u64) {
        let mut to_remove = Vec::new();

        for (&id, actor) in self.alive_instances.iter_mut() {
            let dx = actor.position.x - main_pos.x;
            let dy = actor.position.y - main_pos.y;
            let dist = (dx * dx + dy * dy).sqrt();

            if dist > CHAR_STAGE_VIEW_BOUND + 10.0 {
                if !actor.is_fading_out {
                    actor.is_fading_out = true;
                    actor.fade_out_start_time = Some(current_time_ms);
                } else if let Some(start_time) = actor.fade_out_start_time {
                    if current_time_ms >= start_time + FADE_OUT_DURATION_MS {
                        to_remove.push(id);
                    }
                }
            } else {
                if actor.is_fading_out {
                    actor.is_fading_out = false;
                    actor.fade_out_start_time = None;
                }
            }
        }

        for id in to_remove {
            self.alive_instances.remove(&id);
        }
    }
}

impl Default for PythonCharacterManager {
    fn default() -> Self {
        Self::new()
    }
}
