with open("src/UserInterface/PythonItem.cpp") as f:
    text = f.read()

    lines = text.split('\n')
    for i, line in enumerate(lines):
        if "socket" in line.lower() or "kd" in line.lower() or "stone" in line.lower() or "metin" in line.lower():
            pass
            
import re
import os
for root, dirs, files in os.walk("src/UserInterface"):
    for file in files:
        if file.endswith(".cpp"):
            with open(os.path.join(root, file), 'r', encoding='latin-1') as f:
                content = f.read()
                if "Find" in content and "Item" in content and "Socket" in content:
                    print(f"Found in {file}")

