import re
with open("src/UserInterface/PythonPlayerModule.cpp") as f:
    text = f.read()
    
    matches = re.findall(r'PyObject\s*\*\s*playerCanUnlock[^\{]+\{.*?(?=\n\})', text, re.DOTALL)
    for m in matches:
        print("=================")
        print(m)

