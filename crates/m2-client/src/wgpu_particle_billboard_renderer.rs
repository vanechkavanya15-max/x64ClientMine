use glam::{Mat4, Quat, Vec2, Vec3};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum BillboardType {
    None = 0,
    All = 1,
    Y = 2,
    Lie = 3,
    TwoFace = 4,
    ThreeFace = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum BlendType {
    Zero = 1,
    One = 2,
    SrcColor = 3,
    InvSrcColor = 4,
    SrcAlpha = 5,
    InvSrcAlpha = 6,
    DestAlpha = 7,
    InvDestAlpha = 8,
    DestColor = 9,
    InvDestColor = 10,
    SrcAlphaSat = 11,
    BothSrcAlpha = 12,
    InvBothSrcAlpha = 13,
}

#[derive(Debug, Clone)]
pub struct WgpuBlendState {
    pub src_blend: BlendType,
    pub dest_blend: BlendType,
}

impl WgpuBlendState {
    pub fn is_additive(&self) -> bool {
        self.src_blend == BlendType::One && self.dest_blend == BlendType::One
    }

    pub fn is_alpha_blend(&self) -> bool {
        self.src_blend == BlendType::SrcAlpha && self.dest_blend == BlendType::InvSrcAlpha
    }
}

pub fn get_blend_state(src_blend: u8, dest_blend: u8) -> WgpuBlendState {
    let to_blend_type = |v: u8| match v {
        2 => BlendType::One,
        5 => BlendType::SrcAlpha,
        6 => BlendType::InvSrcAlpha,
        _ => BlendType::One, // fallback
    };
    WgpuBlendState {
        src_blend: to_blend_type(src_blend),
        dest_blend: to_blend_type(dest_blend),
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct ParticleVertex {
    pub position: [f32; 3],
    pub tex_coord: [f32; 2],
}

pub struct ParticleBillboardRenderer;

impl ParticleBillboardRenderer {
    pub fn calculate_billboard_vertices(
        position: Vec3,
        scale: Vec2,
        half_size: Vec2,
        rotation: f32, // in degrees
        billboard_type: BillboardType,
        camera_up: Vec3,
        camera_cross: Vec3,
        camera_view: Vec3,
    ) -> [ParticleVertex; 4] {
        let mut v3_up = Vec3::ZERO;
        let mut v3_cross = Vec3::ZERO;

        match billboard_type {
            BillboardType::Lie => {
                let rot_rad = rotation.to_radians();
                let f_cos = rot_rad.cos();
                let f_sin = rot_rad.sin();
                v3_up = Vec3::new(f_cos, -f_sin, 0.0);
                v3_cross = Vec3::new(f_sin, f_cos, 0.0);
            }
            BillboardType::TwoFace | BillboardType::ThreeFace | BillboardType::Y => {
                v3_up = Vec3::new(0.0, 0.0, 1.0);
                
                // if v3Up.x * c_rv3View.y - v3Up.y * c_rv3View.x < 0
                if v3_up.x * camera_view.y - v3_up.y * camera_view.x < 0.0 {
                    v3_up = -v3_up;
                }
                
                let view_xy = Vec3::new(camera_view.x, camera_view.y, 0.0);
                v3_cross = v3_up.cross(view_xy).normalize();

                if rotation != 0.0 {
                    let rot_rad = rotation.to_radians();
                    let f_cos = -rot_rad.sin(); // -sinf(D3DXToRadian(m_fRotation)) // + 90
                    let f_sin = rot_rad.cos();  // cosf(D3DXToRadian(m_fRotation))

                    let temp_up = v3_up * f_cos - v3_cross * f_sin;
                    v3_cross = v3_cross * f_cos + v3_up * f_sin;
                    v3_up = temp_up;
                }
            }
            BillboardType::All | _ => {
                if rotation == 0.0 {
                    v3_up = -camera_cross;
                    v3_cross = camera_up;
                } else {
                    let rot_rad = rotation.to_radians();
                    let q = Quat::from_axis_angle(camera_view, rot_rad);
                    let qc = q.conjugate();

                    // D3DXQUATERNION qr(-c_rv3Cross.x, -c_rv3Cross.y, -c_rv3Cross.z, 0);
                    // D3DXQuaternionMultiply(&qr,&qc,&qr);
                    // D3DXQuaternionMultiply(&qr,&qr,&q);
                    // This is equivalent to rotating the vector by q
                    v3_up = q * (-camera_cross);
                    v3_cross = q * camera_up;
                }
            }
        }

        v3_cross = -(half_size.x * scale.x) * v3_cross;
        v3_up = (half_size.y * scale.y) * v3_up;

        [
            ParticleVertex { position: (position - v3_up + v3_cross).into(), tex_coord: [0.0, 1.0] },
            ParticleVertex { position: (position - v3_up - v3_cross).into(), tex_coord: [0.0, 0.0] },
            ParticleVertex { position: (position + v3_up + v3_cross).into(), tex_coord: [1.0, 1.0] },
            ParticleVertex { position: (position + v3_up - v3_cross).into(), tex_coord: [1.0, 0.0] },
        ]
    }
    
    pub fn get_time_event_blend_value_color(time: f32, events: &[(f32, [f32; 4])]) -> [f32; 4] {
        if events.is_empty() {
            return [1.0, 1.0, 1.0, 1.0];
        }
    
        if events.len() == 1 || time <= events[0].0 {
            return events[0].1;
        }
    
        if time >= events.last().unwrap().0 {
            return events.last().unwrap().1;
        }
    
        let idx = match events.binary_search_by(|e| e.0.partial_cmp(&time).unwrap()) {
            Ok(index) => return events[index].1,
            Err(index) => index,
        };
    
        let low = &events[idx - 1];
        let high = &events[idx];
    
        let time_diff = high.0 - low.0;
        let perc = (time - low.0) / time_diff;
    
        let mut result = [0.0; 4];
        for i in 0..4 {
            let val_diff = high.1[i] - low.1[i];
            result[i] = low.1[i] + perc * val_diff;
        }
    
        result
    }

    pub fn get_time_event_blend_value_f32(time: f32, events: &[(f32, f32)]) -> f32 {
        if events.is_empty() {
            return 1.0; 
        }

        if events.len() == 1 {
            return events[0].1;
        }

        if time <= events[0].0 {
            return events[0].1;
        }

        if time >= events.last().unwrap().0 {
            return events.last().unwrap().1;
        }

        let idx = match events.binary_search_by(|e| e.0.partial_cmp(&time).unwrap()) {
            Ok(index) => return events[index].1,
            Err(index) => index,
        };

        let low = &events[idx - 1];
        let high = &events[idx];

        let time_diff = high.0 - low.0;
        let perc = (time - low.0) / time_diff;
        let val_diff = high.1 - low.1;

        low.1 + perc * val_diff
    }
}