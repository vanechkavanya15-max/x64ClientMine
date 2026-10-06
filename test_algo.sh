RUSTFLAGS="-l sodium" SODIUM_USE_PKG_CONFIG=1 cargo test --package m2-client > output.txt 2>&1
grep -A 10 "test result: FAILED" output.txt
