use crate::dragon_soul_deck_v2::{DragonSoulRefine, DragonSoulGrade, DragonSoulClarity, StoneCategory};

pub const DS_REFINE_WINDOW_MAX_NUM: usize = 15;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum DragonSoulRefineWindowRefineType {
    Upgrade = 0,
    Improvement = 1,
    Refine = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum DragonSoulSub {
    Open = 0,
    Close = 1,
    DoUpgrade = 2,
    DoImprovement = 3,
    DoRefine = 4,
    RefineFail = 5,
    RefineFailMaxRefine = 6,
    RefineFailInvalidMaterial = 7,
    RefineFailNotEnoughMoney = 8,
    RefineFailNotEnoughMaterial = 9,
    RefineFailTooMuchMaterial = 10,
    RefineSucceed = 11,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RefineResult {
    Open,
    Succeed { window_type: u8, cell: u16 },
    Fail { reason: DragonSoulSub, window_type: u8, cell: u16 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DragonSoulRefineError {
    InvalidSlot,
    InvalidMaterial,
    MismatchedCategory,
    MismatchedGrade,
    MismatchedClarity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RefineMaterial {
    pub window_type: u8,
    pub cell: u16,
    pub vnum: u32,
    pub category: Option<StoneCategory>,
    pub grade: Option<DragonSoulGrade>,
    pub clarity: Option<DragonSoulClarity>,
}

pub struct DragonSoulRefineEngine {
    pub refine_type: DragonSoulRefineWindowRefineType,
    pub slots: [Option<RefineMaterial>; DS_REFINE_WINDOW_MAX_NUM],
}

impl DragonSoulRefineEngine {
    pub fn new(refine_type: DragonSoulRefineWindowRefineType) -> Self {
        Self {
            refine_type,
            slots: [None; DS_REFINE_WINDOW_MAX_NUM],
        }
    }

    pub fn add_material(
        &mut self,
        slot_index: usize,
        window_type: u8,
        cell: u16,
        vnum: u32,
    ) -> Result<(), DragonSoulRefineError> {
        if slot_index >= DS_REFINE_WINDOW_MAX_NUM {
            return Err(DragonSoulRefineError::InvalidSlot);
        }

        let parsed = DragonSoulRefine::parse_vnum(vnum);

        let material = match self.refine_type {
            DragonSoulRefineWindowRefineType::Upgrade | DragonSoulRefineWindowRefineType::Improvement => {
                if let Some((cat, grade, clarity, _)) = parsed {
                    // Check against the first non-empty slot
                    if let Some(first) = self.slots.iter().flatten().next() {
                        if first.category != Some(cat) {
                            return Err(DragonSoulRefineError::MismatchedCategory);
                        }
                        if first.grade != Some(grade) {
                            return Err(DragonSoulRefineError::MismatchedGrade);
                        }
                        if self.refine_type == DragonSoulRefineWindowRefineType::Improvement {
                            if first.clarity != Some(clarity) {
                                return Err(DragonSoulRefineError::MismatchedClarity);
                            }
                        }
                    }

                    RefineMaterial {
                        window_type,
                        cell,
                        vnum,
                        category: Some(cat),
                        grade: Some(grade),
                        clarity: Some(clarity),
                    }
                } else {
                    return Err(DragonSoulRefineError::InvalidMaterial);
                }
            }
            DragonSoulRefineWindowRefineType::Refine => {
                // For strength refine (Refine), we need a dragon soul stone in slot 0, and green/blue/pink beans in others
                if slot_index == 0 {
                    if let Some((cat, grade, clarity, _)) = parsed {
                        RefineMaterial {
                            window_type,
                            cell,
                            vnum,
                            category: Some(cat),
                            grade: Some(grade),
                            clarity: Some(clarity),
                        }
                    } else {
                        return Err(DragonSoulRefineError::InvalidMaterial);
                    }
                } else {
                    match vnum {
                        70251 | 70252 | 70253 => {
                            RefineMaterial {
                                window_type,
                                cell,
                                vnum,
                                category: None,
                                grade: None,
                                clarity: None,
                            }
                        }
                        _ => return Err(DragonSoulRefineError::InvalidMaterial),
                    }
                }
            }
        };

        self.slots[slot_index] = Some(material);
        Ok(())
    }

    pub fn remove_material(&mut self, slot_index: usize) -> Result<(), DragonSoulRefineError> {
        if slot_index >= DS_REFINE_WINDOW_MAX_NUM {
            return Err(DragonSoulRefineError::InvalidSlot);
        }
        self.slots[slot_index] = None;
        Ok(())
    }

    pub fn parse_server_response(
        sub_type: u8,
        window_type: u8,
        cell: u16,
    ) -> Option<RefineResult> {
        match sub_type {
            0 => Some(RefineResult::Open),
            5 => Some(RefineResult::Fail { reason: DragonSoulSub::RefineFail, window_type, cell }),
            6 => Some(RefineResult::Fail { reason: DragonSoulSub::RefineFailMaxRefine, window_type, cell }),
            7 => Some(RefineResult::Fail { reason: DragonSoulSub::RefineFailInvalidMaterial, window_type, cell }),
            8 => Some(RefineResult::Fail { reason: DragonSoulSub::RefineFailNotEnoughMoney, window_type, cell }),
            9 => Some(RefineResult::Fail { reason: DragonSoulSub::RefineFailNotEnoughMaterial, window_type, cell }),
            10 => Some(RefineResult::Fail { reason: DragonSoulSub::RefineFailTooMuchMaterial, window_type, cell }),
            11 => Some(RefineResult::Succeed { window_type, cell }),
            _ => None,
        }
    }
}
