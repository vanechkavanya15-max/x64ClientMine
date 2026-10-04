import os
for root, dirs, files in os.walk("src/UserInterface"):
    for file in files:
        if file.endswith(".cpp"):
            with open(os.path.join(root, file), 'r', encoding='latin-1') as f:
                content = f.read()
                lines = content.split('\n')
                for i, line in enumerate(lines):
                    if "FindItem" in line or "FindSocket" in line:
                        print(f"{file}:{i} {line}")

