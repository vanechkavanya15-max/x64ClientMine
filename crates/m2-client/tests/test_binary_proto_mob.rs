use m2_client::binary_proto_mob_reader::{
    BinaryProtoMobReader, TMobTable, MOB_PROTO_FOURCC_MOPX,
};

#[test]
fn test_binary_proto_mob_reader_direct() {
    let mut reader = BinaryProtoMobReader::new();
    assert!(reader.is_empty());

    let mut mob: TMobTable = unsafe { std::mem::zeroed() };
    mob.dwVnum = 101; // Dziki pies
    mob.bType = 0; // Potwor
    mob.bRank = 0;
    mob.bLevel = 1;
    let name = b"Wild Dog\0";
    mob.szName[..name.len()].copy_from_slice(name);

    reader.register_mob(mob);

    assert_eq!(reader.len(), 1);
    let fetched = reader.get_mob(101).expect("Powinien znalezc dzikiego psa");
    let vnum = fetched.dwVnum;
    let level = fetched.bLevel;
    assert_eq!(vnum, 101);
    assert_eq!(level, 1);
}

#[test]
fn test_binary_proto_mob_reader_header_validation() {
    let mut reader = BinaryProtoMobReader::new();

    // Zly naglowek
    let bad_header = [0u8; 16];
    let res = reader.load_from_memory(&bad_header, None);
    assert!(res.is_err());

    // Prawidlowy naglowek MOPX z 0 wpisow
    let mut valid_header = Vec::new();
    valid_header.extend_from_slice(&MOB_PROTO_FOURCC_MOPX.to_le_bytes());
    valid_header.extend_from_slice(&0u32.to_le_bytes()); // elements = 0
    valid_header.extend_from_slice(&0u32.to_le_bytes()); // data_size = 0

    let res_ok = reader.load_from_memory(&valid_header, None);
    assert!(res_ok.is_ok());
    assert_eq!(reader.len(), 0);
}
