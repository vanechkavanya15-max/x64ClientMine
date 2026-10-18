import re

with open('src/EterLib/GrpScreen.cpp', 'r') as f:
    content = f.read()

# Fix the accidental leftover lines from the previous broken regex substitution
content = re.sub(
    r'\s*for \(count=0; count<iStep - 1; count\+\+\)\n\s*\{\n\s*RenderLine3d\(pts\[count\]\.x, pts\[count\]\.y, pts\[count\]\.z, pts\[count \+ 1\]\.x, pts\[count \+ 1\]\.y, pts\[count \+ 1\]\.z\);\n\s*\}\n\s*RenderLine3d\(pts\[iStep - 1\]\.x, pts\[iStep - 1\]\.y, pts\[iStep - 1\]\.z, pts\[0\]\.x, pts\[0\]\.y, pts\[0\]\.z\);\n\}',
    '', content)

content = re.sub(
    r'\s*for \(count=0; count<iStep - 1; count\+\+\)\n\s*\{\n\s*RenderLine3d\(fx\+pts\[count\]\.x, fy\+pts\[count\]\.y, fz\+pts\[count\]\.z,\n\s*fx\+pts\[count \+ 1\]\.x, fy\+pts\[count \+ 1\]\.y, fz\+pts\[count \+ 1\]\.z\);\n\s*\}\n\s*RenderLine3d\(fx\+pts\[iStep - 1\]\.x, fy\+pts\[iStep - 1\]\.y, fz\+pts\[iStep - 1\]\.z,\n\s*fx\+pts\[0\]\.x, fy\+pts\[0\]\.y, fz\+pts\[0\]\.z\);',
    '', content)

with open('src/EterLib/GrpScreen.cpp', 'w') as f:
    f.write(content)
