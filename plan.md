1.  **Implement the Shadow PCF Pipeline in Rust:**
    *   Create a new file `crates/m2-client/src/wgpu_shadow_pcf_pipeline.rs`.
    *   Define the `WgpuShadowPcfPipeline` struct.
    *   Implement initialization logic ensuring a 2048x2048 shadow map and a 16-tap PCF 4x4 filter, with near and far cascades, mirroring C++ terrain/shadow properties.
    *   The WGSL shader must include 16-tap PCF and smooth cascade blend zones.
    *   Add a method to calculate the sun light matrix (used for cascades).

2.  **Expose module:**
    *   Add `pub mod wgpu_shadow_pcf_pipeline;` to `crates/m2-client/src/lib.rs`.

3.  **Implement Tests:**
    *   Create a new file `crates/m2-client/tests/test_wgpu_shadow_pcf.rs`.
    *   Write 3-4 comprehensive tests:
        *   Test initialization (texture sizes, cascades).
        *   Test light matrix calculation.
        *   Test that the WGSL shader compiles correctly (creates a render pipeline).
        *   Test cascade blend logic (using stub logic or matrices calculations).

4.  **Pre-commit checks:**
    *   Run Cargo checks and formatting (`cargo check -p m2-client --lib`, `cargo test -p m2-client --test test_wgpu_shadow_pcf`). Ensure 0 warnings and unused imports.

5.  **Submit changes:**
    *   Commit changes via `submit`.
