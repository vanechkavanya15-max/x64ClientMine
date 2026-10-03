use crate::character_manager_core::get_character_manager;
use crate::wgpu_skeletal_mesh_renderer::{SkeletalUniforms, WgpuSkeletalMeshRenderer};
use glam::{Mat4, Quat, Vec3};

/// Zarzadza faza renderowania wszystkich instancji postaci (aktorow) na scenie.
pub struct WgpuActorRenderPass;

impl WgpuActorRenderPass {
    /// Funkcja odpowiedzialna za rysowanie wszystkich widocznych aktorow
    /// za pomoca WgpuSkeletalMeshRenderer.
    pub fn render_actors<'a>(
        render_pass: &mut wgpu::RenderPass<'a>,
        queue: &wgpu::Queue,
        renderer: &'a WgpuSkeletalMeshRenderer,
        uniform_buffer: &'a wgpu::Buffer,
        uniform_bind_group: &'a wgpu::BindGroup,
        vertex_buffer: &'a wgpu::Buffer,
        index_buffer: &'a wgpu::Buffer,
        texture_bind_group: &'a wgpu::BindGroup,
        num_indices: u32,
        view_proj: [[f32; 4]; 4],
    ) {
        let char_mgr = get_character_manager().read().unwrap();

        for instance in char_mgr.alive_instances.values() {
            // Obliczamy macierz swiata dla danego aktora (model matrix)
            let rot_quat = Quat::from_rotation_z(instance.rot_deg.to_radians());
            let model = Mat4::from_scale_rotation_translation(
                Vec3::ONE,
                rot_quat,
                Vec3::new(instance.x, instance.y, instance.z),
            );

            // W prawdziwym scenariuszu uzywamy GrannyBoneAnimEvaluator do ewaluacji klatki
            // Z braku podpietej instancji animacji uzywamy glownej macierzy modelu dla wszystkich
            // kosci, symulujac rig jako jedno sztywne cialo o pozycji aktora.
            let mut bone_matrices = [[[0.0; 4]; 4]; 128];
            let model_array = model.to_cols_array_2d();
            for bone_matrix in bone_matrices.iter_mut() {
                *bone_matrix = model_array;
            }

            let uniforms = SkeletalUniforms {
                view_proj,
                light_dir: [0.0, 0.0, -1.0, 0.5], // Typowe oswietlenie srodowiskowe
                light_color: [1.0, 1.0, 1.0, 1.0],
                bone_matrices,
            };

            // Przeslanie danych do GPU
            queue.write_buffer(uniform_buffer, 0, bytemuck::bytes_of(&uniforms));

            // Zlecenie renderowania za posrednictwem loadera
            renderer.draw(
                render_pass,
                vertex_buffer,
                index_buffer,
                uniform_bind_group,
                texture_bind_group,
                num_indices,
            );
        }
    }
}

/// Zgodnosc z FFI/C-ABI dla systemow operacyjnych bezposrednio komunikujacych sie z klientem.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FfiActorRenderInfo {
    pub total_drawn: u32,
    pub cull_count: u32,
}

#[no_mangle]
pub extern "C" fn m2_actor_pass_get_info() -> FfiActorRenderInfo {
    FfiActorRenderInfo {
        total_drawn: 0, // Placeholder do symulacji FFI w kontekscie headless
        cull_count: 0,
    }
}
