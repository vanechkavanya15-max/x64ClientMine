#![allow(non_snake_case)]
#![allow(dead_code)]

use std::mem::size_of;
use std::ptr;

pub const QUEST_SEND_IS_BEGIN: u8 = 1 << 0;
pub const QUEST_SEND_TITLE: u8 = 1 << 1;
pub const QUEST_SEND_CLOCK_NAME: u8 = 1 << 2;
pub const QUEST_SEND_CLOCK_VALUE: u8 = 1 << 3;
pub const QUEST_SEND_COUNTER_NAME: u8 = 1 << 4;
pub const QUEST_SEND_COUNTER_VALUE: u8 = 1 << 5;
pub const QUEST_SEND_ICON_FILE: u8 = 1 << 6;

pub const QUEST_INPUT_STRING_MAX_NUM: usize = 64;

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketGCScript {
    pub header: u16,
    pub length: u16,
    pub skin: u8,
    pub src_size: u16,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketGCQuestInfo {
    pub header: u16,
    pub length: u16,
    pub index: u16,
    pub flag: u8,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketCGScriptAnswer {
    pub header: u16,
    pub length: u16,
    pub answer: u8,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketCGScriptButton {
    pub header: u16,
    pub length: u16,
    pub idx: u32,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketCGQuestInputString {
    pub header: u16,
    pub length: u16,
    pub szString: [u8; QUEST_INPUT_STRING_MAX_NUM + 1],
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketGCQuestConfirm {
    pub header: u16,
    pub length: u16,
    pub msg: [u8; 64 + 1],
    pub timeout: i32,
    pub requestPID: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum QuestPacketType {
    #[default]
    None,
    Begin,
    Update,
    End,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ParsedQuestInfo {
    pub index: u16,
    pub packet_type: QuestPacketType,
    pub title: Option<String>,
    pub clock_name: Option<String>,
    pub clock_value: Option<i32>,
    pub counter_name: Option<String>,
    pub counter_value: Option<i32>,
    pub icon_file: Option<String>,
}

fn read_string_from_slice(data: &[u8], offset: &mut usize, max_len: usize) -> Option<String> {
    if *offset + max_len > data.len() {
        return None;
    }
    let slice = &data[*offset..*offset + max_len];
    *offset += max_len;

    // C++ strings are null-terminated or have junk after the null byte.
    // Metin2 reads exact sizes and null-terminates the last byte.
    let mut actual_len = max_len;
    for (i, &byte) in slice.iter().enumerate() {
        if byte == 0 {
            actual_len = i;
            break;
        }
    }
    Some(String::from_utf8_lossy(&slice[..actual_len]).to_string())
}

fn read_i32_from_slice(data: &[u8], offset: &mut usize) -> Option<i32> {
    if *offset + size_of::<i32>() > data.len() {
        return None;
    }
    let val = i32::from_ne_bytes([
        data[*offset],
        data[*offset + 1],
        data[*offset + 2],
        data[*offset + 3],
    ]);
    *offset += size_of::<i32>();
    Some(val)
}

fn read_u8_from_slice(data: &[u8], offset: &mut usize) -> Option<u8> {
    if *offset + size_of::<u8>() > data.len() {
        return None;
    }
    let val = data[*offset];
    *offset += size_of::<u8>();
    Some(val)
}

pub fn parse_script_packet(data: &[u8]) -> Result<(TPacketGCScript, String), &'static str> {
    if data.len() < size_of::<TPacketGCScript>() {
        return Err("RecvScriptPacket_RecvError");
    }

    let packet_ptr = data.as_ptr() as *const TPacketGCScript;
    let packet = unsafe { ptr::read_unaligned(packet_ptr) };

    if (packet.length as usize) < size_of::<TPacketGCScript>() {
        return Err("RecvScriptPacket_SizeError");
    }

    if data.len() < (packet.length as usize) {
        return Err("RecvScriptPacket_RecvError (String)");
    }

    let str_len = packet.length as usize - size_of::<TPacketGCScript>();
    let str_data = &data[size_of::<TPacketGCScript>()..size_of::<TPacketGCScript>() + str_len];

    // C++ behavior: str[str.size()-1] = '\0';
    let mut actual_len = str_len;
    if actual_len > 0 && str_data[actual_len - 1] == 0 {
        actual_len -= 1;
    }
    
    // Also, handle early null terminators if present in the buffer
    for (i, &b) in str_data.iter().enumerate() {
        if i < actual_len && b == 0 {
            actual_len = i;
            break;
        }
    }

    let script_str = String::from_utf8_lossy(&str_data[..actual_len]).to_string();

    Ok((packet, script_str))
}

pub fn parse_quest_info_packet(data: &[u8]) -> Result<ParsedQuestInfo, &'static str> {
    if data.len() < size_of::<TPacketGCQuestInfo>() {
        return Err("Recv Quest Info Packet Error #1");
    }

    let packet_ptr = data.as_ptr() as *const TPacketGCQuestInfo;
    let packet = unsafe { ptr::read_unaligned(packet_ptr) };

    let mut offset = size_of::<TPacketGCQuestInfo>();
    let flag = packet.flag;

    let mut result = ParsedQuestInfo {
        index: packet.index,
        packet_type: QuestPacketType::None,
        ..Default::default()
    };

    if (flag & QUEST_SEND_IS_BEGIN) != 0 {
        if let Some(is_begin) = read_u8_from_slice(data, &mut offset) {
            if is_begin != 0 {
                result.packet_type = QuestPacketType::Begin;
            } else {
                result.packet_type = QuestPacketType::End;
            }
        } else {
            return Err("Recv Quest Info Packet Error (isBegin)");
        }
    } else {
        result.packet_type = QuestPacketType::Update;
    }

    if (flag & QUEST_SEND_TITLE) != 0 {
        if let Some(title) = read_string_from_slice(data, &mut offset, 30) {
            result.title = Some(title);
        } else {
            return Err("Recv Quest Info Packet Error (Title)");
        }
    }
    if (flag & QUEST_SEND_CLOCK_NAME) != 0 {
        if let Some(clock_name) = read_string_from_slice(data, &mut offset, 16) {
            result.clock_name = Some(clock_name);
        } else {
            return Err("Recv Quest Info Packet Error (ClockName)");
        }
    }
    if (flag & QUEST_SEND_CLOCK_VALUE) != 0 {
        if let Some(clock_value) = read_i32_from_slice(data, &mut offset) {
            result.clock_value = Some(clock_value);
        } else {
            return Err("Recv Quest Info Packet Error (ClockValue)");
        }
    }
    if (flag & QUEST_SEND_COUNTER_NAME) != 0 {
        if let Some(counter_name) = read_string_from_slice(data, &mut offset, 16) {
            result.counter_name = Some(counter_name);
        } else {
            return Err("Recv Quest Info Packet Error (CounterName)");
        }
    }
    if (flag & QUEST_SEND_COUNTER_VALUE) != 0 {
        if let Some(counter_value) = read_i32_from_slice(data, &mut offset) {
            result.counter_value = Some(counter_value);
        } else {
            return Err("Recv Quest Info Packet Error (CounterValue)");
        }
    }
    if (flag & QUEST_SEND_ICON_FILE) != 0 {
        if let Some(icon_file) = read_string_from_slice(data, &mut offset, 24) {
            result.icon_file = Some(icon_file);
        } else {
            return Err("Recv Quest Info Packet Error (IconFile)");
        }
    }

    Ok(result)
}

pub fn parse_quest_confirm_packet(data: &[u8]) -> Result<TPacketGCQuestConfirm, &'static str> {
    if data.len() < size_of::<TPacketGCQuestConfirm>() {
        return Err("RecvQuestConfirmPacket Error");
    }
    let packet_ptr = data.as_ptr() as *const TPacketGCQuestConfirm;
    Ok(unsafe { ptr::read_unaligned(packet_ptr) })
}

pub fn build_script_answer_packet(answer: u8) -> TPacketCGScriptAnswer {
    TPacketCGScriptAnswer {
        header: 0x0901, // CG::SCRIPT_ANSWER
        length: size_of::<TPacketCGScriptAnswer>() as u16,
        answer,
    }
}

pub fn build_script_button_packet(idx: u32) -> TPacketCGScriptButton {
    TPacketCGScriptButton {
        header: 0x0902, // CG::SCRIPT_BUTTON
        length: size_of::<TPacketCGScriptButton>() as u16,
        idx,
    }
}

pub fn build_quest_input_string_packet(input_string: &str) -> TPacketCGQuestInputString {
    let mut packet = TPacketCGQuestInputString {
        header: 0x0904, // CG::QUEST_INPUT_STRING
        length: size_of::<TPacketCGQuestInputString>() as u16,
        szString: [0; QUEST_INPUT_STRING_MAX_NUM + 1],
    };
    
    let bytes = input_string.as_bytes();
    let copy_len = bytes.len().min(QUEST_INPUT_STRING_MAX_NUM);
    packet.szString[..copy_len].copy_from_slice(&bytes[..copy_len]);
    
    packet
}
