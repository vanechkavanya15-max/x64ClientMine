import re

with open('crates/m2-client/src/wgpu_shadow_pcf_pipeline.rs', 'r') as f:
    code = f.read()

# Fix WGSL struct alignment correctly
code = re.sub(
r'@size\(12\) _padding: array<u32, 3>,',
r'''_padding: vec3<u32>,''',
code)

# We need the struct size to be 176 bytes in Rust to match the WGSL padding.
# The Rust struct is currently:
# pub light_view_proj_near: [f32; 16], (64)
# pub light_view_proj_far: [f32; 16], (64)
# pub cascade_splits: [f32; 4], (16)
# pub bias: f32, (4)
# WGSL vec3<u32> adds 12 bytes of padding to align the vec3 to 16 bytes.
# So the padding should be 12 bytes padding, then 12 bytes for vec3, then 4 bytes padding to make it a multiple of 16.
# Let's just use 4 f32/u32 values instead to be fully safe and match Rust 1:1.

code = re.sub(
r'    _padding: vec3<u32>,',
r'''    _padding0: u32,
    _padding1: u32,
    _padding2: u32,''',
code)

code = re.sub(
r'    pub _padding: \[u32; 3\],',
r'''    pub _padding: [u32; 3],''',
code)

with open('crates/m2-client/src/wgpu_shadow_pcf_pipeline.rs', 'w') as f:
    f.write(code)

