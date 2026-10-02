// test_item_mob_proto_vfs_loader.rs
use m2_client::item_mob_proto_vfs_loader::load_protos_from_vfs;

#[test]
fn test_load_protos_from_vfs_graceful_fallback() {
    // Nawet gdy archiwa VFS nie sa zamontowane w izolowanym srodowisku testowym,
    // funkcja powinna zwrocic Ok z bezpiecznym fallbackiem (0 rekordow) i nie spanikowac.
    let res = load_protos_from_vfs();
    assert!(res.is_ok(), "load_protos_from_vfs powonno zwrocic Ok");
    let result = res.unwrap();
    println!("Zaladowano itemow: {}, mobow: {}", result.item_count, result.mob_count);
}
