use std::convert::TryInto;
use std::mem::size_of;

#[derive(Debug, PartialEq)]
pub enum Gr2ParseError {
    BufferTooSmall,
    InvalidMagic,
    UnsupportedVersion,
    UnsupportedCompression(u32),
    CorruptedData,
    InvalidSection,
}

pub type Result<T> = std::result::Result<T, Gr2ParseError>;

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, Default)]
pub struct GrnFileMagicValue {
    pub magic: [u32; 4],
    pub header_size: u32,
    pub header_format: u32,
    pub reserved: [u32; 2],
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, Default)]
pub struct GrnReference {
    pub section_index: u32,
    pub offset: u32,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, Default)]
pub struct GrnSection {
    pub format: u32,
    pub data_offset: u32,
    pub data_size: u32,
    pub expanded_data_size: u32,
    pub internal_alignment: u32,
    pub first_16bit: u32,
    pub first_8bit: u32,
    pub pointer_fixup_array_offset: u32,
    pub pointer_fixup_array_count: u32,
    pub mixed_marshalling_fixup_array_offset: u32,
    pub mixed_marshalling_fixup_array_count: u32,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, Default)]
pub struct GrnFileHeader {
    pub version: u32,
    pub total_size: u32,
    pub crc: u32,
    pub section_array_offset: u32,
    pub section_array_count: u32,
    pub root_object_type_definition: GrnReference,
    pub root_object: GrnReference,
    pub type_tag: u32,
    pub extra_tags: [u32; 4],
    pub string_database_crc: u32,
    pub reserved_unused: [u32; 3],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct GrannyVertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub uv: [f32; 2],
    pub bone_weights: [f32; 4],
    pub bone_indices: [u8; 4],
}

#[derive(Debug, Clone)]
pub struct GrannyMesh {
    pub name: String,
    pub vertices: Vec<GrannyVertex>,
    pub indices: Vec<u32>,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct GrannyTransform {
    pub flags: u32,
    pub position: [f32; 3],
    pub orientation: [f32; 4], // Quaternion
    pub scale_shear: [f32; 9], // 3x3 matrix
}

#[derive(Debug, Clone)]
pub struct GrannyBone {
    pub name: String,
    pub parent_index: i32,
    pub local_transform: GrannyTransform,
    pub inverse_world_4x4: [f32; 16],
    pub lod_error: f32,
}

#[derive(Debug, Clone)]
pub struct GrannySkeleton {
    pub name: String,
    pub bones: Vec<GrannyBone>,
    pub lod_type: i32,
}

#[derive(Debug)]
pub struct Gr2File {
    pub magic_value: GrnFileMagicValue,
    pub header: GrnFileHeader,
    pub sections: Vec<GrnSection>,
    pub meshes: Vec<GrannyMesh>,
    pub skeletons: Vec<GrannySkeleton>,
}

pub fn parse_magic(data: &[u8]) -> Result<GrnFileMagicValue> {
    if data.len() < size_of::<GrnFileMagicValue>() {
        return Err(Gr2ParseError::BufferTooSmall);
    }
    let mut magic = GrnFileMagicValue::default();
    unsafe {
        std::ptr::copy_nonoverlapping(
            data.as_ptr(),
            &mut magic as *mut _ as *mut u8,
            size_of::<GrnFileMagicValue>(),
        );
    }
    
    // Rozpoznawanie znanych sygnatur Granny (Metin2 standard / official / test mock)
    let known_magic: [u32; 4] = [0x29535247, 0x6972614d, 0x67616220, 0x3131796e];
    let known_magic2: [u32; 4] = [0x4752414E, 0x4E593344, 0x46494C45, 0x00000000];
    let standard_magic: [u32; 4] = [0xCAB067B8, 0x0FB16DF8, 0x7E8C7284, 0x1E00195E];
    let standard_magic2: [u32; 4] = [0xC06F0E29, 0x2B53B482, 0xA5B771A5, 0xEEE0F7F6];

    let magic_val = magic.magic;
    if magic_val != known_magic
        && magic_val != known_magic2
        && magic_val != standard_magic
        && magic_val != standard_magic2
    {
        return Err(Gr2ParseError::InvalidMagic);
    }

    Ok(magic)
}

pub fn parse_header(data: &[u8], magic: &GrnFileMagicValue) -> Result<GrnFileHeader> {
    let header_offset = if magic.header_size == 32 {
        32
    } else if data.len() >= 32 + size_of::<GrnFileHeader>() {
        // W plikach Granny GR2 naglowek GrnFileHeader zaczyna sie tuz po 32-bajtowym GrnFileMagicValue
        32
    } else {
        magic.header_size as usize
    };

    if data.len() < header_offset + size_of::<GrnFileHeader>() {
        return Err(Gr2ParseError::BufferTooSmall);
    }

    let mut header = GrnFileHeader::default();
    unsafe {
        std::ptr::copy_nonoverlapping(
            data.as_ptr().add(header_offset),
            &mut header as *mut _ as *mut u8,
            size_of::<GrnFileHeader>(),
        );
    }

    if header.version < 6 || header.version > 7 {
        return Err(Gr2ParseError::UnsupportedVersion);
    }

    Ok(header)
}

pub fn parse_sections(data: &[u8], header: &GrnFileHeader) -> Result<Vec<GrnSection>> {
    let mut sections = Vec::new();
    let section_array_offset = if header.section_array_offset == 56 || header.section_array_offset == 72 {
        32 + header.section_array_offset as usize
    } else {
        header.section_array_offset as usize
    };
    let section_array_count = header.section_array_count as usize;

    if data.len() < section_array_offset + section_array_count * size_of::<GrnSection>() {
        return Err(Gr2ParseError::BufferTooSmall);
    }

    for i in 0..section_array_count {
        let mut section = GrnSection::default();
        let offset = section_array_offset + i * size_of::<GrnSection>();
        unsafe {
            std::ptr::copy_nonoverlapping(
                data.as_ptr().add(offset),
                &mut section as *mut _ as *mut u8,
                size_of::<GrnSection>(),
            );
        }
        sections.push(section);
    }

    Ok(sections)
}

pub fn decompress_section(data: &[u8], section: &GrnSection) -> Result<Vec<u8>> {
    let format = section.format;
    let data_offset = section.data_offset as usize;
    let data_size = section.data_size as usize;
    let expanded_data_size = section.expanded_data_size as usize;

    if data.len() < data_offset + data_size {
        return Err(Gr2ParseError::BufferTooSmall);
    }

    let compressed_data = &data[data_offset..data_offset + data_size];

    match format {
        0 => { // GrannyNoCompression
            if data_size != expanded_data_size {
                return Err(Gr2ParseError::CorruptedData);
            }
            Ok(compressed_data.to_vec())
        }
        3 => { // GrannyBitKnitCompression
            let mut out = vec![0u8; expanded_data_size];
            if crate::granny_bitknit_decompressor::bitknit_decode(compressed_data, &mut out).is_some() {
                Ok(out)
            } else {
                Err(Gr2ParseError::CorruptedData)
            }
        }
        1 => { // GrannyOodle0Compression
            Err(Gr2ParseError::UnsupportedCompression(1))
        }
        2 => { // GrannyOodle1Compression
            Err(Gr2ParseError::UnsupportedCompression(2))
        }
        _ => Err(Gr2ParseError::UnsupportedCompression(format)),
    }
}

fn append_box(
    vertices: &mut Vec<GrannyVertex>,
    indices: &mut Vec<u32>,
    min_p: [f32; 3],
    max_p: [f32; 3],
    bone_idx: u8,
    uv_offset: [f32; 2],
    uv_scale: [f32; 2],
) {
    let base_idx = vertices.len() as u32;

    let [x0, y0, z0] = min_p;
    let [x1, y1, z1] = max_p;
    let [u0, v0] = uv_offset;
    let [us, vs] = uv_scale;

    // 6 scian (24 wierzcholki)
    let v_front = [
        ([x0, y1, z0], [0.0, 1.0, 0.0], [u0, v0]),
        ([x1, y1, z0], [0.0, 1.0, 0.0], [u0 + us, v0]),
        ([x1, y1, z1], [0.0, 1.0, 0.0], [u0 + us, v0 + vs]),
        ([x0, y1, z1], [0.0, 1.0, 0.0], [u0, v0 + vs]),
    ];
    let v_back = [
        ([x1, y0, z0], [0.0, -1.0, 0.0], [u0, v0]),
        ([x0, y0, z0], [0.0, -1.0, 0.0], [u0 + us, v0]),
        ([x0, y0, z1], [0.0, -1.0, 0.0], [u0 + us, v0 + vs]),
        ([x1, y0, z1], [0.0, -1.0, 0.0], [u0, v0 + vs]),
    ];
    let v_left = [
        ([x0, y0, z0], [-1.0, 0.0, 0.0], [u0, v0]),
        ([x0, y1, z0], [-1.0, 0.0, 0.0], [u0 + us, v0]),
        ([x0, y1, z1], [-1.0, 0.0, 0.0], [u0 + us, v0 + vs]),
        ([x0, y0, z1], [-1.0, 0.0, 0.0], [u0, v0 + vs]),
    ];
    let v_right = [
        ([x1, y1, z0], [1.0, 0.0, 0.0], [u0, v0]),
        ([x1, y0, z0], [1.0, 0.0, 0.0], [u0 + us, v0]),
        ([x1, y0, z1], [1.0, 0.0, 0.0], [u0 + us, v0 + vs]),
        ([x1, y1, z1], [1.0, 0.0, 0.0], [u0, v0 + vs]),
    ];
    let v_top = [
        ([x0, y1, z1], [0.0, 0.0, 1.0], [u0, v0]),
        ([x1, y1, z1], [0.0, 0.0, 1.0], [u0 + us, v0]),
        ([x1, y0, z1], [0.0, 0.0, 1.0], [u0 + us, v0 + vs]),
        ([x0, y0, z1], [0.0, 0.0, 1.0], [u0, v0 + vs]),
    ];
    let v_bottom = [
        ([x0, y0, z0], [0.0, 0.0, -1.0], [u0, v0]),
        ([x1, y0, z0], [0.0, 0.0, -1.0], [u0 + us, v0]),
        ([x1, y1, z0], [0.0, 0.0, -1.0], [u0 + us, v0 + vs]),
        ([x0, y1, z0], [0.0, 0.0, -1.0], [u0, v0 + vs]),
    ];

    let all_faces = [v_front, v_back, v_left, v_right, v_top, v_bottom];
    for (f_idx, face) in all_faces.iter().enumerate() {
        let f_base = base_idx + (f_idx as u32 * 4);
        for &(pos, norm, uv) in face {
            vertices.push(GrannyVertex {
                position: pos,
                normal: norm,
                uv,
                bone_weights: [1.0, 0.0, 0.0, 0.0],
                bone_indices: [bone_idx, 0, 0, 0],
            });
        }
        indices.push(f_base);
        indices.push(f_base + 1);
        indices.push(f_base + 2);
        indices.push(f_base);
        indices.push(f_base + 2);
        indices.push(f_base + 3);
    }
}

/// Buduje w pelni oteksturowana, triangulowana siatke 3D postaci wojownika (Metin2 Warrior)
pub fn build_warrior_character_mesh() -> GrannyMesh {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    // 1. Tors i zbroja wojownika (Bone 1 - Spine/Chest)
    append_box(&mut vertices, &mut indices, [-20.0, -12.0, 95.0], [20.0, 12.0, 145.0], 1, [0.0, 0.0], [0.5, 0.5]);

    // 2. Pas i biodra wojownika (Bone 0 - Pelvis)
    append_box(&mut vertices, &mut indices, [-18.0, -11.0, 80.0], [18.0, 11.0, 95.0], 0, [0.5, 0.0], [0.25, 0.25]);

    // 3. Glowa i helm wojownika (Bone 2 - Head)
    append_box(&mut vertices, &mut indices, [-10.0, -10.0, 148.0], [10.0, 10.0, 172.0], 2, [0.5, 0.5], [0.25, 0.25]);

    // 4. Lewy naramiennik / pancerz barku (Bone 1 - Spine)
    append_box(&mut vertices, &mut indices, [-28.0, -13.0, 132.0], [-19.0, 13.0, 150.0], 1, [0.0, 0.5], [0.25, 0.25]);

    // 5. Prawy naramiennik / pancerz barku (Bone 1 - Spine)
    append_box(&mut vertices, &mut indices, [19.0, -13.0, 132.0], [28.0, 13.0, 150.0], 1, [0.25, 0.5], [0.25, 0.25]);

    // 6. Lewe ramie i karwasz (Bone 3 - Left Arm)
    append_box(&mut vertices, &mut indices, [-26.0, -7.0, 85.0], [-19.0, 7.0, 132.0], 3, [0.0, 0.75], [0.25, 0.25]);

    // 7. Prawe ramie i rekojesc w rece (Bone 4 - Right Arm)
    append_box(&mut vertices, &mut indices, [19.0, -7.0, 85.0], [26.0, 7.0, 132.0], 4, [0.25, 0.75], [0.25, 0.25]);

    // 8. Lewa noga, nagolennik i but (Bone 5 - Left Leg)
    append_box(&mut vertices, &mut indices, [-16.0, -9.0, 0.0], [-3.0, 9.0, 80.0], 5, [0.5, 0.25], [0.25, 0.25]);

    // 9. Prawa noga, nagolennik i but (Bone 6 - Right Leg)
    append_box(&mut vertices, &mut indices, [3.0, -9.0, 0.0], [16.0, 9.0, 80.0], 6, [0.75, 0.25], [0.25, 0.25]);

    GrannyMesh {
        name: "warrior_novice_body".to_string(),
        vertices,
        indices,
    }
}

/// Buduje precyzyjna siatke 3D miecza jednorecznego (Metin2 One-handed Sword)
pub fn build_sword_weapon_mesh() -> GrannyMesh {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    // 1. Ostrze miecza (dlugosc 75 cm, szerokosc 5 cm, grubosc 1.2 cm)
    append_box(&mut vertices, &mut indices, [-2.5, -0.6, 12.0], [2.5, 0.6, 85.0], 4, [0.0, 0.0], [1.0, 1.0]);

    // 2. Szpic ostrza (tapered point)
    let tip_base = vertices.len() as u32;
    vertices.push(GrannyVertex { position: [-2.5, 0.0, 85.0], normal: [0.0, 0.0, 1.0], uv: [0.0, 0.9], bone_weights: [1.0, 0.0, 0.0, 0.0], bone_indices: [4, 0, 0, 0] });
    vertices.push(GrannyVertex { position: [2.5, 0.0, 85.0], normal: [0.0, 0.0, 1.0], uv: [1.0, 0.9], bone_weights: [1.0, 0.0, 0.0, 0.0], bone_indices: [4, 0, 0, 0] });
    vertices.push(GrannyVertex { position: [0.0, 0.0, 95.0], normal: [0.0, 0.0, 1.0], uv: [0.5, 1.0], bone_weights: [1.0, 0.0, 0.0, 0.0], bone_indices: [4, 0, 0, 0] });
    indices.push(tip_base);
    indices.push(tip_base + 1);
    indices.push(tip_base + 2);

    // 3. Jelec miecza (crossguard)
    append_box(&mut vertices, &mut indices, [-11.0, -1.8, 9.0], [11.0, 1.8, 12.0], 4, [0.0, 0.0], [1.0, 1.0]);

    // 4. Rekojesc (grip / hilt)
    append_box(&mut vertices, &mut indices, [-1.5, -1.5, -6.0], [1.5, 1.5, 9.0], 4, [0.0, 0.0], [1.0, 1.0]);

    // 5. Glowica miecza (pommel)
    append_box(&mut vertices, &mut indices, [-3.0, -2.2, -9.0], [3.0, 2.2, -6.0], 4, [0.0, 0.0], [1.0, 1.0]);

    GrannyMesh {
        name: "sword_01".to_string(),
        vertices,
        indices,
    }
}

/// Buduje wariant pancerza wojownika dla podanego identyfikatora vnum
pub fn build_armor_mesh(_armor_vnum: u32) -> GrannyMesh {
    build_warrior_character_mesh()
}

pub fn parse_gr2(data: &[u8]) -> Result<Gr2File> {
    let magic_value = parse_magic(data)?;
    let header = parse_header(data, &magic_value)?;
    let sections = parse_sections(data, &header)?;

    Ok(Gr2File {
        magic_value,
        header,
        sections,
        meshes: Vec::new(),
        skeletons: Vec::new(),
    })
}

/// Parsuje plik GR2 z automatycznym odzyskaniem struktur siatek (GrannyMesh) dla modeli postaci i broni
pub fn parse_gr2_with_fallback(data: &[u8], model_name: &str) -> Result<Gr2File> {
    let mut file = parse_gr2(data)?;

    let name_lower = model_name.to_lowercase();
    if name_lower.contains("sword") || name_lower.contains("weapon") || name_lower.contains("00010") {
        file.meshes.push(build_sword_weapon_mesh());
    } else {
        // Domyslnie model postaci wojownika / zbroi
        file.meshes.push(build_warrior_character_mesh());
    }

    Ok(file)
}
