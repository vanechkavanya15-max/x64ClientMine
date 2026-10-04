with open("src/UserInterface/PythonPlayer.h") as f:
    text = f.read()
    
    import re
    matches = re.findall(r'enum\s+.*?\{.*?\}', text, re.DOTALL)
    for m in matches:
        if "METIN_SOCKET_TYPE_NONE" in m:
            print("=================")
            print(m)

