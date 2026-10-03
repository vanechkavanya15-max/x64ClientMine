use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::RwLock;
use m2_client_bridge::net::send_queue::enqueue_outgoing_packet;
use m2_client_bridge::ffi_guard::safe_ffi_call;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FishingState {
    Idle,
    Casting,
    Waiting,
    Catching,
    Done,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BaitType {
    None,
    Paste, // Papka (vnum 27800)
    Worm,  // Robak (vnum 27801)
}

pub const VNUM_BAIT_PASTE: u32 = 27800;
pub const VNUM_BAIT_WORM: u32 = 27801;
pub const BASE_ROD_VNUM: u32 = 27400;
pub const MAX_ROD_LEVEL: u8 = 20;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FishingRodState {
    pub vnum: u32,
    pub level: u8,
    pub current_points: u32,
    pub max_points: u32,
    pub bait: BaitType,
}

impl FishingRodState {
    pub fn new(level: u8) -> Self {
        let lvl = level.clamp(1, MAX_ROD_LEVEL);
        Self {
            vnum: BASE_ROD_VNUM + (lvl as u32) * 10,
            level: lvl,
            current_points: 0,
            max_points: Self::required_points_for_level(lvl),
            bait: BaitType::None,
        }
    }

    pub fn new_default() -> Self {
        Self::new(1)
    }

    /// Oblicza wymagana liczbe punktow dla danego poziomu wedki
    pub fn required_points_for_level(level: u8) -> u32 {
        match level {
            1 => 10,
            2 => 20,
            3 => 40,
            4 => 80,
            5 => 120,
            6 => 160,
            7 => 200,
            8 => 240,
            9 => 300,
            _ => (level as u32) * 40,
        }
    }

    /// Zaklada przynete (Papka lub Robak)
    pub fn attach_bait(&mut self, bait: BaitType) -> Result<(), &'static str> {
        if bait == BaitType::None {
            return Err("Nie mozna zalozyc pustej przynety");
        }
        self.bait = bait;
        Ok(())
    }

    /// Zaklada przynete wg VNUM (27800 = Papka, 27801 = Robak)
    pub fn attach_bait_by_vnum(&mut self, vnum: u32) -> Result<BaitType, &'static str> {
        let bait = match vnum {
            VNUM_BAIT_PASTE => BaitType::Paste,
            VNUM_BAIT_WORM => BaitType::Worm,
            _ => return Err("Nieznany vnum przynety"),
        };
        self.bait = bait;
        Ok(bait)
    }

    /// Zuzywa (sciaga) przynete z haczyka
    pub fn consume_bait(&mut self) -> Option<BaitType> {
        if self.bait != BaitType::None {
            let used = self.bait;
            self.bait = BaitType::None;
            Some(used)
        } else {
            None
        }
    }

    pub fn has_bait(&self) -> bool {
        self.bait != BaitType::None
    }

    /// Naliczanie punktow wedki. Zwraca true jesli osiagnieto lub przekroczono max_points
    pub fn add_points(&mut self, points: u32) -> bool {
        self.current_points = (self.current_points + points).min(self.max_points);
        self.current_points >= self.max_points
    }

    /// Sprawdza czy wedka moze awansowac na wyzszy poziom
    pub fn can_upgrade(&self) -> bool {
        self.current_points >= self.max_points && self.level < MAX_ROD_LEVEL
    }

    /// Awans poziomu wedki
    pub fn upgrade_level(&mut self) -> Result<u8, &'static str> {
        if !self.can_upgrade() {
            if self.level >= MAX_ROD_LEVEL {
                return Err("Wedka osiagnela maksymalny poziom");
            } else {
                return Err("Niewystarczajaca liczba punktow do awansu wedki");
            }
        }

        self.level += 1;
        self.vnum = BASE_ROD_VNUM + (self.level as u32) * 10;
        self.current_points = 0;
        self.max_points = Self::required_points_for_level(self.level);
        Ok(self.level)
    }
}

pub static FISHING_STATE: RwLock<FishingState> = RwLock::new(FishingState::Idle);
pub static HIT_WINDOW_MS: AtomicU32 = AtomicU32::new(0);
pub static FISHING_ROD: RwLock<FishingRodState> = RwLock::new(FishingRodState {
    vnum: BASE_ROD_VNUM + 10,
    level: 1,
    current_points: 0,
    max_points: 10,
    bait: BaitType::None,
});

pub const HEADER_CG_FISHING: u16 = 0x0B01;

#[repr(C, packed)]
pub struct TPacketCGFishing {
    pub header: u16,
    pub length: u16,
    pub dir: u8,
}

impl TPacketCGFishing {
    pub const SIZE: u16 = std::mem::size_of::<Self>() as u16;
    
    pub fn new(dir: u8) -> Self {
        Self {
            header: HEADER_CG_FISHING,
            length: Self::SIZE,
            dir,
        }
    }
    
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(Self::SIZE as usize);
        buf.extend_from_slice(&self.header.to_le_bytes());
        buf.extend_from_slice(&self.length.to_le_bytes());
        buf.push(self.dir);
        buf
    }
}

pub const FISHING_SUB_HEADER_START: u8 = 0;
pub const FISHING_SUB_HEADER_HIT: u8 = 1;
pub const FISHING_SUB_HEADER_SUCCESS: u8 = 2;
pub const FISHING_SUB_HEADER_FAIL: u8 = 3;

#[repr(C, packed)]
pub struct TPacketGCFishing {
    pub header: u16,
    pub length: u16,
    pub sub_header: u8,
    pub info: u32,
    pub dir: u8,
}

#[no_mangle]
pub extern "C" fn m2_fishing_get_hit_window() -> u32 {
    HIT_WINDOW_MS.load(Ordering::Relaxed)
}

#[no_mangle]
pub extern "C" fn m2_fishing_attach_bait(vnum: u32) -> bool {
    safe_ffi_call(false, || {
        if let Ok(mut rod) = FISHING_ROD.write() {
            rod.attach_bait_by_vnum(vnum).is_ok()
        } else {
            false
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_fishing_has_bait() -> bool {
    safe_ffi_call(false, || {
        if let Ok(rod) = FISHING_ROD.read() {
            rod.has_bait()
        } else {
            false
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_fishing_get_rod_points() -> u32 {
    safe_ffi_call(0, || {
        if let Ok(rod) = FISHING_ROD.read() {
            rod.current_points
        } else {
            0
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_fishing_get_rod_max_points() -> u32 {
    safe_ffi_call(0, || {
        if let Ok(rod) = FISHING_ROD.read() {
            rod.max_points
        } else {
            0
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_fishing_get_rod_level() -> u8 {
    safe_ffi_call(0, || {
        if let Ok(rod) = FISHING_ROD.read() {
            rod.level
        } else {
            0
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_fishing_upgrade_rod() -> bool {
    safe_ffi_call(false, || {
        if let Ok(mut rod) = FISHING_ROD.write() {
            rod.upgrade_level().is_ok()
        } else {
            false
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_fishing_send_cast(dir: u8) -> bool {
    safe_ffi_call(false, || {
        if let Ok(mut state) = FISHING_STATE.write() {
            *state = FishingState::Casting;
        }
        let packet = TPacketCGFishing::new(dir);
        enqueue_outgoing_packet(packet.to_bytes())
    })
}

pub fn handle_fishing_packet(packet: &TPacketGCFishing) {
    if let Ok(mut state) = FISHING_STATE.write() {
        match packet.sub_header {
            FISHING_SUB_HEADER_START => {
                *state = FishingState::Waiting;
            },
            FISHING_SUB_HEADER_HIT => {
                *state = FishingState::Catching;
                HIT_WINDOW_MS.store(packet.info, Ordering::Relaxed);
            },
            FISHING_SUB_HEADER_SUCCESS => {
                *state = FishingState::Done;
                HIT_WINDOW_MS.store(0, Ordering::Relaxed);
                // Naliczanie punktow wedki i zuzycie przynety przy udanym polowie
                if let Ok(mut rod) = FISHING_ROD.write() {
                    rod.add_points(1);
                    rod.consume_bait();
                }
            },
            FISHING_SUB_HEADER_FAIL => {
                *state = FishingState::Done;
                HIT_WINDOW_MS.store(0, Ordering::Relaxed);
                // Zuzycie przynety przy nieudanym polowie
                if let Ok(mut rod) = FISHING_ROD.write() {
                    rod.consume_bait();
                }
            },
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bait_attaching_and_consuming() {
        let mut rod = FishingRodState::new(1);
        assert!(!rod.has_bait());
        assert_eq!(rod.bait, BaitType::None);

        // Zakladanie robaka
        assert!(rod.attach_bait_by_vnum(VNUM_BAIT_WORM).is_ok());
        assert!(rod.has_bait());
        assert_eq!(rod.bait, BaitType::Worm);

        // Zuzycie robaka
        assert_eq!(rod.consume_bait(), Some(BaitType::Worm));
        assert!(!rod.has_bait());

        // Zakladanie papki
        assert!(rod.attach_bait_by_vnum(VNUM_BAIT_PASTE).is_ok());
        assert_eq!(rod.bait, BaitType::Paste);
        assert_eq!(rod.consume_bait(), Some(BaitType::Paste));
        assert!(!rod.has_bait());

        // Nieprawidlowy vnum
        assert!(rod.attach_bait_by_vnum(99999).is_err());
    }

    #[test]
    fn test_rod_points_and_upgrade() {
        let mut rod = FishingRodState::new(1);
        assert_eq!(rod.level, 1);
        assert_eq!(rod.current_points, 0);
        assert_eq!(rod.max_points, 10);
        assert!(!rod.can_upgrade());

        // Dodanie 5 punktow
        assert!(!rod.add_points(5));
        assert_eq!(rod.current_points, 5);
        assert!(!rod.can_upgrade());

        // Dodanie kolejnych 5 punktow -> osiagniecie 10/10
        assert!(rod.add_points(5));
        assert_eq!(rod.current_points, 10);
        assert!(rod.can_upgrade());

        // Awans na poziom 2
        let new_lvl = rod.upgrade_level().unwrap();
        assert_eq!(new_lvl, 2);
        assert_eq!(rod.level, 2);
        assert_eq!(rod.current_points, 0);
        assert_eq!(rod.max_points, 20);
        assert!(!rod.can_upgrade());
    }

    #[test]
    fn test_handle_fishing_success_adds_points_and_consumes_bait() {
        {
            let mut rod = FISHING_ROD.write().unwrap();
            rod.current_points = 0;
            rod.attach_bait(BaitType::Worm).unwrap();
        }

        let packet = TPacketGCFishing {
            header: 0,
            length: 0,
            sub_header: FISHING_SUB_HEADER_SUCCESS,
            info: 0,
            dir: 0,
        };
        handle_fishing_packet(&packet);

        let rod = FISHING_ROD.read().unwrap();
        assert_eq!(rod.current_points, 1);
        assert!(!rod.has_bait());
    }
}
