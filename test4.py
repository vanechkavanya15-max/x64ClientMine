with open("src/UserInterface/PythonPlayer.cpp") as f:
    text = f.read()
    
    if "PythonPlayer::Find" in text:
        pass
    import re
    matches = re.findall(r'int\s+CPythonPlayer::Find[^\{]+\{.*?(?=\n\})', text, re.DOTALL)
    for m in matches:
        if "Item" in m or "Socket" in m or "Empty" in m:
            print("=================")
            print(m)
            
