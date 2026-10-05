import re

with open('crates/m2-client/src/wgpu_shadow_pcf_pipeline.rs', 'r') as f:
    code = f.read()

# Fix WGSL struct alignment by replacing vec3<u32> with array<u32, 3>
code = re.sub(r'_padding: vec3<u32>,', r'_padding: array<u32, 3>,', code)

with open('crates/m2-client/src/wgpu_shadow_pcf_pipeline.rs', 'w') as f:
    f.write(code)

