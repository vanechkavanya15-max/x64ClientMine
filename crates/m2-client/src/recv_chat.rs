use std::convert::TryFrom;

pub const CHARACTER_NAME_MAX_LEN: usize = 64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ChatType {
    Talking = 0,
    Info = 1,
    Notice = 2,
    Party = 3,
    Guild = 4,
    Command = 5,
    Shout = 6,
    Whisper = 7,
    BigNotice = 8,
    MaxNum = 9,
}

impl TryFrom<u8> for ChatType {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ChatType::Talking),
            1 => Ok(ChatType::Info),
            2 => Ok(ChatType::Notice),
            3 => Ok(ChatType::Party),
            4 => Ok(ChatType::Guild),
            5 => Ok(ChatType::Command),
            6 => Ok(ChatType::Shout),
            7 => Ok(ChatType::Whisper),
            8 => Ok(ChatType::BigNotice),
            9 => Ok(ChatType::MaxNum),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum WhisperType {
    Chat = 0,
    NotExist = 1,
    TargetBlocked = 2,
    SenderBlocked = 3,
    ErrorType = 4,
    Gm = 5,
    System = 0xFF,
}

impl TryFrom<u8> for WhisperType {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(WhisperType::Chat),
            1 => Ok(WhisperType::NotExist),
            2 => Ok(WhisperType::TargetBlocked),
            3 => Ok(WhisperType::SenderBlocked),
            4 => Ok(WhisperType::ErrorType),
            5 => Ok(WhisperType::Gm),
            0xFF => Ok(WhisperType::System),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChatMessage {
    pub chat_type: ChatType,
    pub vid: u32,
    pub empire: u8,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhisperMessage {
    pub whisper_type: WhisperType,
    pub from: String,
    pub text: String,
}

#[derive(Debug, Clone)]
pub enum ParseError {
    BufferTooSmall,
    InvalidType,
}

#[repr(C, packed)]
struct TPacketGCChat {
    header: u16,
    length: u16,
    chat_type: u8,
    dw_vid: u32,
    b_empire: u8,
}

#[repr(C, packed)]
struct TPacketGCWhisper {
    header: u16,
    length: u16,
    b_type: u8,
    sz_name_from: [u8; CHARACTER_NAME_MAX_LEN + 1],
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

pub fn parse_chat(data: &[u8]) -> Result<(usize, ChatMessage), ParseError> {
    let header_size = std::mem::size_of::<TPacketGCChat>();
    if data.len() < header_size {
        return Err(ParseError::BufferTooSmall);
    }

    let ptr = data.as_ptr() as *const TPacketGCChat;
    let packet = unsafe { &*ptr };

    let length = u16::from_le(packet.length) as usize;
    if data.len() < length || length < header_size {
        return Err(ParseError::BufferTooSmall);
    }

    let chat_type_raw = packet.chat_type;
    let chat_type = ChatType::try_from(chat_type_raw).map_err(|_| ParseError::InvalidType)?;
    
    let text_data = &data[header_size..length];
    
    let text = String::from_utf8_lossy(text_data).into_owned();
    let text = text.trim_end_matches('\0');
    let text = sanitize_chat_text(text);

    Ok((
        length,
        ChatMessage {
            chat_type,
            vid: u32::from_le(packet.dw_vid),
            empire: packet.b_empire,
            text,
        },
    ))
}

pub fn parse_whisper(data: &[u8]) -> Result<(usize, WhisperMessage), ParseError> {
    let header_size = std::mem::size_of::<TPacketGCWhisper>();
    if data.len() < header_size {
        return Err(ParseError::BufferTooSmall);
    }

    let ptr = data.as_ptr() as *const TPacketGCWhisper;
    let packet = unsafe { &*ptr };

    let length = u16::from_le(packet.length) as usize;
    if data.len() < length || length < header_size {
        return Err(ParseError::BufferTooSmall);
    }

    let whisper_type_raw = packet.b_type;
    let whisper_type = WhisperType::try_from(whisper_type_raw).map_err(|_| ParseError::InvalidType)?;

    let name_bytes = &packet.sz_name_from;
    let name = String::from_utf8_lossy(name_bytes)
        .trim_end_matches('\0')
        .to_string();
    let name = sanitize_chat_text(&name);

    let text_data = &data[header_size..length];
    
    let text = String::from_utf8_lossy(text_data).into_owned();
    let text = text.trim_end_matches('\0');
    let text = sanitize_chat_text(text);

    Ok((
        length,
        WhisperMessage {
            whisper_type,
            from: name,
            text,
        },
    ))
}
