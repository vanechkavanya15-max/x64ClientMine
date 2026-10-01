// crates/m2-client/src/target_selection_system.rs

use std::ptr;

pub const HEADER_CG_TARGET: u16 = 0x0A01;

/// Struktura pakietu dla zaznaczenia celu, 1:1 z kodem C++ 
/// (TPacketCGTarget w /app/src/UserInterface/Packet.h)
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketCGTarget {
    pub header: u16,
    pub length: u16,
    pub dw_vid: u32,
}

/// Dodatkowe informacje wykorzystywane przez gorna belke celu (TargetBoard)
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct TargetBoardInfo {
    pub name: String,
    pub hp_percentage: u8,
    pub level: u32,
    pub empire_id: u8,
    pub aggressiveness_rank: i32, // Opcjonalnie do uzycia przy wyswietlaniu zlych/dobrych
}

#[derive(Debug, Default, Clone)]
pub struct TargetSelectionSystem {
    pub selected_target_vid: Option<u32>,
    pub target_board_info: Option<TargetBoardInfo>,
}

impl TargetSelectionSystem {
    /// Tworzy nowa, pusta instancje menedzera celu.
    pub fn new() -> Self {
        Self::default()
    }

    /// Symuluje wykrywanie klikniecia na aktora (potwor, gracz, NPC)
    /// Sprawdza czy odleglosc miedzy punktem klikniecia a srodkiem aktora
    /// jest mniejsza lub rowna promieniowi `bounding_sphere_radius`.
    pub fn pick_target(
        &self,
        click_x: f32,
        click_y: f32,
        click_z: f32,
        actor_x: f32,
        actor_y: f32,
        actor_z: f32,
        bounding_sphere_radius: f32,
    ) -> bool {
        let dx = click_x - actor_x;
        let dy = click_y - actor_y;
        let dz = click_z - actor_z;
        
        let dist_sq = dx * dx + dy * dy + dz * dz;
        dist_sq <= bounding_sphere_radius * bounding_sphere_radius
    }

    /// Wybiera cel i ustawia informacje dla TargetBoard
    /// Generuje rowniez odpowiedni pakiet sieciowy CG_TARGET.
    /// Zwraca ilosc zapisanych bajtow do bufora `out_buf` (jesli udane).
    pub fn select_target(
        &mut self,
        vid: u32,
        name: String,
        hp: i32,
        max_hp: i32,
        level: u32,
        empire_id: u8,
        aggressiveness_rank: i32,
        out_buf: &mut [u8],
    ) -> Result<usize, &'static str> {
        self.selected_target_vid = Some(vid);

        let mut board_info = TargetBoardInfo {
            name,
            hp_percentage: 0,
            level,
            empire_id,
            aggressiveness_rank,
        };

        if max_hp > 0 && hp > 0 {
            let clamped_hp = hp.min(max_hp);
            let percentage = (clamped_hp as i64 * 100) / (max_hp as i64);
            board_info.hp_percentage = percentage.clamp(0, 100) as u8;
        }

        self.target_board_info = Some(board_info);

        let size = std::mem::size_of::<TPacketCGTarget>();
        if out_buf.len() < size {
            return Err("Buffer is too small to serialize TPacketCGTarget");
        }

        let packet = TPacketCGTarget {
            header: HEADER_CG_TARGET,
            length: size as u16,
            dw_vid: vid,
        };

        unsafe {
            ptr::copy_nonoverlapping(
                &packet as *const _ as *const u8,
                out_buf.as_mut_ptr(),
                size,
            );
        }

        Ok(size)
    }

    /// Aktualizuje pasek HP zaznaczonego celu (0..100%).
    /// Automatycznie odznacza cel, jesli jego HP spadnie do 0 lub ponizej (smierc).
    pub fn update_target_hp(&mut self, current_hp: i32, max_hp: i32) {
        if self.selected_target_vid.is_none() {
            return;
        }

        if current_hp <= 0 {
            let _ = self.clear_target(None);
            return;
        }

        if max_hp <= 0 {
            if let Some(info) = &mut self.target_board_info {
                info.hp_percentage = 0;
            }
            return;
        }

        if let Some(info) = &mut self.target_board_info {
            let clamped_hp = current_hp.min(max_hp);
            let percentage = (clamped_hp as i64 * 100) / (max_hp as i64);
            info.hp_percentage = percentage.clamp(0, 100) as u8;
        }
    }

    /// Kasuje cel (np. po nacisnieciu Escape, lub po kliknieciu w puste miejsce)
    /// i generuje pakiet odznaczenia (z vid=0).
    pub fn clear_target(&mut self, out_buf: Option<&mut [u8]>) -> Result<usize, &'static str> {
        self.selected_target_vid = None;
        self.target_board_info = None;

        if let Some(buf) = out_buf {
            let size = std::mem::size_of::<TPacketCGTarget>();
            if buf.len() < size {
                return Err("Buffer is too small to serialize TPacketCGTarget");
            }
            
            let packet = TPacketCGTarget {
                header: HEADER_CG_TARGET,
                length: size as u16,
                dw_vid: 0,
            };

            unsafe {
                ptr::copy_nonoverlapping(
                    &packet as *const _ as *const u8,
                    buf.as_mut_ptr(),
                    size,
                );
            }
            return Ok(size);
        }
        
        Ok(0)
    }
}
