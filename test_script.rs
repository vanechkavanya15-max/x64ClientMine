fn main() {
    let desc = wgpu::RenderPipelineDescriptor {
        label: None,
        layout: None,
        vertex: wgpu::VertexState {
            module: unsafe { std::mem::zeroed() },
            entry_point: None,
            buffers: &[],
            compilation_options: Default::default(),
        },
        primitive: Default::default(),
        depth_stencil: None,
        multisample: Default::default(),
        fragment: None,
        multiview_mask: 0,
        cache: None,
    };
    println!("OK");
}
