use m2_client::msa_script_parser_v2::*;

#[test]
fn test_crlf() {
    let script = "Group MotionEventData\r\n{\r\nMotionEventDataCount 1\r\nGroup Event00\r\n{\r\nMotionEventType 1\r\nStartingTime 0.5\r\nDuringTime 1.0\r\nPower 50\r\nAffectingRange 100\r\n}\r\n}";
    let data = parse_msa_data(script);
    println!("{:?}", data);
    assert!(data.is_ok());
}
