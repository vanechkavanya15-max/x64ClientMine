#![allow(dead_code)]

pub const TEXT_TAG_PLAIN: i32 = 0;
pub const TEXT_TAG_TAG: i32 = 1;
pub const TEXT_TAG_COLOR: i32 = 2;
pub const TEXT_TAG_HYPERLINK_START: i32 = 3;
pub const TEXT_TAG_HYPERLINK_END: i32 = 4;
pub const TEXT_TAG_RESTORE_COLOR: i32 = 5;

pub fn get_text_tag(src: &[u16], max_len: i32, tag_len: &mut i32, extra_info: &mut Vec<u16>) -> i32 {
    *tag_len = 1;

    if max_len < 2 || src.len() < 2 || src[0] != b'|' as u16 {
        return TEXT_TAG_PLAIN;
    }

    let cur = src[1];

    if cur == b'c' as u16 {
        if max_len < 10 || src.len() < 10 {
            return TEXT_TAG_PLAIN;
        }

        *tag_len = 10;
        extra_info.clear();
        extra_info.extend_from_slice(&src[2..10]);
        return TEXT_TAG_COLOR;
    } else if cur == b'|' as u16 {
        *tag_len = 2;
        return TEXT_TAG_TAG;
    } else if cur == b'r' as u16 {
        *tag_len = 2;
        return TEXT_TAG_RESTORE_COLOR;
    } else if cur == b'H' as u16 {
        *tag_len = 2;
        return TEXT_TAG_HYPERLINK_START;
    } else if cur == b'h' as u16 {
        *tag_len = 2;
        return TEXT_TAG_HYPERLINK_END;
    }

    TEXT_TAG_PLAIN
}

pub fn get_text_tag_output_string(src: &[u16], src_len: i32) -> Vec<u16> {
    let mut len = 0;
    let mut dst = Vec::new();
    let mut extra_info = Vec::new();
    let mut hyperlink_step = 0;

    let mut i = 0;
    while i < src_len {
        let idx = i as usize;
        if idx >= src.len() {
            break;
        }
        let tag = get_text_tag(&src[idx..], src_len - i, &mut len, &mut extra_info);

        if tag == TEXT_TAG_PLAIN || tag == TEXT_TAG_TAG {
            if hyperlink_step == 0 || hyperlink_step == 2 {
                dst.push(src[idx]);
            }
        } else if tag == TEXT_TAG_HYPERLINK_START {
            hyperlink_step = 1;
        } else if tag == TEXT_TAG_HYPERLINK_END {
            if hyperlink_step == 1 {
                hyperlink_step = 2;
            } else if hyperlink_step == 2 {
                hyperlink_step = 0;
            }
        }

        i += len;
    }
    dst
}

pub fn get_text_tag_internal_pos_from_render_pos(src: &[u16], src_len: i32, offset: i32) -> i32 {
    let mut len = 0;
    let mut extra_info = Vec::new();
    let mut output_len = 0;
    let mut hyperlink_step = 0;
    let mut color_tag = false;
    let mut internal_offset = 0;

    let mut i = 0;
    while i < src_len {
        let idx = i as usize;
        if idx >= src.len() {
            break;
        }
        let tag = get_text_tag(&src[idx..], src_len - i, &mut len, &mut extra_info);

        if tag == TEXT_TAG_COLOR {
            color_tag = true;
            internal_offset = i;
        } else if tag == TEXT_TAG_RESTORE_COLOR {
            color_tag = false;
        } else if tag == TEXT_TAG_PLAIN || tag == TEXT_TAG_TAG {
            if hyperlink_step == 0 || hyperlink_step == 2 {
                if !color_tag {
                    internal_offset = i;
                }

                if offset <= output_len {
                    return internal_offset;
                }

                output_len += 1;
            }
        } else if tag == TEXT_TAG_HYPERLINK_START {
            hyperlink_step = 1;
        } else if tag == TEXT_TAG_HYPERLINK_END {
            if hyperlink_step == 1 {
                hyperlink_step = 2;
            } else if hyperlink_step == 2 {
                hyperlink_step = 0;
            }
        }

        i += len;
    }

    if offset >= output_len {
        return src_len;
    }

    internal_offset
}

pub fn get_text_tag_output_len(src: &[u16], src_len: i32) -> i32 {
    let mut len = 0;
    let mut extra_info = Vec::new();
    let mut output_len = 0;
    let mut hyperlink_step = 0;

    let mut i = 0;
    while i < src_len {
        let idx = i as usize;
        if idx >= src.len() {
            break;
        }
        let tag = get_text_tag(&src[idx..], src_len - i, &mut len, &mut extra_info);

        if tag == TEXT_TAG_PLAIN || tag == TEXT_TAG_TAG {
            if hyperlink_step == 0 {
                output_len += 1;
            }
        } else if tag == TEXT_TAG_HYPERLINK_START {
            hyperlink_step = 1;
        } else if tag == TEXT_TAG_HYPERLINK_END {
            hyperlink_step = 0;
        }

        i += len;
    }
    output_len
}

pub fn find_color_tag_start_position(src_up_to_cur: &[u16], src_len: i32) -> i32 {
    if src_len < 2 || src_up_to_cur.is_empty() {
        return 0;
    }

    let mut cur = src_up_to_cur.len() - 1;

    if src_up_to_cur[cur] == b'r' as u16 && cur >= 1 && src_up_to_cur[cur - 1] == b'|' as u16 {
        let mut len = src_len;

        if len >= 2 && cur >= 2 && src_up_to_cur[cur - 2] == b'|' as u16 {
            return 1;
        }

        if cur >= 2 {
            cur -= 2;
            len -= 2;
        } else {
            return src_len;
        }

        while len > 1 {
            if src_up_to_cur[cur] == b'c' as u16 && cur >= 1 && src_up_to_cur[cur - 1] == b'|' as u16 {
                return (src_up_to_cur.len() - 1 - cur) as i32 + 1;
            }

            if cur >= 1 {
                cur -= 1;
                len -= 1;
            } else {
                break;
            }
        }
        return src_len;
    } else if src_up_to_cur[cur] == b'|' as u16 && cur >= 1 && src_up_to_cur[cur - 1] == b'|' as u16 {
        return 1;
    }

    0
}

pub fn find_color_tag_end_position(src_from_cur: &[u16], src_len: i32) -> i32 {
    let mut cur = 0;

    if src_len >= 4 && src_from_cur.get(cur) == Some(&(b'|' as u16)) && src_from_cur.get(cur + 1) == Some(&(b'c' as u16)) {
        let mut left = src_len - 2;
        cur += 2;

        while left > 1 {
            if src_from_cur.get(cur) == Some(&(b'|' as u16)) && src_from_cur.get(cur + 1) == Some(&(b'r' as u16)) {
                return cur as i32 + 1;
            }

            left -= 1;
            cur += 1;
        }
    } else if src_len >= 2 && src_from_cur.get(cur) == Some(&(b'|' as u16)) && src_from_cur.get(cur + 1) == Some(&(b'|' as u16)) {
        return 1;
    }

    0
}
