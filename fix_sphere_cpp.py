import re

with open('src/SphereLib/sphere.cpp', 'r') as f:
    content = f.read()

content = content.replace('#include "Stdafx.h"', '')

with open('src/SphereLib/sphere.cpp', 'w') as f:
    f.write(content)
