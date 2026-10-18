import re

with open('src/EterLib/GrpScreen.cpp', 'r') as f:
    content = f.read()

content = content.replace('}\n}\n\nclass CD3DXMeshRenderingOption', '}\n\nclass CD3DXMeshRenderingOption')

with open('src/EterLib/GrpScreen.cpp', 'w') as f:
    f.write(content)
