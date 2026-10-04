use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct DungeonQuestInstance {
    pub index: u32,
    pub icon_file_name: String,
    pub title: String,
    pub clock_name: String,
    pub counter_name: String,
    pub clock_value: i32,
    pub counter_value: i32,
    pub start_time: i32,
}

impl Default for DungeonQuestInstance {
    fn default() -> Self {
        Self {
            index: 0,
            icon_file_name: String::new(),
            title: String::new(),
            clock_name: String::new(),
            counter_name: String::new(),
            clock_value: 0,
            counter_value: 0,
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

    pub fn set_quest_title(&mut self, index: u32, title: &str) {
        if let Some(quest) = self.quests.iter_mut().find(|q| q.index == index) {
            quest.title = title.to_string();
        }
    }

    pub fn set_quest_clock_name(&mut self, index: u32, clock_name: &str) {
        if let Some(quest) = self.quests.iter_mut().find(|q| q.index == index) {
            quest.clock_name = clock_name.to_string();
        }
    }

    pub fn set_quest_counter_name(&mut self, index: u32, counter_name: &str) {
        if let Some(quest) = self.quests.iter_mut().find(|q| q.index == index) {
            quest.counter_name = counter_name.to_string();
        }
    }

    pub fn set_quest_clock_value(&mut self, index: u32, clock_value: i32) {
        let current_time = self.get_current_second();
        if let Some(quest) = self.quests.iter_mut().find(|q| q.index == index) {
            quest.clock_value = clock_value;
            quest.start_time = current_time;
        }
    }

    pub fn set_quest_counter_value(&mut self, index: u32, counter_value: i32) {
        if let Some(quest) = self.quests.iter_mut().find(|q| q.index == index) {
            quest.counter_value = counter_value;
        }
    }

    pub fn set_quest_icon_file_name(&mut self, index: u32, icon_file_name: &str) {
        if let Some(quest) = self.quests.iter_mut().find(|q| q.index == index) {
            quest.icon_file_name = icon_file_name.to_string();
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
    
    pub fn get_quest_last_time(&self, array_index: usize) -> Option<(String, i32)> {
        if let Some(quest) = self.quests.get(array_index) {
            let mut last_time = 0;
            if quest.clock_value >= 0 {
                last_time = (quest.start_time + quest.clock_value) - self.get_current_second();
            }
            Some((quest.clock_name.clone(), last_time))
        } else {
            None
        }
    }
}

pub fn format_time(seconds: i32) -> String {
    if seconds < 0 {
        return "00:00".to_string();
    }
    let mins = seconds / 60;
    let secs = seconds % 60;
    format!("{:02}:{:02}", mins, secs)
}
