use std::mem::size_of;

pub const CHAT_TYPE_TALKING: u8 = 0;
pub const CHAT_TYPE_INFO: u8 = 1;
pub const CHAT_TYPE_NOTICE: u8 = 2;
pub const CHAT_TYPE_PARTY: u8 = 3;
pub const CHAT_TYPE_GUILD: u8 = 4;
pub const CHAT_TYPE_COMMAND: u8 = 5;
pub const CHAT_TYPE_SHOUT: u8 = 6;
pub const CHAT_TYPE_WHISPER: u8 = 7;
pub const CHAT_TYPE_BIG_NOTICE: u8 = 8;
pub const CHAT_TYPE_MAX_NUM: u8 = 9;

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct TPacketGCChat {
    pub header: u16,
    pub length: u16,
    pub type_: u8,
    pub dw_vid: u32,
    pub b_empire: u8,
}

pub trait ChatContext {
    fn localize_item_links(&mut self, text: &mut String);
    fn server_command(&mut self, command: &str);

    fn get_instance_exists(&self, vid: u32) -> bool;
    fn get_instance_is_npc(&self, vid: u32) -> bool;
    fn get_instance_is_enemy(&self, vid: u32) -> bool;
    fn get_instance_is_pc(&self, vid: u32) -> bool;
    fn get_instance_empire_id(&self, vid: u32) -> u8;
    fn get_main_instance_exists(&self) -> bool;
    fn is_same_empire(&self, vid: u32) -> bool;

    fn parse_emoticon(&self, text: &str) -> Option<u32>;
    fn set_emoticon(&mut self, vid: u32, emoticon: u32);

    fn is_empire_language_enable(&self) -> bool;
    fn convert_empire_text(&mut self, empire_id: u8, text: &mut String);

    fn is_enable_chat_insult_filter(&self) -> bool;
    fn filter_insult(&mut self, text: &mut String);

    fn register_chat_tail(&mut self, vid: u32, text: &str);
    fn append_chat(&mut self, chat_type: u8, text: &str);

    fn set_tip_message(&mut self, text: &str);
    fn set_big_message(&mut self, text: &str);
}

pub fn recv_chat_packet(data: &[u8], ctx: &mut impl ChatContext) -> Result<usize, &'static str> {
    if data.len() < size_of::<TPacketGCChat>() {
        return Err("Packet too small for header");
    }

    let packet_ptr = data.as_ptr() as *const TPacketGCChat;
    let packet = unsafe { std::ptr::read_unaligned(packet_ptr) };

    let length = u16::from_le(packet.length) as usize;
    if data.len() < length {
        return Err("Packet payload incomplete");
    }

    if length < size_of::<TPacketGCChat>() {
        return Err("Invalid packet length");
    }

    let payload_size = length - size_of::<TPacketGCChat>();
    let payload = &data[size_of::<TPacketGCChat>()..length];

    let null_idx = payload.iter().position(|&b| b == 0).unwrap_or(payload_size);
    let mut buf = String::from_utf8_lossy(&payload[..null_idx]).into_owned();

    ctx.localize_item_links(&mut buf);

    if packet.type_ >= CHAT_TYPE_MAX_NUM {
        return Ok(length);
    }

    if packet.type_ == CHAT_TYPE_COMMAND {
        ctx.server_command(&buf);
        return Ok(length);
    }

    let dw_vid = u32::from_le(packet.dw_vid);

    if dw_vid != 0 {
        if !ctx.get_instance_exists(dw_vid) {
            return Ok(length);
        }

        let mut line = String::new();

        match packet.type_ {
            CHAT_TYPE_TALKING | CHAT_TYPE_PARTY | CHAT_TYPE_GUILD | CHAT_TYPE_SHOUT | CHAT_TYPE_WHISPER => {
                let p_idx = match buf.find(':') {
                    Some(idx) => {
                        let mut next_idx = idx + 2;
                        while next_idx < buf.len() && !buf.is_char_boundary(next_idx) {
                            next_idx += 1;
                        }
                        if next_idx > buf.len() {
                            buf.len()
                        } else {
                            next_idx
                        }
                    }
                    None => 0,
                };

                let mut p_text = buf[p_idx..].to_string();

                if let Some(emoticon_id) = ctx.parse_emoticon(&p_text) {
                    ctx.set_emoticon(dw_vid, emoticon_id);
                    return Ok(length);
                } else {
                    if ctx.is_empire_language_enable() {
                        if ctx.get_main_instance_exists() {
                            if !ctx.is_same_empire(dw_vid) {
                                let empire_id = ctx.get_instance_empire_id(dw_vid);
                                ctx.convert_empire_text(empire_id, &mut p_text);
                            }
                        }
                    }

                    if ctx.is_enable_chat_insult_filter() {
                        if !ctx.get_instance_is_npc(dw_vid) && !ctx.get_instance_is_enemy(dw_vid) {
                            ctx.filter_insult(&mut p_text);
                        }
                    }

                    line = p_text.clone();
                    buf = format!("{}{}", &buf[..p_idx], p_text);
                }
            }
            _ => {
                line = buf.clone();
            }
        }

        if packet.type_ != CHAT_TYPE_SHOUT {
            ctx.register_chat_tail(dw_vid, &line);
        }

        if ctx.get_instance_is_pc(dw_vid) {
            ctx.append_chat(packet.type_, &buf);
        }
    } else {
        if packet.type_ == CHAT_TYPE_NOTICE {
            ctx.set_tip_message(&buf);
        } else if packet.type_ == CHAT_TYPE_BIG_NOTICE {
            ctx.set_big_message(&buf);
        } else if packet.type_ == CHAT_TYPE_SHOUT {
            if let Some(idx) = buf.find(':') {
                if ctx.is_enable_chat_insult_filter() {
                    let mut p_text = buf[idx..].to_string();
                    ctx.filter_insult(&mut p_text);
                    buf = format!("{}{}", &buf[..idx], p_text);
                }
            }
        }

        ctx.append_chat(packet.type_, &buf);
    }

    Ok(length)
}
