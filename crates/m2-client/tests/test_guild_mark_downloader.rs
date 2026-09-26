#[path = "../src/guild_mark_downloader.rs"]
mod guild_mark_downloader;

use guild_mark_downloader::GuildMarkDownloader;

#[test]
fn test_guild_mark_downloader_new() {
    let downloader = GuildMarkDownloader::new();
    assert_eq!(downloader.get_guild_mark(1), None);
}

#[test]
fn test_guild_mark_downloader_default() {
    let downloader: GuildMarkDownloader = Default::default();
    assert_eq!(downloader.get_guild_mark(1), None);
}

#[test]
fn test_set_guild_mark_success() {
    let mut downloader = GuildMarkDownloader::new();
    let data = vec![0u8; 384]; // 16 * 12 * 2
    
    let result = downloader.set_guild_mark(100, &data);
    assert!(result, "Expected to successfully set guild mark for size 384");
    
    let retrieved = downloader.get_guild_mark(100);
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap(), data.as_slice());
}

#[test]
fn test_set_guild_mark_failure_too_small() {
    let mut downloader = GuildMarkDownloader::new();
    let data = vec![0u8; 383];
    
    let result = downloader.set_guild_mark(101, &data);
    assert!(!result, "Expected failure when setting guild mark for size 383");
    
    let retrieved = downloader.get_guild_mark(101);
    assert!(retrieved.is_none());
}

#[test]
fn test_set_guild_mark_failure_too_large() {
    let mut downloader = GuildMarkDownloader::new();
    let data = vec![0u8; 385];
    
    let result = downloader.set_guild_mark(102, &data);
    assert!(!result, "Expected failure when setting guild mark for size 385");
    
    let retrieved = downloader.get_guild_mark(102);
    assert!(retrieved.is_none());
}

#[test]
fn test_set_guild_mark_failure_empty() {
    let mut downloader = GuildMarkDownloader::new();
    let data: Vec<u8> = vec![];
    
    let result = downloader.set_guild_mark(103, &data);
    assert!(!result, "Expected failure when setting guild mark for empty buffer");
    
    let retrieved = downloader.get_guild_mark(103);
    assert!(retrieved.is_none());
}

#[test]
fn test_get_nonexistent_guild_mark() {
    let downloader = GuildMarkDownloader::new();
    let retrieved = downloader.get_guild_mark(999);
    assert!(retrieved.is_none());
}

#[test]
fn test_overwrite_guild_mark() {
    let mut downloader = GuildMarkDownloader::new();
    
    let data1 = vec![1u8; 384];
    let result1 = downloader.set_guild_mark(200, &data1);
    assert!(result1);
    
    let data2 = vec![2u8; 384];
    let result2 = downloader.set_guild_mark(200, &data2);
    assert!(result2);
    
    let retrieved = downloader.get_guild_mark(200);
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap(), data2.as_slice());
}
