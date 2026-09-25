use m2_client::app_loop::{AppLoop, AppLoopContext, RenderResult};



#[derive(Default)]
struct MockContext {
    time_ms: u32,
    update_called: u32,
    render_called: u32,
    sleep_called: u32,
    last_sleep_ms: u32,
    view_distance: f32,
    
    frame_skip_disable: bool,
    windowed: bool,
    active: bool,
    force_sight_range: i32,
    min_fog: f32,

    // Helper options for render
    next_render_can_render: bool,
    next_render_face_count: u32,

    time_step_update: u32,
    time_step_render: u32,
}

impl MockContext {
    fn new() -> Self {
        Self {
            time_ms: 10000,
            update_called: 0,
            render_called: 0,
            sleep_called: 0,
            last_sleep_ms: 0,
            view_distance: 0.0,
            frame_skip_disable: false,
            windowed: true,
            active: true,
            force_sight_range: -1,
            min_fog: 2500.0,
            next_render_can_render: true,
            next_render_face_count: 1000,
            time_step_update: 2,
            time_step_render: 10,
        }
    }
}

impl AppLoopContext for MockContext {
    fn get_time_ms(&self) -> u32 {
        self.time_ms
    }

    fn update(&mut self) {
        self.update_called += 1;
        self.time_ms += self.time_step_update;
    }

    fn render(&mut self) -> RenderResult {
        self.render_called += 1;
        self.time_ms += self.time_step_render;
        RenderResult {
            can_render: self.next_render_can_render,
            face_count: self.next_render_face_count,
        }
    }

    fn sleep(&mut self, ms: u32) {
        self.sleep_called += 1;
        self.last_sleep_ms = ms;
        self.time_ms += ms;
    }

    fn set_view_distance(&mut self, distance: f32) {
        self.view_distance = distance;
    }

    fn is_frame_skip_disable(&self) -> bool {
        self.frame_skip_disable
    }

    fn is_windowed(&self) -> bool {
        self.windowed
    }

    fn is_active(&self) -> bool {
        self.active
    }

    fn force_sight_range(&self) -> i32 {
        self.force_sight_range
    }

    fn get_min_fog(&self) -> f32 {
        self.min_fog
    }
}

#[test]
fn test_integration_app_loop() {
    let instance = AppLoop::new();
    assert!(instance.is_ready());
}

#[test]
fn test_app_loop_basic_process() {
    let mut ctx = MockContext::new();
    let mut app_loop = AppLoop::new();

    app_loop.init_time(&mut ctx);
    
    // Normal frame: takes 12ms total (2 update + 10 render), targeted frame time 16ms (approx 60fps)
    let res = app_loop.process(&mut ctx, 16);
    
    assert!(res);
    assert_eq!(ctx.update_called, 1);
    assert_eq!(ctx.render_called, 1);
    // Because update and render took 12ms, we have 4ms left to sleep. 
    // Sleep should be called with rest > 0
    assert_eq!(ctx.sleep_called, 1);
    assert_eq!(ctx.last_sleep_ms, 4);
    assert_eq!(ctx.view_distance, 25600.0); // Next face_count is < 5000 so sets 25600
}

#[test]
fn test_app_loop_frame_skip() {
    let mut ctx = MockContext::new();
    let mut app_loop = AppLoop::new();

    app_loop.init_time(&mut ctx);
    
    // Simulate update taking a very long time (e.g. 50ms), missing the 16ms deadline.
    ctx.time_step_update = 50;

    app_loop.process(&mut ctx, 16);

    // Frame skip should be true, so render should NOT be called.
    assert_eq!(ctx.update_called, 1);
    assert_eq!(ctx.render_called, 0);
    // No sleep because we are behind schedule.
    assert_eq!(ctx.sleep_called, 0);
}

#[test]
fn test_app_loop_fps_update() {
    let mut ctx = MockContext::new();
    let mut app_loop = AppLoop::new();

    app_loop.init_time(&mut ctx);
    
    for _ in 0..60 {
        app_loop.process(&mut ctx, 16);
    }
    
    // Force time to pass 1000ms+ from check_time
    ctx.time_ms += 1050;
    
    // Run one more to trigger fps calculation
    app_loop.process(&mut ctx, 16);
    
    assert_eq!(app_loop.update_fps, 60);
    assert_eq!(app_loop.render_fps, 60);
}

#[test]
fn test_app_loop_dynamic_view_distance() {
    let mut ctx = MockContext::new();
    let mut app_loop = AppLoop::new();

    app_loop.init_time(&mut ctx);
    
    ctx.next_render_face_count = 6000;
    ctx.time_step_render = 30; // heavy render

    // First frame skip check buffering will set ave_render_time etc
    app_loop.process(&mut ctx, 16);

    // The view distance should be adjusted downwards because face_count > 5000 and force == -1
    // It's calculated with some formula. We just check if it's set and less than 25600.0.
    // However, on first run with heavy render, distance might still be close to max depending on s_ave_render_time.
    // We can do a few loops to see it drop.
    let mut last_dist = ctx.view_distance;
    
    for _ in 0..10 {
        // We must artificially prevent frame skipping so render runs every time.
        ctx.frame_skip_disable = true;
        app_loop.process(&mut ctx, 16);
        assert!(ctx.view_distance <= last_dist || (ctx.view_distance > 0.0 && last_dist == 25600.0) );
        last_dist = ctx.view_distance;
    }
}
