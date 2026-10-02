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
    
    // Check known magic (32-bit Little Endian for Metin2)
    let known_magic: [u32; 4] = [0x29535247, 0x6972614d, 0x67616220, 0x3131796e];
    let magic_val = magic.magic;
    if magic_val != known_magic {
        // As a fallback check magic 2
        let known_magic2: [u32; 4] = [0x4752414E, 0x4E593344, 0x46494C45, 0x00000000];
        if magic_val != known_magic2 {
             return Err(Gr2ParseError::InvalidMagic);
        }
    }

    Ok(magic)
}

pub fn parse_header(data: &[u8], magic: &GrnFileMagicValue) -> Result<GrnFileHeader> {
    let header_offset = magic.header_size as usize;
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
    let section_array_offset = header.section_array_offset as usize;
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
        1 => { // GrannyOodle0Compression
            Err(Gr2ParseError::UnsupportedCompression(1))
        }
        2 => { // GrannyOodle1Compression
            Err(Gr2ParseError::UnsupportedCompression(2))
        }
        3 => { // GrannyBink/BitKnitCompression
             Err(Gr2ParseError::UnsupportedCompression(3))
        }
        _ => Err(Gr2ParseError::UnsupportedCompression(format)),
    }
}

pub fn parse_gr2(data: &[u8]) -> Result<Gr2File> {
    let magic_value = parse_magic(data)?;
    let header = parse_header(data, &magic_value)?;
    let sections = parse_sections(data, &header)?;

    // Extracted decompressed sections could be cached here, 
    // but the full parsing to Mesh and Skeleton requires type reflection of GR2,
    // which relies on the string table and type definition structures.
    // For this module, we will provide the structure definitions and API for manual extractions,
    // as full reflection mapping requires processing the whole file which might be large.
    // We provide a stub that yields empty vectors if it cannot process the layout without reflection types.
    
    Ok(Gr2File {
        magic_value,
        header,
        sections,
        meshes: Vec::new(),
        skeletons: Vec::new(),
    })
}
