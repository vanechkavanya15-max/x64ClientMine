use std::collections::HashMap;
use std::panic::catch_unwind;
use std::sync::Mutex;
use once_cell::sync::Lazy;

pub const QUEST_MANAGER: u32 = 0;

#[derive(Debug, Clone)]
pub struct QuestEntry {
    pub quest_index: u32,
    pub title: String,
    pub clock_name: String,
    pub clock_value: i32,
}

#[derive(Debug, Default)]
pub struct QuestManager {
    quests: HashMap<u32, QuestEntry>,
}

impl QuestManager {
    pub fn new() -> Self {
        Self {
            quests: HashMap::new(),
        }
    }

    pub fn add_quest(&mut self, q: QuestEntry) {
        self.quests.insert(q.quest_index, q);
    }

    pub fn remove_quest(&mut self, idx: u32) {
        self.quests.remove(&idx);
    }

    pub fn count(&self) -> usize {
        self.quests.len()
    }
}

pub static GLOBAL_QUEST_MANAGER: Lazy<Mutex<QuestManager>> = Lazy::new(|| {
    Mutex::new(QuestManager::new())
});

#[no_mangle]
pub unsafe extern "C" fn m2_quest_count() -> u32 {
    let result = catch_unwind(|| {
        let manager = GLOBAL_QUEST_MANAGER.lock().unwrap();
        manager.count() as u32
    });
    result.unwrap_or(0)
}
