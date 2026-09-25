use m2_client_render::font_atlas::{FontAtlas, Glyph};

fn setup_atlas() -> FontAtlas {
    let mut atlas = FontAtlas::new(16);
    
    // Dodajemy podstawowe znaki, kazdy ma advance = 10
    let chars = ['a', 'b', 'c', 'd', 'e', ' ', '|', '1', '2', '3', '4', 'S', 'w', 'o', 'r', 'H', 'l'];
    for ch in chars.iter() {
        atlas.add_glyph(*ch, Glyph {
            width: 10,
            height: 16,
            advance: 10,
            tex_x: 0.0,
            tex_y: 0.0,
            tex_w: 1.0,
            tex_h: 1.0,
        });
    }
    
    atlas
}

#[test]
fn test_integration_font_atlas() {
    let instance = FontAtlas::new(16);
    assert!(instance.is_ready());
    assert_eq!(instance.get_line_height(), 16);
}

#[test]
fn test_measure_text_width() {
    let atlas = setup_atlas();
    
    // Normalny tekst "abc" - szerokosc 30
    assert_eq!(atlas.measure_text_width("abc"), 30);
    
    // Tekst ze spacja "a b" - szerokosc 30
    assert_eq!(atlas.measure_text_width("a b"), 30);
    
    // Tekst z tagiem koloru - tag powinien byc zignorowany przy liczeniu szerokosci
    // "a|cFFFF0000b|rc" - szerokosc dla 'a', 'b', 'c' to 30
    assert_eq!(atlas.measure_text_width("a|cFFFF0000b|rc"), 30);
    
    // Tekst z tagiem hiperlacza
    // "|Hitem:1234|ha b|h" - szerokosc dla 'a', ' ', 'b' to 30
    assert_eq!(atlas.measure_text_width("|Hitem:1234|ha b|h"), 30);
    
    // Tekst z literalnym pipe "||" - traktowane jako jeden znak '|', wiec szerokosc 10
    assert_eq!(atlas.measure_text_width("||"), 10);
    
    // Zlozony przypadek
    let complex = "Hello |cFFFF0000World|r! || |Hitem:1234|h[Sword]|h";
    // Litery majace zdefiniowany glyph: H, e, l, l, o, spacja, W(0), o, r, l, d(0), !(0), spacja, pipe(||), spacja, [(0), S, w, o, r, d(0), ](0)
    // Brakujace: W, d, !, [, d, ] => advance 0. (W setup nie ma: W, d (jest d! d=10), !, [, ])
    // Policzmy:
    // H(10) e(10) l(10) l(10) o(10) sp(10) 
    // W(0) o(10) r(10) l(10) d(10) 
    // !(0) sp(10) 
    // ||(10) sp(10) 
    // [(0) S(10) w(10) o(10) r(10) d(10) ](0)
    // Znaki, ktore Maja glyph (i advance 10):
    // H(1), e(1), l(2), o(1), sp(1) -> 6 * 10 = 60
    // o(1), r(1), l(1), d(1) -> 4 * 10 = 40
    // sp(1) -> 10
    // ||(1), sp(1) -> 20
    // S(1), w(1), o(1), r(1), d(1) -> 5 * 10 = 50
    // 60 + 40 + 10 + 20 + 50 = 180
    
    let width = atlas.measure_text_width(complex);
    assert_eq!(width, 180);
}

#[test]
fn test_layout_lines_basic() {
    let atlas = setup_atlas();
    
    // "a b c d" kazdy po 10 szerokosci, spacja tez 10.
    // Przy max 30 zmiesci sie "a b" (30), "c d" (30)
    let lines = atlas.layout_lines("a b c d", 30);
    assert_eq!(lines.len(), 2);
    assert_eq!(lines[0], "a b");
    assert_eq!(lines[1], "c d");
}

#[test]
fn test_layout_lines_with_tags() {
    let atlas = setup_atlas();
    
    // Tekst z tagiem, max width 30.
    // "a|cFFFF0000 b c|r d"
    let lines = atlas.layout_lines("a|cFFFF0000 b c|r d", 30);
    assert_eq!(lines.len(), 2);
    assert_eq!(lines[0], "a|cFFFF0000 b");
    assert_eq!(lines[1], "c|r d");
}

#[test]
fn test_layout_lines_newline() {
    let atlas = setup_atlas();
    
    let lines = atlas.layout_lines("a\nb", 30);
    assert_eq!(lines.len(), 2);
    assert_eq!(lines[0], "a");
    assert_eq!(lines[1], "b");
}

#[test]
fn test_layout_lines_long_word() {
    let atlas = setup_atlas();
    
    // "abcd" ma 40, przekracza 30, ale nie ma spacji, wiec trafi do jednej linii
    let lines = atlas.layout_lines("abcd", 30);
    assert_eq!(lines.len(), 1);
    assert_eq!(lines[0], "abcd");
}
