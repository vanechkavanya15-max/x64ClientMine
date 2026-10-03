use m2_client_bridge::ffi_guard::safe_ffi_call;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::Mutex;
use std::collections::VecDeque;

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct TelemetryData {
    pub ram_usage_bytes: u64,
    pub allocation_count: usize,
    pub last_frame_time_us: u64,
    pub avg_frame_time_us: u64,
    pub min_frame_time_us: u64,
    pub max_frame_time_us: u64,
}

const MAX_FRAME_SAMPLES: usize = 60;

pub struct PerformanceTelemetry {
    ram_usage_bytes: AtomicU64,
    allocation_count: AtomicUsize,
    frame_times: Mutex<VecDeque<u64>>,
}

impl PerformanceTelemetry {
    pub const fn new() -> Self {
        Self {
            ram_usage_bytes: AtomicU64::new(0),
            allocation_count: AtomicUsize::new(0),
            frame_times: Mutex::new(VecDeque::new()),
        }
    }

    pub fn update_ram_usage(&self, bytes: u64) {
        self.ram_usage_bytes.store(bytes, Ordering::Relaxed);
    }

    pub fn add_allocations(&self, count: usize) {
        self.allocation_count.fetch_add(count, Ordering::Relaxed);
    }

    pub fn record_frame_time(&self, time_us: u64) {
        let mut frames = self.frame_times.lock().unwrap();
        if frames.len() >= MAX_FRAME_SAMPLES {
            frames.pop_front();
        }
        frames.push_back(time_us);
    }

    pub fn get_data(&self) -> TelemetryData {
        let frames = self.frame_times.lock().unwrap();
        
        let last_frame_time_us = frames.back().copied().unwrap_or(0);
        
        let (min, max, avg) = if frames.is_empty() {
            (0, 0, 0)
        } else {
            let mut min_t = u64::MAX;
            let mut max_t = 0;
            let mut sum_t: u64 = 0;
            for &t in frames.iter() {
                if t < min_t { min_t = t; }
                if t > max_t { max_t = t; }
                sum_t += t;
            }
            (min_t, max_t, sum_t / (frames.len() as u64))
        };

        TelemetryData {
            ram_usage_bytes: self.ram_usage_bytes.load(Ordering::Relaxed),
            allocation_count: self.allocation_count.load(Ordering::Relaxed),
            last_frame_time_us,
            avg_frame_time_us: avg,
            min_frame_time_us: min,
            max_frame_time_us: max,
        }
    }

    pub fn reset(&self) {
        self.ram_usage_bytes.store(0, Ordering::Relaxed);
        self.allocation_count.store(0, Ordering::Relaxed);
        self.frame_times.lock().unwrap().clear();
    }
}

lazy_static::lazy_static! {
    pub static ref GLOBAL_TELEMETRY: PerformanceTelemetry = PerformanceTelemetry::new();
}

#[no_mangle]
pub extern "C" fn m2_telemetry_update_ram_usage(bytes: u64) {
    safe_ffi_call((), || {
        GLOBAL_TELEMETRY.update_ram_usage(bytes);
    });
}

#[no_mangle]
pub extern "C" fn m2_telemetry_add_allocations(count: usize) {
    safe_ffi_call((), || {
        GLOBAL_TELEMETRY.add_allocations(count);
    });
}

#[no_mangle]
pub extern "C" fn m2_telemetry_record_frame_time(time_us: u64) {
    safe_ffi_call((), || {
        GLOBAL_TELEMETRY.record_frame_time(time_us);
    });
}

#[no_mangle]
pub extern "C" fn m2_telemetry_get_data(out_data: *mut TelemetryData) {
    safe_ffi_call((), || {
        if out_data.is_null() {
            return;
        }
        let data = GLOBAL_TELEMETRY.get_data();
        unsafe {
            *out_data = data;
        }
    });
}

#[no_mangle]
pub extern "C" fn m2_telemetry_reset() {
    safe_ffi_call((), || {
        GLOBAL_TELEMETRY.reset();
    });
}
