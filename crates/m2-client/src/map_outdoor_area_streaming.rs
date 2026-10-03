use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SectorCoord {
    pub x: i32,
    pub y: i32,
}

impl SectorCoord {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

pub struct AreaStreamingManager {
    center: Option<SectorCoord>,
    active_sectors: HashSet<SectorCoord>,
}

impl AreaStreamingManager {
    pub fn new() -> Self {
        Self {
            center: None,
            active_sectors: HashSet::new(),
        }
    }

    /// Updates the center and computes sectors to load and unload.
    /// The streaming area is a 3x3 grid around the center.
    pub fn update_center(
        &mut self,
        new_x: i32,
        new_y: i32,
    ) -> (Vec<SectorCoord>, Vec<SectorCoord>) {
        let new_center = SectorCoord::new(new_x, new_y);

        // If the center hasn't changed, nothing to do.
        if Some(new_center) == self.center {
            return (Vec::new(), Vec::new());
        }

        self.center = Some(new_center);

        let mut required_sectors = HashSet::with_capacity(9);
        for dx in -1..=1 {
            for dy in -1..=1 {
                required_sectors.insert(SectorCoord::new(new_x + dx, new_y + dy));
            }
        }

        let mut sectors_to_load = Vec::new();
        let mut sectors_to_unload = Vec::new();

        // Find sectors to unload (currently active but not in required)
        for &sector in self.active_sectors.iter() {
            if !required_sectors.contains(&sector) {
                sectors_to_unload.push(sector);
            }
        }

        // Find sectors to load (required but not currently active)
        for &sector in required_sectors.iter() {
            if !self.active_sectors.contains(&sector) {
                sectors_to_load.push(sector);
            }
        }

        // Update active sectors state
        for sector in &sectors_to_unload {
            self.active_sectors.remove(sector);
        }
        for sector in &sectors_to_load {
            self.active_sectors.insert(*sector);
        }

        (sectors_to_load, sectors_to_unload)
    }

    /// Unloads all active sectors.
    pub fn clear(&mut self) -> Vec<SectorCoord> {
        self.center = None;
        let sectors_to_unload: Vec<SectorCoord> = self.active_sectors.drain().collect();
        sectors_to_unload
    }

    pub fn get_active_sectors(&self) -> &HashSet<SectorCoord> {
        &self.active_sectors
    }

    pub fn get_center(&self) -> Option<SectorCoord> {
        self.center
    }
}
