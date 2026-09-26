use std::mem::size_of;
use m2_client_bridge::ffi_guard::safe_ffi_call;
use crate::character_manager_core::get_character_manager;

pub const FLY_TYPE_NORMAL: u8 = 0;
pub const FLY_TYPE_FIRE_CRACKER: u8 = 1;
pub const FLY_TYPE_AUTO_FIRE: u8 = 2;
pub const FLY_TYPE_MAGIC: u8 = 3;
pub const FLY_TYPE_ARROW: u8 = 4;

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(non_snake_case)]
pub struct TPacketGCFly {
    pub header: u16,
    pub length: u16,
    pub bType: u8,
    pub dwStartVID: u32,
    pub dwEndVID: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BallisticTrajectory {
    pub start_x: f32,
    pub start_y: f32,
    pub start_z: f32,
    pub end_x: f32,
    pub end_y: f32,
    pub end_z: f32,
    pub distance: f32,
    pub travel_time: f32,
    pub max_height: f32,
}

impl BallisticTrajectory {
    /// Oblicza lot po luku paraboli (interpolacja balistyczna).
    pub fn calculate(
        start_x: f32, start_y: f32, start_z: f32,
        end_x: f32, end_y: f32, end_z: f32,
        speed: f32,
        gravity: f32,
    ) -> Self {
        let dx = end_x - start_x;
        let dy = end_y - start_y;
        let dz = end_z - start_z;
        let distance_xy = (dx * dx + dy * dy).sqrt();
        let distance_xyz = (dx * dx + dy * dy + dz * dz).sqrt();
        
        let travel_time = if speed > 0.0 { distance_xyz / speed } else { 0.0 };
        
        // height = max(z) = v0z * (t/2) - 0.5 * g * (t/2)^2 = ... 
        // W uproszczeniu obliczamy maksymalna wysokosc paraboli lotu wzgledem prostej:
        // h = (g * t^2) / 8
        let max_height = if gravity > 0.0 && travel_time > 0.0 {
            (gravity * travel_time * travel_time) / 8.0
        } else {
            0.0
        };

        Self {
            start_x,
            start_y,
            start_z,
            end_x,
            end_y,
            end_z,
            distance: distance_xyz,
            travel_time,
            max_height,
        }
    }

    /// Pobiera pozycje w danym momencie czasu [0, travel_time]
    pub fn get_position_at(&self, t: f32, gravity: f32) -> (f32, f32, f32) {
        if self.travel_time <= 0.0 {
            return (self.end_x, self.end_y, self.end_z);
        }

        let clamped_t = t.clamp(0.0, self.travel_time);
        let progress = clamped_t / self.travel_time;
        
        let x = self.start_x + (self.end_x - self.start_x) * progress;
        let y = self.start_y + (self.end_y - self.start_y) * progress;
        
        // Z liniowo
        let base_z = self.start_z + (self.end_z - self.start_z) * progress;
        
        // Z grawitacja (parabola: h = v0 * t - 0.5 * g * t^2, gdzie v0 = g * travel_time / 2 zeby wrocilo na 0)
        let v0_z = (gravity * self.travel_time) / 2.0;
        let gravity_offset = v0_z * clamped_t - 0.5 * gravity * clamped_t * clamped_t;
        
        let z = base_z + gravity_offset;
        
        (x, y, z)
    }
}

pub fn parse_recv_fly(data: &[u8]) -> Result<(u8, u32, u32, Option<BallisticTrajectory>), &'static str> {
    if data.len() < size_of::<TPacketGCFly>() {
        return Err("Buffer too short");
    }

    let header = u16::from_le_bytes(data[0..2].try_into().unwrap());
    let length = u16::from_le_bytes(data[2..4].try_into().unwrap());
    let bType = data[4];
    let dwStartVID = u32::from_le_bytes(data[5..9].try_into().unwrap());
    let dwEndVID = u32::from_le_bytes(data[9..13].try_into().unwrap());

    let mut trajectory = None;

    if let Ok(mgr) = get_character_manager().read() {
        if let (Some(start_inst), Some(end_inst)) = (mgr.get_instance(dwStartVID), mgr.get_instance(dwEndVID)) {
            let speed = 1000.0; // Przykładowa prędkość lotu
            let gravity = 980.0; // Przykładowa grawitacja (ok. 9.8m/s^2 skalowane w cm)
            trajectory = Some(BallisticTrajectory::calculate(
                start_inst.x, start_inst.y, start_inst.z,
                end_inst.x, end_inst.y, end_inst.z,
                speed, gravity
            ));
        }
    }

    Ok((bType, dwStartVID, dwEndVID, trajectory))
}

#[no_mangle]
pub extern "C" fn m2_net_recv_fly(buf: *const u8, len: usize) -> bool {
    safe_ffi_call(false, || {
        if buf.is_null() || len == 0 {
            return false;
        }
        let slice = unsafe { std::slice::from_raw_parts(buf, len) };
        parse_recv_fly(slice).is_ok()
    })
}
