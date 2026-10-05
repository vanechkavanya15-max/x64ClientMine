import re

with open('crates/m2-client/tests/test_wgpu_shadow_pcf.rs', 'r') as f:
    code = f.read()

# Fix wgpu::InstanceDescriptor::default()
code = code.replace('wgpu::InstanceDescriptor::default()', 'wgpu::InstanceDescriptor::new_without_display_handle()')

# Fix adapter.request_device(..., None)
code = re.sub(r'adapter\.request_device\(\s*&wgpu::DeviceDescriptor::default\(\),\s*None,\s*\)', r'adapter.request_device(&wgpu::DeviceDescriptor::default())', code)

with open('crates/m2-client/tests/test_wgpu_shadow_pcf.rs', 'w') as f:
    f.write(code)

