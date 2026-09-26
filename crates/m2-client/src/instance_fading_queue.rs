use std::sync::{RwLock, OnceLock};

/// Struktura reprezentujaca znikajaca postac (zwalniana po spadku alpha).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FadingInstance {
    pub vid: u32,
    pub cur_alpha: f32,
    pub fade_speed: f32,
}

impl FadingInstance {
    pub fn new(vid: u32) -> Self {
        Self {
            vid,
            cur_alpha: 1.0,
            fade_speed: 1.5, // Zgodnie z C++: float fAlpha = __GetAlphaValue() - (rApp.GetGlobalElapsedTime() * 1.5f);
        }
    }
}

/// Globalna kolejka postaci usuwanych z opoznieniem (fade out).
pub struct InstanceFadingQueue {
    instances: Vec<FadingInstance>,
}

impl InstanceFadingQueue {
    pub fn new() -> Self {
        Self {
            instances: Vec::new(),
        }
    }

    /// Dodaje postac do kolejki zanikania.
    pub fn add_instance(&mut self, vid: u32) {
        if !self.instances.iter().any(|i| i.vid == vid) {
            self.instances.push(FadingInstance::new(vid));
        }
    }

    /// Aktualizuje kolejkę zanikania i zwraca VID postaci, które powinny zostać ostatecznie usunięte.
    pub fn update(&mut self, delta_time: f32) -> Vec<u32> {
        let mut to_delete = Vec::new();
        let mut i = 0;

        while i < self.instances.len() {
            let instance = &mut self.instances[i];
            instance.cur_alpha -= delta_time * instance.fade_speed;

            if instance.cur_alpha <= 0.0 {
                to_delete.push(instance.vid);
                self.instances.remove(i);
            } else {
                i += 1;
            }
        }

        to_delete
    }

    /// Zwraca bieżącą widoczność (alpha) danej postaci z kolejki.
    pub fn get_alpha(&self, vid: u32) -> Option<f32> {
        self.instances.iter().find(|i| i.vid == vid).map(|i| i.cur_alpha)
    }

    /// Natychmiastowo zwalnia całą kolejkę zanikania. Zwraca wszystkie VID.
    pub fn clear(&mut self) -> Vec<u32> {
        let vids: Vec<u32> = self.instances.iter().map(|i| i.vid).collect();
        self.instances.clear();
        vids
    }
}

impl Default for InstanceFadingQueue {
    fn default() -> Self {
        Self::new()
    }
}

// Globalny stan (wzorzec singletonu per instancja gry) na potrzeby FFI
static GLOBAL_FADING_QUEUE: OnceLock<RwLock<InstanceFadingQueue>> = OnceLock::new();

fn get_fading_queue() -> &'static RwLock<InstanceFadingQueue> {
    GLOBAL_FADING_QUEUE.get_or_init(|| RwLock::new(InstanceFadingQueue::new()))
}

/// Zewnetrzny interfejs FFI dla C++ pozwalajacy na dodanie postaci do kolejki fade out.
/// Zwraca true w przypadku sukcesu, false jesli wystapil problem.
#[no_mangle]
pub extern "C" fn m2_chr_delete_by_fade(vid: u32) -> bool {
    let queue_res = get_fading_queue().write();
    match queue_res {
        Ok(mut queue) => {
            queue.add_instance(vid);
            true
        }
        Err(_) => false,
    }
}

/// Czysto testowa funkcja pomocnicza do czyszczenia stanu.
pub fn clear_fading_queue_for_tests() {
    if let Ok(mut queue) = get_fading_queue().write() {
        queue.clear();
    }
}
