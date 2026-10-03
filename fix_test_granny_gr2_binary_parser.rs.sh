sed -i 's/magic.magic/{ let v = magic.magic; v }/g' crates/m2-client/tests/test_granny_gr2_binary_parser.rs
sed -i 's/magic.header_size/{ let v = magic.header_size; v }/g' crates/m2-client/tests/test_granny_gr2_binary_parser.rs
sed -i 's/header.version/{ let v = header.version; v }/g' crates/m2-client/tests/test_granny_gr2_binary_parser.rs
sed -i 's/sections\[0\].format/{ let v = sections[0].format; v }/g' crates/m2-client/tests/test_granny_gr2_binary_parser.rs
