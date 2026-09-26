use std::collections::HashMap;

/// Reprezentuje dane pakietu `HEADER_GC_CHARACTER_MOVE` (0x020B)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CharacterMoveData {
    pub vid: u32,
    pub func: u8,
    pub arg: u8,
    pub rot: u8,
    pub x: i32,
    pub y: i32,
    pub time: u32,
    pub duration: u32,
}

/// Reprezentuje pojedynczy wpis z pakietu `HEADER_GC_SYNC_POSITION`
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SyncPositionData {
    pub vid: u32,
    pub x: i32,
    pub y: i32,
}

/// Przechowuje aktualny stan przemieszczania sie postaci i interpolatora
#[derive(Debug, Clone, PartialEq)]
pub struct CharacterMoveState {
    pub vid: u32,
    pub current_x: i32,
    pub current_y: i32,
    pub dst_x: i32,
    pub dst_y: i32,
    pub rot: f32,
    pub func: u8,
    pub arg: u8,
    pub start_time: u32,
    pub duration: u32,
}

impl CharacterMoveState {
    pub fn new(vid: u32, start_x: i32, start_y: i32) -> Self {
        Self {
            vid,
            current_x: start_x,
            current_y: start_y,
            dst_x: start_x,
            dst_y: start_y,
            rot: 0.0,
            func: 0,
            arg: 0,
            start_time: 0,
            duration: 0,
        }
    }

    pub fn set_target(&mut self, data: &CharacterMoveData) {
        // C++: kNetMoveActorData.m_fRot=kMovePacket.bRot*5.0f;
        self.rot = data.rot as f32 * 5.0;
        self.dst_x = data.x;
        self.dst_y = data.y;
        self.func = data.func;
        self.arg = data.arg;
        self.start_time = data.time;
        self.duration = data.duration;
    }

    pub fn sync_position(&mut self, data: &SyncPositionData) {
        self.current_x = data.x;
        self.current_y = data.y;
        // Na wypadek twardej synchronizacji, anulujemy tez biezacy cel ruchu,
        // zeby uniknac skakania postaci jesli synchronizacja przyszla pozniej.
        self.dst_x = data.x;
        self.dst_y = data.y;
    }
}

/// Zarzadca ruchu dla wszystkich instancji w grze, rejestruje i aktualizuje interpolatory
pub struct MovementManager {
    pub main_vid: Option<u32>,
    pub actors: HashMap<u32, CharacterMoveState>,
}

impl Default for MovementManager {
    fn default() -> Self {
        Self::new()
    }
}

impl MovementManager {
    pub fn new() -> Self {
        Self {
            main_vid: None,
            actors: HashMap::new(),
        }
    }

    pub fn set_main_actor_vid(&mut self, vid: u32) {
        self.main_vid = Some(vid);
    }

    pub fn add_actor(&mut self, vid: u32, x: i32, y: i32) {
        self.actors.insert(vid, CharacterMoveState::new(vid, x, y));
    }

    pub fn remove_actor(&mut self, vid: u32) {
        self.actors.remove(&vid);
    }

    /// Obsluguje pakiet `HEADER_GC_CHARACTER_MOVE`
    pub fn handle_move_packet(&mut self, data: &CharacterMoveData) -> Result<(), &'static str> {
        if let Some(state) = self.actors.get_mut(&data.vid) {
            state.set_target(data);
            Ok(())
        } else {
            Err("Actor not found for move packet")
        }
    }

    /// Obsluguje pakiet `HEADER_GC_SYNC_POSITION`
    pub fn handle_sync_position(&mut self, data: &SyncPositionData) -> Result<(), &'static str> {
        if let Some(state) = self.actors.get_mut(&data.vid) {
            state.sync_position(data);
            Ok(())
        } else {
            Err("Actor not found for sync position")
        }
    }
}
