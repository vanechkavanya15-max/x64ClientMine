use glam::Vec3;
use std::collections::VecDeque;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TimePoint {
    pub age: f32,
    pub pos: Vec3,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TraceVertex {
    pub position: Vec3,
    pub u: f32,
    pub v: f32,
    pub alpha: f32,
}

pub struct WeaponTrace {
    pub life_time: f32,
    pub sampling_time: f32,
    pub is_playing: bool,
    pub short_points: VecDeque<TimePoint>,
    pub long_points: VecDeque<TimePoint>,
    pub vertices: Vec<TraceVertex>,
}

impl WeaponTrace {
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
        // Increase age of all points
        for point in self.short_points.iter_mut() {
            point.age += elapsed_time;
        }
        for point in self.long_points.iter_mut() {
            point.age += elapsed_time;
        }

        // Remove points that exceed life_time
        while let Some(back) = self.short_points.back() {
            if back.age > self.life_time {
                self.short_points.pop_back();
            } else {
                break;
            }
        }
        while let Some(back) = self.long_points.back() {
            if back.age > self.life_time {
                self.long_points.pop_back();
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

    pub fn build_vertices(&mut self) -> bool {
        self.vertices.clear();

        if self.long_points.len() <= 1 || self.short_points.len() <= 1 {
            return false;
        }

        let short_verts = Self::build_spline(&self.short_points, self.life_time, self.sampling_time, false);
        let long_verts = Self::build_spline(&self.long_points, self.life_time, self.sampling_time, true);

        // Zip them together into a triangle strip equivalent (quads)
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
    ) -> Vec<TraceVertex> {
        let mut out = Vec::new();
        let n = inputs.len() - 1;
        if n == 0 {
            return out;
        }

        let mut h = vec![0.0; n];
        let mut r = vec![Vec3::ZERO; n + 1];

        for i in 0..n {
            h[i] = (inputs[i + 1].age - inputs[i].age).max(1e-5);
            r[i] = (inputs[i + 1].pos - inputs[i].pos) * (3.0 / h[i]);
        }
        r[n] = Vec3::ZERO;

        for i in (1..=n).rev() {
            let prev = r[i - 1];
            r[i] += prev;
        }

        let mut stk = vec![0.0; n + 1];
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

        let get_abcd = |base_idx: usize, h_arr: &[f32], r_arr: &[Vec3], inp: &VecDeque<TimePoint>| -> (Vec3, Vec3, Vec3, Vec3) {
            let v3_tmp = inp[base_idx + 1].pos - inp[base_idx].pos;
            let a = inp[base_idx].pos;
            let b = r_arr[base_idx];
            let c = (v3_tmp * 3.0 - r_arr[base_idx + 1] * h_arr[base_idx] - r_arr[base_idx] * (2.0 * h_arr[base_idx]))
                * (1.0 / (h_arr[base_idx] * h_arr[base_idx]));
            let d = (v3_tmp * -2.0 + (r_arr[base_idx + 1] + r_arr[base_idx]) * h_arr[base_idx])
                * (1.0 / (h_arr[base_idx] * h_arr[base_idx] * h_arr[base_idx]));
            (a, b, c, d)
        };

        let (mut a, mut b, mut c, mut d) = get_abcd(base, &h, &r, inputs);

        let length = life_time.min(inputs.back().unwrap().age);
        let mut t = 0.0;
        let dt = sampling_time.max(1e-5);

        while t <= length {
            while t > timenext {
                timebase = timenext;
                base += 1;
                if base >= n {
                    break;
                }
                let abcd = get_abcd(base, &h, &r, inputs);
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

            let ttt = ((t + inputs[0].age) / life_time).clamp(0.0, 1.0);
            let alpha = if is_long {
                ((1.0 - ttt) * (1.0 - ttt) / 2.5 - 0.1).clamp(0.0, 1.0)
            } else {
                0.0
            };

            out.push(TraceVertex {
                position: pos,
                u: t / life_time,
                v: if is_long { 0.0 } else { 1.0 },
                alpha,
            });

            t += dt;
        }

        out
    }
}
