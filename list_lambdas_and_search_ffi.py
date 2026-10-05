import re
import os
import subprocess

with open("extracted_bridge.py") as f:
    text = f.read()

lambdas = []
for line in text.splitlines():
    if "lambda" in line and "staticmethod" in line:
        match = re.search(r"setattr\((.*?),\s*'(.*?)',\s*staticmethod\((.*?)\)\)", line)
        if match:
            lambdas.append((match.group(1), match.group(2)))

print(f"Found {len(lambdas)} lambda attributes")
print(lambdas[:5])

# Find if there are already FFI functions matching these.
for mod, attr in lambdas:
    snake_attr = attr.lower()
    # just print to see some examples
