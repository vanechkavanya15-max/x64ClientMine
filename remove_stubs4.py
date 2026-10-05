import re

with open("crates/m2-client/src/py_bridge_wndmgr.rs", "r") as f:
    text = f.read()

def remove_fn(text, fn_name):
    # Match #[no_mangle] and the function definition.
    # Handles nested braces by finding the matching closing brace.
    
    start_idx = text.find("pub extern \"C\" fn " + fn_name)
    if start_idx == -1:
        return text
    
    # Find #[no_mangle] before the function
    no_mangle_idx = text.rfind("#[no_mangle]", 0, start_idx)
    if no_mangle_idx == -1:
        no_mangle_idx = start_idx
    
    # Find opening brace
    open_brace_idx = text.find("{", start_idx)
    if open_brace_idx == -1:
        return text
        
    brace_count = 1
    curr_idx = open_brace_idx + 1
    
    while brace_count > 0 and curr_idx < len(text):
        if text[curr_idx] == '{':
            brace_count += 1
        elif text[curr_idx] == '}':
            brace_count -= 1
        curr_idx += 1
        
    if brace_count == 0:
        return text[:no_mangle_idx] + text[curr_idx:]
    return text

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
