//! Modul narzedziowy sciezek i tekstu (odpowiednik EterBase/Utils.cpp)

/// Zwraca rozszerzenie sciezki (np. "d:/ymir work/ui/intro.sub" -> "sub")
pub fn get_file_extension(path: &str) -> &str {
    if let Some(pos) = path.rfind('.') {
        &path[pos + 1..]
    } else {
        ""
    }
}

/// Zwraca nazwe pliku ze sciezki (np. "d:/ymir work/ui/intro.sub" -> "intro.sub")
pub fn get_file_name(path: &str) -> &str {
    let clean = path.replace('\\', "/");
    if let Some(pos) = clean.rfind('/') {
        &path[pos + 1..]
    } else {
        path
    }
}

/// Normalizuje ukosniki sciezki do formatu UNIX-owego ('/')
pub fn normalize_path(path: &str) -> String {
    path.replace('\\', "/")
}

/// Sprawdza czy sciezka ma dane rozszerzenie (case-insensitive)
pub fn has_extension(path: &str, ext: &str) -> bool {
    let file_ext = get_file_extension(path);
    file_ext.eq_ignore_ascii_case(ext)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_file_extension() {
        assert_eq!(get_file_extension("test.txt"), "txt");
        assert_eq!(get_file_extension("d:/path/to/archive.epk"), "epk");
        assert_eq!(get_file_extension("no_ext"), "");
    }

    #[test]
    fn test_get_file_name() {
        assert_eq!(get_file_name("d:/ymir/root.eix"), "root.eix");
        assert_eq!(get_file_name("d:\\ymir\\root.eix"), "root.eix");
        assert_eq!(get_file_name("simple.png"), "simple.png");
    }

    #[test]
    fn test_has_extension() {
        assert!(has_extension("item.proto", "PROTO"));
        assert!(has_extension("item.proto", "proto"));
        assert!(!has_extension("item.proto", "txt"));
    }
}
