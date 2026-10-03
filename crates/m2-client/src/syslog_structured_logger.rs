use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct SyslogStructuredLogger {
    file: Mutex<File>,
}

impl SyslogStructuredLogger {
    pub fn new(path: &str) -> io::Result<Self> {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)?;
        
        Ok(Self {
            file: Mutex::new(file),
        })
    }

    fn write_log(&self, category: &str, message: &str) -> io::Result<()> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        let log_entry = format!("[{}] [{}] {}\n", now, category, message);
        
        let mut file = self.file.lock().map_err(|e| {
            io::Error::new(io::ErrorKind::Other, format!("Mutex lock failed: {}", e))
        })?;
        
        file.write_all(log_entry.as_bytes())?;
        file.flush()?;
        
        Ok(())
    }

    pub fn log_engine_stage(&self, stage: &str) -> io::Result<()> {
        self.write_log("ENGINE", &format!("Stage: {}", stage))
    }

    pub fn log_gpu_info(&self, gpu_name: &str) -> io::Result<()> {
        self.write_log("GPU", &format!("Detected GPU: {}", gpu_name))
    }

    pub fn log_loaded_module(&self, module: &str) -> io::Result<()> {
        self.write_log("MODULE", &format!("Loaded module: {}", module))
    }
}
