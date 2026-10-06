#[path = "../src/recv_chat_message_handler.rs"]
mod recv_chat_message_handler;

use recv_chat_message_handler::*;

#[derive(Default)]
struct MockChatContext {
    appended_chats: Vec<(u8, String)>,
    registered_tails: Vec<(u32, String)>,
    server_commands: Vec<String>,
    tip_messages: Vec<String>,
    big_messages: Vec<String>,
    emoticons_set: Vec<(u32, u32)>,

    // configuration
    is_empire_language_enable: bool,
    is_enable_chat_insult_filter: bool,
}

impl ChatContext for MockChatContext {
    fn localize_item_links(&mut self, _text: &mut String) {}

    fn server_command(&mut self, command: &str) {
        self.server_commands.push(command.to_string());
    }

    fn get_instance_exists(&self, _vid: u32) -> bool { true }
    fn get_instance_is_npc(&self, _vid: u32) -> bool { false }
    fn get_instance_is_enemy(&self, _vid: u32) -> bool { false }
    fn get_instance_is_pc(&self, _vid: u32) -> bool { true }
    fn get_instance_empire_id(&self, _vid: u32) -> u8 { 1 }
    fn get_main_instance_exists(&self) -> bool { true }
    fn is_same_empire(&self, _vid: u32) -> bool { true }

    fn parse_emoticon(&self, text: &str) -> Option<u32> {
        if text.starts_with("(emoticon)") {
            Some(1)
        } else {
            None
        }
    }

    fn set_emoticon(&mut self, vid: u32, emoticon: u32) {
        self.emoticons_set.push((vid, emoticon));
    }

    fn is_empire_language_enable(&self) -> bool { self.is_empire_language_enable }

    fn convert_empire_text(&mut self, _empire_id: u8, text: &mut String) {
        *text = format!("(empire){}", text);
    }

    fn is_enable_chat_insult_filter(&self) -> bool { self.is_enable_chat_insult_filter }

    fn filter_insult(&mut self, text: &mut String) {
        *text = text.replace("badword", "***");
    }

    fn register_chat_tail(&mut self, vid: u32, text: &str) {
        self.registered_tails.push((vid, text.to_string()));
    }

    fn append_chat(&mut self, chat_type: u8, text: &str) {
        self.appended_chats.push((chat_type, text.to_string()));
    }

    fn set_tip_message(&mut self, text: &str) {
        self.tip_messages.push(text.to_string());
    }

    fn set_big_message(&mut self, text: &str) {
        self.big_messages.push(text.to_string());
    }
}

fn create_packet(chat_type: u8, dw_vid: u32, text: &str) -> Vec<u8> {
    let mut data = vec![0; std::mem::size_of::<TPacketGCChat>()];
    let packet = TPacketGCChat {
        header: 0,
        length: (std::mem::size_of::<TPacketGCChat>() + text.len()) as u16,
        type_: chat_type,
        dw_vid,
        b_empire: 0,
    };
    unsafe {
        std::ptr::write_unaligned(data.as_mut_ptr() as *mut TPacketGCChat, packet);
    }
    data.extend_from_slice(text.as_bytes());
    data
}

#[test]
fn test_command() {
    let mut ctx = MockChatContext::default();
    let data = create_packet(CHAT_TYPE_COMMAND, 0, "killall");
    let res = recv_chat_packet(&data, &mut ctx);
    assert!(res.is_ok());
    assert_eq!(ctx.server_commands.len(), 1);
    assert_eq!(ctx.server_commands[0], "killall");
}

#[test]
fn test_normal_chat_vid() {
    let mut ctx = MockChatContext::default();
    let data = create_packet(CHAT_TYPE_TALKING, 123, "Player: hello world");
    let res = recv_chat_packet(&data, &mut ctx);
    assert!(res.is_ok());
    
    assert_eq!(ctx.registered_tails.len(), 1);
    assert_eq!(ctx.registered_tails[0].0, 123);
    assert_eq!(ctx.registered_tails[0].1, "hello world"); // pointer skip after ':'

    assert_eq!(ctx.appended_chats.len(), 1);
    assert_eq!(ctx.appended_chats[0].0, CHAT_TYPE_TALKING);
    assert_eq!(ctx.appended_chats[0].1, "Player: hello world");
}

#[test]
fn test_emoticon_parsing() {
    let mut ctx = MockChatContext::default();
    let data = create_packet(CHAT_TYPE_TALKING, 123, "Player: (emoticon)");
    let res = recv_chat_packet(&data, &mut ctx);
    assert!(res.is_ok());
    
    // Returns early
    assert_eq!(ctx.emoticons_set.len(), 1);
    assert_eq!(ctx.emoticons_set[0], (123, 1));
    assert_eq!(ctx.registered_tails.len(), 0);
    assert_eq!(ctx.appended_chats.len(), 0);
}

#[test]
fn test_insult_filter() {
    let mut ctx = MockChatContext::default();
    ctx.is_enable_chat_insult_filter = true;
    let data = create_packet(CHAT_TYPE_TALKING, 123, "Player: you are a badword");
    let res = recv_chat_packet(&data, &mut ctx);
    assert!(res.is_ok());
    
    assert_eq!(ctx.registered_tails[0].1, "you are a ***");
    assert_eq!(ctx.appended_chats[0].1, "Player: you are a ***");
}

#[test]
fn test_notice_vid_0() {
    let mut ctx = MockChatContext::default();
    let data = create_packet(CHAT_TYPE_NOTICE, 0, "Server restart");
    let res = recv_chat_packet(&data, &mut ctx);
    assert!(res.is_ok());
    
    assert_eq!(ctx.tip_messages.len(), 1);
    assert_eq!(ctx.tip_messages[0], "Server restart");
    
    assert_eq!(ctx.appended_chats.len(), 1);
    assert_eq!(ctx.appended_chats[0].0, CHAT_TYPE_NOTICE);
    assert_eq!(ctx.appended_chats[0].1, "Server restart");
}

#[test]
fn test_shout_vid_0_insult() {
    let mut ctx = MockChatContext::default();
    ctx.is_enable_chat_insult_filter = true;
    let data = create_packet(CHAT_TYPE_SHOUT, 0, "Shouter: I said badword");
    let res = recv_chat_packet(&data, &mut ctx);
    assert!(res.is_ok());
    
    assert_eq!(ctx.appended_chats.len(), 1);
    assert_eq!(ctx.appended_chats[0].1, "Shouter: I said ***");
}
