use std::collections::{VecDeque, HashSet};
use std::sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}, Condvar};
use std::thread;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TileCoord {
    pub x: i32,
    pub y: i32,
}

impl TileCoord {
    pub fn is_within_3x3_of(&self, center: &TileCoord) -> bool {
        (self.x - center.x).abs() <= 1 && (self.y - center.y).abs() <= 1
    }

    pub fn get_map_id(&self) -> u32 {
        (self.x.max(0) as u32) * 1000 + (self.y.max(0) as u32)
    }
}

#[derive(Debug, Clone)]
pub struct TerrainPatchData {
    pub coord: TileCoord,
    pub map_id: u32,
    pub area_property_path: String,
    pub height_raw_path: String,
    pub splat_path: String,
    pub attr_path: String,
    pub water_path: String,
    pub shadow_tex_path: String,
    pub shadow_map_path: String,
    pub mini_map_tex_path: String,
}

pub struct AreaLoaderThread {
    load_queue: Arc<Mutex<VecDeque<TileCoord>>>,
    completed_queue: Arc<Mutex<VecDeque<TerrainPatchData>>>,
    shutdown_flag: Arc<AtomicBool>,
    current_center: Arc<Mutex<Option<TileCoord>>>,
    worker_handle: Option<thread::JoinHandle<()>>,
    condvar: Arc<Condvar>,
}

impl AreaLoaderThread {
    pub fn new() -> Self {
        let load_queue: Arc<Mutex<VecDeque<TileCoord>>> = Arc::new(Mutex::new(VecDeque::new()));
        let completed_queue: Arc<Mutex<VecDeque<TerrainPatchData>>> = Arc::new(Mutex::new(VecDeque::new()));
        let shutdown_flag = Arc::new(AtomicBool::new(false));
        let current_center: Arc<Mutex<Option<TileCoord>>> = Arc::new(Mutex::new(None));
        let condvar = Arc::new(Condvar::new());

        let worker_load_queue = Arc::clone(&load_queue);
        let worker_completed_queue = Arc::clone(&completed_queue);
        let worker_shutdown_flag = Arc::clone(&shutdown_flag);
        let worker_current_center = Arc::clone(&current_center);
        let worker_condvar = Arc::clone(&condvar);

        let worker_handle = thread::spawn(move || {
            loop {
                if worker_shutdown_flag.load(Ordering::Relaxed) {
                    break;
                }

                let mut queue_lock = worker_load_queue.lock().unwrap();
                while queue_lock.is_empty() && !worker_shutdown_flag.load(Ordering::Relaxed) {
                    queue_lock = worker_condvar.wait(queue_lock).unwrap();
                }

                if worker_shutdown_flag.load(Ordering::Relaxed) {
                    break;
                }

                if let Some(tile) = queue_lock.pop_front() {
                    drop(queue_lock);

                    // Check if tile is still within the 3x3 grid around current_center
                    let center_guard = worker_current_center.lock().unwrap();
                    let is_valid = match *center_guard {
                        Some(center) => tile.is_within_3x3_of(&center),
                        None => false,
                    };
                    drop(center_guard);

                    if is_valid {
                        let map_id = tile.get_map_id();
                        let map_name = format!("metin2_map_test_{}", map_id);

                        let area_property_path = format!("{}\\{:06}\\AreaProperty.txt", map_name, map_id);
                        
                        // Simulate AreaProperty.txt parsing mimicking LoadMultipleTextData in C++
                        let mut script_type = String::new();
                        let mut area_name = String::new();

                        // Simulate checking for valid scripttype and areaname
                        if map_id <= 100000000 {
                            script_type = "AreaProperty".to_string();
                            area_name = format!("Area_{}", map_id);
                        }

                        if script_type == "AreaProperty" && !area_name.is_empty() {
                            let height_raw_path = format!("{}\\{:06}\\height.raw", map_name, map_id);
                            let splat_path = format!("{}\\{:06}\\tile.raw", map_name, map_id);
                            let attr_path = format!("{}\\{:06}\\attr.atr", map_name, map_id);
                            let water_path = format!("{}\\{:06}\\water.wtr", map_name, map_id);
                            let shadow_tex_path = format!("{}\\{:06}\\shadowmap.dds", map_name, map_id);
                            let shadow_map_path = format!("{}\\{:06}\\shadowmap.raw", map_name, map_id);
                            let mini_map_tex_path = format!("{}\\{:06}\\minimap.dds", map_name, map_id);

                            let patch_data = TerrainPatchData {
                                coord: tile,
                                map_id,
                                area_property_path,
                                height_raw_path,
                                splat_path,
                                attr_path,
                                water_path,
                                shadow_tex_path,
                                shadow_map_path,
                                mini_map_tex_path,
                            };

                            worker_completed_queue.lock().unwrap().push_back(patch_data);
                        }
                    }
                }
            }
        });

        Self {
            load_queue,
            completed_queue,
            shutdown_flag,
            current_center,
            worker_handle: Some(worker_handle),
            condvar,
        }
    }

    pub fn request_load(&self, center: TileCoord) {
        let mut center_guard = self.current_center.lock().unwrap();
        *center_guard = Some(center);
        drop(center_guard);

        // Maintain deterministic order: loop over y then x
        let mut required_tiles_ordered = Vec::new();
        let mut required_tiles_set = HashSet::new();
        
        for dy in -1..=1 {
            for dx in -1..=1 {
                let tile = TileCoord {
                    x: center.x + dx,
                    y: center.y + dy,
                };
                required_tiles_ordered.push(tile);
                required_tiles_set.insert(tile);
            }
        }

        let mut queue_lock = self.load_queue.lock().unwrap();
        
        // Remove tiles that are no longer in the 3x3 grid
        queue_lock.retain(|tile| required_tiles_set.contains(tile));
        
        let existing_tiles: HashSet<TileCoord> = queue_lock.iter().cloned().collect();
        
        for tile in required_tiles_ordered {
            if !existing_tiles.contains(&tile) {
                queue_lock.push_back(tile);
            }
        }
        
        // Notify all because we pushed multiple items
        self.condvar.notify_all();
    }

    pub fn fetch_completed(&self) -> Option<TerrainPatchData> {
        self.completed_queue.lock().unwrap().pop_front()
    }

    pub fn shutdown(&mut self) {
        self.shutdown_flag.store(true, Ordering::Relaxed);
        self.condvar.notify_all();
        if let Some(handle) = self.worker_handle.take() {
            let _ = handle.join();
        }
        self.load_queue.lock().unwrap().clear();
        self.completed_queue.lock().unwrap().clear();
    }
}

impl Drop for AreaLoaderThread {
    fn drop(&mut self) {
        self.shutdown();
    }
}

impl Default for AreaLoaderThread {
    fn default() -> Self {
        Self::new()
    }
}
