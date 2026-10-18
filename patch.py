with open('src/EterLib/Camera.cpp', 'r') as f:
    lines = f.readlines()

new_lines = []
skip = False
for line in lines:
    if line.startswith('CCameraManager::~CCameraManager()'):
        new_lines.append(line)
        new_lines.append("{\n\t// unique_ptr handles destruction\n}\n")
        skip = True
    elif skip and line.startswith('CCamera * CCameraManager::GetCurrentCamera()'):
        skip = False
        new_lines.append(line)
    elif not skip:
        new_lines.append(line)

with open('src/EterLib/Camera.cpp', 'w') as f:
    f.writelines(new_lines)
