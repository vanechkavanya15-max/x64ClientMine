
#[repr(C, packed)]
struct TPacketGCChat {
    header: u16,
    length: u16,
    chat_type: u8,
}

#[repr(C, packed)]
struct TPacketGCDamageInfo {
    header: u16,
    length: u16,
    vid: u32,
    flag: u8,
    damage: i32,
}

// Strip color codes format like |cFFRRGGBB...|r
pub fn filter_color_tags(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '|' {
            if let Some(&next_c) = chars.peek() {
                if next_c == 'c' || next_c == 'C' {
                    chars.next();
                    // consume 8 hex chars
                    for _ in 0..8 {
                        if let Some(hc) = chars.peek() {
                            if hc.is_ascii_hexdigit() {
                                chars.next();
                            } else {
                                break;
                            }
                        }
                    }
                    continue;
                } else if next_c == 'r' || next_c == 'R' {
                    chars.next();
                    continue;
                }
            }
        }
        out.push(c);
    }
    out
}

pub fn sanitize_chat_text(input: &str) -> String {
    let filtered = filter_color_tags(input);
    filtered
        .chars()
        .filter(|c| !c.is_control() || *c == '\n' || *c == '\t')
        .collect()
}

pub fn parse_chat(data: &[u8]) -> Option<(u8, String)> {
    let header_size = std::mem::size_of::<TPacketGCChat>();
    if data.len() < header_size {
        return None;
    }

    let ptr = data.as_ptr() as *const TPacketGCChat;
    let packet = unsafe { &*ptr };

    let length = u16::from_le(packet.length) as usize;
    if data.len() < length || length < header_size {
        return None;
    }

    let chat_type = packet.chat_type;
    
    let text_data = &data[header_size..length];
    
    let text = String::from_utf8_lossy(text_data).into_owned();
    let text = text.trim_end_matches('\0');
    let text = sanitize_chat_text(text);

    Some((chat_type, text))
}

pub fn parse_damage_info(data: &[u8]) -> Option<(u32, u8, i32)> {
    let header_size = std::mem::size_of::<TPacketGCDamageInfo>();
    if data.len() < header_size {
        return None;
    }

    let ptr = data.as_ptr() as *const TPacketGCDamageInfo;
    let packet = unsafe { &*ptr };

    let length = u16::from_le(packet.length) as usize;
    if data.len() < length || length < header_size {
        return None;
    }

    let vid = u32::from_le(packet.vid);
    let flag = packet.flag;
    let damage = i32::from_le(packet.damage);

    Some((vid, flag, damage))
}

#[no_mangle]
pub extern "C" fn m2_net_parse_damage_info(
    buf: *const u8,
    len: usize,
    out_vid: *mut u32,
    out_dmg: *mut i32,
) -> bool {
    if buf.is_null() || len == 0 || out_vid.is_null() || out_dmg.is_null() {
        return false;
    }

    let data = unsafe { std::slice::from_raw_parts(buf, len) };
    if let Some((vid, _flag, damage)) = parse_damage_info(data) {
        unsafe {
            *out_vid = vid;
            *out_dmg = damage;
        }
        true
    } else {
        false
    }
}
