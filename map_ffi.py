import re
import os
import glob
import textwrap

# Read all ffi files to find exported m2_* functions
rust_files = glob.glob("crates/m2-client/src/*.rs")
exported_funcs = set()
for f in rust_files:
    with open(f, 'r') as fp:
        content = fp.read()
        matches = re.findall(r'pub\s+extern\s+"C"\s+fn\s+(m2_[a-zA-Z0-9_]+)', content)
        for m in matches:
            exported_funcs.add(m)

with open("extracted_bridge.py") as f:
    text = f.read()

lambdas = []
for line in text.splitlines():
    if "lambda" in line and "staticmethod" in line:
        match = re.search(r"setattr\((.*?),\s*'(.*?)',\s*staticmethod\((.*?)\)\)", line)
        if match:
            lambdas.append((match.group(1), match.group(2)))

# Try to match lambda (Module, Method) to m2_module_method
def camel_to_snake(name):
    s1 = re.sub('(.)([A-Z][a-z]+)', r'\1_\2', name)
    return re.sub('([a-z0-9])([A-Z])', r'\1_\2', s1).lower()

matches_found = 0
not_found = []

for mod, attr in lambdas:
    mod_name = mod.replace('Module', '').lower()
    snake_attr = camel_to_snake(attr)
    
    # Check common prefixes
    candidates = [
        f"m2_{mod_name}_{snake_attr}",
        f"m2_{snake_attr}",
        f"m2_py_{mod_name}_{snake_attr}",
        f"m2_py_app_{snake_attr}",
        f"m2_py_{snake_attr}"
    ]
    
    found = False
    for c in candidates:
        if c in exported_funcs:
            matches_found += 1
            found = True
            break
            
    if not found:
        # custom checks
        if mod == "WndMgrModule":
            for c in [f"m2_ui_{snake_attr}", f"m2_py_ui_{snake_attr}"]:
                if c in exported_funcs:
                    found = True
                    matches_found += 1
                    break
    
    if not found:
        not_found.append((mod, attr, candidates))

print(f"Found matches for {matches_found} out of {len(lambdas)}")
print(f"Not found: {len(not_found)}")
for nf in not_found[:10]:
    print(nf)

