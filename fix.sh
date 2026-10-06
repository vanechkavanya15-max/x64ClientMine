sed -i 's/pub const CHR_EQUIPPART_HEAD: usize = 2;/#\[allow(dead_code)\]\npub const CHR_EQUIPPART_HEAD: usize = 2;/g' crates/m2-client/src/recv_char_update_extended.rs
sed -i '1i #![allow(non_snake_case)]\n#![allow(dead_code)]\n' crates/m2-client/src/recv_char_update_extended.rs
