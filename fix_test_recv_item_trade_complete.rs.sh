sed -i 's/info.src_vnum/{ let v = info.src_vnum; v }/g' crates/m2-client/tests/test_recv_item_trade_complete.rs
sed -i 's/info.result_vnum/{ let v = info.result_vnum; v }/g' crates/m2-client/tests/test_recv_item_trade_complete.rs
sed -i 's/info.cost/{ let v = info.cost; v }/g' crates/m2-client/tests/test_recv_item_trade_complete.rs
sed -i 's/info.prob/{ let v = info.prob; v }/g' crates/m2-client/tests/test_recv_item_trade_complete.rs
sed -i 's/info.materials\[0\].vnum/{ let v = info.materials[0].vnum; v }/g' crates/m2-client/tests/test_recv_item_trade_complete.rs
sed -i 's/info.materials\[0\].count/{ let v = info.materials[0].count; v }/g' crates/m2-client/tests/test_recv_item_trade_complete.rs
