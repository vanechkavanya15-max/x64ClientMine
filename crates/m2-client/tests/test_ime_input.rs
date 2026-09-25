use m2_client::ime_input::ImeInput;

#[test]
fn test_skeleton_ime_input() {
    let instance = ImeInput::new();
    assert!(instance.is_ready());
}

#[test]
fn test_insert_and_get_text() {
    let mut ime = ImeInput::new();
    ime.on_char('a');
    ime.on_char('b');
    ime.on_char('c');
    assert_eq!(ime.get_text(), "abc");
    assert_eq!(ime.get_raw_text(), "abc");
    assert_eq!(ime.get_cursor_position(), 3);
}

#[test]
fn test_cursor_movement_and_insertion() {
    let mut ime = ImeInput::new();
    ime.paste_text("abc");
    ime.move_left(); // kursor miedzy b i c
    ime.on_char('x');
    assert_eq!(ime.get_text(), "abxc");
    ime.move_home();
    ime.on_char('z');
    assert_eq!(ime.get_text(), "zabxc");
    ime.move_end();
    ime.on_char('d');
    assert_eq!(ime.get_text(), "zabxcd");
    
    // move left multiple times
    ime.move_left();
    ime.move_left();
    assert_eq!(ime.get_cursor_position(), 4); // po 'x'
}

#[test]
fn test_delete_and_backspace() {
    let mut ime = ImeInput::new();
    ime.paste_text("hello");
    ime.backspace(); // usun 'o'
    assert_eq!(ime.get_text(), "hell");
    
    ime.move_left(); // miedzy 'l' i 'l'
    ime.delete(); // usun 'l'
    assert_eq!(ime.get_text(), "hel");
    
    ime.move_home();
    ime.delete(); // usun 'h'
    assert_eq!(ime.get_text(), "el");
    
    ime.backspace(); // nic nie powinno sie stac, kursor na poczatku
    assert_eq!(ime.get_text(), "el");
}

#[test]
fn test_password_mode() {
    let mut ime = ImeInput::new();
    ime.set_password_mode(true);
    ime.paste_text("secret");
    assert_eq!(ime.get_text(), "******");
    assert_eq!(ime.get_raw_text(), "secret");
}

#[test]
fn test_number_mode() {
    let mut ime = ImeInput::new();
    ime.set_number_mode(true);
    ime.paste_text("123abc456");
    assert_eq!(ime.get_text(), "123456");
}

#[test]
fn test_max_len() {
    let mut ime = ImeInput::new();
    ime.set_max_len(3);
    ime.paste_text("hello");
    assert_eq!(ime.get_text(), "hel");
    
    // Przekroczenie przez wstawianie
    ime.move_home();
    ime.on_char('a');
    assert_eq!(ime.get_text(), "hel"); // Nic sie nie dodalo
}

#[test]
fn test_set_text() {
    let mut ime = ImeInput::new();
    ime.set_max_len(4);
    ime.set_text("abcdef");
    assert_eq!(ime.get_text(), "abcd");
    assert_eq!(ime.get_cursor_position(), 4);
}

#[test]
fn test_utf8_support() {
    let mut ime = ImeInput::new();
    ime.paste_text("zażółć"); // chociaz wg polecenia z polskimi to moga byc diakrytyki z klawiatury
    ime.move_left();
    ime.on_char('x');
    assert_eq!(ime.get_text(), "zażółxć");
    
    ime.move_left();
    ime.backspace(); // usuwa ł
    assert_eq!(ime.get_text(), "zażóxć");
    
    ime.delete(); // usuwa x
    assert_eq!(ime.get_text(), "zażóć");
}

#[test]
fn test_history() {
    let mut ime = ImeInput::new();
    ime.paste_text("cmd1");
    ime.add_to_history();
    ime.clear();
    
    ime.paste_text("cmd2");
    ime.add_to_history();
    ime.clear();
    
    assert_eq!(ime.get_history_prev(), Some("cmd2".to_string()));
    assert_eq!(ime.get_history_prev(), Some("cmd1".to_string()));
    assert_eq!(ime.get_history_prev(), None);
    
    assert_eq!(ime.get_history_next(), Some("cmd2".to_string()));
    assert_eq!(ime.get_history_next(), Some("".to_string()));
    assert_eq!(ime.get_history_next(), None);
}
