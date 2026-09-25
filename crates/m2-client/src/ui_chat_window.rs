//! Modul `ui_chat_window` — Okno czatu z kolorowaniem kanalow i przewijaniem CPythonChat
//! Wzorzec referencyjny C++: `source/UserInterface/PythonChat.cpp`
//! Opis: Odwzorowanie CPythonChat: historia wiadomosci, kolorowanie wg typu (TALK bialy, INFO czerwony/zloty, NOTICE zolty, PARTY niebieski, GUILD zielony, SHOUT), pasek wpisywania tekstu.

use std::collections::{HashMap, HashSet, VecDeque};
use std::rc::Rc;
use std::cell::RefCell;

pub const CHAT_LINE_MAX_NUM: usize = 300;
pub const CHAT_LINE_COLOR_ARRAY_MAX_NUM: usize = 3;
pub const START_DISAPPEARING_TIME: f32 = 5.0;
pub const MAX_LINE_COUNT: usize = 5;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ChatType {
    Talking = 0,
    Info,
    Notice,
    Party,
    Guild,
    Command,
    Shout,
    Whisper,
    BigNotice,
    MaxNum,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum WhisperType {
    Chat = 0,
    NotExist = 1,
    TargetBlocked = 2,
    SenderBlocked = 3,
    Error = 4,
    Gm = 5,
    System = 0xFF,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoardState {
    View,
    Edit,
    Log,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
}

impl Default for Color {
    fn default() -> Self {
        Self { r: 1.0, g: 1.0, b: 1.0, a: 1.0 }
    }
}

#[derive(Debug, Clone)]
pub struct ChatLine {
    pub i_type: ChatType,
    pub appended_time: f32,
    pub colors: [Color; CHAT_LINE_COLOR_ARRAY_MAX_NUM],
    pub text: String,
}

impl ChatLine {
    pub fn new(i_type: ChatType, text: String) -> Self {
        Self {
            i_type,
            appended_time: 0.0,
            colors: [Color::new(0.0, 0.0, 1.0, 1.0); CHAT_LINE_COLOR_ARRAY_MAX_NUM],
            text,
        }
    }

    pub fn set_color_all(&mut self, color: Color) {
        for i in 0..CHAT_LINE_COLOR_ARRAY_MAX_NUM {
            self.colors[i] = color;
        }
    }

    pub fn get_color_mut(&mut self, id: usize) -> &mut Color {
        if id < CHAT_LINE_COLOR_ARRAY_MAX_NUM {
            &mut self.colors[id]
        } else {
            &mut self.colors[0] // fallback
        }
    }
}

#[derive(Debug, Clone)]
pub struct WaitChat {
    pub i_type: ChatType,
    pub chat: String,
    pub appending_time: u32,
}

#[derive(Debug, Clone)]
pub struct ChatSet {
    pub x: i32,
    pub y: i32,
    pub height: i32,
    pub step: i32,
    pub end_pos: f32,
    pub board_state: BoardState,
    pub mode: Vec<bool>,
    pub showing_chat_line_list: VecDeque<Rc<RefCell<ChatLine>>>,
}

impl ChatSet {
    pub fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            height: 0,
            step: 15,
            end_pos: 1.0,
            board_state: BoardState::View,
            mode: vec![true; ChatType::MaxNum as usize],
            showing_chat_line_list: VecDeque::new(),
        }
    }

    pub fn check_mode(&self, type_: ChatType) -> bool {
        let index = type_ as usize;
        if index < self.mode.len() {
            self.mode[index]
        } else {
            false
        }
    }
}

#[derive(Debug, Clone)]
pub struct Whisper {
    pub line_step: f32,
    pub width: f32,
    pub height: f32,
    pub cur_position: f32,
    pub chat_line_deque: VecDeque<Rc<RefCell<ChatLine>>>,
}

impl Whisper {
    pub fn new() -> Self {
        Self {
            line_step: 15.0,
            width: 300.0,
            height: 120.0,
            cur_position: 1.0,
            chat_line_deque: VecDeque::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct UiChatWindow {
    pub initialized: bool,
    pub chat_line_deque: VecDeque<Rc<RefCell<ChatLine>>>,
    pub chat_set_map: HashMap<i32, ChatSet>,
    pub whisper_map: HashMap<String, Whisper>,
    pub ignore_character_set: HashSet<String>,
    pub wait_chat_list: VecDeque<WaitChat>,
    pub chat_colors: [Color; ChatType::MaxNum as usize],
    pub input_history: VecDeque<String>,
    pub current_time: f32,
}

impl UiChatWindow {
    pub fn new() -> Self {
        let mut chat_colors = [Color::default(); ChatType::MaxNum as usize];
        chat_colors[ChatType::Talking as usize] = Color::new(1.0, 1.0, 1.0, 1.0);
        chat_colors[ChatType::Info as usize] = Color::new(1.0, 0.785, 0.785, 1.0);
        chat_colors[ChatType::Notice as usize] = Color::new(1.0, 0.902, 0.730, 1.0);
        chat_colors[ChatType::Party as usize] = Color::new(0.542, 1.0, 0.949, 1.0);
        chat_colors[ChatType::Guild as usize] = Color::new(0.906, 0.847, 1.0, 1.0);
        chat_colors[ChatType::Command as usize] = Color::new(0.658, 1.0, 0.835, 1.0);
        chat_colors[ChatType::Shout as usize] = Color::new(0.658, 1.0, 0.835, 1.0);
        chat_colors[ChatType::Whisper as usize] = Color::new(0.29, 0.88, 0.29, 1.0); // 0xff4AE14A
        chat_colors[ChatType::BigNotice as usize] = Color::new(1.0, 0.902, 0.730, 1.0);

        Self {
            initialized: true,
            chat_line_deque: VecDeque::new(),
            chat_set_map: HashMap::new(),
            whisper_map: HashMap::new(),
            ignore_character_set: HashSet::new(),
            wait_chat_list: VecDeque::new(),
            chat_colors,
            input_history: VecDeque::new(),
            current_time: 0.0,
        }
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }
}

impl UiChatWindow {
    pub fn create_chat_set(&mut self, id: i32) -> i32 {
        self.chat_set_map.insert(id, ChatSet::new());
        id
    }

    pub fn set_board_state(&mut self, id: i32, state: BoardState) {
        if let Some(chat_set) = self.chat_set_map.get_mut(&id) {
            chat_set.board_state = state;
        }
    }

    pub fn set_position(&mut self, id: i32, x: i32, y: i32) {
        if let Some(chat_set) = self.chat_set_map.get_mut(&id) {
            chat_set.x = x;
            chat_set.y = y;
        }
    }

    pub fn set_height(&mut self, id: i32, height: i32) {
        if let Some(chat_set) = self.chat_set_map.get_mut(&id) {
            chat_set.height = height;
        }
    }

    pub fn set_step(&mut self, id: i32, step: i32) {
        if let Some(chat_set) = self.chat_set_map.get_mut(&id) {
            chat_set.step = step;
        }
    }

    pub fn set_end_pos(&mut self, id: i32, pos: f32) {
        if let Some(chat_set) = self.chat_set_map.get_mut(&id) {
            chat_set.end_pos = pos;
        }
    }

    pub fn toggle_chat_mode(&mut self, id: i32, mode: ChatType) {
        if let Some(chat_set) = self.chat_set_map.get_mut(&id) {
            let index = mode as usize;
            if index < chat_set.mode.len() {
                chat_set.mode[index] = !chat_set.mode[index];
            }
        }
    }

    pub fn enable_chat_mode(&mut self, id: i32, mode: ChatType) {
        if let Some(chat_set) = self.chat_set_map.get_mut(&id) {
            let index = mode as usize;
            if index < chat_set.mode.len() {
                chat_set.mode[index] = true;
            }
        }
    }

    pub fn disable_chat_mode(&mut self, id: i32, mode: ChatType) {
        if let Some(chat_set) = self.chat_set_map.get_mut(&id) {
            let index = mode as usize;
            if index < chat_set.mode.len() {
                chat_set.mode[index] = false;
            }
        }
    }

    pub fn get_chat_color(&self, i_type: ChatType) -> Color {
        let index = i_type as usize;
        if index < self.chat_colors.len() {
            self.chat_colors[index]
        } else {
            Color::new(0.0, 0.0, 1.0, 1.0)
        }
    }

    pub fn append_chat(&mut self, i_type: ChatType, chat: String) {
        let mut chat_line = ChatLine::new(i_type, chat);
        chat_line.appended_time = self.current_time;
        let color = self.get_chat_color(i_type);
        chat_line.set_color_all(color);

        let chat_line_rc = Rc::new(RefCell::new(chat_line));

        self.chat_line_deque.push_back(Rc::clone(&chat_line_rc));
        if self.chat_line_deque.len() > CHAT_LINE_MAX_NUM {
            self.chat_line_deque.pop_front();
        }

        // We can't borrow self as mutable twice in the loop directly if we need to call arrange_showing_chat,
        // so we collect the IDs of chat sets that need arranging.
        let mut sets_to_arrange = Vec::new();

        for (&id, chat_set) in &mut self.chat_set_map {
            if chat_set.board_state == BoardState::Edit {
                sets_to_arrange.push(id);
            } else {
                if chat_set.check_mode(i_type) {
                    chat_set.showing_chat_line_list.push_back(Rc::clone(&chat_line_rc));
                    if chat_set.showing_chat_line_list.len() > CHAT_LINE_MAX_NUM {
                        chat_set.showing_chat_line_list.pop_front();
                    }
                }
            }
        }

        for id in sets_to_arrange {
            self.arrange_showing_chat(id);
        }
    }

    pub fn append_chat_with_delay(&mut self, i_type: ChatType, chat: String, delay: u32) {
        self.wait_chat_list.push_back(WaitChat {
            i_type,
            chat,
            appending_time: delay,
        });
    }

    pub fn arrange_showing_chat(&mut self, id: i32) {
        if let Some(chat_set) = self.chat_set_map.get_mut(&id) {
            chat_set.showing_chat_line_list.clear();
            for chat_line_rc in self.chat_line_deque.iter().rev() {
                let chat_line = chat_line_rc.borrow();
                if chat_set.check_mode(chat_line.i_type) {
                    chat_set.showing_chat_line_list.push_front(Rc::clone(chat_line_rc));
                    if chat_set.showing_chat_line_list.len() >= CHAT_LINE_MAX_NUM {
                        break;
                    }
                }
            }
        }
    }
}

impl UiChatWindow {
    pub fn append_input_history(&mut self, command: String) {
        if !command.is_empty() {
            self.input_history.push_back(command);
            if self.input_history.len() > CHAT_LINE_MAX_NUM {
                self.input_history.pop_front();
            }
        }
    }

    pub fn update(&mut self, id: i32, current_time: f32) {
        self.current_time = current_time;
        let board_state = if let Some(chat_set) = self.chat_set_map.get(&id) {
            chat_set.board_state
        } else {
            return;
        };

        match board_state {
            BoardState::View => self.update_view_mode(id, current_time),
            BoardState::Edit => self.update_edit_mode(id),
            BoardState::Log => self.update_log_mode(id),
        }
    }

    pub fn update_view_mode(&mut self, id: i32, current_time: f32) {
        if let Some(chat_set) = self.chat_set_map.get_mut(&id) {
            let mut i = 0;
            while i < chat_set.showing_chat_line_list.len() {
                let remove = {
                    let mut chat_line = chat_set.showing_chat_line_list[i].borrow_mut();
                    
                    let elapsed_time = current_time - chat_line.appended_time;
                    let line_index = chat_set.showing_chat_line_list.len() - i;

                    if elapsed_time >= START_DISAPPEARING_TIME || line_index >= MAX_LINE_COUNT {
                        let color = chat_line.get_color_mut(0);
                        color.a -= color.a / 10.0;
                        color.a <= 0.1
                    } else {
                        false
                    }
                };

                if remove {
                    chat_set.showing_chat_line_list.remove(i);
                } else {
                    i += 1;
                }
            }
        }
    }

    pub fn update_edit_mode(&mut self, id: i32) {
        let (visible_line_count, editable_line_count) = (self.get_visible_line_count(id), self.get_editable_line_count(id));
        let alpha_line = std::cmp::max(0, visible_line_count as i32 - editable_line_count as i32 + 2) as usize;

        if let Some(chat_set) = self.chat_set_map.get_mut(&id) {
            let mut alpha = 0.0_f32;
            let alpha_step = if alpha_line > 0 { 1.0 / alpha_line as f32 } else { 0.0 };

            for (i, chat_line_rc) in chat_set.showing_chat_line_list.iter().enumerate() {
                let mut chat_line = chat_line_rc.borrow_mut();
                let color = chat_line.get_color_mut(0); // Using 0 as fallback ID for color

                if i < alpha_line {
                    color.a += (alpha - color.a) / 10.0;
                    alpha = (alpha + alpha_step).min(1.0);
                } else {
                    color.a = (color.a + 0.05).min(1.0);
                }
            }
        }
    }

    pub fn update_log_mode(&mut self, _id: i32) {
        // Logging mode rendering updates would be handled here
    }

    pub fn get_visible_line_count(&self, id: i32) -> usize {
        if let Some(chat_set) = self.chat_set_map.get(&id) {
            chat_set.showing_chat_line_list.len()
        } else {
            0
        }
    }

    pub fn get_editable_line_count(&self, id: i32) -> usize {
        if let Some(chat_set) = self.chat_set_map.get(&id) {
            if chat_set.step > 0 {
                (chat_set.height / chat_set.step) as usize
            } else {
                0
            }
        } else {
            0
        }
    }

    pub fn get_line_count(&self, id: i32) -> usize {
        self.get_visible_line_count(id)
    }

    pub fn get_line_step(&self, id: i32) -> i32 {
        if let Some(chat_set) = self.chat_set_map.get(&id) {
            chat_set.step
        } else {
            0
        }
    }

    pub fn ignore_character(&mut self, name: &str) {
        if self.ignore_character_set.contains(name) {
            self.ignore_character_set.remove(name);
        } else {
            self.ignore_character_set.insert(name.to_string());
        }
    }

    pub fn is_ignore_character(&self, name: &str) -> bool {
        self.ignore_character_set.contains(name)
    }

    pub fn create_whisper(&mut self, name: &str) {
        self.whisper_map.insert(name.to_string(), Whisper::new());
    }

    pub fn append_whisper(&mut self, i_type: WhisperType, name: &str, chat: &str) {
        if !self.whisper_map.contains_key(name) {
            self.create_whisper(name);
        }

        if let Some(whisper) = self.whisper_map.get_mut(name) {
            let mut chat_line = ChatLine::new(ChatType::Whisper, chat.to_string());
            let color = match i_type {
                WhisperType::System => Color::new(1.0, 0.785, 0.785, 1.0),
                WhisperType::Gm => Color::new(1.0, 0.632, 0.0, 1.0),
                _ => Color::new(1.0, 1.0, 1.0, 1.0),
            };
            chat_line.set_color_all(color);

            whisper.chat_line_deque.push_back(Rc::new(RefCell::new(chat_line)));
        }
    }

    pub fn clear_whisper(&mut self, name: &str) {
        self.whisper_map.remove(name);
    }

    pub fn get_whisper(&self, name: &str) -> Option<&Whisper> {
        self.whisper_map.get(name)
    }
}
