use std::os::raw::{c_char, c_void};

pub type GrannyInt32 = i32;
pub type GrannyUInt32 = u32;
pub type GrannyReal32 = f32;
pub type GrannyBool32 = i32;

pub type GrannyTriple = [GrannyReal32; 3];
pub type GrannyQuad = [GrannyReal32; 4];
pub type GrannyMatrix4x4 = [[GrannyReal32; 4]; 4];

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum GrannyCompressionType {
    GrannyNoCompression = 0,
    GrannyOodle0Compression = 1,
    GrannyOodle1Compression = 2,
    GrannyBitKnitCompression = 3,
    GrannyBitKnit2Compression = 4,
    GrannyOnePastLastCompressionType = 5,
    GrannyCompressionTypeForceInt = 0x7fffffff,
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct GrannyMemberType(pub GrannyInt32);

impl GrannyMemberType {
    pub const GRANNY_END_MEMBER: Self = Self(0);
    pub const GRANNY_INLINE_MEMBER: Self = Self(1);
    pub const GRANNY_REFERENCE_MEMBER: Self = Self(2);
    pub const GRANNY_REFERENCE_TO_ARRAY_MEMBER: Self = Self(3);
    pub const GRANNY_ARRAY_OF_REFERENCES_MEMBER: Self = Self(4);
    pub const GRANNY_VARIANT_REFERENCE_MEMBER: Self = Self(5);
    pub const GRANNY_UNSUPPORTED_MEMBER_TYPE_REMOVE: Self = Self(6);
    pub const GRANNY_REFERENCE_TO_VARIANT_ARRAY_MEMBER: Self = Self(7);
    pub const GRANNY_STRING_MEMBER: Self = Self(8);
    pub const GRANNY_TRANSFORM_MEMBER: Self = Self(9);
    pub const GRANNY_REAL32_MEMBER: Self = Self(10);
    pub const GRANNY_INT8_MEMBER: Self = Self(11);
    pub const GRANNY_UINT8_MEMBER: Self = Self(12);
    pub const GRANNY_BINORMAL_INT8_MEMBER: Self = Self(13);
}

#[repr(C)]
pub struct GrannyDataTypeDefinition {
    pub member_type: GrannyMemberType,
    pub name: *const c_char,
    pub reference_type: *mut GrannyDataTypeDefinition,
    pub array_width: GrannyInt32,
    pub extra: [GrannyInt32; 3],
    pub ignored_ignored: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GrannyVariant {
    pub variant_type: *mut GrannyDataTypeDefinition,
    pub object: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GrannyCurve2 {
    pub curve_data: GrannyVariant,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GrannyTransform {
    pub flags: GrannyUInt32,
    pub position: GrannyTriple,
    pub orientation: GrannyQuad,
    pub scale_shear: [GrannyTriple; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GrannyBone {
    pub name: *const c_char,
    pub parent_index: GrannyInt32,
    pub local_transform: GrannyTransform,
    pub inverse_world_4x4: GrannyMatrix4x4,
    pub lod_error: GrannyReal32,
    pub extended_data: GrannyVariant,
}

#[repr(C)]
pub struct GrannySkeleton {
    pub name: *const c_char,
    pub bone_count: GrannyInt32,
    pub bones: *mut GrannyBone,
    pub lod_type: GrannyInt32,
    pub extended_data: GrannyVariant,
}

#[repr(C)]
pub struct GrannyVertexAnnotationSet {
    pub name: *const c_char,
    pub annotation_type: *mut GrannyDataTypeDefinition,
    pub annotation_count: GrannyInt32,
    pub annotations: *mut u8,
    pub indices_map_from_vertex_to_annotation: *mut GrannyInt32,
}

#[repr(C)]
pub struct GrannyVertexData {
    pub vertex_type: *mut GrannyDataTypeDefinition,
    pub vertex_count: GrannyInt32,
    pub vertices: *mut u8,
    pub vertex_component_name_count: GrannyInt32,
    pub vertex_component_names: *mut *const c_char,
    pub vertex_annotation_set_count: GrannyInt32,
    pub vertex_annotation_sets: *mut GrannyVertexAnnotationSet,
}

#[repr(C)]
pub struct GrannyTriMaterialGroup {
    pub material_index: GrannyInt32,
    pub tri_first: GrannyInt32,
    pub tri_count: GrannyInt32,
}

#[repr(C)]
pub struct GrannyTriTopology {
    pub group_count: GrannyInt32,
    pub groups: *mut GrannyTriMaterialGroup,
    pub index_count: GrannyInt32,
    pub indices: *mut GrannyInt32,
    pub index16_count: GrannyInt32,
    pub indices16: *mut u16,
    pub vertex_to_vertex_count: GrannyInt32,
    pub vertex_to_vertex_map: *mut GrannyInt32,
    pub vertex_to_triangle_count: GrannyInt32,
    pub vertex_to_triangle_map: *mut GrannyInt32,
    pub side_to_neighbor_count: GrannyInt32,
    pub side_to_neighbor_map: *mut GrannyUInt32,
    pub polygon_index_start_count: GrannyInt32,
    pub polygon_index_starts: *mut GrannyInt32,
}

#[repr(C)]
pub struct GrannyMaterialBinding {
    pub material: *mut c_void, // opaque material
}

#[repr(C)]
pub struct GrannyBoneBinding {
    pub bone_name: *const c_char,
    pub obb_min: GrannyTriple,
    pub obb_max: GrannyTriple,
    pub triangle_count: GrannyInt32,
    pub triangle_indices: *mut GrannyInt32,
}

#[repr(C)]
pub struct GrannyMorphTarget {
    pub scalar_name: *const c_char,
    pub vertex_data: *mut GrannyVertexData,
    pub data_has_normals: GrannyInt32,
}

#[repr(C)]
pub struct GrannyMesh {
    pub name: *const c_char,
    pub primary_vertex_data: *mut GrannyVertexData,
    pub morph_target_count: GrannyInt32,
    pub morph_targets: *mut GrannyMorphTarget,
    pub primary_topology: *mut GrannyTriTopology,
    pub material_binding_count: GrannyInt32,
    pub material_bindings: *mut GrannyMaterialBinding,
    pub bone_binding_count: GrannyInt32,
    pub bone_bindings: *mut GrannyBoneBinding,
    pub extended_data: GrannyVariant,
}

#[repr(C)]
pub struct GrannyModel {
    pub name: *const c_char,
    pub skeleton: *mut GrannySkeleton,
    pub initial_placement: GrannyTransform,
    pub mesh_binding_count: GrannyInt32,
    pub mesh_bindings: *mut GrannyModelMeshBinding,
    pub extended_data: GrannyVariant,
}

#[repr(C)]
pub struct GrannyModelMeshBinding {
    pub mesh: *mut GrannyMesh,
}

#[repr(C)]
pub struct GrannyTrackGroup {
    pub name: *const c_char,
    pub vector_track_count: GrannyInt32,
    pub vector_tracks: *mut c_void, // opaque for now since not fully used in the scope
    pub transform_track_count: GrannyInt32,
    pub transform_tracks: *mut c_void,
    pub transform_lod_error: GrannyReal32,
    pub text_track_count: GrannyInt32,
    pub text_tracks: *mut c_void,
    pub initial_placement: GrannyTransform,
    pub flags: GrannyInt32,
    pub loop_translation: GrannyTriple,
    pub extended_data: GrannyVariant,
}

#[repr(C)]
pub struct GrannyAnimation {
    pub name: *const c_char,
    pub duration: GrannyReal32,
    pub time_step: GrannyReal32,
    pub oversampling: GrannyReal32,
    pub track_group_count: GrannyInt32,
    pub track_groups: *mut *mut GrannyTrackGroup,
    pub extended_data: GrannyVariant,
}

#[repr(C)]
pub struct GrannyTexture {
    pub from_file_name: *const c_char,
    pub texture_type: GrannyInt32,
    pub width: GrannyInt32,
    pub height: GrannyInt32,
    pub encoding: GrannyInt32,
    pub sub_format: GrannyInt32,
    pub layout: *mut c_void,
    pub image_count: GrannyInt32,
    pub images: *mut c_void,
    pub extended_data: GrannyVariant,
}

#[repr(C)]
pub struct GrannyMaterial {
    pub name: *const c_char,
    pub map_count: GrannyInt32,
    pub maps: *mut c_void,
    pub texture: *mut GrannyTexture,
    pub extended_data: GrannyVariant,
}

#[repr(C)]
pub struct GrannyFileInfo {
    pub art_tool_info: *mut c_void,
    pub exporter_info: *mut c_void,
    pub from_file_name: *const c_char,
    pub texture_count: GrannyInt32,
    pub textures: *mut *mut GrannyTexture,
    pub material_count: GrannyInt32,
    pub materials: *mut *mut GrannyMaterial,
    pub skeleton_count: GrannyInt32,
    pub skeletons: *mut *mut GrannySkeleton,
    pub vertex_data_count: GrannyInt32,
    pub vertex_datas: *mut *mut GrannyVertexData,
    pub tri_topology_count: GrannyInt32,
    pub tri_topologies: *mut *mut GrannyTriTopology,
    pub mesh_count: GrannyInt32,
    pub meshes: *mut *mut GrannyMesh,
    pub model_count: GrannyInt32,
    pub models: *mut *mut GrannyModel,
    pub track_group_count: GrannyInt32,
    pub track_groups: *mut *mut GrannyTrackGroup,
    pub animation_count: GrannyInt32,
    pub animations: *mut *mut GrannyAnimation,
    pub extended_data: GrannyVariant,
}

#[repr(C)]
pub struct GrannyFile {
    pub is_byte_reversed: GrannyBool32,
    pub header: *mut c_void,
    pub source_magic_value: *mut c_void,
    pub section_count: GrannyInt32,
    pub sections: *mut *mut c_void,
    pub marshalled: *mut bool,
    pub is_user_memory: *mut bool,
    pub conversion_buffer: *mut c_void,
    pub conversion_buffer_size: usize,
}

#[cfg(windows)]
extern "stdcall" {
    pub fn GrannyDecompressData(
        format: GrannyCompressionType,
        file_is_byte_reversed: bool,
        compressed_bytes_size: GrannyInt32,
        compressed_bytes: *mut c_void,
        stop0: GrannyInt32,
        stop1: GrannyInt32,
        stop2: GrannyInt32,
        decompressed_bytes: *mut c_void,
    ) -> bool;

    pub fn GrannyGetFileInfo(file: *mut GrannyFile) -> *mut GrannyFileInfo;

    pub fn GrannyFreeFile(file: *mut GrannyFile);
}

#[allow(non_snake_case)]
#[cfg(not(windows))]
pub unsafe fn GrannyDecompressData(
    _format: GrannyCompressionType,
    _file_is_byte_reversed: bool,
    _compressed_bytes_size: GrannyInt32,
    _compressed_bytes: *mut c_void,
    _stop0: GrannyInt32,
    _stop1: GrannyInt32,
    _stop2: GrannyInt32,
    _decompressed_bytes: *mut c_void,
) -> bool {
    // Safe stub for non-Windows platforms (e.g., CI testing)
    false
}

#[allow(non_snake_case)]
#[cfg(not(windows))]
pub unsafe fn GrannyGetFileInfo(_file: *mut GrannyFile) -> *mut GrannyFileInfo {
    // Safe stub for non-Windows platforms (e.g., CI testing)
    std::ptr::null_mut()
}

#[allow(non_snake_case)]
#[cfg(not(windows))]
pub unsafe fn GrannyFreeFile(_file: *mut GrannyFile) {
    // Safe stub for non-Windows platforms (e.g., CI testing)
}
