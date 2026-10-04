//! Centralna fasada integrujaca wszystkie podsystemy klienta Metin2 w czystym Rust.
//! Spina moduly wdrozone przez roj 105 agentow: VFS, Audio 3D, wndMgr UI, WGPU Render Pipelines i Mechaniki Gry.

use crate::audio_spatial_3d_sound_engine::AudioSpatial3DEngine;
use crate::audio_bgm_crossfade_mixer::BgmCrossfadeMixer;
use crate::ui_wndmgr_hierarchy::WndMgrHierarchy;
use crate::ui_wndmgr_widget_edit_line::EditLineWidget;
use crate::ui_wndmgr_widget_expanded_image::ExpandedImageBox;
use crate::ui_wndmgr_widget_slot_window::SlotWindow;
use crate::wgpu_camera_shake_fx::CameraShakeFx;
use crate::wgpu_skybox_atmosphere_renderer::SkyboxAtmosphereRenderer;
use crate::wgpu_frustum_culling_map_engine::FrustumCullingMapEngine;
use crate::game_combat_knockback_physics_engine::KnockbackPhysicsEngine;
use crate::game_safebox_storage_manager::GameSafeboxStorageManager;
use crate::game_belt_inventory_manager::BeltInventoryHelper;
use crate::game_skill_splash_area_geometry::{ConeSplash, CylinderSplash, BoxSplash};
use glam::Vec3;

/// Glowny stan zintegrowanego runtime klienta Metin2 Pure Rust.
pub struct ClientIntegratedRuntime {
    pub is_running: bool,
    pub current_phase: u8,
    pub audio_spatial: AudioSpatial3DEngine,
    pub bgm_mixer: BgmCrossfadeMixer,
    pub window_hierarchy: WndMgrHierarchy,
    pub camera_shake: CameraShakeFx,
    pub skybox_renderer: SkyboxAtmosphereRenderer,
    pub culling_engine: FrustumCullingMapEngine,
    pub knockback_physics: KnockbackPhysicsEngine,
    pub safebox: GameSafeboxStorageManager,
    pub edit_line: EditLineWidget,
    pub expanded_image: ExpandedImageBox,
    pub slot_window: SlotWindow,
}

impl ClientIntegratedRuntime {
    /// Inicjalizuje zintegrowany runtime ze wszystkimi podsystemami.
    pub fn new() -> Self {
        Self {
            is_running: true,
            current_phase: 1, // PHASE_LOGIN domyslnie
            audio_spatial: AudioSpatial3DEngine::new(),
            bgm_mixer: BgmCrossfadeMixer::new(1.5),
            window_hierarchy: WndMgrHierarchy::new(),
            camera_shake: CameraShakeFx::new(0.3, 5.0, 20.0, 2.0),
            skybox_renderer: SkyboxAtmosphereRenderer::new(),
            culling_engine: FrustumCullingMapEngine::new(),
            knockback_physics: KnockbackPhysicsEngine::new(Vec3::ZERO),
            safebox: GameSafeboxStorageManager::new(),
            edit_line: EditLineWidget::new(32),
            expanded_image: ExpandedImageBox::new(),
            slot_window: SlotWindow::new(),
        }
    }

    /// Glowna petla aktualizacji klatki gry (Frame Update).
    pub fn update(&mut self, delta_time: f32) {
        if !self.is_running {
            return;
        }

        self.camera_shake.update(delta_time);
        self.bgm_mixer.update(delta_time);
        self.knockback_physics.update(delta_time);
        self.slot_window.update(delta_time);
    }

    /// Zmiana fazy klienta gry (np. Login -> Select -> Game).
    pub fn switch_phase(&mut self, next_phase: u8) {
        self.current_phase = next_phase;
    }

    /// Sprawdzenie czy punkt znajduje sie w stozku ataku umiejetnosci (AOE Splash).
    pub fn is_point_in_cone_splash(origin: Vec3, direction: Vec3, max_dist: f32, angle_deg: f32, target: Vec3) -> bool {
        let splash = ConeSplash::new(origin, direction, max_dist, angle_deg);
        splash.contains(target)
    }

    /// Sprawdzenie czy przedmiot moze trafic do slotu ekwipunku pasa.
    pub fn can_store_in_belt(item_vnum: u32) -> bool {
        BeltInventoryHelper::is_belt_item(item_vnum)
    }
}
