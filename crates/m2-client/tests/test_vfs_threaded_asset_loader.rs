extern crate tempfile;
use std::fs::File;
use std::io::Write;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use tempfile::tempdir;

#[path = "../src/vfs_threaded_asset_loader.rs"]
mod vfs_threaded_asset_loader;

pub mod texture_vfs_pipeline {
    pub fn read_vfs_bytes(path: &str) -> Option<Vec<u8>> {
        std::fs::read(path).ok()
    }
}

use vfs_threaded_asset_loader::{AssetType, VfsAssetLoadEvent, VfsAssetLoadRequest, VfsThreadedAssetLoader};

#[test]
fn test_loader_initialization_and_termination() {
    let mut loader = VfsThreadedAssetLoader::new(2);
    assert_eq!(loader.sync_all_events().len(), 0);
    loader.stop_all();
}

#[test]
fn test_single_thread_load_success() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test_texture.tga");
    let mut file = File::create(&file_path).unwrap();
    file.write_all(b"mock_texture_data").unwrap();

    let mut loader = VfsThreadedAssetLoader::new(1);
    let sender = loader.get_request_sender();
    
    sender.send(VfsAssetLoadRequest::Load {
        path: file_path.to_str().unwrap().to_string(),
        asset_type: AssetType::Texture,
    }).unwrap();

    thread::sleep(Duration::from_millis(100));

    let events = loader.sync_all_events();
    assert_eq!(events.len(), 1);
    
    match &events[0] {
        VfsAssetLoadEvent::Loaded { path, asset_type, data } => {
            assert_eq!(path, file_path.to_str().unwrap());
            assert_eq!(*asset_type, AssetType::Texture);
            assert_eq!(data, b"mock_texture_data");
        }
        _ => panic!("Expected Loaded event"),
    }
}

#[test]
fn test_load_file_not_found() {
    let mut loader = VfsThreadedAssetLoader::new(1);
    let sender = loader.get_request_sender();
    
    sender.send(VfsAssetLoadRequest::Load {
        path: "non_existent_file.gr2".to_string(),
        asset_type: AssetType::Model,
    }).unwrap();

    thread::sleep(Duration::from_millis(50));

    let events = loader.sync_all_events();
    assert_eq!(events.len(), 1);
    
    match &events[0] {
        VfsAssetLoadEvent::LoadFailed { path, reason } => {
            assert_eq!(path, "non_existent_file.gr2");
            assert!(reason.contains("not found"));
        }
        _ => panic!("Expected LoadFailed event"),
    }
}

#[test]
fn test_concurrent_loading() {
    let dir = tempdir().unwrap();
    
    let path1 = dir.path().join("file1.txt");
    File::create(&path1).unwrap().write_all(b"data1").unwrap();
    
    let path2 = dir.path().join("file2.txt");
    File::create(&path2).unwrap().write_all(b"data2").unwrap();

    let path3 = dir.path().join("file3.txt");
    File::create(&path3).unwrap().write_all(b"data3").unwrap();

    let mut loader = VfsThreadedAssetLoader::new(4);
    let sender = loader.get_request_sender();
    
    sender.send(VfsAssetLoadRequest::Load { path: path1.to_str().unwrap().to_string(), asset_type: AssetType::Texture }).unwrap();
    sender.send(VfsAssetLoadRequest::Load { path: path2.to_str().unwrap().to_string(), asset_type: AssetType::Texture }).unwrap();
    sender.send(VfsAssetLoadRequest::Load { path: path3.to_str().unwrap().to_string(), asset_type: AssetType::Texture }).unwrap();

    thread::sleep(Duration::from_millis(200));

    let events = loader.sync_all_events();
    assert_eq!(events.len(), 3);
    
    for event in events {
        match event {
            VfsAssetLoadEvent::Loaded { data, .. } => {
                assert!(data == b"data1" || data == b"data2" || data == b"data3");
            }
            _ => panic!("Expected all to be loaded"),
        }
    }
}

#[test]
fn test_mixed_asset_types() {
    let dir = tempdir().unwrap();
    
    let model_path = dir.path().join("model.gr2");
    File::create(&model_path).unwrap().write_all(b"GR2_DATA").unwrap();
    
    let tex_path = dir.path().join("tex.dds");
    File::create(&tex_path).unwrap().write_all(b"DDS_DATA").unwrap();

    let mut loader = VfsThreadedAssetLoader::new(2);
    let sender = loader.get_request_sender();
    
    sender.send(VfsAssetLoadRequest::Load { path: model_path.to_str().unwrap().to_string(), asset_type: AssetType::Model }).unwrap();
    sender.send(VfsAssetLoadRequest::Load { path: tex_path.to_str().unwrap().to_string(), asset_type: AssetType::Texture }).unwrap();

    thread::sleep(Duration::from_millis(100));

    let events = loader.sync_all_events();
    assert_eq!(events.len(), 2);
    
    let mut model_found = false;
    let mut tex_found = false;

    for event in events {
        match event {
            VfsAssetLoadEvent::Loaded { asset_type, data, .. } => {
                if asset_type == AssetType::Model {
                    assert_eq!(data, b"GR2_DATA");
                    model_found = true;
                } else if asset_type == AssetType::Texture {
                    assert_eq!(data, b"DDS_DATA");
                    tex_found = true;
                }
            }
            _ => panic!("Unexpected event"),
        }
    }
    
    assert!(model_found);
    assert!(tex_found);
}

#[test]
fn test_try_recv_event() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("fast_file.txt");
    File::create(&file_path).unwrap().write_all(b"fast").unwrap();

    let mut loader = VfsThreadedAssetLoader::new(1);
    let sender = loader.get_request_sender();
    
    sender.send(VfsAssetLoadRequest::Load {
        path: file_path.to_str().unwrap().to_string(),
        asset_type: AssetType::Model,
    }).unwrap();

    let mut received = false;
    for _ in 0..100 {
        if let Some(event) = loader.try_recv_event() {
            match event {
                VfsAssetLoadEvent::Loaded { data, .. } => {
                    assert_eq!(data, b"fast");
                    received = true;
                    break;
                }
                _ => panic!("Expected Loaded event"),
            }
        }
        thread::sleep(Duration::from_millis(10));
    }
    
    assert!(received, "Did not receive event in time");
}
