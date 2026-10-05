import re

with open('crates/m2-client/src/wgpu_shadow_pcf_pipeline.rs', 'r') as f:
    code = f.read()

# Fix WGSL struct alignment by replacing array<u32, 3> with vec3<u32> and adding alignment annotations
code = re.sub(
r'_padding: array<u32, 3>,',
r'''@size(12) _padding: array<u32, 3>,''',
code)

with open('crates/m2-client/src/wgpu_shadow_pcf_pipeline.rs', 'w') as f:
    f.write(code)

