import re
with open("src/UserInterface/PythonPlayerModule.cpp") as f:
    text = f.read()
    
    matches = re.findall(r'PyObject\s*\*\s*player[^\{]+\{.*?(?=\n\})', text, re.DOTALL)
    for m in matches:
        if "socket" in m.lower() or "kd" in m.lower():
            print("=================")
            print(m)
