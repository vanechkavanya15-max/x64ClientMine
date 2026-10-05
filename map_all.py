import re
import glob

# read exported funcs from py_bridge_*.rs
rust_files = glob.glob("crates/m2-client/src/py_bridge_*.rs")
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

def camel_to_snake(name):
    s1 = re.sub('(.)([A-Z][a-z]+)', r'\1_\2', name)
    return re.sub('([a-z0-9])([A-Z])', r'\1_\2', s1).lower()

missing = []

for mod, attr in lambdas:
    if mod == 'AppModule': 
        continue
    mod_name = mod.replace('Module', '').lower()
    if mod == 'WndMgrModule': mod_name = 'ui'
    elif mod == 'GrpModule': mod_name = 'graphics'
    
    snake_attr = camel_to_snake(attr)
    found = False
    for ef in exported_funcs:
        if attr.lower() in ef.replace('_', '').lower():
            found = True
            break
            
    if not found:
        missing.append((mod, attr))

print(f"Total non-AppModule lambdas: {len(lambdas) - 23}")
print(f"Missing mapped functions: {len(missing)}")
if len(missing) < 30:
    for m in missing: print(m)
