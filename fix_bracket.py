import re

with open('src/SphereLib/frustum.cpp', 'r') as f:
    content = f.read()

# Fix double bracket at namespace end
content = content.replace('	}\n	}\n}', '	}\n}')

with open('src/SphereLib/frustum.cpp', 'w') as f:
    f.write(content)
