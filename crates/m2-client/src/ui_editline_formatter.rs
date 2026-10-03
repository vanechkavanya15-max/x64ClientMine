// ui_editline_formatter.rs - Zaawansowane formatowanie pola EditLine (maskowanie hasla, MultiLine, Outline)

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditLineWrapMode {
    None,
    WordWrap,
    CharWrap,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TextOutlineStyle {
    pub enabled: bool,
    pub outline_color: [f32; 4],
    pub thickness: f32,
}

impl Default for TextOutlineStyle {
    fn default() -> Self {
        Self {
            enabled: false,
            outline_color: [0.0, 0.0, 0.0, 1.0], // czarny obrys domyslnie
            thickness: 1.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UiEditLineFormatter {
    pub is_secret: bool,
    pub mask_char: char,
    pub max_chars: usize,
    pub wrap_mode: EditLineWrapMode,
    pub max_line_width_px: f32,
    pub char_width_px: f32,
    pub outline_style: TextOutlineStyle,
}

impl Default for UiEditLineFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl UiEditLineFormatter {
    pub fn new() -> Self {
        Self {
            is_secret: false,
            mask_char: '*',
            max_chars: 128,
            wrap_mode: EditLineWrapMode::None,
            max_line_width_px: 200.0,
            char_width_px: 8.0,
            outline_style: TextOutlineStyle::default(),
        }
    }

    pub fn set_secret_mode(&mut self, secret: bool) {
        self.is_secret = secret;
    }

    pub fn set_mask_char(&mut self, c: char) {
        self.mask_char = c;
    }

    pub fn set_outline(&mut self, enabled: bool, color: [f32; 4], thickness: f32) {
        self.outline_style = TextOutlineStyle {
            enabled,
            outline_color: color,
            thickness: thickness.clamp(0.5, 3.0),
        };
    }

    pub fn set_wrap_mode(&mut self, mode: EditLineWrapMode, max_width_px: f32, char_width_px: f32) {
        self.wrap_mode = mode;
        self.max_line_width_px = max_width_px.max(10.0);
        self.char_width_px = char_width_px.max(1.0);
    }

    /// Przeksztalca surowy tekst w widoczna reprezentacje (np. zastepuje znaki gwiazdkami w trybie secret)
    pub fn format_display_text(&self, raw_text: &str) -> String {
        let truncated: String = raw_text.chars().take(self.max_chars).collect();
        if self.is_secret {
            std::iter::repeat(self.mask_char).take(truncated.chars().count()).collect()
        } else {
            truncated
        }
    }

    /// Dzieli tekst na linie zgodnie z wybranym trybem zawijania
    pub fn wrap_lines(&self, display_text: &str) -> Vec<String> {
        if self.wrap_mode == EditLineWrapMode::None || display_text.is_empty() {
            return vec![display_text.to_string()];
        }

        let chars_per_line = (self.max_line_width_px / self.char_width_px).floor() as usize;
        let limit = chars_per_line.max(1);

        match self.wrap_mode {
            EditLineWrapMode::CharWrap => {
                let mut lines = Vec::new();
                let chars: Vec<char> = display_text.chars().collect();
                for chunk in chars.chunks(limit) {
                    lines.push(chunk.iter().collect::<String>());
                }
                lines
            }
            EditLineWrapMode::WordWrap => {
                let mut lines = Vec::new();
                let mut current_line = String::new();

                for word in display_text.split_whitespace() {
                    if current_line.is_empty() {
                        if word.chars().count() > limit {
                            // Jesli pojedyncze slowo jest dluzsze niz limit linii, dzielimy je
                            let chars: Vec<char> = word.chars().collect();
                            for chunk in chars.chunks(limit) {
                                lines.push(chunk.iter().collect::<String>());
                            }
                        } else {
                            current_line.push_str(word);
                        }
                    } else if current_line.chars().count() + 1 + word.chars().count() <= limit {
                        current_line.push(' ');
                        current_line.push_str(word);
                    } else {
                        lines.push(current_line);
                        current_line = word.to_string();
                    }
                }

                if !current_line.is_empty() {
                    lines.push(current_line);
                }

                if lines.is_empty() {
                    lines.push(String::new());
                }
                lines
            }
            EditLineWrapMode::None => vec![display_text.to_string()],
        }
    }

    /// Oblicza przesuniecia dla obrysu (8 kierunkow wokol tekstu)
    pub fn get_outline_offsets(&self) -> Vec<[f32; 2]> {
        if !self.outline_style.enabled {
            return Vec::new();
        }
        let t = self.outline_style.thickness;
        vec![
            [-t, -t], [0.0, -t], [t, -t],
            [-t,  0.0],          [t,  0.0],
            [-t,  t], [0.0,  t], [t,  t],
        ]
    }
}
