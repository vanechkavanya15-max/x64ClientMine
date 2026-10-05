import re
import pprint

with open("extracted_bridge.py") as f:
    text = f.read()

lambdas = []
for line in text.splitlines():
    if "lambda" in line:
        lambdas.append(line.strip())

print(f"Found {len(lambdas)} lines with lambda")
for l in lambdas[:20]:
    print(l)
