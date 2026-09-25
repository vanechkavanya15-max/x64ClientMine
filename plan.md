1. **Create `rust_bridge/src/vfs/hybrid_crypt.rs`:**
   - Define a struct `SdbInfo` holding the SDB stream (`Vec<u8>`).
   - Define `HybridCrypt` struct with a `HashMap<u32, SdbInfo>` to map file hashes to their SDBs.
   - Implement `read_sdb_from_stream` method in `HybridCrypt` that parses the stream (`DWORD` hash, `BYTE` size, and then `size` bytes of data) and returns the number of bytes read.
   - Implement `get_sdb` method in `HybridCrypt` to retrieve SDB by file hash.
   - Export FFI functions using `safe_ffi_call`: `m2_hybrid_crypt_new`, `m2_hybrid_crypt_free`, `m2_hybrid_crypt_read_sdb`, `m2_hybrid_crypt_get_sdb`.
   - Ensure all Polish comments are diacritic-free.

2. **Register module in `lib.rs` and `mod.rs`:**
   - Add `pub mod vfs;` to `lib.rs`.
   - Update `rust_bridge/src/vfs/mod.rs` to export `hybrid_crypt`.
   
3. **Create `rust_bridge/tests/test_vfs_hybrid_crypt.rs`:**
   - Add unit tests for `read_sdb_from_stream` and `get_sdb`.
   - Test FFI functions.
   - Ensure 100% tests pass.

4. **Complete pre-commit steps:**
   - Ensure proper testing, verification, review, and reflection are done.

5. **Submit the changes:**
   - Commit and push to a new branch.
