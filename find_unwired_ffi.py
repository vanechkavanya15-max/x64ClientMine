import re
import os
import glob

# Read all ffi files to find exported m2_* functions
rust_files = glob.glob("crates/m2-client/src/*.rs")
exported_funcs = set()
for f in rust_files:
    with open(f, 'r') as fp:
        content = fp.read()
        # look for pub extern "C" fn m2_...
        matches = re.findall(r'pub\s+extern\s+"C"\s+fn\s+(m2_[a-zA-Z0-9_]+)', content)
        for m in matches:
            exported_funcs.add(m)

print(f"Found {len(exported_funcs)} exported m2_ functions.")

# Now check which ones are used in py_master_runtime_bridge.rs
with open("crates/m2-client/src/py_master_runtime_bridge.rs") as fp:
    bridge_content = fp.read()

unused = []
for func in exported_funcs:
    if func not in bridge_content:
        unused.append(func)

print(f"Found {len(unused)} UNUSED exported m2_ functions:")
for u in unused[:20]:
    print(u)
    
