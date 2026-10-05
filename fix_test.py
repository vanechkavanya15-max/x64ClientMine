import re

with open('crates/m2-client/tests/test_wgpu_shadow_pcf.rs', 'r') as f:
    code = f.read()

# Fix wgpu::InstanceDescriptor::default()
code = re.sub(r'wgpu::InstanceDescriptor \{\s*backends: wgpu::Backends::all\(\),\s*\.\.Default::default\(\)\s*\}', r'wgpu::InstanceDescriptor::default()', code)

# Fix RequestAdapterOptions
code = re.sub(r'&wgpu::RequestAdapterOptions \{\s*power_preference: wgpu::PowerPreference::LowPower,\s*compatible_surface: None,\s*force_fallback_adapter: false,\s*\}', r'&wgpu::RequestAdapterOptions::default()', code)

# Fix Result<Adapter, ...>
code = re.sub(r'if let Some\(adapter\) = adapter', r'if let Ok(adapter) = adapter', code)

# Fix DeviceDescriptor
code = re.sub(r'&wgpu::DeviceDescriptor \{\s*label: None,\s*required_features: wgpu::Features::empty\(\),\s*required_limits: wgpu::Limits::downlevel_defaults\(\),\s*\}', r'&wgpu::DeviceDescriptor::default()', code)

with open('crates/m2-client/tests/test_wgpu_shadow_pcf.rs', 'w') as f:
    f.write(code)

