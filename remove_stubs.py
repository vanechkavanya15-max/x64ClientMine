import re

with open("crates/m2-client/src/py_bridge_wndmgr.rs", "r") as f:
    text = f.read()

def remove_fn(text, fn_name):
    # Regex to match the function definition until the closing brace
    # Assuming #[no_mangle] precedes it
    pattern = r'#\[no_mangle\]\s*pub extern "C" fn ' + fn_name + r'\([^)]*\)(?:\s*->\s*[^{]+)?\s*\{[^}]*\}'
    new_text = re.sub(pattern, '', text, count=1)
    return new_text

stubs_to_remove = [
    "wndMgr_GetLocalPosition",
    "wndMgr_GetWindowLocalPosition",
    "wndMgr_GetWindowGlobalPosition",
    "wndMgr_GetChildCount",
    "wndMgr_ShowCursor",
    "wndMgr_HideCursor"
]

for stub in stubs_to_remove:
    text = remove_fn(text, stub)

with open("crates/m2-client/src/py_bridge_wndmgr.rs", "w") as f:
    f.write(text)
