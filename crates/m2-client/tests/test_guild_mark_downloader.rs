use m2_client::guild_mark_downloader::GuildMarkDownloader;

#[test]
fn test_integration_guild_mark_downloader() {
    let instance = GuildMarkDownloader::new();
    assert!(instance.is_ready());
}
