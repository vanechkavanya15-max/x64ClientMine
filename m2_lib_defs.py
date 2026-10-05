import re
with open("extracted_bridge.py") as f:
    text = f.read()

funcs = []
lines = text.splitlines()
for i, line in enumerate(lines):
    if "argtypes =" in line:
        print(lines[i-1].strip())
        print(line.strip())
        if i+1 < len(lines) and "restype =" in lines[i+1]:
            print(lines[i+1].strip())
        print("")
