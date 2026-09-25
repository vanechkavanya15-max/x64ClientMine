//! Modul tile_splatting dla skrzynki m2-client-terrain
//! Mieszanie wielowarstwowe tekstur terenu (Alpha Splatting)
//! Odczyt mapy wag alfa dla 4 warstw tekstur na kazdy wierzcholek terenu

/// Reprezentuje wagi alfa dla 4 warstw tekstur na pojedynczym wierzcholku
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SplatWeights {
    pub layers: [u8; 4],
}

impl Default for SplatWeights {
    fn default() -> Self {
        Self { layers: [0; 4] }
    }
}

/// Glowna struktura implementacyjna zarzadzajaca wagami tekstur na calej mapie/sektorze
#[derive(Debug, Clone)]
pub struct TileSplatting {
    width: u32,
    height: u32,
    weights: Vec<SplatWeights>,
    initialized: bool,
}

impl Default for TileSplatting {
    fn default() -> Self {
        Self {
            width: 0,
            height: 0,
            weights: Vec::new(),
            initialized: false,
        }
    }
}

impl TileSplatting {
    /// Tworzy nowa, pusta instancje TileSplatting
    pub fn new() -> Self {
        Self::default()
    }

    /// Inicjalizuje mape z zadanymi wymiarami, domyslnie wagi to 0
    pub fn init(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
        self.weights = vec![SplatWeights::default(); (width * height) as usize];
        self.initialized = true;
    }

    /// Zwraca true, jesli struktura zostala zainicjalizowana
    pub fn is_ready(&self) -> bool {
        self.initialized
    }

    /// Pobiera wagi dla danego wierzcholka (x, y) w sposob bezpieczny
    pub fn get_weights(&self, x: u32, y: u32) -> Option<&SplatWeights> {
        if x < self.width && y < self.height {
            let index = (y * self.width + x) as usize;
            self.weights.get(index)
        } else {
            None
        }
    }

    /// Ustawia wagi dla danego wierzcholka (x, y). Zwraca true jesli sie udalo.
    pub fn set_weights(&mut self, x: u32, y: u32, weights: SplatWeights) -> bool {
        if x < self.width && y < self.height {
            let index = (y * self.width + x) as usize;
            if let Some(elem) = self.weights.get_mut(index) {
                *elem = weights;
                return true;
            }
        }
        false
    }

    /// Laduje wagi tekstur z zewnetrznego bufora bajtow (np. wczytanego z pliku).
    /// Zwraca blad, jesli bufor jest za krotki w stosunku do deklarowanych wymiarow.
    pub fn load_from_buffer(&mut self, width: u32, height: u32, buffer: &[u8]) -> Result<(), &'static str> {
        let expected_size = (width * height * 4) as usize;
        if buffer.len() < expected_size {
            return Err("Bufor jest za maly, aby zaladowac mape wag splattingu");
        }

        self.width = width;
        self.height = height;
        self.weights.clear();
        self.weights.reserve((width * height) as usize);

        for i in 0..(width * height) as usize {
            let start = i * 4;
            self.weights.push(SplatWeights {
                layers: [
                    buffer[start],
                    buffer[start + 1],
                    buffer[start + 2],
                    buffer[start + 3],
                ],
            });
        }
        self.initialized = true;
        Ok(())
    }

    /// Eksportuje wagi do jednowymiarowego wektora bajtow (4 bajty na wierzcholek).
    pub fn export_to_buffer(&self) -> Vec<u8> {
        let mut buffer = Vec::with_capacity(self.weights.len() * 4);
        for weight in &self.weights {
            buffer.extend_from_slice(&weight.layers);
        }
        buffer
    }
}
