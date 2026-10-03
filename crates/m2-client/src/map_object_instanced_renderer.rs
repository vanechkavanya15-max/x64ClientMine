use std::collections::HashMap;
use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct MapObjectVertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub tex_coord: [f32; 2],
}

impl MapObjectVertex {
    pub const fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<MapObjectVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: (std::mem::size_of::<[f32; 3]>() * 2) as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct InstanceData {
    pub model_matrix_0: [f32; 4],
    pub model_matrix_1: [f32; 4],
    pub model_matrix_2: [f32; 4],
    pub model_matrix_3: [f32; 4],
}

impl InstanceData {
    pub const fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<InstanceData>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 6,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: (std::mem::size_of::<[f32; 4]>() * 2) as wgpu::BufferAddress,
                    shader_location: 7,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: (std::mem::size_of::<[f32; 4]>() * 3) as wgpu::BufferAddress,
                    shader_location: 8,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }
    }
}

pub struct MeshGroup {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub index_count: u32,
    pub instances: Vec<InstanceData>,
    pub instance_buffer: Option<wgpu::Buffer>,
}

pub struct MaterialGroup {
    pub material_id: u32,
    pub meshes: HashMap<u32, MeshGroup>, // mesh_id -> MeshGroup
}

pub struct MapObjectInstancedRenderer {
    pub material_groups: HashMap<u32, MaterialGroup>, // material_id -> MaterialGroup
}

impl MapObjectInstancedRenderer {
    pub fn new() -> Self {
        Self {
            material_groups: HashMap::new(),
        }
    }

    pub fn add_object_type(
        &mut self,
        device: &wgpu::Device,
        mesh_id: u32,
        material_id: u32,
        vertices: &[MapObjectVertex],
        indices: &[u32],
    ) {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("Map Object Vertex Buffer (mesh_id: {})", mesh_id)),
            contents: bytemuck::cast_slice(vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("Map Object Index Buffer (mesh_id: {})", mesh_id)),
            contents: bytemuck::cast_slice(indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        let mesh_group = MeshGroup {
            vertex_buffer,
            index_buffer,
            index_count: indices.len() as u32,
            instances: Vec::new(),
            instance_buffer: None,
        };

        let material_group = self
            .material_groups
            .entry(material_id)
            .or_insert_with(|| MaterialGroup {
                material_id,
                meshes: HashMap::new(),
            });

        material_group.meshes.insert(mesh_id, mesh_group);
    }

    pub fn add_instance(&mut self, mesh_id: u32, material_id: u32, instance_data: InstanceData) -> Result<(), &'static str> {
        if let Some(material_group) = self.material_groups.get_mut(&material_id) {
            if let Some(mesh_group) = material_group.meshes.get_mut(&mesh_id) {
                mesh_group.instances.push(instance_data);
                return Ok(());
            }
        }
        Err("Mesh or Material not found")
    }

    pub fn build_instance_buffers(&mut self, device: &wgpu::Device) {
        for material_group in self.material_groups.values_mut() {
            for (mesh_id, mesh_group) in material_group.meshes.iter_mut() {
                if !mesh_group.instances.is_empty() {
                    let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some(&format!("Instance Buffer (mesh_id: {})", mesh_id)),
                        contents: bytemuck::cast_slice(&mesh_group.instances),
                        usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                    });
                    mesh_group.instance_buffer = Some(instance_buffer);
                } else {
                    mesh_group.instance_buffer = None;
                }
            }
        }
    }

    pub fn clear_instances(&mut self) {
        for material_group in self.material_groups.values_mut() {
            for mesh_group in material_group.meshes.values_mut() {
                mesh_group.instances.clear();
                mesh_group.instance_buffer = None;
            }
        }
    }

    pub fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        for material_group in self.material_groups.values() {
            for mesh_group in material_group.meshes.values() {
                if let Some(instance_buffer) = &mesh_group.instance_buffer {
                    if mesh_group.instances.is_empty() {
                        continue;
                    }
                    render_pass.set_vertex_buffer(0, mesh_group.vertex_buffer.slice(..));
                    render_pass.set_vertex_buffer(1, instance_buffer.slice(..));
                    render_pass.set_index_buffer(
                        mesh_group.index_buffer.slice(..),
                        wgpu::IndexFormat::Uint32,
                    );
                    render_pass.draw_indexed(
                        0..mesh_group.index_count,
                        0,
                        0..mesh_group.instances.len() as u32,
                    );
                }
            }
        }
    }

    pub fn draw_by_material<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>, material_id: u32) {
        if let Some(material_group) = self.material_groups.get(&material_id) {
            for mesh_group in material_group.meshes.values() {
                if let Some(instance_buffer) = &mesh_group.instance_buffer {
                    if mesh_group.instances.is_empty() {
                        continue;
                    }
                    render_pass.set_vertex_buffer(0, mesh_group.vertex_buffer.slice(..));
                    render_pass.set_vertex_buffer(1, instance_buffer.slice(..));
                    render_pass.set_index_buffer(
                        mesh_group.index_buffer.slice(..),
                        wgpu::IndexFormat::Uint32,
                    );
                    render_pass.draw_indexed(
                        0..mesh_group.index_count,
                        0,
                        0..mesh_group.instances.len() as u32,
                    );
                }
            }
        }
    }
}
