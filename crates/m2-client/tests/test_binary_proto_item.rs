use m2_client::binary_proto_item_reader::{
    BinaryProtoItemReader, TItemTable, ITEM_PROTO_FOURCC_MIPX,
};

#[test]
fn test_binary_proto_item_reader_direct() {
    let mut reader = BinaryProtoItemReader::new();
    assert!(reader.is_empty());

    let mut item: TItemTable = unsafe { std::mem::zeroed() };
    item.dwVnum = 10; // Miecz +0
    item.bType = 1; // ITEM_WEAPON
    item.bSubType = 0; // WEAPON_SWORD
    item.dwIBuyItemPrice = 1000;
    let name = b"Sword+0\0";
    item.szName[..name.len()].copy_from_slice(name);

    reader.register_item(item);

    assert_eq!(reader.len(), 1);
    let fetched = reader.get_item(10).expect("Miecz powinien istniec");
    let vnum = fetched.dwVnum;
    let price = fetched.dwIBuyItemPrice;
    assert_eq!(vnum, 10);
    assert_eq!(price, 1000);
}

#[test]
fn test_binary_proto_item_reader_header() {
    let mut reader = BinaryProtoItemReader::new();

    // Prawidlowy naglowek MIPX
    let mut valid_header = Vec::new();
    valid_header.extend_from_slice(&ITEM_PROTO_FOURCC_MIPX.to_le_bytes());
    valid_header.extend_from_slice(&0u32.to_le_bytes());
    valid_header.extend_from_slice(&0u32.to_le_bytes());

    let res = reader.load_from_memory(&valid_header, None);
    assert!(res.is_ok());
    assert_eq!(reader.len(), 0);
}
