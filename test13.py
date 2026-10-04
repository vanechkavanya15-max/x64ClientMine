import os
for root, dirs, files in os.walk("src/UserInterface"):
    for file in files:
        if file.endswith(".cpp") or file.endswith(".h"):
            with open(os.path.join(root, file), 'r', encoding='latin-1') as f:
                content = f.read()
                if "FindItemSlot" in content or "FindEmptyItem" in content or "FindEmptySocket" in content or "FindEmpty" in content:
                    print(f"Found in {file}")

