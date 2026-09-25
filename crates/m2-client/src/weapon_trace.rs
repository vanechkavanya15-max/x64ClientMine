//! Modul `weapon_trace` — Generator smugi miecza podczas ciecia CWeaponTrace
//! Wzorzec referencyjny C++: `source/GameLib/ActorInstanceWeaponTrace.cpp oraz source/GameLib/WeaponTrace.cpp`
//! Opis: Odwzorowanie CWeaponTrace: bufor historii pozycji koncowki i rekojesci miecza, generowanie wstegi poligonow z zanikajacym kanalem alpha.

use std::collections::VecDeque;
use std::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};

#[derive(Debug, Clone, Copy, Default)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Add for Vector3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }
}

impl AddAssign for Vector3 {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl Sub for Vector3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
    }
}

impl SubAssign for Vector3 {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl Mul<f32> for Vector3 {
    type Output = Self;
    fn mul(self, scalar: f32) -> Self {
        Self { x: self.x * scalar, y: self.y * scalar, z: self.z * scalar }
    }
}

impl Div<f32> for Vector3 {
    type Output = Self;
    fn div(self, scalar: f32) -> Self {
        Self { x: self.x / scalar, y: self.y / scalar, z: self.z / scalar }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct PdtVertex {
    pub position: Vector3,
    pub diffuse: Color,
    pub tex_coord: Vector2,
}

#[derive(Debug, Clone)]
pub struct TimePoint {
    pub time: f32,
    pub position: Vector3,
}

#[derive(Debug, Clone, Default)]
pub struct WeaponTrace {
    pub initialized: bool,
    
    pub short_time_point_list: VecDeque<TimePoint>,
    pub long_time_point_list: VecDeque<TimePoint>,
    pub pdt_vertex_vector: Vec<PdtVertex>,
    
    pub life_time: f32,
    pub sampling_time: f32,
    
    pub is_playing: bool,
    pub use_texture: bool,
    
    pub fx: f32,
    pub fy: f32,
    pub fz: f32,
    pub rotation: f32,
}

impl WeaponTrace {
    pub fn new() -> Self {
        let mut trace = Self::default();
        trace.initialize();
        trace
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }
    
    pub fn initialize(&mut self) {
        self.fx = 0.0;
        self.fy = 0.0;
        self.fz = 0.0;
        self.rotation = 0.0;
        
        self.life_time = 0.18;
        self.sampling_time = 0.003;
        
        self.is_playing = false;
        self.use_texture = false;
        
        self.initialized = true;
    }
    
    pub fn clear(&mut self) {
        self.short_time_point_list.clear();
        self.long_time_point_list.clear();
        self.pdt_vertex_vector.clear();
        self.initialize();
    }
    
    pub fn turn_on(&mut self) {
        self.is_playing = true;
    }
    
    pub fn turn_off(&mut self) {
        self.is_playing = false;
    }
    
    pub fn use_alpha(&mut self) {
        self.use_texture = false;
    }
    
    pub fn use_texture_mode(&mut self) {
        self.use_texture = true;
    }
    
    pub fn set_position(&mut self, fx: f32, fy: f32, fz: f32) {
        self.fx = fx;
        self.fy = fy;
        self.fz = fz;
    }
    
    pub fn set_rotation(&mut self, rotation: f32) {
        self.rotation = rotation;
    }
    
    pub fn set_life_time(&mut self, life_time: f32) {
        self.life_time = life_time;
    }
    
    pub fn set_sampling_time(&mut self, sampling_time: f32) {
        self.sampling_time = sampling_time;
    }
}

impl WeaponTrace {
    pub fn update_with_points(&mut self, elapsed_time: f32, short_pos: Option<Vector3>, long_pos: Option<Vector3>) {
        // Zaktualizuj czas w krotkiej liscie i usun stare punkty
        let mut i = 0;
        while i < self.short_time_point_list.len() {
            self.short_time_point_list[i].time += elapsed_time;
            if self.short_time_point_list[i].time > self.life_time {
                break;
            }
            i += 1;
        }
        self.short_time_point_list.truncate(i);

        // Zaktualizuj czas w dlugiej liscie i usun stare punkty
        let mut i = 0;
        while i < self.long_time_point_list.len() {
            self.long_time_point_list[i].time += elapsed_time;
            if self.long_time_point_list[i].time > self.life_time {
                break;
            }
            i += 1;
        }
        self.long_time_point_list.truncate(i);

        if self.is_playing {
            if let Some(pos) = short_pos {
                self.short_time_point_list.push_front(TimePoint {
                    time: 0.0,
                    position: pos,
                });
            }
            if let Some(pos) = long_pos {
                self.long_time_point_list.push_front(TimePoint {
                    time: 0.0,
                    position: pos,
                });
            }
        }
    }

    pub fn build_vertex(&mut self) -> bool {
        if self.long_time_point_list.len() <= 1 {
            return false;
        }

        let mut short_vertex_vector = Vec::new();
        let mut long_vertex_vector = Vec::new();

        let length = self.life_time.min(self.long_time_point_list.back().map(|p| p.time).unwrap_or(0.0));
        let n = self.long_time_point_list.len() - 1;
        
        let max_size = 300;
        if n >= max_size - 1 {
            // Unikamy paniki w razie zbyt dlugiej historii
            return false; 
        }

        let mut h = vec![0.0; max_size];
        let mut r = vec![Vector3::default(); max_size];
        let mut stk = vec![0.0; max_size];

        for loop_idx in 0..=1 {
            let input = if loop_idx == 1 {
                &self.long_time_point_list
            } else {
                &self.short_time_point_list
            };
            
            // Jesli krotka lista jest zbyt krotka wzgledem dlugiej, przerywamy
            if input.len() <= n {
                continue;
            }

            let output = if loop_idx == 1 {
                &mut long_vertex_vector
            } else {
                &mut short_vertex_vector
            };

            for i in 0..n {
                h[i] = input[i + 1].time - input[i].time;
                if h[i] <= 0.00001 { h[i] = 0.00001; } // ochrona przed dzieleniem przez 0
                r[i] = (input[i + 1].position - input[i].position) * (3.0 / h[i]);
            }
            r[n] = Vector3::default();
            for i in (1..=n).rev() {
                r[i] = r[i] + r[i - 1];
            }

            let mut sp = 0;
            let mut rate = 0.5;
            r[0] = r[0] * 0.5;
            stk[sp] = rate;
            sp += 1;
            
            for i in 1..n {
                r[i] = r[i] - r[i - 1];
                rate = 1.0 / (4.0 - rate);
                r[i] = r[i] * rate;
                stk[sp] = rate;
                sp += 1;
            }
            r[n] = r[n] - r[n - 1];
            rate = 1.0 / (2.0 - rate);
            r[n] = r[n] * rate;

            for i in (0..n).rev() {
                sp -= 1;
                r[i] = r[i] - r[i + 1] * stk[sp];
            }

            let mut base = 0;
            let mut timebase = 0.0;
            let mut timenext = h[base];
            let dt = self.sampling_time;

            let mut a = input[base].position;
            let mut b = r[base];
            let mut v3_tmp = input[base + 1].position - input[base].position;
            
            let mut c = (v3_tmp * 3.0 - r[base + 1] * h[base] - r[base] * (2.0 * h[base])) * (1.0 / (h[base] * h[base]));
            let mut d = (v3_tmp * -2.0 + (r[base + 1] + r[base]) * h[base]) * (1.0 / (h[base] * h[base] * h[base]));

            let mut t = 0.0;
            while t <= length {
                while t > timenext {
                    timebase = timenext;
                    base += 1;
                    if base >= n {
                        break;
                    }
                    v3_tmp = input[base + 1].position - input[base].position;
                    a = input[base].position;
                    b = r[base];
                    c = (v3_tmp * 3.0 - r[base + 1] * h[base] - r[base] * (2.0 * h[base])) * (1.0 / (h[base] * h[base]));
                    d = (v3_tmp * -2.0 + (r[base + 1] + r[base]) * h[base]) * (1.0 / (h[base] * h[base] * h[base]));
                    
                    timenext += h[base];
                }
                
                if base > n {
                    break;
                }

                let cc = t - timebase;
                let ttt = ((t + input[0].time) / self.life_time).max(0.0).min(1.0);
                
                let alpha = if loop_idx != 0 {
                    let val = (1.0 - ttt) * (1.0 - ttt) / 2.5 - 0.1;
                    val.max(0.0).min(1.0)
                } else {
                    0.0
                };
                
                let diffuse = Color::new(0.3, 0.8, 1.0, alpha);
                let position = a + (b + (c + d * cc) * cc) * cc;
                let tex_coord = Vector2 {
                    x: t / self.life_time,
                    y: if loop_idx != 0 { 0.0 } else { 1.0 },
                };

                output.push(PdtVertex { position, diffuse, tex_coord });
                
                t += dt;
            }
        }

        self.pdt_vertex_vector.clear();
        
        let len = long_vertex_vector.len().min(short_vertex_vector.len());
        for i in 0..len {
            self.pdt_vertex_vector.push(long_vertex_vector[i]);
            self.pdt_vertex_vector.push(short_vertex_vector[i]);
        }

        true
    }
}
