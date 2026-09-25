//! Modul py_runtime dla skrzynki m2-client-pyapi
//! Zarzadca srodowiska wykonawczego Pythona i ladowania skryptow
//! Inicjalizacja GIL, rejestr modulow C-API, wykonywanie skryptow systemowych

use std::collections::HashMap;

/// Glowna struktura implementacyjna zarzadzajaca srodowiskiem Pythona
#[derive(Debug, Clone, Default)]
pub struct PyRuntime {
    /// Flaga okreslajaca czy GIL (Global Interpreter Lock) jest zainicjalizowany
    pub gil_initialized: bool,
    /// Zarejestrowane moduly C-API (nazwa -> opis)
    pub registered_modules: HashMap<String, String>,
    /// Historia wykonanych skryptow
    pub executed_scripts: Vec<String>,
}

impl PyRuntime {
    /// Tworzy nowa instancje zarzadcy srodowiska Pythona
    pub fn new() -> Self {
        Self {
            gil_initialized: false,
            registered_modules: HashMap::new(),
            executed_scripts: Vec::new(),
        }
    }

    /// Inicjalizuje GIL (Global Interpreter Lock)
    pub fn initialize_gil(&mut self) -> Result<(), String> {
        if self.gil_initialized {
            return Err("GIL jest juz zainicjalizowany".to_string());
        }
        self.gil_initialized = true;
        Ok(())
    }

    /// Zwalnia GIL (Global Interpreter Lock)
    pub fn finalize_gil(&mut self) -> Result<(), String> {
        if !self.gil_initialized {
            return Err("GIL nie jest zainicjalizowany".to_string());
        }
        self.gil_initialized = false;
        Ok(())
    }

    /// Rejestruje nowy modul C-API w srodowisku
    pub fn register_module(&mut self, name: &str, description: &str) -> Result<(), String> {
        if self.registered_modules.contains_key(name) {
            return Err(format!("Modul '{}' jest juz zarejestrowany", name));
        }
        self.registered_modules.insert(name.to_string(), description.to_string());
        Ok(())
    }

    /// Pobiera opis zarejestrowanego modulu C-API
    pub fn get_module_description(&self, name: &str) -> Option<String> {
        self.registered_modules.get(name).cloned()
    }

    /// Pobiera liste nazw wszystkich zarejestrowanych modulow
    pub fn get_registered_modules(&self) -> Vec<String> {
        let mut modules: Vec<String> = self.registered_modules.keys().cloned().collect();
        modules.sort();
        modules
    }

    /// Wykonuje skrypt systemowy
    pub fn execute_script(&mut self, script: &str) -> Result<(), String> {
        if !self.gil_initialized {
            return Err("Nie mozna wykonac skryptu: GIL nie jest zainicjalizowany".to_string());
        }
        self.executed_scripts.push(script.to_string());
        Ok(())
    }

    /// Pobiera historie wykonanych skryptow
    pub fn get_executed_scripts(&self) -> Vec<String> {
        self.executed_scripts.clone()
    }

    /// Resetuje stan srodowiska (czysci moduly i historie skryptow, wylacza GIL)
    pub fn reset(&mut self) {
        self.gil_initialized = false;
        self.registered_modules.clear();
        self.executed_scripts.clear();
    }

    /// Sprawdza czy srodowisko jest gotowe do pracy (czy GIL jest zainicjalizowany)
    pub fn is_ready(&self) -> bool {
        self.gil_initialized
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skeleton_py_runtime() {
        let mut obj = PyRuntime::new();
        assert!(!obj.is_ready());
        assert!(obj.initialize_gil().is_ok());
        assert!(obj.is_ready());
    }
}
