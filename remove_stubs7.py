import re

with open("crates/m2-client/src/py_bridge_wndmgr.rs", "r") as f:
    text = f.read()

stubs_to_remove = [
    r'#\[no_mangle\]\s*pub extern "C" fn wndMgr_GetLocalPosition[^}]*\}\s*\}\s*\}',
    r'#\[no_mangle\]\s*pub extern "C" fn wndMgr_GetWindowGlobalPosition[^}]*\}\s*\}\s*\}',
]

for stub_pattern in stubs_to_remove:
    text = re.sub(stub_pattern, '', text, count=1, flags=re.DOTALL)

with open("crates/m2-client/src/py_bridge_wndmgr.rs", "w") as f:
    f.write(text)
