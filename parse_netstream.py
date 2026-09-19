import re

with open("source/UserInterface/PythonNetworkStream.h", "r", encoding="iso-8859-1") as f:
    text = f.read()
    
structs = re.findall(r'struct\s+\w+[\s\S]*?\}', text)
for s in structs:
    print(s)

enums = re.findall(r'enum\s*\{[\s\S]*?\}', text)
for e in enums:
    print(e)
