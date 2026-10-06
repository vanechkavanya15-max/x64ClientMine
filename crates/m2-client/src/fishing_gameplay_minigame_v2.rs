#![allow(dead_code)]

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketCGFishing {
    pub header: u16,
    pub length: u16,
    pub dir: u8,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketGCFishing {
    pub header: u16,
    pub length: u16,
    pub subheader: u8,
    pub info: u32,
    pub dir: u8,
}

pub mod packet_headers {
    pub const CG_FISHING: u16 = 0x0B01; // From Packet.h:129 (or 0x0B10, using 0x0B01 for CG usually matching GC but we don't strictly need precise values, only parity. Let's use 0x0B10 as per newer enum)
    pub const GC_FISHING: u16 = 0x0B10; // From Packet.h:265
}

pub mod fishing_sub_gc {
    pub const START: u8 = 0;
    pub const STOP: u8 = 1;
    pub const REACT: u8 = 2;
    pub const SUCCESS: u8 = 3;
    pub const FAIL: u8 = 4;
    pub const FISH: u8 = 5;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FishingState {
    Idle,
    Throwing,
    Waiting,
    Reacting,
    Success,
    Fail,
}

#[derive(Debug)]
pub enum FishingError {
    InvalidStateTransition,
    UnknownSubheader(u8),
    UnknownFishID(u32),
    NotFishing,
}

pub struct FishingGameplayMinigame {
    pub current_state: FishingState,
    pub current_fish_id: u32,
    pub is_fish_emoticon_visible: bool,
    pub last_direction: u8,
}

impl Default for FishingGameplayMinigame {
    fn default() -> Self {
        Self::new()
    }
}

impl FishingGameplayMinigame {
    pub fn new() -> Self {
        Self {
            current_state: FishingState::Idle,
            current_fish_id: 0,
            is_fish_emoticon_visible: false,
            last_direction: 0,
        }
    }

    pub fn start_fishing(&mut self, dir: u8) -> Result<(), FishingError> {
        self.current_state = FishingState::Throwing; // In real game it quickly transitions to Wait
        self.last_direction = dir;
        self.is_fish_emoticon_visible = false;
        // Assume animation transition throws -> waits
        self.current_state = FishingState::Waiting;
        Ok(())
    }

    pub fn stop_fishing(&mut self) -> Result<(), FishingError> {
        if self.current_state != FishingState::Waiting && self.current_state != FishingState::Reacting {
            return Err(FishingError::NotFishing);
        }
        self.current_state = FishingState::Idle;
        self.is_fish_emoticon_visible = false;
        Ok(())
    }

    pub fn react_fishing(&mut self) -> Result<(), FishingError> {
        if self.current_state != FishingState::Waiting {
            return Err(FishingError::InvalidStateTransition);
        }
        self.current_state = FishingState::Reacting;
        self.is_fish_emoticon_visible = true;
        Ok(())
    }

    pub fn catch_success(&mut self) -> Result<(), FishingError> {
        if self.current_state != FishingState::Reacting {
            return Err(FishingError::InvalidStateTransition);
        }
        self.current_state = FishingState::Success;
        self.is_fish_emoticon_visible = false;
        Ok(())
    }

    pub fn catch_fail(&mut self) -> Result<(), FishingError> {
        if self.current_state != FishingState::Reacting {
            return Err(FishingError::InvalidStateTransition);
        }
        self.current_state = FishingState::Fail;
        self.is_fish_emoticon_visible = false;
        Ok(())
    }

    pub fn handle_gc_fishing_packet(&mut self, packet: &TPacketGCFishing) -> Result<Option<String>, FishingError> {
        match packet.subheader {
            fishing_sub_gc::START => {
                self.start_fishing(packet.dir)?;
                Ok(None)
            }
            fishing_sub_gc::STOP => {
                let _ = self.stop_fishing(); // C++ ignores error if not fishing
                Ok(None)
            }
            fishing_sub_gc::REACT => {
                let _ = self.react_fishing();
                Ok(None)
            }
            fishing_sub_gc::SUCCESS => {
                self.catch_success()?;
                Ok(None)
            }
            fishing_sub_gc::FAIL => {
                self.catch_fail()?;
                Ok(Some("OnFishingFailure".to_string()))
            }
            fishing_sub_gc::FISH => {
                let info = packet.info;
                self.current_fish_id = info;
                if self.current_fish_id == 0 {
                    return Ok(Some("OnFishingNotifyUnknown".to_string()));
                }
                
                // Emulate item type check logic
                let is_fishing = self.current_state == FishingState::Waiting || self.current_state == FishingState::Reacting;
                if is_fishing {
                    Ok(Some(format!("OnFishingNotify({}, {})", info, "FISH")))
                } else {
                    Ok(Some(format!("OnFishingSuccess({}, {})", info, "FISH")))
                }
            }
            _ => Err(FishingError::UnknownSubheader(packet.subheader)),
        }
    }

    pub fn create_cg_fishing_packet(dir: u8) -> TPacketCGFishing {
        TPacketCGFishing {
            header: packet_headers::CG_FISHING,
            length: std::mem::size_of::<TPacketCGFishing>() as u16,
            dir,
        }
    }
}
