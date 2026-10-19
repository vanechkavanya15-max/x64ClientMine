import re

with open('src/SphereLib/frustum.cpp', 'r') as f:
    content = f.read()

content = content.replace('''	for (int i = 0; i < FRUSTUM_PLANE_COUNT; ++i)
	{
		NormalizePlaneUnified(pPlanesOut[i]);
		if (bNegatePlaneY)
		{
			pPlanesOut[i].b = -pPlanesOut[i].b;
		}
}''', '''	for (int i = 0; i < FRUSTUM_PLANE_COUNT; ++i)
	{
		NormalizePlaneUnified(pPlanesOut[i]);
		if (bNegatePlaneY)
		{
			pPlanesOut[i].b = -pPlanesOut[i].b;
		}
	}
}''')

with open('src/SphereLib/frustum.cpp', 'w') as f:
    f.write(content)
