#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TargetRank {
    Normal,
    Cursed,
    Commander,
    Boss,
}

impl TargetRank {
    pub fn as_str(&self) -> &'static str {
        match self {
            TargetRank::Normal => "Zwykly",
            TargetRank::Cursed => "Przeklety",
            TargetRank::Commander => "Dowodca",
            TargetRank::Boss => "Boss",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElementType {
    Fire,
    Ice,
    Lightning,
    Wind,
    Dark,
    Earth,
}

impl ElementType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ElementType::Fire => "Ogien",
            ElementType::Ice => "Lod",
            ElementType::Lightning => "Blyskawica",
            ElementType::Wind => "Wiatr",
            ElementType::Dark => "Mrok",
            ElementType::Earth => "Ziemia",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TargetInfo {
    pub vid: u32,
    pub name: String,
    pub level: u32,
    pub rank: TargetRank,
    pub hp_percent: f32, // 0.0 to 100.0
    pub element: Option<ElementType>,
}

#[derive(Debug, Default)]
pub struct TargetBoardInfoController {
    current_target: Option<TargetInfo>,
}

impl TargetBoardInfoController {
    pub fn new() -> Self {
        Self {
            current_target: None,
        }
    }

    pub fn set_target(&mut self, vid: u32, name: String, level: u32, rank: TargetRank, element: Option<ElementType>) {
        self.current_target = Some(TargetInfo {
            vid,
            name,
            level,
            rank,
            hp_percent: 100.0,
            element,
        });
    }

    pub fn clear_target(&mut self) {
        self.current_target = None;
    }

    pub fn update_from_packet(&mut self, vid: u32, hp_percent_u8: u8) {
        if let Some(target) = &mut self.current_target {
            if target.vid == vid {
                target.hp_percent = hp_percent_u8.clamp(0, 100) as f32;
            }
        }
    }

    pub fn get_formatted_name(&self) -> Option<String> {
        self.current_target.as_ref().map(|target| {
            format!("Lv.{} {} [{}]", target.level, target.name, target.rank.as_str())
        })
    }

    pub fn get_hp_percent(&self) -> Option<f32> {
        self.current_target.as_ref().map(|target| target.hp_percent)
    }

    pub fn get_element_icon_name(&self) -> Option<&'static str> {
        self.current_target.as_ref().and_then(|target| target.element.map(|e| e.as_str()))
    }
    
    pub fn get_current_vid(&self) -> Option<u32> {
        self.current_target.as_ref().map(|t| t.vid)
    }
}
