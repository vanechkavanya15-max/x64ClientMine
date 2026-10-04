// vfs_motion_sound_player.rs - Odtwarzacz dzwiekow z plikow .mss powiazanych z klatkami animacji z VFS
use std::collections::HashMap;
use std::ffi::CString;
use std::sync::{Mutex, OnceLock};

use crate::engine_mss_animation_audio_wire::parse_mss_file;

#[derive(Debug, Clone, PartialEq)]
pub struct SoundMarker {
    pub time_sec: f32,
    pub sound_file: String,
}

#[derive(Debug, Default)]
pub struct MotionSoundTimeline {
    pub sound_markers: Vec<SoundMarker>,
    pub last_eval_time: f32,
    pub played_indices: Vec<bool>,
}

impl MotionSoundTimeline {
    pub fn from_markers(markers: Vec<SoundMarker>) -> Self {
        let count = markers.len();
        Self {
            sound_markers: markers,
            last_eval_time: 0.0,
            played_indices: vec![false; count],
        }
    }

    /// Aktualizuje klatke animacji i zwraca liste plikow dzwiekowych do odtworzenia w tej klatce
    pub fn update_time(&mut self, current_time_sec: f32) -> Vec<String> {
        let mut sounds_to_play = Vec::new();

        // Jesli animacja zapetlila sie (czas sie cofnal), resetujemy flagi odtworzenia
        if current_time_sec < self.last_eval_time {
            for played in &mut self.played_indices {
                *played = false;
            }
        }

        for (i, marker) in self.sound_markers.iter().enumerate() {
            if !self.played_indices[i] && marker.time_sec <= current_time_sec {
                // Jesli czas znacznika miesci sie w biezacym oknie
                sounds_to_play.push(marker.sound_file.clone());
                self.played_indices[i] = true;
            }
        }

        self.last_eval_time = current_time_sec;
        sounds_to_play
    }
}

pub struct VfsMotionSoundManager {
    cached_timelines: HashMap<String, Vec<SoundMarker>>,
}

static MOTION_SOUND_MGR: OnceLock<Mutex<VfsMotionSoundManager>> = OnceLock::new();

pub fn get_motion_sound_mgr() -> &'static Mutex<VfsMotionSoundManager> {
    MOTION_SOUND_MGR.get_or_init(|| Mutex::new(VfsMotionSoundManager {
        cached_timelines: HashMap::new(),
    }))
}

impl VfsMotionSoundManager {
    /// Laduje i parsuje plik .mss z VFS lub zwraca z pamieci cache
    pub fn get_or_load_sound_markers(&mut self, mss_path: &str) -> Vec<SoundMarker> {
        let norm_path = mss_path.to_lowercase().replace('\\', "/");
        if let Some(markers) = self.cached_timelines.get(&norm_path) {
            return markers.clone();
        }

        // Odczyt z VFS
        let content_opt = if let Some(bytes) = crate::texture_vfs_pipeline::read_vfs_bytes(&norm_path) {
            String::from_utf8(bytes).ok()
        } else if let Ok(s) = std::fs::read_to_string(&norm_path) {
            Some(s)
        } else {
            None
        };

        let mut markers = Vec::new();
        if let Some(content) = content_opt {
            if let Ok(instances) = parse_mss_file(&content, 30) {
                for inst in instances {
                    let time_sec = inst.frame as f32 / 30.0;
                    markers.push(SoundMarker {
                        time_sec,
                        sound_file: inst.sound_file_name,
                    });
                }
            }
        }

        self.cached_timelines.insert(norm_path, markers.clone());
        markers
    }

    /// Odtwarza dzwieki z osi czasu w przestrzeni 3D
    pub fn play_frame_sounds(
        &mut self,
        timeline: &mut MotionSoundTimeline,
        current_time_sec: f32,
        pos_x: f32, pos_y: f32, pos_z: f32,
    ) {
        let sounds = timeline.update_time(current_time_sec);
        for s in sounds {
            if let Ok(c_name) = CString::new(s) {
                crate::sound_ffi::m2_audio_play_sound_3d(c_name.as_ptr(), pos_x, pos_y, pos_z, 1.0);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_motion_sound_timeline_progression_and_loop() {
        let markers = vec![
            SoundMarker {
                time_sec: 0.2,
                sound_file: "sound/common/swing/w_sword_1.wav".to_string(),
            },
            SoundMarker {
                time_sec: 0.5,
                sound_file: "sound/common/hit/hit_flesh_1.wav".to_string(),
            },
        ];

        let mut timeline = MotionSoundTimeline::from_markers(markers);

        // Klatka 0.1s -> nic sie nie odtwarza
        let s1 = timeline.update_time(0.1);
        assert!(s1.is_empty());

        // Klatka 0.25s -> odtworzenie pierwszego dzwieku (0.2s)
        let s2 = timeline.update_time(0.25);
        assert_eq!(s2.len(), 1);
        assert_eq!(s2[0], "sound/common/swing/w_sword_1.wav");

        // Klatka 0.3s -> dzwiek 0.2s nie powinien sie powtorzyc
        let s3 = timeline.update_time(0.3);
        assert!(s3.is_empty());

        // Klatka 0.6s -> odtworzenie drugiego dzwieku (0.5s)
        let s4 = timeline.update_time(0.6);
        assert_eq!(s4.len(), 1);
        assert_eq!(s4[0], "sound/common/hit/hit_flesh_1.wav");

        // Zapetlenie animacji (czas wraca do 0.05s)
        let s5 = timeline.update_time(0.05);
        assert!(s5.is_empty());

        // Nowy cykl dochodzi do 0.25s -> ponowne odtworzenie pierwszego dzwieku
        let s6 = timeline.update_time(0.25);
        assert_eq!(s6.len(), 1);
        assert_eq!(s6[0], "sound/common/swing/w_sword_1.wav");
    }
}
