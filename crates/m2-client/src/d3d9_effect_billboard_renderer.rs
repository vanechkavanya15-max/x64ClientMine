#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct BillboardQuad {
    pub center: [f32; 3],
    pub size_x: f32,
    pub size_y: f32,
    pub rotation_rad: f32,
    pub color: u32,
    pub uv_rect: [f32; 4], // [u_min, v_min, u_max, v_max]
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
struct VertexPCU {
    pos: [f32; 3],
    color: u32,
    uv: [f32; 2],
}

pub fn align_billboard_to_camera(
    quad: &BillboardQuad,
    cam_right: [f32; 3],
    cam_up: [f32; 3],
    is_cylindrical: bool,
) -> [[f32; 3]; 4] {
    let mut right = cam_right;
    let mut up = cam_up;

    if is_cylindrical {
        up = [0.0, 0.0, 1.0];
        let r_len = (cam_right[0] * cam_right[0] + cam_right[1] * cam_right[1]).sqrt();
        if r_len > 1e-6 {
            right = [cam_right[0] / r_len, cam_right[1] / r_len, 0.0];
        } else {
            right = [1.0, 0.0, 0.0];
        }
    }

    let half_w = quad.size_x * 0.5;
    let half_h = quad.size_y * 0.5;

    let cos_r = quad.rotation_rad.cos();
    let sin_r = quad.rotation_rad.sin();

    // 0: Top-Left, 1: Top-Right, 2: Bottom-Left, 3: Bottom-Right
    let local_corners = [
        [-half_w, half_h],
        [half_w, half_h],
        [-half_w, -half_h],
        [half_w, -half_h],
    ];

    let mut world_corners = [[0.0; 3]; 4];

    for i in 0..4 {
        let x = local_corners[i][0];
        let y = local_corners[i][1];

        let rx = x * cos_r - y * sin_r;
        let ry = x * sin_r + y * cos_r;

        world_corners[i] = [
            quad.center[0] + right[0] * rx + up[0] * ry,
            quad.center[1] + right[1] * rx + up[1] * ry,
            quad.center[2] + right[2] * rx + up[2] * ry,
        ];
    }

    world_corners
}

pub fn transform_particles_to_screen_batch(
    particles: &[BillboardQuad],
    cam_right: [f32; 3],
    cam_up: [f32; 3],
    is_cylindrical: bool,
) -> Vec<u8> {
    let mut buffer = Vec::with_capacity(particles.len() * 6 * std::mem::size_of::<VertexPCU>());

    for quad in particles {
        let corners = align_billboard_to_camera(quad, cam_right, cam_up, is_cylindrical);

        let u_min = quad.uv_rect[0];
        let v_min = quad.uv_rect[1];
        let u_max = quad.uv_rect[2];
        let v_max = quad.uv_rect[3];

        let v0 = VertexPCU { pos: corners[0], color: quad.color, uv: [u_min, v_min] }; // Top-Left
        let v1 = VertexPCU { pos: corners[1], color: quad.color, uv: [u_max, v_min] }; // Top-Right
        let v2 = VertexPCU { pos: corners[2], color: quad.color, uv: [u_min, v_max] }; // Bottom-Left
        let v3 = VertexPCU { pos: corners[3], color: quad.color, uv: [u_max, v_max] }; // Bottom-Right

        let vertices = [
            v0, v1, v3, // Triangle 1: Top-Left, Top-Right, Bottom-Right
            v0, v3, v2, // Triangle 2: Top-Left, Bottom-Right, Bottom-Left
        ];

        let bytes: &[u8] = unsafe {
            std::slice::from_raw_parts(
                vertices.as_ptr() as *const u8,
                std::mem::size_of_val(&vertices),
            )
        };
        buffer.extend_from_slice(bytes);
    }

    buffer
}
