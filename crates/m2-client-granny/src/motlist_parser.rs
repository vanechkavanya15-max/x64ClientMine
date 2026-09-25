//! Modul motlist_parser dla skrzynki m2-client-granny
//! Parser rejestru ruchow postaci motlist.txt
//! Mapowanie trybow walki (WAIT, WALK, RUN, ATTACK_01, COMBO) na sciezki plikow .msa

use std::collections::HashMap;

/// Struktura reprezentujaca pojedynczy wpis animacji
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MotionEntry {
    pub mode: String,
    pub path: String,
    pub probability: u8,
}

/// Glowna struktura implementacyjna do parsowania motlist.txt
#[derive(Debug, Clone, Default)]
pub struct MotlistParser {
    pub initialized: bool,
    pub motions: HashMap<String, Vec<MotionEntry>>,
}

impl MotlistParser {
    /// Tworzy nowa instancje parsera
    pub fn new() -> Self {
        Self { 
            initialized: true,
            motions: HashMap::new(),
        }
    }

    /// Zwraca true jesli parser jest gotowy
    pub fn is_ready(&self) -> bool {
        self.initialized
    }

    /// Parsuje zawartosc pliku motlist.txt
    pub fn parse(&mut self, content: &str) {
        for line in content.lines() {
            let line = line.trim();
            
            // Ignorowanie pustych linii i komentarzy
            if line.is_empty() || line.starts_with("//") || line.starts_with('#') {
                continue;
            }

            // Rozdzielanie tokenow z uwzglednieniem sciezek w cudzyslowach
            let mut parts = Vec::new();
            let mut current = String::new();
            let mut in_quotes = false;

            for c in line.chars() {
                if c == '"' {
                    in_quotes = !in_quotes;
                } else if (c == ' ' || c == '\t') && !in_quotes {
                    if !current.is_empty() {
                        parts.push(current.clone());
                        current.clear();
                    }
                } else {
                    current.push(c);
                }
            }
            if !current.is_empty() {
                parts.push(current);
            }

            if parts.len() < 2 {
                continue; // Niepoprawna linia, brak trybu lub sciezki
            }

            let mode = parts[0].to_uppercase();
            let path = parts[1].clone();
            
            // Jesli nie podano prawdopodobienstwa, domyslnie 100
            let probability = if parts.len() > 2 {
                parts[2].parse::<u8>().unwrap_or(100)
            } else {
                100
            };

            let entry = MotionEntry {
                mode: mode.clone(),
                path,
                probability,
            };

            self.motions
                .entry(mode)
                .or_insert_with(Vec::new)
                .push(entry);
        }
    }

    /// Pobiera wszystkie wpisy dla danego trybu
    pub fn get_motions(&self, mode: &str) -> Option<&Vec<MotionEntry>> {
        self.motions.get(&mode.to_uppercase())
    }
}
