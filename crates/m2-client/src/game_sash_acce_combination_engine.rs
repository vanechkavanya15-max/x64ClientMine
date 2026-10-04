#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SashGrade {
    Simple = 0,
    Fine = 1,
    Noble = 2,
    Custom = 3,
}

impl SashGrade {
    pub fn base_absorption_rate(&self) -> u8 {
        match self {
            SashGrade::Simple => 1,
            SashGrade::Fine => 5,
            SashGrade::Noble => 10,
            SashGrade::Custom => 11, // Base for custom, can go up to 25%
        }
    }
}

pub struct SashItem {
    pub grade: SashGrade,
    pub absorption_rate: u8,
}

impl SashItem {
    pub fn new(grade: SashGrade, custom_absorption: Option<u8>) -> Self {
        let absorption_rate = match grade {
            SashGrade::Simple | SashGrade::Fine | SashGrade::Noble => grade.base_absorption_rate(),
            SashGrade::Custom => {
                let rate = custom_absorption.unwrap_or(11);
                rate.clamp(11, 25)
            }
        };

        Self {
            grade,
            absorption_rate,
        }
    }
}

pub fn calculate_absorbed_bonus(original_value: i32, absorption_rate: u8) -> i32 {
    if original_value == 0 {
        return 0;
    }
    
    // Using integer math to calculate percentage
    let mut absorbed = (original_value.saturating_abs() * (absorption_rate as i32)) / 100;
    
    // Always grant at least 1 point if there was a bonus (except when original value is 0)
    if absorbed == 0 {
        absorbed = 1;
    }
    
    if original_value < 0 {
        -absorbed
    } else {
        absorbed
    }
}

pub fn combine_sashes(sash1: &SashItem, sash2: &SashItem, success_chance: u8, roll: u8) -> Result<SashItem, &'static str> {
    if sash1.grade != sash2.grade {
        return Err("Sashes must be of the same grade to combine.");
    }
    
    if sash1.grade == SashGrade::Custom {
        return Err("Custom grade sashes cannot be combined further.");
    }

    if roll <= success_chance {
        let new_grade = match sash1.grade {
            SashGrade::Simple => SashGrade::Fine,
            SashGrade::Fine => SashGrade::Noble,
            SashGrade::Noble => SashGrade::Custom,
            SashGrade::Custom => unreachable!(),
        };
        
        let new_absorption = match new_grade {
            SashGrade::Custom => Some(11), // Default custom start at 11%
            _ => None,
        };

        Ok(SashItem::new(new_grade, new_absorption))
    } else {
        Err("Combination failed. The second sash is destroyed.")
    }
}

// Function to alter Custom grade absorption rate (e.g. using special items)
pub fn upgrade_custom_sash_absorption(sash: &mut SashItem, success: bool, increment: u8) -> Result<(), &'static str> {
    if sash.grade != SashGrade::Custom {
        return Err("Only Custom grade sashes can have their absorption rate upgraded.");
    }

    if sash.absorption_rate >= 25 {
        return Err("Sash has reached maximum absorption rate.");
    }

    if success {
        let new_rate = (sash.absorption_rate + increment).min(25);
        sash.absorption_rate = new_rate;
        Ok(())
    } else {
        Err("Upgrade failed.")
    }
}
