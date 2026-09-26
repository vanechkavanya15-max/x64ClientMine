use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct GlyphMetric {
    pub char_code: char,
    pub uv_rect: (f32, f32, f32, f32),
    pub width: f32,
    pub height: f32,
    pub bearing_x: f32,
    pub bearing_y: f32,
    pub advance_x: f32,
}

#[derive(Debug, Clone, Default)]
pub struct FontLayoutEngine {
    metrics: HashMap<char, GlyphMetric>,
}

impl FontLayoutEngine {
    pub fn new() -> Self {
        Self {
            metrics: HashMap::new(),
        }
    }

    pub fn register_glyph(&mut self, metric: GlyphMetric) {
        self.metrics.insert(metric.char_code, metric);
    }

    pub fn get_glyph(&self, char_code: char) -> Option<&GlyphMetric> {
        self.metrics.get(&char_code)
    }

    pub fn measure_text_width(&self, text: &str) -> f32 {
        let mut max_width = 0.0_f32;
        let mut current_width = 0.0_f32;

        for c in text.chars() {
            if c == '\n' {
                if current_width > max_width {
                    max_width = current_width;
                }
                current_width = 0.0;
            } else if c != '\r' {
                let advance = self.metrics.get(&c).map(|m| m.advance_x).unwrap_or(0.0);
                current_width += advance;
            }
        }

        if current_width > max_width {
            max_width = current_width;
        }

        max_width
    }

    pub fn wrap_text(&self, text: &str, max_width_pixels: f32) -> Vec<String> {
        let mut lines = Vec::new();
        if text.is_empty() {
            return lines;
        }

        let clean_text = text.replace('\r', "");

        for explicit_line in clean_text.split('\n') {
            if explicit_line.is_empty() {
                lines.push(String::new());
                continue;
            }

            let mut current_line = String::new();
            let mut current_width = 0.0_f32;

            let words: Vec<&str> = explicit_line.split(' ').collect();
            let space_advance = self.metrics.get(&' ').map(|m| m.advance_x).unwrap_or(0.0);

            for (i, word) in words.iter().enumerate() {
                let word_width = self.measure_text_width(word);
                
                if i == 0 {
                    current_line.push_str(word);
                    current_width += word_width;
                } else {
                    let width_with_space = current_width + space_advance + word_width;
                    if width_with_space > max_width_pixels {
                        lines.push(current_line.clone());
                        current_line = word.to_string();
                        current_width = word_width;
                    } else {
                        current_line.push(' ');
                        current_line.push_str(word);
                        current_width += space_advance + word_width;
                    }
                }
            }
            lines.push(current_line);
        }

        lines
    }
}
