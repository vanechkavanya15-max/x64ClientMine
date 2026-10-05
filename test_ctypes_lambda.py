import re

with open("crates/m2-client/src/py_master_runtime_bridge.rs", "r") as f:
    content = f.read()

import ast

def find_ctypes_signatures(content):
    res = []
    lines = content.splitlines()
    for i, line in enumerate(lines):
        if "m2_lib" in line and "argtypes" in line:
            res.append(line.strip())
        if "m2_lib" in line and "restype" in line:
            res.append(line.strip())
    return res

print(find_ctypes_signatures(content))
