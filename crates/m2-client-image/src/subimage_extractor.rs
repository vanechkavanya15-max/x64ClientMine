//! Modul subimage_extractor dla skrzynki m2-client-image
//! Parser deskryptorow atlasow pod-obrazow Metin2 (.sub)
//! Odczyt plikow .sub, sciezka do atlasu bazowego, prostokat wycinka UV (left, top, right, bottom)

/// Struktura przechowujaca dane wycinka obrazu
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SubimageDescriptor {
    pub image_path: String,
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

/// Glowna struktura implementacyjna
#[derive(Debug, Clone, Default)]
pub struct SubimageExtractor {
    pub initialized: bool,
}

impl SubimageExtractor {
    pub fn new() -> Self {
        Self { initialized: true }
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }

    /// Funkcja parsujaca zawartosc pliku .sub
    pub fn parse(&self, content: &str) -> Result<SubimageDescriptor, String> {
        let mut descriptor = SubimageDescriptor::default();
        let mut has_image = false;
        let mut has_left = false;
        let mut has_top = false;
        let mut has_right = false;
        let mut has_bottom = false;

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.is_empty() {
                continue;
            }

            let key = parts[0].to_lowercase();
            match key.as_str() {
                "title" | "version" => {
                    // Ignorujemy te klucze
                }
                "image" => {
                    let value = line[parts[0].len()..].trim();
                    let value = value.trim_matches('"');
                    descriptor.image_path = value.to_string();
                    has_image = true;
                }
                "left" => {
                    if parts.len() > 1 {
                        descriptor.left = parts[1].parse::<i32>().map_err(|_| "Nieprawidlowa wartosc dla left".to_string())?;
                        has_left = true;
                    }
                }
                "top" => {
                    if parts.len() > 1 {
                        descriptor.top = parts[1].parse::<i32>().map_err(|_| "Nieprawidlowa wartosc dla top".to_string())?;
                        has_top = true;
                    }
                }
                "right" => {
                    if parts.len() > 1 {
                        descriptor.right = parts[1].parse::<i32>().map_err(|_| "Nieprawidlowa wartosc dla right".to_string())?;
                        has_right = true;
                    }
                }
                "bottom" => {
                    if parts.len() > 1 {
                        descriptor.bottom = parts[1].parse::<i32>().map_err(|_| "Nieprawidlowa wartosc dla bottom".to_string())?;
                        has_bottom = true;
                    }
                }
                _ => {
                    // Nieznany klucz, ignorujemy dla kompatybilnosci
                }
            }
        }

        if !has_image {
            return Err("Brak sciezki do obrazu (image)".to_string());
        }
        if !has_left || !has_top || !has_right || !has_bottom {
            return Err("Brak wymaganych wspolrzednych wycinka (left, top, right, bottom)".to_string());
        }

        Ok(descriptor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skeleton_subimage_extractor() {
        let obj = SubimageExtractor::new();
        assert!(obj.is_ready());
    }
}
