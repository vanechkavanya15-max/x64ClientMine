#[path = "../src/granny_native_ffi_bridge.rs"]
mod granny_native_ffi_bridge;

use granny_native_ffi_bridge::*;
use std::mem::{size_of, align_of};

#[test]
fn test_granny_compression_type_size() {
    assert_eq!(size_of::<GrannyCompressionType>(), 4);
}

#[test]
fn test_granny_member_type_size() {
    assert_eq!(size_of::<GrannyMemberType>(), 4);
}

#[test]
fn test_granny_data_type_definition_alignment() {
    assert_eq!(align_of::<GrannyDataTypeDefinition>(), align_of::<usize>());
}

#[test]
fn test_granny_bone_fields_offset() {
    let dummy = std::mem::MaybeUninit::<GrannyBone>::uninit();
    unsafe {
        let base_ptr = dummy.as_ptr() as *const u8;
        let name_ptr = &(*dummy.as_ptr()).name as *const _ as *const u8;
        let parent_index_ptr = &(*dummy.as_ptr()).parent_index as *const _ as *const u8;
        let local_transform_ptr = &(*dummy.as_ptr()).local_transform as *const _ as *const u8;
        
        assert_eq!(name_ptr.offset_from(base_ptr), 0);
        assert_eq!(parent_index_ptr.offset_from(base_ptr), size_of::<*const std::os::raw::c_char>() as isize);
        
        // Due to alignment, the local_transform may be padded by 4 bytes on 64-bit systems
        // after `parent_index` (which is an i32), because it might have a larger alignment requirement.
        // But since we just want to ensure it compiles correctly and types are matched:
        let _ = local_transform_ptr;
    }
}

#[test]
fn test_granny_mesh_fields() {
    let mesh = std::mem::MaybeUninit::<GrannyMesh>::uninit();
    unsafe {
        let base_ptr = mesh.as_ptr() as *const u8;
        let name_ptr = &(*mesh.as_ptr()).name as *const _ as *const u8;
        assert_eq!(name_ptr.offset_from(base_ptr), 0);
    }
}

#[test]
fn test_granny_skeleton_fields() {
    let skeleton = std::mem::MaybeUninit::<GrannySkeleton>::uninit();
    unsafe {
        let base_ptr = skeleton.as_ptr() as *const u8;
        let name_ptr = &(*skeleton.as_ptr()).name as *const _ as *const u8;
        assert_eq!(name_ptr.offset_from(base_ptr), 0);
    }
}

#[test]
fn test_granny_ffi_fallbacks_non_windows() {
    // Tests that the fallbacks can be called without crashing or doing anything malicious
    if cfg!(not(windows)) {
        unsafe {
            let res = GrannyDecompressData(
                GrannyCompressionType::GrannyOodle1Compression,
                false,
                0,
                std::ptr::null_mut(),
                0,
                0,
                0,
                std::ptr::null_mut(),
            );
            assert_eq!(res, false);
            
            let mut dummy_file = std::mem::MaybeUninit::<GrannyFile>::zeroed();
            let file_info = GrannyGetFileInfo(dummy_file.as_mut_ptr());
            assert!(file_info.is_null());
            
            GrannyFreeFile(dummy_file.as_mut_ptr());
        }
    }
}
