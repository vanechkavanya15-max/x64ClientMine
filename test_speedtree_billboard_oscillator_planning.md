The goal is to translate `CSpeedTreeWrapper` from `src/SpeedTreeLib/SpeedTreeWrapper.cpp` to Rust in `crates/m2-client/src/speedtree_billboard_oscillator.rs` and `crates/m2-client/tests/test_speedtree_billboard_oscillator.rs`.

Requirements:
1.  SpeedTree LOD Management: 3D mesh in near distance -> Cross Billboard in medium -> Single Billboard in far.
2.  Wind Sway Oscillation: Angular sine wave dependent on wind strength and vertex height.
3.  LOD transition (Alpha Dither Cross-Fade) between full mesh and billboard.
4.  FFI export for fetching tree instances in the visible quadrant of the viewing frustum (Frustum Culling).
5.  Tests in `tests/test_speedtree_billboard_oscillator.rs` verifying branch deflection calculations and LOD distance thresholds.

File names:
- `crates/m2-client/src/speedtree_billboard_oscillator.rs`
- `crates/m2-client/tests/test_speedtree_billboard_oscillator.rs`

Also need to register the module in `crates/m2-client/src/lib.rs`.
