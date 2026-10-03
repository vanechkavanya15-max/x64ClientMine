import re

c_handlers = []
with open('src/UserInterface/PythonNetworkStream.cpp', 'r') as f:
    text = f.read()

m = re.search(r'void CPythonNetworkStream::RegisterGameHandlers\(\)(.*?)void CPythonNetworkStream::RegisterLoadingHandlers', text, re.DOTALL)
if m:
    lines = m.group(1).split('\n')
    for line in lines:
        if 'h[' in line:
            m2 = re.search(r'h\[GC::(.*?)\]', line)
            if m2:
                c_handlers.append(m2.group(1).strip())

print(c_handlers)
print(len(c_handlers))
