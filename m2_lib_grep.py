import re

with open("crates/m2-client/src/py_master_runtime_bridge.rs", "r") as f:
    content = f.read()

import ctypes

print("Looking for how m2_lib is used...")
lines = content.splitlines()
for i, line in enumerate(lines):
    if "m2_lib" in line and "ctypes" in line:
        print(f"Line {i+1}: {line.strip()}")

print("Looking for m2_lib initialization...")
for i, line in enumerate(lines):
    if "m2_lib =" in line:
        print(f"Line {i+1}: {line.strip()}")
