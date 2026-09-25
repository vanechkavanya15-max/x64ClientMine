 //! Wzorzec referencyjny C++: `source/UserInterface/PythonEventManager.cpp oraz source/UserInterface/PythonQuest.cpp`
 //! Opis: Odwzorowanie CPythonEventManager: parsowanie znacznikow skryptow dialogowych ([ENTER], [WAIT], [NEXT]), przyciski wyboru opcji rozmowy z NPC, lista aktywnych zadan.
 
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonType {
    Next,
    Done,
    Cancel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EventType {
    Letter,
    Color,
    Delay,
    Enter,
    Wait,
    Clear,
    Question,
    Next,
    Done,
    LeftImage,
    TopImage,
    BackgroundImage,
    Image,
    AddMapSignal,
    ClearMapSignal,
    SetMessagePosition,
    AdjustMessagePosition,
    SetCenterMapPosition,
    QuestButton,
    QuestButtonClose,
    Sleep,
    SetCamera,
    BlendCamera,
    RestoreCamera,
    FadeOut,
    FadeIn,
    WhiteOut,
    WhiteIn,
    ClearText,
    TextHorizontalAlignCenter,
    TitleImage,
    RunCinema,
    DungeonResult,
    ItemName,
    MonsterName,
    Color256,
    WindowSize,
    Input,
    ConfirmWait,
    EndConfirmWait,
    InsertImage,
    SelectItem,
}

#[derive(Debug, Clone)]
pub struct TextLine {
    pub local_x: i32,
    pub local_y: i32,
    pub text: String,
    pub color: [f32; 4],
    pub is_center: bool,
}

#[derive(Debug, Clone)]
pub struct EventSet {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub local_y: i32,

    pub is_lock: bool,
    pub last_delay_time: i64,
    pub current_letter: usize,
    pub current_color: [f32; 4],
    pub current_line: String,

    pub script_text_line_list: Vec<TextLine>,
    pub current_text_line: Option<TextLine>,

    pub is_confirm_wait: bool,
    pub confirm_end_time: i64,

    pub visible_start_line: i32,
    pub visible_line_count: i32,
    pub adjust_line: i32,

    pub waiting_time: i64,
    pub restricted_character_count: usize,
    pub answer: i32,

    pub is_text_center_mode: bool,
    pub is_wait_flag: bool,
    
    pub script_commands: Vec<(EventType, HashMap<String, String>)>,
    pub command_index: usize,
}

impl Default for EventSet {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            width: 0,
            local_y: 0,
            is_lock: false,
            last_delay_time: 0,
            current_letter: 0,
            current_color: [1.0, 1.0, 1.0, 1.0],
            current_line: String::new(),
            script_text_line_list: Vec::new(),
            current_text_line: None,
            is_confirm_wait: false,
            confirm_end_time: 0,
            visible_start_line: 0,
            visible_line_count: 5,
            adjust_line: 0,
            waiting_time: 10,
            restricted_character_count: 30,
            answer: -1,
            is_text_center_mode: false,
            is_wait_flag: false,
            script_commands: Vec::new(),
            command_index: 0,
        }
    }
}

impl EventSet {
    pub fn insert_line(&mut self, is_center: bool, x_pos: i32) {
        if let Some(mut current) = self.current_text_line.take() {
            current.text = self.current_line.clone();
            current.is_center = is_center;
            current.local_x = x_pos;
            current.local_y = self.local_y;
            self.script_text_line_list.push(current);
        }

        self.local_y += 16;
        self.current_line.clear();
        self.current_letter = 0;
        self.current_text_line = Some(TextLine {
            local_x: x_pos,
            local_y: self.local_y,
            text: String::new(),
            color: self.current_color,
            is_center,
        });
        
        self.adjust_line += 1;
    }
    
    pub fn make_next_button(&mut self, _button_type: ButtonType) {
        // Mocking the button text line insertion
        let button_text = "[NEXT/DONE/CANCEL]";
        
        let text_line = TextLine {
            local_x: 0,
            local_y: self.local_y,
            text: button_text.to_string(),
            color: [1.0, 1.0, 1.0, 1.0],
            is_center: false,
        };
        
        self.script_text_line_list.push(text_line);
        self.local_y += 16;
    }
}

#[derive(Debug, Clone, Default)]
pub struct QuestInstance {
    pub index: u32,
    pub icon_file_name: String,
    pub title: String,
    pub clock_name: String,
    pub counter_name: String,
    pub clock_value: i32,
    pub counter_value: i32,
    pub start_time: i32,
}

 #[derive(Debug, Clone, Default)]
 pub struct UiQuestDialog {
     pub initialized: bool,
    pub event_sets: HashMap<u32, EventSet>,
    pub quests: Vec<QuestInstance>,
    pub next_event_set_id: u32,
    pub left_time_string: String,
 }
 
 impl UiQuestDialog {
     pub fn new() -> Self {
        Self {
            initialized: true,
            event_sets: HashMap::new(),
            quests: Vec::new(),
            next_event_set_id: 0,
            left_time_string: "Pozostaly czas : %d sek".to_string(),
        }
     }
 
     pub fn is_ready(&self) -> bool {
         self.initialized
     }
    
    pub fn register_event_set_from_script(&mut self, script: Vec<(EventType, HashMap<String, String>)>) -> i32 {
        let mut event_set = EventSet::default();
        event_set.script_commands = script;
        event_set.current_text_line = Some(TextLine {
            local_x: 0,
            local_y: 0,
            text: String::new(),
            color: [1.0, 1.0, 1.0, 1.0],
            is_center: false,
        });
        
        let id = self.next_event_set_id;
        self.event_sets.insert(id, event_set);
        self.next_event_set_id += 1;
        
        id as i32
    }

    pub fn update_event_set(&mut self, index: u32, elapsed_time: i64) {
        let is_wait;
        let is_lock;
        if let Some(es) = self.event_sets.get(&index) {
            is_wait = es.is_wait_flag;
            is_lock = es.is_lock;
        } else {
            return;
        }

        if is_wait || is_lock {
            return;
        }

        let mut remaining_time = elapsed_time;
        if let Some(es) = self.event_sets.get_mut(&index) {
            es.last_delay_time = 0.max(es.last_delay_time - remaining_time);
        }

        while remaining_time > 0 {
            let last_delay = if let Some(es) = self.event_sets.get_mut(&index) {
                es.last_delay_time -= remaining_time;
                es.last_delay_time
            } else {
                break;
            };
 
            if last_delay <= 0 {
                remaining_time = -last_delay;

                self.process_event_set(index);

                if let Some(es) = self.event_sets.get_mut(&index) {
                    if es.last_delay_time < 0 {
                        es.last_delay_time = 0;
                        break;
                    }
                    if es.is_lock || es.is_wait_flag {
                        break;
                    }
                }
            } else {
                break;
            }
        }
    }

    pub fn process_event_set(&mut self, index: u32) {
        let event_set = if let Some(es) = self.event_sets.get_mut(&index) {
            es
        } else {
            return;
        };

        if event_set.is_lock {
            return;
        }

        if event_set.command_index >= event_set.script_commands.len() {
            event_set.is_lock = true;
            return;
        }

        let (event_type, args) = event_set.script_commands[event_set.command_index].clone();
        event_set.command_index += 1;

        match event_type {
            EventType::Letter => {
                if let Some(val) = args.get("value") {
                    event_set.current_line.push_str(val);
                    event_set.current_letter += val.chars().count();
                    
                    if event_set.current_letter >= event_set.restricted_character_count {
                        event_set.insert_line(false, 0);
                    }
                }
                event_set.last_delay_time = event_set.waiting_time;
            }
            EventType::Color => {
                let r = args.get("r").and_then(|s| s.parse::<f32>().ok()).unwrap_or(1.0);
                let g = args.get("g").and_then(|s| s.parse::<f32>().ok()).unwrap_or(1.0);
                let b = args.get("b").and_then(|s| s.parse::<f32>().ok()).unwrap_or(1.0);
                event_set.current_color = [r, g, b, 1.0];
            }
            EventType::Delay => {
                if let Some(val) = args.get("value") {
                    if let Ok(v) = val.parse::<i64>() {
                        event_set.waiting_time = v;
                    }
                }
            }
            EventType::Enter => {
                event_set.insert_line(false, 0);
            }
            EventType::Wait => {
                event_set.make_next_button(ButtonType::Next);
                event_set.adjust_line += 2;
                event_set.is_wait_flag = true;
            }
            EventType::Clear => {
                event_set.script_text_line_list.clear();
                event_set.current_line.clear();
                event_set.current_letter = 0;
                event_set.local_y = 0;
                event_set.is_lock = true;
            }
            EventType::Next => {
                event_set.make_next_button(ButtonType::Next);
                event_set.adjust_line += 2;
            }
            EventType::Done => {
                event_set.make_next_button(ButtonType::Done);
                event_set.adjust_line += 2;
            }
            EventType::LeftImage | EventType::TopImage | EventType::BackgroundImage | EventType::Image | EventType::InsertImage | EventType::TitleImage => {
                // Image loading logic would go here
            }
            EventType::AddMapSignal | EventType::ClearMapSignal | EventType::SetCenterMapPosition => {
                // MiniMap signal logic
            }
            EventType::SetMessagePosition | EventType::AdjustMessagePosition => {
                // Positioning logic
            }
            EventType::QuestButton | EventType::QuestButtonClose => {
                // Quest specific UI logic
            }
            EventType::Sleep => {
                if let Some(val) = args.get("value") {
                    if let Ok(v) = val.parse::<i64>() {
                        event_set.last_delay_time = v;
                    }
                }
            }
            EventType::SetCamera | EventType::BlendCamera | EventType::RestoreCamera => {
                // Camera controls
            }
            EventType::FadeOut | EventType::FadeIn | EventType::WhiteOut | EventType::WhiteIn => {
                // Screen effects
            }
            EventType::ClearText => {
                event_set.current_line.clear();
                event_set.current_letter = 0;
            }
            EventType::TextHorizontalAlignCenter => {
                event_set.is_text_center_mode = true;
            }
            EventType::RunCinema | EventType::DungeonResult => {
                // Specific cinematic sequences
            }
            EventType::ItemName | EventType::MonsterName => {
                // Item/Mob name fetching
            }
            EventType::Color256 => {
                // 256 color format to D3DColor
            }
            EventType::WindowSize => {
                // Window size adjustment
            }
            EventType::Input => {
                event_set.insert_line(false, 0);
            }
            EventType::ConfirmWait => {
                event_set.is_confirm_wait = true;
                event_set.insert_line(true, 0);
                event_set.make_next_button(ButtonType::Cancel);
            }
            EventType::EndConfirmWait => {
                event_set.is_confirm_wait = false;
                event_set.confirm_end_time = 0;
            }
            EventType::SelectItem => {
                // Item selection UI
            }
            _ => {
                // Ignore other unhandled event types for now
            }
        }
    }
    
    pub fn skip(&mut self, index: u32) {
        let event_set = if let Some(es) = self.event_sets.get_mut(&index) {
            es
        } else {
            return;
        };

        event_set.waiting_time = 0;
    }

    // Quest System Methods
    pub fn register_quest_instance(&mut self, instance: QuestInstance) {
        self.delete_quest_instance(instance.index);
        let start_time = 0; // Mock current time
        let mut inst = instance;
        inst.start_time = start_time;
        self.quests.push(inst);
    }

    pub fn delete_quest_instance(&mut self, index: u32) {
        self.quests.retain(|q| q.index != index);
    }

    pub fn is_quest(&self, index: u32) -> bool {
        self.quests.iter().any(|q| q.index == index)
    }

    pub fn make_quest(&mut self, index: u32) {
        self.delete_quest_instance(index);
        let mut inst = QuestInstance::default();
        inst.index = index;
        inst.start_time = 0; // Mock current time
        self.quests.push(inst);
    }

    pub fn set_quest_title(&mut self, index: u32, title: String) {
        if let Some(q) = self.quests.iter_mut().find(|q| q.index == index) {
            q.title = title;
        }
    }

    pub fn set_quest_clock_name(&mut self, index: u32, clock_name: String) {
        if let Some(q) = self.quests.iter_mut().find(|q| q.index == index) {
            q.clock_name = clock_name;
        }
    }

    pub fn set_quest_counter_name(&mut self, index: u32, counter_name: String) {
        if let Some(q) = self.quests.iter_mut().find(|q| q.index == index) {
            q.counter_name = counter_name;
        }
    }

    pub fn set_quest_clock_value(&mut self, index: u32, clock_value: i32) {
        if let Some(q) = self.quests.iter_mut().find(|q| q.index == index) {
            q.clock_value = clock_value;
            q.start_time = 0; // Mock current time
        }
    }

    pub fn set_quest_counter_value(&mut self, index: u32, counter_value: i32) {
        if let Some(q) = self.quests.iter_mut().find(|q| q.index == index) {
            q.counter_value = counter_value;
        }
    }

    pub fn set_quest_icon_file_name(&mut self, index: u32, icon_file_name: String) {
        if let Some(q) = self.quests.iter_mut().find(|q| q.index == index) {
            q.icon_file_name = icon_file_name;
        }
    }
 
    pub fn get_quest_count(&self) -> usize {
        self.quests.len()
     }
 }
