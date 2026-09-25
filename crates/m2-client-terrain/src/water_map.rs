//! Modul water_map dla skrzynki m2-client-terrain
//! Parser plaszczyzny wody .wat
//! Odczyt siatki obecnosci wody i wysokosci tafli wodnej

use std::mem::size_of;

/// Wymiar X dla mapy wody
pub const WATERMAP_XSIZE: u16 = 128;
/// Wymiar Y dla mapy wody
pub const WATERMAP_YSIZE: u16 = 128;
/// Magiczna liczba dla mapy wody
pub const WATER_MAGIC: u16 = 5426;
/// Maksymalna ilosc warstw wodnych
pub const MAX_WATER_NUM: u8 = 255;

/// Naglowek pliku WaterMap w przestrzeni C++
#[repr(C, packed)]
struct WaterMapHeader {
    magic: u16,
    width: u16,
    height: u16,
    layer_count: u8,
}

/// Glowna struktura implementacyjna do mapy wewnetrznej C++
#[derive(Debug, Clone)]
pub struct WaterMap {
    pub initialized: bool,
    water_map: Vec<u8>,
    water_heights: Vec<i32>,
    num_water: u8,
}

impl Default for WaterMap {
    fn default() -> Self {
        Self {
            initialized: false,
            water_map: vec![0xFF; (WATERMAP_XSIZE as usize) * (WATERMAP_YSIZE as usize)],
            water_heights: vec![-1; (MAX_WATER_NUM as usize) + 1],
            num_water: 0,
        }
    }
}

impl WaterMap {
    pub fn new() -> Self {
        let mut map = Self::default();
        map.initialized = true;
        map
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }

    /// Pobiera liczbe warstw wodnych na mapie
    pub fn get_num_water(&self) -> u8 {
        self.num_water
    }

    /// Pobiera wysokosc konkretnej warstwy wodnej (domyslnie -1)
    pub fn get_water_height(&self, layer_index: u8) -> i32 {
        if layer_index <= MAX_WATER_NUM {
            self.water_heights[layer_index as usize]
        } else {
            -1
        }
    }

    /// Pobiera ID warstwy wodnej w danym punkcie X, Y (X, Y zakres 0..128)
    pub fn get_water_map_value(&self, x: u16, y: u16) -> u8 {
        if x < WATERMAP_XSIZE && y < WATERMAP_YSIZE {
            let index = (y as usize) * (WATERMAP_XSIZE as usize) + (x as usize);
            self.water_map[index]
        } else {
            0xFF
        }
    }

    /// Ladowanie wprost z binarnych danych mapy
    pub fn load_from_bytes(data: &[u8]) -> Result<Self, &'static str> {
        let header_size = size_of::<WaterMapHeader>();

        if data.len() < header_size {
            return Err("FILE SIZE ERROR");
        }

        let mut magic_bytes = [0u8; 2];
        magic_bytes.copy_from_slice(&data[0..2]);
        let magic = u16::from_le_bytes(magic_bytes);

        if magic != WATER_MAGIC {
            return Err("MAGIC NUMBER ERROR");
        }

        let mut width_bytes = [0u8; 2];
        width_bytes.copy_from_slice(&data[2..4]);
        let width = u16::from_le_bytes(width_bytes);

        if width != WATERMAP_XSIZE {
            return Err("WIDTH ERROR");
        }

        let mut height_bytes = [0u8; 2];
        height_bytes.copy_from_slice(&data[4..6]);
        let height = u16::from_le_bytes(height_bytes);

        if height != WATERMAP_YSIZE {
            return Err("HEIGHT ERROR");
        }

        let layer_count = data[6];
        let num_water = std::cmp::min(layer_count, MAX_WATER_NUM);

        let map_size = (WATERMAP_XSIZE as usize) * (WATERMAP_YSIZE as usize);
        
        let file_rest_size = data.len() - header_size;
        let file_need_size_i32 = map_size + (num_water as usize) * size_of::<i32>();
        let file_need_size_u16 = map_size + (num_water as usize) * size_of::<u16>();

        let mut instance = Self::default();
        instance.num_water = num_water;

        if file_rest_size == file_need_size_u16 {
            // Wersja 1: Heights sa WORDami
            instance.water_map.copy_from_slice(&data[header_size..(header_size + map_size)]);

            let heights_offset = header_size + map_size;
            if num_water > 0 {
                for i in 0..(num_water as usize) {
                    let mut b = [0u8; 2];
                    b.copy_from_slice(&data[heights_offset + i * 2 .. heights_offset + i * 2 + 2]);
                    instance.water_heights[i] = u16::from_le_bytes(b) as i32;
                }
            }
        } else if file_rest_size == file_need_size_i32 {
            // Wersja 2: Heights sa LONGami
            instance.water_map.copy_from_slice(&data[header_size..(header_size + map_size)]);

            let heights_offset = header_size + map_size;
            if num_water > 0 {
                for i in 0..(num_water as usize) {
                    let mut b = [0u8; 4];
                    b.copy_from_slice(&data[heights_offset + i * 4 .. heights_offset + i * 4 + 4]);
                    instance.water_heights[i] = i32::from_le_bytes(b);
                }
            }
        } else {
            return Err("FILE DATA SIZE ERROR");
        }

        instance.initialized = true;

        Ok(instance)
    }
}
