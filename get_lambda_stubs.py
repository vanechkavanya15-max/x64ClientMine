import re

with open("crates/m2-client/src/py_master_runtime_bridge.rs", "r") as f:
    content = f.read()

lambdas = []
for i, line in enumerate(content.splitlines()):
    if "lambda" in line and "staticmethod" in line:
        match = re.search(r"setattr\((.*?),\s*'(.*?)',\s*staticmethod\((.*?)\)\)", line)
        if match:
            lambdas.append(f"{match.group(1)}.{match.group(2)}: {match.group(3)}")
        else:
            lambdas.append(f"Line {i+1}: {line.strip()}")

print(f"Found {len(lambdas)} lambdas:")
for l in lambdas:
    print(l)
