import re

with open('src/SphereLib/frustum.cpp', 'r') as f:
    content = f.read()

# Replace matrix _11 etc with m[...]
# Row major: _11 -> 0, _12 -> 1, _13 -> 2, _14 -> 3
# _21 -> 4, _22 -> 5, _23 -> 6, _24 -> 7
# _31 -> 8, _32 -> 9, _33 -> 10, _34 -> 11
# _41 -> 12, _42 -> 13, _43 -> 14, _44 -> 15
replacements = {
    'mat._11': 'mat.m[0]', 'mat._12': 'mat.m[1]', 'mat._13': 'mat.m[2]', 'mat._14': 'mat.m[3]',
    'mat._21': 'mat.m[4]', 'mat._22': 'mat.m[5]', 'mat._23': 'mat.m[6]', 'mat._24': 'mat.m[7]',
    'mat._31': 'mat.m[8]', 'mat._32': 'mat.m[9]', 'mat._33': 'mat.m[10]', 'mat._34': 'mat.m[11]',
    'mat._41': 'mat.m[12]', 'mat._42': 'mat.m[13]', 'mat._43': 'mat.m[14]', 'mat._44': 'mat.m[15]'
}

for k, v in replacements.items():
    content = content.replace(k, v)

content = content.replace('D3DXMATRIX', 'Client::Math::Mat4')
content = content.replace('D3DXPLANE', 'CleanPlane')
content = content.replace('D3DXVECTOR3', 'Client::Collision::CleanVec3')

# Replace D3DXPlaneNormalize
normalize_func = '''inline void NormalizePlaneUnified(CleanPlane& plane) noexcept
	{
		const float magSq = plane.a * plane.a + plane.b * plane.b + plane.c * plane.c;
		if (magSq > 1e-12f)
		{
			float invMag = 1.0f / std::sqrt(magSq);
			plane.a *= invMag;
			plane.b *= invMag;
			plane.c *= invMag;
			plane.d *= invMag;
		}
	}'''

content = re.sub(r'inline void NormalizePlaneUnified.*?\}', normalize_func, content, flags=re.DOTALL)

# Replace ExtractPlanes declaration
content = content.replace('void Frustum::ExtractPlanes(const Client::Math::Mat4 & mat, CleanPlane * pPlanesOut, bool bNegatePlaneY)', 'void Frustum::ExtractPlanes(const Client::Math::Mat4 & mat, CleanPlane * pPlanesOut, bool bNegatePlaneY)')

# CleanPlane constructors are replaced with initializer lists
content = re.sub(r'CleanPlane\((.*?)\)', r'CleanPlane{\1}', content)

# D3DXVec3Normalize
content = content.replace('D3DXVec3Normalize(&vLookNorm, &vLookNorm);', '''{
		float len = vLookNorm.Length();
		if (len > 0.0f) {
			vLookNorm = vLookNorm / len;
		}
	}''')

# D3DXPlaneDotCoord(&m_plane[i], &p)
content = re.sub(r'D3DXPlaneDotCoord\(&m_plane\[i\], &([a-zA-Z0-9_]+)\)', r'(m_plane[i].a * \1.x + m_plane[i].b * \1.y + m_plane[i].c * \1.z + m_plane[i].d)', content)

with open('src/SphereLib/frustum.cpp', 'w') as f:
    f.write(content)
