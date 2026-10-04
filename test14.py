with open("src/UserInterface/PythonPlayerModule.cpp") as f:
    text = f.read()
    
    import re
    matches = re.findall(r'PyObject\s*\*\s*playerGetItemLink[^\{]+\{.*?(?=\n\})', text, re.DOTALL)
    for m in matches:
        print("=================")
        print(m)

