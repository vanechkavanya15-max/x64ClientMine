import os
for root, dirs, files in os.walk("src/UserInterface"):
    for file in files:
        if file.endswith(".cpp") or file.endswith(".h"):
            with open(os.path.join(root, file), 'r', encoding='latin-1') as f:
                content = f.read()
                lines = content.split('\n')
                for i, line in enumerate(lines):
                    if "FindEmpty" in line or "socket" in line.lower() or "find" in line.lower():
                        if "socket" in line.lower() and "empty" in line.lower():
                            print(f"{file}:{i} {line}")

