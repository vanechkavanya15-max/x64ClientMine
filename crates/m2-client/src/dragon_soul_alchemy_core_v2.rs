#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum DragonSoulDeckType {
    DS_DECK_1 = 0,
    DS_DECK_2 = 1,
}

pub const DS_DECK_MAX_NUM: u8 = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum DragonSoulGradeTypes {
    Normal = 0,
    Brilliant = 1,
    Rare = 2,
    Ancient = 3,
    Legendary = 4,
}

pub const DRAGON_SOUL_GRADE_MAX: u8 = 5;

impl DragonSoulGradeTypes {
    pub fn next(&self) -> Option<Self> {
        match self {
            DragonSoulGradeTypes::Normal => Some(DragonSoulGradeTypes::Brilliant),
            DragonSoulGradeTypes::Brilliant => Some(DragonSoulGradeTypes::Rare),
            DragonSoulGradeTypes::Rare => Some(DragonSoulGradeTypes::Ancient),
            DragonSoulGradeTypes::Ancient => Some(DragonSoulGradeTypes::Legendary),
            DragonSoulGradeTypes::Legendary => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum DragonSoulStepTypes {
    Lowest = 0,
    Low = 1,
    Mid = 2,
    High = 3,
    Highest = 4,
}

pub const DRAGON_SOUL_STEP_MAX: u8 = 5;

impl DragonSoulStepTypes {
    pub fn next(&self) -> Option<Self> {
        match self {
            DragonSoulStepTypes::Lowest => Some(DragonSoulStepTypes::Low),
            DragonSoulStepTypes::Low => Some(DragonSoulStepTypes::Mid),
            DragonSoulStepTypes::Mid => Some(DragonSoulStepTypes::High),
            DragonSoulStepTypes::High => Some(DragonSoulStepTypes::Highest),
            DragonSoulStepTypes::Highest => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum DragonSoulSubType {
    Diamond = 0,
    Ruby = 1,
    Jade = 2,
    Sapphire = 3,
    Garnet = 4,
    Onyx = 5,
}

pub const DS_SLOT_NUM_TYPES: u8 = 6;
pub const DS_MAX_REFINE_LEVEL: u8 = 6;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DragonSoulVnum {
    pub category: DragonSoulSubType,
    pub grade: DragonSoulGradeTypes,
    pub step: DragonSoulStepTypes,
    pub level: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DragonSoulRefineError {
    InvalidVnum,
    MaxGradeReached,
    MaxStepReached,
    MaxLevelReached,
    RefineFailed,
}

impl DragonSoulVnum {
    pub fn parse(vnum: u32) -> Result<Self, DragonSoulRefineError> {
        if !(110000..=170000).contains(&vnum) {
            return Err(DragonSoulRefineError::InvalidVnum);
        }

        let rem = vnum - 100000;
        let cat_idx = rem / 10000;
        let grade_idx = (rem % 10000) / 1000;
        let step_idx = (rem % 1000) / 100;
        let level = ((rem % 100) / 10) as u8;

        let category = match cat_idx {
            1 => DragonSoulSubType::Diamond,
            2 => DragonSoulSubType::Ruby,
            3 => DragonSoulSubType::Jade,
            4 => DragonSoulSubType::Sapphire,
            5 => DragonSoulSubType::Garnet,
            6 => DragonSoulSubType::Onyx,
            _ => return Err(DragonSoulRefineError::InvalidVnum),
        };

        let grade = match grade_idx {
            0 => DragonSoulGradeTypes::Normal,
            1 => DragonSoulGradeTypes::Brilliant,
            2 => DragonSoulGradeTypes::Rare,
            3 => DragonSoulGradeTypes::Ancient,
            4 => DragonSoulGradeTypes::Legendary,
            _ => return Err(DragonSoulRefineError::InvalidVnum),
        };

        let step = match step_idx {
            0 => DragonSoulStepTypes::Lowest,
            1 => DragonSoulStepTypes::Low,
            2 => DragonSoulStepTypes::Mid,
            3 => DragonSoulStepTypes::High,
            4 => DragonSoulStepTypes::Highest,
            _ => return Err(DragonSoulRefineError::InvalidVnum),
        };

        if level > DS_MAX_REFINE_LEVEL {
            return Err(DragonSoulRefineError::InvalidVnum);
        }

        Ok(Self {
            category,
            grade,
            step,
            level,
        })
    }

    pub fn to_vnum(&self) -> u32 {
        let cat_val = match self.category {
            DragonSoulSubType::Diamond => 1,
            DragonSoulSubType::Ruby => 2,
            DragonSoulSubType::Jade => 3,
            DragonSoulSubType::Sapphire => 4,
            DragonSoulSubType::Garnet => 5,
            DragonSoulSubType::Onyx => 6,
        };

        100000
            + (cat_val * 10000)
            + ((self.grade as u32) * 1000)
            + ((self.step as u32) * 100)
            + ((self.level as u32) * 10)
    }

    pub fn upgrade_grade(&self, success: bool) -> Result<Self, DragonSoulRefineError> {
        let next_grade = self.grade.next().ok_or(DragonSoulRefineError::MaxGradeReached)?;
        
        if success {
            Ok(Self {
                category: self.category,
                grade: next_grade,
                step: DragonSoulStepTypes::Lowest,
                level: 0,
            })
        } else {
            Err(DragonSoulRefineError::RefineFailed)
        }
    }

    pub fn improve_step(&self, success: bool) -> Result<Self, DragonSoulRefineError> {
        let next_step = self.step.next().ok_or(DragonSoulRefineError::MaxStepReached)?;
        
        if success {
            Ok(Self {
                category: self.category,
                grade: self.grade,
                step: next_step,
                level: 0,
            })
        } else {
            Err(DragonSoulRefineError::RefineFailed)
        }
    }

    pub fn refine_level(&self, success: bool) -> Result<Self, DragonSoulRefineError> {
        if self.level >= DS_MAX_REFINE_LEVEL {
            return Err(DragonSoulRefineError::MaxLevelReached);
        }

        if success {
            Ok(Self {
                category: self.category,
                grade: self.grade,
                step: self.step,
                level: self.level + 1,
            })
        } else {
            Err(DragonSoulRefineError::RefineFailed)
        }
    }
}
