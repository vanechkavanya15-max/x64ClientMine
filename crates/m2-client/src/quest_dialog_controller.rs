#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TPacketCGScriptAnswer {
    pub header: u16,
    pub length: u16,
    pub answer: u8,
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TPacketCGScriptButton {
    pub header: u16,
    pub length: u16,
    pub idx: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EButtonType {
    Next,
    Done,
    Cancel,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EEventType {
    Letter,
    Color,
    Delay,
    Enter,
    Wait,
    Clear,
    Question,
    Next,
    Done,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TextFragment {
    pub text: String,
    pub color: Option<(u8, u8, u8)>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DialogLine {
    pub fragments: Vec<TextFragment>,
}

pub struct QuestInstance {
    pub index: u32,
    pub title: String,
    pub clock_name: String,
    pub counter_name: String,
    pub clock_value: i32,
    pub counter_value: i32,
    pub start_time: i32,
    pub icon_file_name: String,
}

pub struct QuestDialogController {
    pub lines: Vec<DialogLine>,
    current_color: Option<(u8, u8, u8)>,
    current_line: DialogLine,
    pub player_name: String,
    pub quest_instances: Vec<QuestInstance>,
}

impl QuestDialogController {
    pub fn new(player_name: &str) -> Self {
        Self {
            lines: Vec::new(),
            current_color: None,
            current_line: DialogLine { fragments: Vec::new() },
            player_name: player_name.to_string(),
            quest_instances: Vec::new(),
        }
    }

    pub fn register_quest_instance(&mut self, instance: QuestInstance) {
        self.quest_instances.retain(|q| q.index != instance.index);
        self.quest_instances.push(instance);
    }

    pub fn get_quest_last_time(&self, index: u32, current_time: i32) -> Option<(String, i32)> {
        if let Some(quest) = self.quest_instances.iter().find(|q| q.index == index) {
            let last_time = if quest.clock_value >= 0 {
                (quest.start_time + quest.clock_value) - current_time
            } else {
                0
            };
            Some((quest.clock_name.clone(), last_time))
        } else {
            None
        }
    }
    
    pub fn format_left_time_string(&self, format_string: &str, time_left: i32) -> String {
        format_string.replace("%d", &time_left.to_string())
    }

    pub fn insert_line(&mut self, text: &str) {
        let mut i = 0;
        let chars: Vec<char> = text.chars().collect();
        let mut current_fragment_text = String::new();

        while i < chars.len() {
            if chars[i] == '[' {
                let mut tag_end = i;
                while tag_end < chars.len() && chars[tag_end] != ']' {
                    tag_end += 1;
                }

                if tag_end < chars.len() && chars[tag_end] == ']' {
                    let tag = &text[i + 1..tag_end];
                    
                    if !current_fragment_text.is_empty() {
                        self.current_line.fragments.push(TextFragment {
                            text: current_fragment_text.clone(),
                            color: self.current_color,
                        });
                        current_fragment_text.clear();
                    }

                    if tag == "ENTER" {
                        self.lines.push(self.current_line.clone());
                        self.current_line = DialogLine { fragments: Vec::new() };
                    } else if tag == "NAME" {
                         current_fragment_text.push_str(&self.player_name);
                    } else if tag.starts_with("COLOR") {
                        if tag == "COLOR" {
                            self.current_color = None;
                        } else {
                            let parts: Vec<&str> = tag.split_whitespace().collect();
                            if parts.len() > 1 {
                                let color_str = parts[1];
                                let mut rgb = color_str.split(';');
                                if let (Some(r), Some(g), Some(b)) = (rgb.next(), rgb.next(), rgb.next()) {
                                    if let (Ok(r), Ok(g), Ok(b)) = (r.parse::<u8>(), g.parse::<u8>(), b.parse::<u8>()) {
                                        self.current_color = Some((r, g, b));
                                    }
                                }
                            }
                        }
                    } else if tag == "/COLOR" {
                        self.current_color = None;
                    }
                    
                    i = tag_end + 1;
                    continue;
                }
            }
            
            current_fragment_text.push(chars[i]);
            i += 1;
        }

        if !current_fragment_text.is_empty() {
            self.current_line.fragments.push(TextFragment {
                text: current_fragment_text,
                color: self.current_color,
            });
        }
    }
    
    pub fn commit_current_line(&mut self) {
        if !self.current_line.fragments.is_empty() {
            self.lines.push(self.current_line.clone());
            self.current_line = DialogLine { fragments: Vec::new() };
        }
    }

    pub fn select_answer(&self, answer_index: u8) -> TPacketCGScriptAnswer {
        TPacketCGScriptAnswer {
            header: 0, // In practice this would be actual header from packet id enum
            length: std::mem::size_of::<TPacketCGScriptAnswer>() as u16,
            answer: answer_index,
        }
    }

    pub fn click_button(&self, idx: u32) -> TPacketCGScriptButton {
        TPacketCGScriptButton {
            header: 0, // In practice this would be actual header from packet id enum
            length: std::mem::size_of::<TPacketCGScriptButton>() as u16,
            idx,
        }
    }
}
