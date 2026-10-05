import re
import glob

# read exported funcs from py_bridge_*.rs
rust_files = glob.glob("crates/m2-client/src/py_bridge_*.rs")
exported_funcs = set()
func_map = {}

for f in rust_files:
    with open(f, 'r') as fp:
        content = fp.read()
        matches = re.findall(r'pub\s+extern\s+"C"\s+fn\s+(m2_[a-zA-Z0-9_]+)', content)
        for m in matches:
            exported_funcs.add(m)
            func_map[m] = f

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

matches = []
not_found = []

for mod, attr in lambdas:
    mod_name = mod.replace('Module', '').lower()
    snake_attr = camel_to_snake(attr)
    
    # Check all possible prefixes based on the file names and typical naming
    candidates = [
        f"m2_py_{mod_name}_{snake_attr}",
        f"m2_py_{snake_attr}",
        f"m2_{mod_name}_{snake_attr}",
        f"m2_{snake_attr}",
        f"m2_py_app_{snake_attr}",
        f"m2_py_net_{snake_attr}",
        f"m2_py_wndmgr_{snake_attr}",
        f"m2_py_player_{snake_attr}",
        f"m2_py_chr_{snake_attr}",
        f"m2_py_item_{snake_attr}",
        f"m2_py_grp_{snake_attr}",
        f"m2_py_texttail_{snake_attr}",
        f"m2_py_guild_{snake_attr}",
        f"m2_py_shop_{snake_attr}",
        f"m2_py_skill_{snake_attr}",
        f"m2_py_quest_{snake_attr}",
        f"m2_py_minimap_{snake_attr}",
        f"m2_py_nonplayer_{snake_attr}",
        f"m2_py_gamesys_{snake_attr}"
    ]
    
    found = False
    for c in candidates:
        if c in exported_funcs:
            matches.append((mod, attr, c))
            found = True
            break
            
    if not found:
        # fuzzy search:
        # maybe m2_py_..._attr
        for ef in exported_funcs:
            if ef.endswith(f"_{snake_attr}") or ef.replace("_", "").endswith(snake_attr.replace("_", "")):
                matches.append((mod, attr, ef))
                found = True
                break

    if not found:
        not_found.append((mod, attr))

print(f"Found {len(matches)} out of {len(lambdas)}")
if len(not_found) < 20:
    print("Not found:")
    for nf in not_found:
        print(nf)
