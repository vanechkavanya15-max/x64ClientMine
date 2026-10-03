import re
with open('src/UserInterface/PythonNetworkStream.cpp', 'r') as f:
    text = f.read()

m = re.search(r'void CPythonNetworkStream::RegisterGameHandlers\(\)(.*?)void CPythonNetworkStream::RegisterLoadingHandlers', text, re.DOTALL)
if m:
    lines = m.group(1).split('\n')
    for line in lines:
        if 'h[' in line:
            print(line.strip())
