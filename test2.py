with open("src/UserInterface/PythonPlayer.cpp") as f:
    text = f.read()

    lines = text.split('\n')
    for i, line in enumerate(lines):
        if "socket" in line.lower() or "kd" in line.lower() or "stone" in line.lower() or "metin" in line.lower():
            if "metin" in line.lower() and "socket" in line.lower():
                pass
            print(f"{i}: {line}")
