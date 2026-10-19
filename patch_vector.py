import re

with open('src/SphereLib/vector.h', 'r') as f:
    content = f.read()

# Replace <d3dx9math.h> with CollisionMathClean.h
content = content.replace('#include <d3dx9math.h>', '#include "../Client/Actors/CollisionMathClean.h"')

# Replace public D3DXVECTOR3 with public Client::Collision::CleanVec3
content = content.replace('public D3DXVECTOR3', 'public Client::Collision::CleanVec3')

# Remove Vector3d(const D3DXVECTOR3 &v) : D3DXVECTOR3(v) {}
content = re.sub(r'Vector3d\(const D3DXVECTOR3 &v\) : D3DXVECTOR3\(v\) \{\}', '', content)

with open('src/SphereLib/vector.h', 'w') as f:
    f.write(content)
