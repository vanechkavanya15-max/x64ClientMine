import re
with open("src/UserInterface/PythonPlayerModule.cpp") as f:
    text = f.read()

    matches = re.findall(r'int\s+CPythonPlayer::Find[^\{]+\{.*?(?=\n\})', text, re.DOTALL)
    for m in matches:
        print("=================")
        print(m)

