//! Modul font_atlas dla skrzynki m2-client-render
//! Rasteryzator czcionek i atlas glifow tekstowych
//! Pomiary szerokosci tekstu, uklad linii napisow nad postaciami i w dymkach czatu

use std::collections::HashMap;

/// Struktura reprezentujaca pojedynczy glif (znak)
#[derive(Debug, Clone)]
pub struct Glyph {
    pub width: u32,
    pub height: u32,
    pub advance: u32,
    pub tex_x: f32,
    pub tex_y: f32,
    pub tex_w: f32,
    pub tex_h: f32,
}

/// Element parsujacy tagi z tekstu
#[derive(Debug, Clone, PartialEq)]
enum TextElement<'a> {
    Char(char),
    Tag(&'a str),
    LiteralPipe,
}

fn parse_text(text: &str) -> Vec<TextElement<'_>> {
    let mut items = Vec::new();
    let mut chars = text.char_indices().peekable();
    
    while let Some((i, c)) = chars.next() {
        if c == '|' {
            if let Some(&(_, next_c)) = chars.peek() {
                match next_c {
                    '|' => {
                        chars.next(); // Zuzywamy drugiego pipe
                        items.push(TextElement::LiteralPipe);
                    }
                    'c' => {
                        let start = i;
                        chars.next(); // Zuzywamy 'c'
                        let mut count = 0;
                        while count < 8 && chars.peek().is_some() {
                            chars.next();
                            count += 1;
                        }
                        let end = if let Some(&(next_i, _)) = chars.peek() { next_i } else { text.len() };
                        items.push(TextElement::Tag(&text[start..end]));
                    }
                    'r' => {
                        let start = i;
                        chars.next(); // Zuzywamy 'r'
                        let end = if let Some(&(next_i, _)) = chars.peek() { next_i } else { text.len() };
                        items.push(TextElement::Tag(&text[start..end]));
                    }
                    'H' => {
                        let start = i;
                        chars.next(); // Zuzywamy 'H'
                        while let Some(&(_, ch)) = chars.peek() {
                            if ch == '|' {
                                let mut temp_chars = chars.clone();
                                temp_chars.next();
                                if let Some(&(_, 'h')) = temp_chars.peek() {
                                    chars.next(); // Zuzywamy '|'
                                    chars.next(); // Zuzywamy 'h'
                                    break;
                                }
                            }
                            chars.next();
                        }
                        let end = if let Some(&(next_i, _)) = chars.peek() { next_i } else { text.len() };
                        items.push(TextElement::Tag(&text[start..end]));
                    }
                    'h' => {
                        let start = i;
                        chars.next(); // Zuzywamy 'h'
                        let end = if let Some(&(next_i, _)) = chars.peek() { next_i } else { text.len() };
                        items.push(TextElement::Tag(&text[start..end]));
                    }
                    _ => {
                        items.push(TextElement::Char('|'));
                    }
                }
            } else {
                items.push(TextElement::Char('|'));
            }
        } else {
            items.push(TextElement::Char(c));
        }
    }
    items
}

/// Glowna struktura implementacyjna
#[derive(Debug, Clone, Default)]
pub struct FontAtlas {
    pub initialized: bool,
    pub glyphs: HashMap<char, Glyph>,
    pub line_height: u32,
}

impl FontAtlas {
    pub fn new(line_height: u32) -> Self {
        Self { 
            initialized: true,
            glyphs: HashMap::new(),
            line_height,
        }
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }

    pub fn add_glyph(&mut self, ch: char, glyph: Glyph) {
        self.glyphs.insert(ch, glyph);
    }

    pub fn get_glyph(&self, ch: char) -> Option<&Glyph> {
        self.glyphs.get(&ch)
    }

    pub fn get_line_height(&self) -> u32 {
        self.line_height
    }

    pub fn measure_text_width(&self, text: &str) -> u32 {
        let elements = parse_text(text);
        let mut width = 0;

        for el in elements {
            match el {
                TextElement::Char(c) => {
                    if let Some(glyph) = self.get_glyph(c) {
                        width += glyph.advance;
                    }
                }
                TextElement::LiteralPipe => {
                    if let Some(glyph) = self.get_glyph('|') {
                        width += glyph.advance;
                    }
                }
                TextElement::Tag(_) => {
                    // Tagi nie wplywaja na szerokosc tekstu
                }
            }
        }

        width
    }

    pub fn layout_lines(&self, text: &str, max_width: u32) -> Vec<String> {
        let mut lines = Vec::new();
        
        let elements = parse_text(text);
        if elements.is_empty() {
            return lines;
        }

        let mut current_line = String::new();
        let mut current_width = 0;
        
        let mut word_buffer = String::new();
        let mut word_width = 0;

        for el in elements {
            match el {
                TextElement::Char(c) => {
                    if c == ' ' {
                        // Kiedy napotkamy spacje, probujemy dodac slowo + spacje do aktualnej linii
                        let space_width = self.get_glyph(' ').map(|g| g.advance).unwrap_or(0);
                        
                        if current_width + word_width <= max_width {
                            current_line.push_str(&word_buffer);
                            current_width += word_width;
                            word_buffer.clear();
                            word_width = 0;
                            
                            // Dodajemy spacje
                            if current_width + space_width <= max_width {
                                current_line.push(' ');
                                current_width += space_width;
                            } else {
                                lines.push(current_line);
                                current_line = String::new();
                                current_width = 0;
                                // Pominiecie spacji na poczatku nowej linii
                            }
                        } else {
                            if !current_line.is_empty() {
                                lines.push(current_line);
                                current_line = String::new();
                                current_width = 0;
                            }
                            
                            // Jesli samo slowo jest szersze niz max_width, musimy je zlamac, ale dla prostoty tutaj je po prostu wpychamy
                            current_line.push_str(&word_buffer);
                            current_width += word_width;
                            
                            // Spacja
                            if current_width + space_width <= max_width {
                                current_line.push(' ');
                                current_width += space_width;
                            } else {
                                lines.push(current_line);
                                current_line = String::new();
                                current_width = 0;
                            }
                            
                            word_buffer.clear();
                            word_width = 0;
                        }
                    } else if c == '\n' {
                        // Koniec linii
                        if current_width + word_width <= max_width {
                            current_line.push_str(&word_buffer);
                        } else {
                            if !current_line.is_empty() {
                                lines.push(current_line);
                                current_line = String::new();
                            }
                            current_line.push_str(&word_buffer);
                        }
                        
                        lines.push(current_line);
                        current_line = String::new();
                        current_width = 0;
                        word_buffer.clear();
                        word_width = 0;
                    } else {
                        // Budujemy slowo
                        word_buffer.push(c);
                        if let Some(glyph) = self.get_glyph(c) {
                            word_width += glyph.advance;
                        }
                    }
                }
                TextElement::LiteralPipe => {
                    word_buffer.push_str("||");
                    if let Some(glyph) = self.get_glyph('|') {
                        word_width += glyph.advance;
                    }
                }
                TextElement::Tag(t) => {
                    // Tag traktujemy jako czesc budowanego slowa, bez zwiekszania jego szerokosci
                    word_buffer.push_str(t);
                }
            }
        }
        
        // Na koniec oprozniamy bufor slowa
        if !word_buffer.is_empty() {
            if current_width + word_width <= max_width {
                current_line.push_str(&word_buffer);
            } else {
                if !current_line.is_empty() {
                    lines.push(current_line);
                    current_line = String::new();
                }
                current_line.push_str(&word_buffer);
            }
        }
        
        if !current_line.is_empty() || lines.is_empty() {
            lines.push(current_line);
        }

        lines
    }
}
