import re

with open('src/SphereLib/frustum.cpp', 'r') as f:
    content = f.read()

content = content.replace('#include "StdAfx.h"', '')

with open('src/SphereLib/frustum.cpp', 'w') as f:
    f.write(content)

with open('src/SphereLib/spherepack.cpp', 'r') as f:
    content = f.read()

content = content.replace('#include "Stdafx.h"', '')

with open('src/SphereLib/spherepack.cpp', 'w') as f:
    f.write(content)
