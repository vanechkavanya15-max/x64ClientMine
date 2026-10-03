use crate::ui_item_tooltip_renderer::{ItemTooltipRenderer, TooltipElement};

#[derive(Debug, Clone, Default)]
pub struct ItemTooltipBuilder {
    pub name: Option<String>,
    pub item_type: Option<u8>,
    pub item_sub_type: Option<u8>,
    pub limit_level: Option<u32>,
    pub anti_flags: Option<u32>,
    pub player_level: Option<u32>,
    pub player_job: Option<u8>,
    pub attack_min: Option<i32>,
    pub attack_max: Option<i32>,
    pub magic_attack_min: Option<i32>,
    pub magic_attack_max: Option<i32>,
    pub defense: Option<i32>,
    pub attack_speed: Option<i32>,
    pub applies: Vec<(u8, i32)>,
    pub attributes: Vec<(u8, i16)>,
    pub sockets: Vec<Option<u32>>,
}

impl ItemTooltipBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn item_type(mut self, item_type: u8, item_sub_type: u8) -> Self {
        self.item_type = Some(item_type);
        self.item_sub_type = Some(item_sub_type);
        self
    }

    pub fn requirements(
        mut self,
        limit_level: u32,
        anti_flags: u32,
        player_level: u32,
        player_job: u8,
    ) -> Self {
        self.limit_level = Some(limit_level);
        self.anti_flags = Some(anti_flags);
        self.player_level = Some(player_level);
        self.player_job = Some(player_job);
        self
    }

    pub fn battle_stats(
        mut self,
        attack_min: i32,
        attack_max: i32,
        magic_attack_min: i32,
        magic_attack_max: i32,
        defense: i32,
        attack_speed: i32,
    ) -> Self {
        self.attack_min = Some(attack_min);
        self.attack_max = Some(attack_max);
        self.magic_attack_min = Some(magic_attack_min);
        self.magic_attack_max = Some(magic_attack_max);
        self.defense = Some(defense);
        self.attack_speed = Some(attack_speed);
        self
    }

    pub fn add_apply(mut self, apply_type: u8, value: i32) -> Self {
        self.applies.push((apply_type, value));
        self
    }

    pub fn add_attribute(mut self, attr_type: u8, value: i16) -> Self {
        self.attributes.push((attr_type, value));
        self
    }

    pub fn add_socket(mut self, vnum: Option<u32>) -> Self {
        self.sockets.push(vnum);
        self
    }

    pub fn build(self) -> ItemTooltipRenderer {
        let mut renderer = ItemTooltipRenderer::new();

        if let Some(name) = &self.name {
            renderer.add_title(name);
        }

        if let (Some(itype), Some(isubtype)) = (self.item_type, self.item_sub_type) {
            renderer.add_item_type(itype, isubtype);
        }

        if let (Some(lim_lvl), Some(a_flags), Some(p_lvl), Some(p_job)) = (
            self.limit_level,
            self.anti_flags,
            self.player_level,
            self.player_job,
        ) {
            renderer.add_requirements(lim_lvl, a_flags, p_lvl, p_job);
        }

        renderer.add_battle_stats(
            self.attack_min.unwrap_or(0),
            self.attack_max.unwrap_or(0),
            self.magic_attack_min.unwrap_or(0),
            self.magic_attack_max.unwrap_or(0),
            self.defense.unwrap_or(0),
            self.attack_speed.unwrap_or(0),
        );

        if !self.applies.is_empty() {
            renderer.add_applies(&self.applies);
        }

        if !self.attributes.is_empty() {
            renderer.add_attributes(&self.attributes);
        }

        if !self.sockets.is_empty() {
            renderer.add_sockets(&self.sockets);
        }

        renderer.finalize_dimensions();
        renderer
    }
}
