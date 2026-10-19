import re

with open('src/SphereLib/spherepack.cpp', 'r') as f:
    content = f.read()

# Replace <windows.h> with empty or fake if it's there
content = content.replace('#include <windows.h>', '')
content = content.replace('#include "EterBase/Debug.h"', '')
content = content.replace('Tracenf', '//')

with open('src/SphereLib/spherepack.cpp', 'w') as f:
    f.write(content)
