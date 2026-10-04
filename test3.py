with open("src/UserInterface/PythonPlayer.cpp") as f:
    text = f.read()

    lines = text.split('\n')
    for i, line in enumerate(lines):
        if "alSockets" in line:
            print(f"{i}: {line}")
