use std::collections::HashMap;

/// A dictionary for handling UI and game translations from locale files.
#[derive(Debug, Default, Clone)]
pub struct LocaleDictionary {
    translations: HashMap<String, String>,
}

impl LocaleDictionary {
    /// Creates a new, empty LocaleDictionary.
    pub fn new() -> Self {
        Self {
            translations: HashMap::new(),
        }
    }

    /// Parses the `locale_string.txt` format which uses paired quoted strings,
    /// usually followed by a semicolon.
    /// Example:
    /// `"Original Text";`
    /// `"Translated Text";`
    pub fn load_locale_string(&mut self, content: &str) {
        let mut current_key: Option<String> = None;
        let mut current_string = String::new();
        let mut in_quote = false;
        let mut escape_next = false;

        for c in content.chars() {
            if in_quote {
                if escape_next {
                    match c {
                        'n' => current_string.push('\n'),
                        'r' => current_string.push('\r'),
                        't' => current_string.push('\t'),
                        _ => current_string.push(c),
                    }
                    escape_next = false;
                } else if c == '\\' {
                    escape_next = true;
                } else if c == '"' {
                    in_quote = false;

                    if let Some(key) = current_key.take() {
                        self.translations.insert(key, current_string.clone());
                    } else {
                        current_key = Some(current_string.clone());
                    }
                    current_string.clear();
                } else {
                    current_string.push(c);
                }
            } else if c == '"' {
                in_quote = true;
                current_string.clear();
                escape_next = false;
            }
        }
    }

    /// Parses the `locale_game.txt` format which is tab-delimited.
    /// Example:
    /// `KEY_NAME\tTranslated text`
    pub fn load_locale_game(&mut self, content: &str) {
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') || line.starts_with("//") || line.starts_with("--") {
                continue;
            }

            if let Some((key, value)) = line.split_once('\t') {
                self.translations.insert(key.trim().to_string(), value.trim().to_string());
            } else if let Some((key, value)) = line.split_once(' ') {
                // Fallback to space split if tab isn't found
                self.translations.insert(key.trim().to_string(), value.trim().to_string());
            }
        }
    }

    /// Translates a key. If the key is not found in the dictionary, returns the key itself.
    pub fn translate<'a>(&'a self, key: &'a str) -> &'a str {
        if let Some(translation) = self.translations.get(key) {
            translation.as_str()
        } else {
            key
        }
    }
}
