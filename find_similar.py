import re
import glob

# read exported funcs
rust_files = glob.glob("crates/m2-client/src/*.rs")
exported_funcs = set()
for f in rust_files:
    with open(f, 'r') as fp:
        content = fp.read()
        matches = re.findall(r'pub\s+extern\s+"C"\s+fn\s+(m2_[a-zA-Z0-9_]+)', content)
        for m in matches:
            exported_funcs.add(m)

print("Searching for 'frame_skip'...")
for f in exported_funcs:
    if "frame_skip" in f.lower() or "random" in f.lower() or "timestamp" in f.lower() or "hair" in f.lower() or "specular" in f.lower() or "locale" in f.lower() or "exist" in f.lower():
        print(f)
