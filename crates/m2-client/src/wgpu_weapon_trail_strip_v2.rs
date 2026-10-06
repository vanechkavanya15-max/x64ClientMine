use std::collections::VecDeque;
use glam::Vec3;

// Assuming TPDTVertex, TPosition, and TTextureCoordinate are available through a similar path or defined here.
// Since we cannot modify shared files like lib.rs, we use the ones defined in particle pipeline module.
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable, PartialEq)]
pub struct TPosition {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable, PartialEq)]
pub struct TTextureCoordinate {
    pub u: f32,
    pub v: f32,
}

pub type TDiffuse = u32;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TPDTVertex {
    pub position: TPosition,
    pub diffuse: TDiffuse,
    pub tex_coord: TTextureCoordinate,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TimePoint {
    pub age: f32,
    pub pos: Vec3,
}

pub struct WeaponTrailStripV2 {
    pub life_time: f32,
    pub sampling_time: f32,
    pub is_playing: bool,
    pub short_points: VecDeque<TimePoint>,
    pub long_points: VecDeque<TimePoint>,
    pub vertices: Vec<TPDTVertex>,
}

impl WeaponTrailStripV2 {
    pub fn new() -> Self {
        Self {
            life_time: 0.18,
            sampling_time: 0.003,
            is_playing: false,
            short_points: VecDeque::new(),
            long_points: VecDeque::new(),
            vertices: Vec::new(),
        }
    }

    pub fn turn_on(&mut self) {
        self.is_playing = true;
    }

    pub fn turn_off(&mut self) {
        self.is_playing = false;
    }

    pub fn clear(&mut self) {
        self.short_points.clear();
        self.long_points.clear();
        self.vertices.clear();
    }

    pub fn set_life_time(&mut self, life_time: f32) {
        self.life_time = life_time;
    }

    pub fn set_sampling_time(&mut self, sampling_time: f32) {
        self.sampling_time = sampling_time;
    }

    pub fn update(&mut self, elapsed_time: f32) {
        if !self.is_playing {
            return;
        }

        // Increase age of all points
        for point in self.short_points.iter_mut() {
            point.age += elapsed_time;
        }
        for point in self.long_points.iter_mut() {
            point.age += elapsed_time;
        }

        // Remove points that exceed life_time (keep the ones that are just slightly above to maintain tail, logic from C++ erase)
        // C++: it->first += fElapsedTime; if (it->first>m_fLifeTime) { it++; break; } erase(it, end)
        // This means it removes everything AFTER the first one that exceeds life_time.
        // VecDeque is ordered from newest (front) to oldest (back), so we pop from back.
        while let Some(back) = self.short_points.back() {
            if back.age > self.life_time {
                // In C++, it keeps one element that just exceeded the lifetime to allow interpolating out of it.
                // We'll mimic this by checking if the second to last also exceeds.
                if self.short_points.len() > 1 && self.short_points[self.short_points.len() - 2].age > self.life_time {
                    self.short_points.pop_back();
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        
        while let Some(back) = self.long_points.back() {
            if back.age > self.life_time {
                if self.long_points.len() > 1 && self.long_points[self.long_points.len() - 2].age > self.life_time {
                    self.long_points.pop_back();
                } else {
                    break;
                }
            } else {
                break;
            }
        }
    }

    pub fn add_point(&mut self, base_position: Vec3, tip_position: Vec3) {
        if !self.is_playing {
            return;
        }

        self.short_points.push_front(TimePoint {
            age: 0.0,
            pos: base_position,
        });

        self.long_points.push_front(TimePoint {
            age: 0.0,
            pos: tip_position,
        });
    }

    pub fn build_vertex(&mut self) -> bool {
        self.vertices.clear();

        if self.long_points.len() <= 1 || self.short_points.len() <= 1 {
            return false;
        }

        let short_verts = Self::build_spline(&self.short_points, self.life_time, self.sampling_time, false);
        let long_verts = Self::build_spline(&self.long_points, self.life_time, self.sampling_time, true);

        let count = short_verts.len().min(long_verts.len());
        for i in 0..count {
            self.vertices.push(long_verts[i]);
            self.vertices.push(short_verts[i]);
        }

        true
    }

    fn build_spline(
        inputs: &VecDeque<TimePoint>,
        life_time: f32,
        sampling_time: f32,
        is_long: bool,
    ) -> Vec<TPDTVertex> {
        let mut out = Vec::new();
        let n = inputs.len() - 1;
        if n == 0 {
            return out;
        }
        
        let max_size = inputs.len() + 1; // dynamically sized instead of fixed 300
        let mut h = vec![0.0; max_size];
        let mut r = vec![Vec3::ZERO; max_size];

        for i in 0..n {
            h[i] = inputs[i + 1].age - inputs[i].age;
            if h[i] <= 0.0 {
                h[i] = 1e-5; // Prevent division by zero if two points have the exact same age
            }
            r[i] = (inputs[i + 1].pos - inputs[i].pos) * (3.0 / h[i]);
        }
        r[n] = Vec3::ZERO;

        for i in (1..=n).rev() {
            let prev = r[i - 1];
            r[i] += prev;
        }

        let mut stk = vec![0.0; max_size];
        let mut sp = 0;
        let mut rate = 0.5;
        r[0] *= 0.5;
        stk[sp] = rate;
        sp += 1;

        for i in 1..n {
            let prev = r[i - 1];
            r[i] -= prev;
            rate = 1.0 / (4.0 - rate);
            r[i] *= rate;
            stk[sp] = rate;
            sp += 1;
        }

        let prev = r[n - 1];
        r[n] -= prev;
        rate = 1.0 / (2.0 - rate);
        r[n] *= rate;

        for i in (0..n).rev() {
            sp -= 1;
            let next = r[i + 1];
            r[i] -= stk[sp] * next;
        }

        let mut base = 0;
        let mut timebase = 0.0;
        let mut timenext = h[base];
        let dt = sampling_time.max(1e-5);

        let mut get_abcd = |base_idx: usize| -> (Vec3, Vec3, Vec3, Vec3) {
            let v3_tmp = inputs[base_idx + 1].pos - inputs[base_idx].pos;
            let a = inputs[base_idx].pos;
            let b = r[base_idx];
            let c = (v3_tmp * 3.0 - r[base_idx + 1] * h[base_idx] - r[base_idx] * (2.0 * h[base_idx]))
                * (1.0 / (h[base_idx] * h[base_idx]));
            let d = (v3_tmp * -2.0 + (r[base_idx + 1] + r[base_idx]) * h[base_idx])
                * (1.0 / (h[base_idx] * h[base_idx] * h[base_idx]));
            (a, b, c, d)
        };

        let (mut a, mut b, mut c, mut d) = get_abcd(base);

        let length = life_time.min(inputs.back().unwrap().age);
        let mut t = 0.0;

        while t <= length {
            while t > timenext {
                timebase = timenext;
                base += 1;
                if base >= n {
                    break;
                }
                let abcd = get_abcd(base);
                a = abcd.0;
                b = abcd.1;
                c = abcd.2;
                d = abcd.3;
                timenext += h[base];
            }
            if base >= n {
                break;
            }

            let cc = t - timebase;
            let pos = a + b * cc + c * (cc * cc) + d * (cc * cc * cc);

            // C++ diffuses alpha calculation
            // float ttt = std::min(std::max((t+Input[0].first)/m_fLifeTime,0.0f),1.0f);
            // v.diffuse = D3DXCOLOR(0.3f,0.8f,1.0f, (loop)?std::min(std::max((1.0f-ttt)*(1.0f-ttt)/2.5f-0.1f,0.0f),1.0f):0.0f );
            let ttt = ((t + inputs[0].age) / life_time).clamp(0.0, 1.0);
            let alpha = if is_long {
                ((1.0 - ttt) * (1.0 - ttt) / 2.5 - 0.1).clamp(0.0, 1.0)
            } else {
                0.0
            };

            let diffuse = Self::pack_d3dxcolor(0.3, 0.8, 1.0, alpha);

            out.push(TPDTVertex {
                position: TPosition { x: pos.x, y: pos.y, z: pos.z },
                diffuse,
                tex_coord: TTextureCoordinate { u: t / life_time, v: if is_long { 0.0 } else { 1.0 } },
            });

            t += dt;
        }

        out
    }

    fn pack_d3dxcolor(r: f32, g: f32, b: f32, a: f32) -> u32 {
        let r_u8 = (r.clamp(0.0, 1.0) * 255.0) as u32;
        let g_u8 = (g.clamp(0.0, 1.0) * 255.0) as u32;
        let b_u8 = (b.clamp(0.0, 1.0) * 255.0) as u32;
        let a_u8 = (a.clamp(0.0, 1.0) * 255.0) as u32;

        // BGRA or ARGB depending on D3DCOLOR. D3DCOLOR is ARGB: (a<<24)|(r<<16)|(g<<8)|b.
        (a_u8 << 24) | (r_u8 << 16) | (g_u8 << 8) | b_u8
    }
}
