import re

with open('src/SphereLib/frustum.h', 'r') as f:
    content = f.read()

content = content.replace('#include <d3dx9math.h>', '#include "../Client/Actors/CollisionMathClean.h"\n#include "../Client/Math/PODMath.h"')

content = content.replace('class Frustum \n{', '''struct CleanPlane { float a, b, c, d; };

class Frustum
{''')

content = content.replace('D3DXMATRIX', 'Client::Math::Mat4')
content = content.replace('D3DXPLANE', 'CleanPlane')
content = content.replace('D3DXVECTOR3', 'Client::Collision::CleanVec3')

with open('src/SphereLib/frustum.h', 'w') as f:
    f.write(content)
