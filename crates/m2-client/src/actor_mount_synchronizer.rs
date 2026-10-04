// actor_mount_synchronizer.rs - Synchronizacja pozycji, rotacji i trybu animacji gracza na wierzchowcu/koniu (1:1 z ActorInstanceMount.cpp)
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

pub const MOTION_MODE_GENERAL: u32 = 1;
pub const MOTION_MODE_HORSE: u32 = 9;

pub const DEFAULT_SADDLE_HEIGHT: f32 = 110.0;

#[derive(Debug, Clone, PartialEq)]
pub struct MountState {
    pub rider_vid: u32,
    pub mount_vid: u32,
    pub mount_vnum: u32,
    pub saddle_height: f32,
    pub is_mounted: bool,
    pub original_motion_mode: u32,
}

#[derive(Default)]
pub struct ActorMountManager {
    /// Mapowanie rider_vid na stan dosiadania wierzchowca
    mount_states: HashMap<u32, MountState>,
}

static MOUNT_MGR: OnceLock<Mutex<ActorMountManager>> = OnceLock::new();

pub fn get_mount_manager() -> &'static Mutex<ActorMountManager> {
    MOUNT_MGR.get_or_init(|| Mutex::new(ActorMountManager::default()))
}

impl ActorMountManager {
    /// Dosiada wierzchowca przez gracza (MountActor)
    pub fn mount_actor(&mut self, rider_vid: u32, mount_vid: u32, mount_vnum: u32) {
        let saddle_height = match mount_vnum {
            20101..=20109 => 110.0, // Standardowy koń
            20110..=20119 => 120.0, // Bojowy / Militarny
            20120..=20200 => 130.0, // Dziki zwierz / lew / wilk / dzik
            _ => DEFAULT_SADDLE_HEIGHT,
        };

        self.mount_states.insert(
            rider_vid,
            MountState {
                rider_vid,
                mount_vid,
                mount_vnum,
                saddle_height,
                is_mounted: true,
                original_motion_mode: MOTION_MODE_GENERAL,
            },
        );
    }

    /// Zsiada z wierzchowca (UnmountActor)
    pub fn unmount_actor(&mut self, rider_vid: u32) -> Option<MountState> {
        self.mount_states.remove(&rider_vid)
    }

    /// Sprawdza czy gracz jest na koniu
    pub fn is_mounted(&self, rider_vid: u32) -> bool {
        self.mount_states.get(&rider_vid).map(|s| s.is_mounted).unwrap_or(false)
    }

    /// Pobiera biezacy tryb animacji postaci (zwraca MODE_HORSE jesli na koniu)
    pub fn get_motion_mode(&self, rider_vid: u32) -> u32 {
        if self.is_mounted(rider_vid) {
            MOTION_MODE_HORSE
        } else {
            MOTION_MODE_GENERAL
        }
    }

    /// Synchronizuje pozycje i rotacje gracza z instancja wierzchowca
    pub fn sync_rider_transform(
        &self,
        rider_vid: u32,
        mount_pos: [f32; 3],
        mount_rot_deg: f32,
        out_rider_pos: &mut [f32; 3],
        out_rider_rot_deg: &mut f32,
    ) -> bool {
        if let Some(state) = self.mount_states.get(&rider_vid) {
            if state.is_mounted {
                out_rider_pos[0] = mount_pos[0];
                out_rider_pos[1] = mount_pos[1];
                out_rider_pos[2] = mount_pos[2] + state.saddle_height;
                *out_rider_rot_deg = mount_rot_deg;
                return true;
            }
        }
        false
    }
}

// ============================================================================
// FFI C-Bridge do integracji z Pythonem (chrmgr / player)
// ============================================================================

#[no_mangle]
pub extern "C" fn m2_mount_actor(rider_vid: u32, mount_vid: u32, mount_vnum: u32) {
    if let Ok(mut mgr) = get_mount_manager().lock() {
        mgr.mount_actor(rider_vid, mount_vid, mount_vnum);
    }
}

#[no_mangle]
pub extern "C" fn m2_unmount_actor(rider_vid: u32) {
    if let Ok(mut mgr) = get_mount_manager().lock() {
        mgr.unmount_actor(rider_vid);
    }
}

#[no_mangle]
pub extern "C" fn m2_is_actor_mounted(rider_vid: u32) -> bool {
    if let Ok(mgr) = get_mount_manager().lock() {
        mgr.is_mounted(rider_vid)
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mount_and_sync_transform() {
        let mut mgr = ActorMountManager::default();

        // Gracz 1001 dosiada konia bojowego (vnum 20110) o vid 5001
        mgr.mount_actor(1001, 5001, 20110);
        assert!(mgr.is_mounted(1001));
        assert_eq!(mgr.get_motion_mode(1001), MOTION_MODE_HORSE);

        let mount_pos = [1000.0, 2000.0, 50.0];
        let mount_rot = 90.0;
        let mut rider_pos = [0.0, 0.0, 0.0];
        let mut rider_rot = 0.0;

        let synced = mgr.sync_rider_transform(1001, mount_pos, mount_rot, &mut rider_pos, &mut rider_rot);
        assert!(synced);
        assert_eq!(rider_pos[0], 1000.0);
        assert_eq!(rider_pos[1], 2000.0);
        assert_eq!(rider_pos[2], 50.0 + 120.0); // 170.0 z uwzglednieniem siodla bojowego
        assert_eq!(rider_rot, 90.0);

        // Zsiasc z konia
        let unmounted = mgr.unmount_actor(1001);
        assert!(unmounted.is_some());
        assert!(!mgr.is_mounted(1001));
        assert_eq!(mgr.get_motion_mode(1001), MOTION_MODE_GENERAL);
    }
}
