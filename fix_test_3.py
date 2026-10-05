import re

with open('crates/m2-client/tests/test_wgpu_shadow_pcf.rs', 'r') as f:
    code = f.read()

# Fix if let Ok(adapter) back to if let Some(adapter)
code = re.sub(r'if let Ok\(adapter\)', r'if let Some(adapter)', code)

with open('crates/m2-client/tests/test_wgpu_shadow_pcf.rs', 'w') as f:
    f.write(code)

