use std::collections::VecDeque;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TPixelPosition {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl TPixelPosition {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct D3DXQUATERNION {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl D3DXQUATERNION {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SCommand {
    pub m_kPPosDst: TPixelPosition,
    pub m_dwChkTime: u32,
    pub m_dwCmdTime: u32,
    pub m_fDstRot: f32,
    pub m_eFunc: u32,
    pub m_uArg: u32,
    pub m_uTargetVID: u32,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct MovementInterpolator {
    pub m_nAverageNetworkGap: i32,
    pub m_kQue_kCmdNew: VecDeque<SCommand>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InterpolationError {
    InvalidTime,
}

impl MovementInterpolator {
    pub fn new() -> Self {
        Self {
            m_nAverageNetworkGap: 0,
            m_kQue_kCmdNew: VecDeque::new(),
        }
    }

    pub fn push_tcp_state_expanded(
        &mut self,
        dw_cmd_time: u32,
        c_rk_p_pos_dst: TPixelPosition,
        f_dst_rot: f32,
        e_func: u32,
        u_arg: u32,
        u_target_vid: u32,
    ) {
        let k_cmd_new = SCommand {
            m_kPPosDst: c_rk_p_pos_dst,
            m_dwChkTime: dw_cmd_time + 100,
            m_dwCmdTime: dw_cmd_time,
            m_fDstRot: f_dst_rot,
            m_eFunc: e_func,
            m_uArg: u_arg,
            m_uTargetVID: u_target_vid,
        };
        self.m_kQue_kCmdNew.push_back(k_cmd_new);
    }

    pub fn push_tcp_state(
        &mut self,
        dw_cmd_time: u32,
        c_rk_p_pos_dst: TPixelPosition,
        f_dst_rot: f32,
        e_func: u32,
        u_arg: u32,
        server_frame_msec: u32,
    ) {
        let n_network_gap = server_frame_msec as i32 - dw_cmd_time as i32;
        self.m_nAverageNetworkGap = (self.m_nAverageNetworkGap * 70 + n_network_gap * 30) / 100;

        let k_cmd_new = SCommand {
            m_kPPosDst: c_rk_p_pos_dst,
            m_dwChkTime: dw_cmd_time + self.m_nAverageNetworkGap as u32,
            m_dwCmdTime: dw_cmd_time,
            m_fDstRot: f_dst_rot,
            m_eFunc: e_func,
            m_uArg: u_arg,
            m_uTargetVID: 0,
        };
        self.m_kQue_kCmdNew.push_back(k_cmd_new);
    }

    pub fn process_state(&mut self, cur_chk_time: u32) -> Option<SCommand> {
        if self.m_kQue_kCmdNew.is_empty() {
            return None;
        }

        let dw_dst_chk_time = self.m_kQue_kCmdNew.front().unwrap().m_dwChkTime;
        if cur_chk_time < dw_dst_chk_time {
            return None;
        }

        self.m_kQue_kCmdNew.pop_front()
    }

    pub fn slerp(q1: D3DXQUATERNION, q2: D3DXQUATERNION, t: f32) -> Result<D3DXQUATERNION, InterpolationError> {
        if t < 0.0 || t > 1.0 {
            return Err(InterpolationError::InvalidTime);
        }

        let mut cos_theta = q1.x * q2.x + q1.y * q2.y + q1.z * q2.z + q1.w * q2.w;
        let mut q2_adj = q2;

        if cos_theta < 0.0 {
            q2_adj.x = -q2.x;
            q2_adj.y = -q2.y;
            q2_adj.z = -q2.z;
            q2_adj.w = -q2.w;
            cos_theta = -cos_theta;
        }

        let scale0: f32;
        let scale1: f32;

        if 1.0 - cos_theta > 0.001 {
            let theta = cos_theta.acos();
            let sin_theta = theta.sin();
            scale0 = ((1.0 - t) * theta).sin() / sin_theta;
            scale1 = (t * theta).sin() / sin_theta;
        } else {
            scale0 = 1.0 - t;
            scale1 = t;
        }

        Ok(D3DXQUATERNION::new(
            scale0 * q1.x + scale1 * q2_adj.x,
            scale0 * q1.y + scale1 * q2_adj.y,
            scale0 * q1.z + scale1 * q2_adj.z,
            scale0 * q1.w + scale1 * q2_adj.w,
        ))
    }

    pub fn interpolate_catmull_rom(
        p0: TPixelPosition,
        p1: TPixelPosition,
        p2: TPixelPosition,
        p3: TPixelPosition,
        t: f32,
    ) -> Result<TPixelPosition, InterpolationError> {
        if t < 0.0 || t > 1.0 {
            return Err(InterpolationError::InvalidTime);
        }

        let t2 = t * t;
        let t3 = t2 * t;

        let calc = |a: f32, b: f32, c: f32, d: f32| {
            0.5 * ((2.0 * b)
                + (-a + c) * t
                + (2.0 * a - 5.0 * b + 4.0 * c - d) * t2
                + (-a + 3.0 * b - 3.0 * c + d) * t3)
        };

        Ok(TPixelPosition::new(
            calc(p0.x, p1.x, p2.x, p3.x),
            calc(p0.y, p1.y, p2.y, p3.y),
            calc(p0.z, p1.z, p2.z, p3.z),
        ))
    }
    
    pub fn interpolate_hermite(
        p1: TPixelPosition,
        t1: TPixelPosition, // tangent 1
        p2: TPixelPosition,
        t2: TPixelPosition, // tangent 2
        t: f32,
    ) -> Result<TPixelPosition, InterpolationError> {
        if t < 0.0 || t > 1.0 {
            return Err(InterpolationError::InvalidTime);
        }
        
        let t_sq = t * t;
        let t_cu = t_sq * t;
        
        let h1 = 2.0 * t_cu - 3.0 * t_sq + 1.0;
        let h2 = -2.0 * t_cu + 3.0 * t_sq;
        let h3 = t_cu - 2.0 * t_sq + t;
        let h4 = t_cu - t_sq;
        
        Ok(TPixelPosition::new(
            h1 * p1.x + h2 * p2.x + h3 * t1.x + h4 * t2.x,
            h1 * p1.y + h2 * p2.y + h3 * t1.y + h4 * t2.y,
            h1 * p1.z + h2 * p2.z + h3 * t1.z + h4 * t2.z,
        ))
    }
    
    pub fn extrapolate_velocity(
        p_last: TPixelPosition,
        p_cur: TPixelPosition,
        time_diff_ms: u32,
        extrapolate_time_ms: u32,
    ) -> Result<TPixelPosition, InterpolationError> {
        if time_diff_ms == 0 {
            return Err(InterpolationError::InvalidTime);
        }
        
        let dt = time_diff_ms as f32 / 1000.0;
        let et = extrapolate_time_ms as f32 / 1000.0;
        
        let vx = (p_cur.x - p_last.x) / dt;
        let vy = (p_cur.y - p_last.y) / dt;
        let vz = (p_cur.z - p_last.z) / dt;
        
        Ok(TPixelPosition::new(
            p_cur.x + vx * et,
            p_cur.y + vy * et,
            p_cur.z + vz * et,
        ))
    }
}
