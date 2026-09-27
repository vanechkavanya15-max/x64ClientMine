#[derive(Debug, Default)]
pub struct ShutdownManager {
    is_shutting_down: bool,
    exit_code: i32,
    cleanup_stages_completed: u32,
}

impl ShutdownManager {
    pub fn new() -> Self {
        Self {
            is_shutting_down: false,
            exit_code: 0,
            cleanup_stages_completed: 0,
        }
    }

    pub fn is_shutting_down(&self) -> bool {
        self.is_shutting_down
    }

    pub fn exit_code(&self) -> i32 {
        self.exit_code
    }

    pub fn cleanup_stages_completed(&self) -> u32 {
        self.cleanup_stages_completed
    }

    pub fn request_shutdown(&mut self, exit_code: i32) {
        self.is_shutting_down = true;
        self.exit_code = exit_code;
    }

    pub fn execute_cleanup_pipeline<F>(&mut self, stages: &mut [F]) -> bool
    where
        F: FnMut() -> bool,
    {
        if !self.is_shutting_down {
            return false;
        }

        self.cleanup_stages_completed = 0;

        for stage in stages {
            if stage() {
                self.cleanup_stages_completed += 1;
            } else {
                return false;
            }
        }

        true
    }
}
