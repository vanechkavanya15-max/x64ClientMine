sed -i 's/dw_vid: 12345,//g' crates/m2-client/tests/test_send_movement_network.rs
sed -i 's/dw_duration: 1500,//g' crates/m2-client/tests/test_send_movement_network.rs
sed -i 's/length: 27/length: 19/g' crates/m2-client/tests/test_send_movement_network.rs
sed -i 's/bytes.len(), 27/bytes.len(), 19/g' crates/m2-client/tests/test_send_movement_network.rs
sed -i 's/27/19/g' crates/m2-client/tests/test_send_movement_network.rs
