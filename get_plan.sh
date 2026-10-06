#!/bin/bash
cat > plan.md << 'EOL'
1. *Create the target file `crates/m2-client/src/send_refine_blacksmith_ops.rs`.*
   - Define constants `HEADER_CG_REFINE = 0x050C` and `HEADER_CG_ITEM_USE_TO_ITEM = 0x0506`.
   - Define structures `TPacketCGRefine` and `TPacketCGItemUseToItem` (for REFINE_SCROLL).
   - Implement formatting functions `format_cg_refine_packet(pos, type_)` and `format_cg_item_use_to_item_packet(source_pos, target_pos)`.

2. *Create the test file `crates/m2-client/tests/test_send_refine_blacksmith_ops.rs`.*
   - Include test cases for standard refining (`HEADER_CG_REFINE`) and scroll refining (`HEADER_CG_ITEM_USE_TO_ITEM`).
   - Check structural invariants (sizes, offsets) of packets using `std::mem`.
   - Validate serialization buffers.
   - Run `cargo test --package m2-client` headless on Google Cloud Jules VM.

3. *Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.*

4. *Commit and complete work.*
EOL
