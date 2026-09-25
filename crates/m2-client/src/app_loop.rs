//! Modul `app_loop` — Glowna petla gry Process/Update/Render i licznik FPS
//! Wzorzec referencyjny C++: `source/UserInterface/PythonApplication.cpp`
//! Opis: Odwzorowanie metody CPythonApplication::Process(): synchronizacja klatek, obliczanie delta_time, licznik FPS, harmonogram wywolan UpdateGame() i RenderGame().

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RenderResult {
    pub can_render: bool,
    pub face_count: u32,
}

pub trait AppLoopContext {
    fn get_time_ms(&self) -> u32;
    fn update(&mut self);
    fn render(&mut self) -> RenderResult;
    fn sleep(&mut self, ms: u32);
    fn set_view_distance(&mut self, distance: f32);

    fn is_frame_skip_disable(&self) -> bool;
    fn is_windowed(&self) -> bool;
    fn is_active(&self) -> bool;
    fn force_sight_range(&self) -> i32;
    fn get_min_fog(&self) -> f32;
}

#[derive(Debug, Clone, Default)]
pub struct AppLoop {
    pub initialized: bool,

    // Liczniki FPS i obciazenia
    pub update_fps: u32,
    pub render_fps: u32,
    pub load: u32,
    pub face_count: u32,

    // Maszyna stanow z oryginalnych zmiennych statycznych
    pub update_frame_count: u32,
    pub render_frame_count: u32,
    pub face_count_acc: u32,
    pub ui_load: u32,
    pub check_time: u32,

    pub next_frame_time: u32,
    pub buf_sleep_skip_time: u32,

    pub cur_update_time: u32,
    pub cur_render_time: u32,
    pub ave_render_time: f32,

    pub face_acc_count: u32,
    pub face_acc_time: u32,
    pub face_spd: f32,

    pub render_check_time: u32,
    pub render_range_time: u32,
    pub render_range_frame: u32,

    pub buf_render_time: f32,
    pub s_ave_render_time: f32,
}

impl AppLoop {
    pub fn new() -> Self {
        Self {
            initialized: true,
            update_fps: 0,
            render_fps: 0,
            load: 0,
            face_count: 0,
            update_frame_count: 0,
            render_frame_count: 0,
            face_count_acc: 0,
            ui_load: 0,
            check_time: 0,
            next_frame_time: 0,
            buf_sleep_skip_time: 0,
            cur_update_time: 0,
            cur_render_time: 0,
            ave_render_time: 0.0,
            face_acc_count: 0,
            face_acc_time: 0,
            face_spd: 0.0,
            render_check_time: 0,
            render_range_time: 0,
            render_range_frame: 0,
            buf_render_time: 0.0,
            s_ave_render_time: 16.0,
        }
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }

    pub fn init_time<T: AppLoopContext>(&mut self, ctx: &mut T) {
        let t = ctx.get_time_ms();
        self.check_time = t;
        self.next_frame_time = t;
        self.render_check_time = t;
    }

    pub fn skip_render_buffering<T: AppLoopContext>(&mut self, ctx: &mut T, ms: u32) {
        self.buf_sleep_skip_time = ctx.get_time_ms().wrapping_add(ms);
    }

    pub fn process<T: AppLoopContext>(&mut self, ctx: &mut T, frame_time: u32) -> bool {
        let dw_start = ctx.get_time_ms();

        if dw_start.wrapping_sub(self.check_time) > 1000 {
            self.update_fps = self.update_frame_count;
            self.render_fps = self.render_frame_count;
            self.load = self.ui_load;
            self.face_count = self.face_count_acc / self.render_frame_count.max(1);

            self.check_time = ctx.get_time_ms();

            self.ui_load = 0;
            self.face_count_acc = 0;
            self.update_frame_count = 0;
            self.render_frame_count = 0;
        }

        self.next_frame_time = self.next_frame_time.wrapping_add(frame_time);

        let update_start = ctx.get_time_ms();

        ctx.update();

        self.cur_update_time = ctx.get_time_ms().wrapping_sub(update_start);

        let dw_current_time = ctx.get_time_ms();
        let mut b_current_late_update = false;
        let mut b_frame_skip = false;

        if dw_current_time > self.next_frame_time {
            let dt = dw_current_time.wrapping_sub(self.next_frame_time) as i32;
            let n_adjust_time = ((dt as f32 / frame_time as f32) * frame_time as f32) as i32;

            if dt >= 500 {
                self.next_frame_time = self.next_frame_time.wrapping_add(n_adjust_time as u32);
            }

            b_frame_skip = true;
            b_current_late_update = true;
        }

        if ctx.is_frame_skip_disable() {
            b_frame_skip = false;
        }

        if !b_frame_skip {
            let dw_render_start_time = ctx.get_time_ms();

            if !ctx.is_active() {
                self.skip_render_buffering(ctx, 3000);
            }

            let render_res = ctx.render();

            if !render_res.can_render {
                self.skip_render_buffering(ctx, 3000);
            } else {
                let dw_render_end_time = ctx.get_time_ms();

                self.cur_render_time = dw_render_end_time.wrapping_sub(dw_render_start_time);
                self.render_range_time = self.render_range_time.wrapping_add(self.cur_render_time);
                self.render_range_frame += 1;

                if dw_render_end_time.wrapping_sub(self.render_check_time) > 1000 {
                    self.ave_render_time = (self.render_range_time as f64 / self.render_range_frame.max(1) as f64) as f32;
                    self.render_check_time = ctx.get_time_ms();
                    self.render_range_time = 0;
                    self.render_range_frame = 0;
                }

                let dw_cur_face_count = render_res.face_count;
                self.face_count_acc += dw_cur_face_count;

                if dw_cur_face_count > 5000 {
                    if dw_render_end_time > self.buf_sleep_skip_time {
                        let f_cur_render_time = self.cur_render_time as f32;

                        if f_cur_render_time > self.buf_render_time {
                            let f_ratio = f32::max(0.5, (f_cur_render_time - self.buf_render_time) / 30.0);
                            self.buf_render_time = (self.buf_render_time * (100.0 - f_ratio) + (f_cur_render_time + 5.0) * f_ratio) / 100.0;
                        } else {
                            let f_ratio = 0.5;
                            self.buf_render_time = (self.buf_render_time * (100.0 - f_ratio) + f_cur_render_time * f_ratio) / 100.0;
                        }

                        if self.buf_render_time > 100.0 {
                            self.buf_render_time = 100.0;
                        }

                        let mut dw_buf_render_time = self.buf_render_time as u32;

                        if ctx.is_windowed() {
                            if dw_buf_render_time > 58 {
                                dw_buf_render_time = 64;
                            } else if dw_buf_render_time > 42 {
                                dw_buf_render_time = 48;
                            } else if dw_buf_render_time > 26 {
                                dw_buf_render_time = 32;
                            } else if dw_buf_render_time > 10 {
                                dw_buf_render_time = 16;
                            } else {
                                dw_buf_render_time = 8;
                            }
                        }

                        // W C++ jest komentarz ze "w pewnym momencie zrobili sleep(buf - cur)"
                        // i jest wykomentowane, ale zostawiaja ave = buf. Wiec zmienna jest ok, 
                        // zeby uniknac ostrzezenia o nieuzywanej zmiennej - po prostu jej uzyjmy w asercji albo nic nie robmy.
                        let _unused = dw_buf_render_time;

                        self.ave_render_time = self.buf_render_time;
                    }

                    self.face_acc_count += dw_cur_face_count;
                    self.face_acc_time += self.cur_render_time;

                    if self.face_acc_time > 0 {
                        self.face_spd = self.face_acc_count as f32 / self.face_acc_time as f32;
                    } else {
                        self.face_spd = 0.0;
                    }

                    if ctx.force_sight_range() == -1 {
                        let f_ratio = 0.3;
                        self.s_ave_render_time = (self.s_ave_render_time * (100.0 - f_ratio) + f32::max(16.0, self.cur_render_time as f32) * f_ratio) / 100.0;

                        let f_far = 25600.0;
                        let f_near = ctx.get_min_fog();
                        let db_ave_pow = 1000.0 / self.s_ave_render_time as f64;
                        let db_max_pow = 60.0;
                        let mut f_distance = f_near + (f_far - f_near) * (db_ave_pow as f32) / (db_max_pow as f32);
                        if f_distance < f_near {
                            f_distance = f_near;
                        }
                        ctx.set_view_distance(f_distance);
                    } else {
                        ctx.set_view_distance(ctx.force_sight_range() as f32);
                    }
                } else {
                    ctx.set_view_distance(25600.0);
                }

                self.render_frame_count += 1;
            }
        }

        let rest = self.next_frame_time.wrapping_sub(ctx.get_time_ms()) as i32;

        if rest > 0 && !b_current_late_update {
            let urest = rest as u32;
            if self.ui_load >= urest {
                self.ui_load -= urest;
            } else {
                self.ui_load = 0;
            }
            ctx.sleep(urest);
        }

        self.update_frame_count += 1;
        self.ui_load += ctx.get_time_ms().wrapping_sub(dw_start);

        true
    }
}
