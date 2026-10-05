import re

with open('crates/m2-client/tests/test_wgpu_shadow_pcf.rs', 'r') as f:
    code = f.read()

# Fix Some(adapter) -> Ok(adapter)
code = re.sub(r'if let Some\(adapter\)', r'if let Ok(adapter)', code)

with open('crates/m2-client/tests/test_wgpu_shadow_pcf.rs', 'w') as f:
    f.write(code)

