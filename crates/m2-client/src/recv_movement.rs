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
    pub src_x: i32,
    pub src_y: i32,
    pub dst_x: i32,
    pub dst_y: i32,
    pub rot: f32,
    pub func: u8,
    pub arg: u8,
    pub start_time: u32,
    pub duration: u32,
    pub client_src_time: u32,
}

impl CharacterMoveState {
    pub fn new(vid: u32, start_x: i32, start_y: i32) -> Self {
        Self {
            vid,
            current_x: start_x,
            current_y: start_y,
            src_x: start_x,
            src_y: start_y,
            dst_x: start_x,
            dst_y: start_y,
            rot: 0.0,
            func: 0,
            arg: 0,
            start_time: 0,
            duration: 0,
            client_src_time: 0,
        }
    }

    pub fn set_target(&mut self, data: &CharacterMoveData, current_client_time: u32) {
        // C++: kNetMoveActorData.m_fRot=kMovePacket.bRot*5.0f;
        self.rot = data.rot as f32 * 5.0;
        
        self.src_x = self.current_x;
        self.src_y = self.current_y;
        self.dst_x = data.x;
        self.dst_y = data.y;
        self.func = data.func;
        self.arg = data.arg;
        self.start_time = data.time;
        self.duration = data.duration;
        self.client_src_time = current_client_time;
    }

    pub fn sync_position(&mut self, data: &SyncPositionData) {
        self.current_x = data.x;
        self.current_y = data.y;
        self.src_x = data.x;
        self.src_y = data.y;
        self.dst_x = data.x;
        self.dst_y = data.y;
    }

    pub fn update_position(&mut self, current_client_time: u32) {
        let elapsed_time = current_client_time.saturating_sub(self.client_src_time);
        
        if self.duration > 0 && elapsed_time < self.duration {
            let rate = (elapsed_time as f32) / (self.duration as f32);
            self.current_x = ((self.dst_x - self.src_x) as f32 * rate) as i32 + self.src_x;
            self.current_y = ((self.dst_y - self.src_y) as f32 * rate) as i32 + self.src_y;
        } else {
            self.current_x = self.dst_x;
            self.current_y = self.dst_y;
        }
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
    pub fn handle_move_packet(&mut self, data: &CharacterMoveData, current_client_time: u32) -> Result<(), &'static str> {
        if let Some(state) = self.actors.get_mut(&data.vid) {
            state.set_target(data, current_client_time);
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

    pub fn update_positions(&mut self, current_client_time: u32) {
        for state in self.actors.values_mut() {
            state.update_position(current_client_time);
        }
    }
}

#[repr(C, packed(1))]
pub struct TPacketGCMove {
    pub header: u16,
    pub length: u16,
    pub b_func: u8,
    pub b_arg: u8,
    pub b_rot: u8,
    pub dw_vid: u32,
    pub l_x: i32,
    pub l_y: i32,
    pub dw_time: u32,
    pub dw_duration: u32,
}

pub static MOVEMENT_MANAGER: std::sync::LazyLock<std::sync::RwLock<MovementManager>> = std::sync::LazyLock::new(|| {
    std::sync::RwLock::new(MovementManager::new())
});

#[no_mangle]
pub extern "C" fn m2_net_recv_move(buf: *const u8, len: usize) -> bool {
    if buf.is_null() || len < std::mem::size_of::<TPacketGCMove>() {
        return false;
    }
    let pkt = unsafe { std::ptr::read_unaligned(buf as *const TPacketGCMove) };
    let move_data = CharacterMoveData {
        vid: { pkt.dw_vid },
        func: { pkt.b_func },
        arg: { pkt.b_arg },
        rot: { pkt.b_rot },
        x: { pkt.l_x },
        y: { pkt.l_y },
        time: { pkt.dw_time },
        duration: { pkt.dw_duration },
    };
    if let Ok(mut mgr) = MOVEMENT_MANAGER.write() {
        // MOCK: using 0 as current_client_time for testing, in real case it should be passed or fetched.
        let _ = mgr.handle_move_packet(&move_data, 0); 
    }
    true
}

// C-FFI functions
#[no_mangle]
pub extern "C" fn recv_movement_create_state() -> *mut CharacterMoveState {
    Box::into_raw(Box::new(CharacterMoveState::new(0, 0, 0)))
}

#[no_mangle]
pub extern "C" fn recv_movement_free_state(state: *mut CharacterMoveState) {
    if !state.is_null() {
        unsafe {
            let _ = Box::from_raw(state);
        }
    }
}

#[no_mangle]
pub extern "C" fn recv_movement_apply_packet(state: *mut CharacterMoveState, packet: *const TPacketGCMove, current_client_time: u32) {
    if state.is_null() || packet.is_null() {
        return;
    }
    let state = unsafe { &mut *state };
    let packet = unsafe { &*packet };
    let move_data = CharacterMoveData {
        vid: { packet.dw_vid },
        func: { packet.b_func },
        arg: { packet.b_arg },
        rot: { packet.b_rot },
        x: { packet.l_x },
        y: { packet.l_y },
        time: { packet.dw_time },
        duration: { packet.dw_duration },
    };
    state.set_target(&move_data, current_client_time);
}

#[no_mangle]
pub extern "C" fn recv_movement_update_position(state: *mut CharacterMoveState, current_client_time: u32) {
    if state.is_null() {
        return;
    }
    let state = unsafe { &mut *state };
    state.update_position(current_client_time);
}
