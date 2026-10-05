use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct DungeonQuestInstance {
    pub index: u32,
    pub active_goal: String,
    pub monsters_killed: u32,
    pub monsters_total: u32,
    pub time_limit_seconds: i32,
    pub start_time: i32,
}

impl Default for DungeonQuestInstance {
    fn default() -> Self {
        Self {
            index: 0,
            active_goal: String::new(),
            monsters_killed: 0,
            monsters_total: 0,
            time_limit_seconds: 0,
            start_time: 0,
        }
    }
}

pub struct DungeonTrackingManager {
    quests: Vec<DungeonQuestInstance>,
}

impl DungeonTrackingManager {
    pub fn new() -> Self {
        Self { quests: Vec::new() }
    }

    pub fn clear(&mut self) {
        self.quests.clear();
    }

    fn get_current_second(&self) -> i32 {
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        since_the_epoch.as_secs() as i32
    }

    pub fn register_quest_instance(&mut self, mut instance: DungeonQuestInstance) {
        self.delete_quest_instance(instance.index);
        instance.start_time = self.get_current_second();
        self.quests.push(instance);
    }

    pub fn delete_quest_instance(&mut self, index: u32) {
        if let Some(pos) = self.quests.iter().position(|q| q.index == index) {
            self.quests.remove(pos);
        }
    }

    pub fn is_quest(&self, index: u32) -> bool {
        self.quests.iter().any(|q| q.index == index)
    }

    pub fn make_quest(&mut self, index: u32) {
        self.delete_quest_instance(index);
        let mut instance = DungeonQuestInstance::default();
        instance.index = index;
        instance.start_time = self.get_current_second();
        self.quests.push(instance);
    }

    // 3. Wskaznik aktywnego celu (znajdz klucz, zniszcz kamien metin, pokonaj bossa)
    pub fn set_active_goal(&mut self, index: u32, goal: &str) {
        if let Some(quest) = self.quests.iter_mut().find(|q| q.index == index) {
            quest.active_goal = goal.to_string();
        }
    }

    pub fn get_active_goal(&self, index: u32) -> String {
        if let Some(quest) = self.quests.iter().find(|q| q.index == index) {
            quest.active_goal.clone()
        } else {
            String::new()
        }
    }

    // 1. Odliczanie czasu do konca pietra (countdown timer)
    pub fn set_countdown_timer(&mut self, index: u32, seconds: i32) {
        let current_time = self.get_current_second();
        if let Some(quest) = self.quests.iter_mut().find(|q| q.index == index) {
            quest.time_limit_seconds = seconds;
            quest.start_time = current_time;
        }
    }

    pub fn get_time_left(&self, index: u32) -> i32 {
        if let Some(quest) = self.quests.iter().find(|q| q.index == index) {
            if quest.time_limit_seconds <= 0 {
                return 0;
            }
            let elapsed = self.get_current_second() - quest.start_time;
            let remaining = quest.time_limit_seconds - elapsed;
            if remaining > 0 {
                return remaining;
            }
        }
        0
    }

    pub fn get_formatted_time_left(&self, index: u32) -> String {
        format_time(self.get_time_left(index))
    }

    // 2. Pasek zliczonych potworow do pokonania (np. 45/100 mobow)
    pub fn set_monster_progress(&mut self, index: u32, killed: u32, total: u32) {
        if let Some(quest) = self.quests.iter_mut().find(|q| q.index == index) {
            quest.monsters_killed = killed;
            quest.monsters_total = total;
        }
    }

    pub fn get_formatted_monster_progress(&self, index: u32) -> String {
        if let Some(quest) = self.quests.iter().find(|q| q.index == index) {
            format!("{}/{} mobow", quest.monsters_killed, quest.monsters_total)
        } else {
            "0/0 mobow".to_string()
        }
    }

    pub fn get_quest_count(&self) -> usize {
        self.quests.len()
    }

    pub fn get_quest_instance(&self, index: u32) -> Option<&DungeonQuestInstance> {
        self.quests.iter().find(|q| q.index == index)
    }

    pub fn get_quest_instance_by_array_index(&self, array_index: usize) -> Option<&DungeonQuestInstance> {
        self.quests.get(array_index)
    }
}

pub fn format_time(seconds: i32) -> String {
    if seconds <= 0 {
        return "00:00".to_string();
    }
    let mins = seconds / 60;
    let secs = seconds % 60;
    format!("{:02}:{:02}", mins, secs)
}
