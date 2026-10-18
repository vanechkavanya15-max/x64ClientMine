import sys

with open("src/GameLib/MapOutdoorWater.cpp", "r") as f:
    content = f.read()

# Add include
content = content.replace('#include "EterLib/RenderOwnership.h"', '#include "EterLib/RenderOwnership.h"\n#include <algorithm>')

# Replace loops
old_loop = """	for (auto i = m_PatchVector.begin(); i != m_PatchVector.end(); ++i)
	{
		if (i->first < fFogDistance)
			DrawWater(i->second, false, hWaterTexture, pD3DTexture, matTexTransformWater, fWaterHeightCurrent, fTimeSec, fFogDistance);
	}

	if (bLegacyDraws)
	{
		STATEMANAGER.SetTexture(0, NULL);
		STATEMANAGER.SetRenderState(D3DRS_ALPHABLENDENABLE, FALSE);
	}

	for (auto i = m_PatchVector.begin(); i != m_PatchVector.end(); ++i)
	{
		if (i->first >= fFogDistance)
			DrawWater(i->second, true, hWaterTexture, pD3DTexture, matTexTransformWater, fWaterHeightCurrent, fTimeSec, fFogDistance);
	}"""

new_loop = """	auto itFogBound = std::lower_bound(m_PatchVector.begin(), m_PatchVector.end(), fFogDistance,
		[](const std::pair<float, long>& patch, float dist) {
			return patch.first < dist;
		});

	for (auto i = m_PatchVector.begin(); i != itFogBound; ++i)
		DrawWater(i->second, false, hWaterTexture, pD3DTexture, matTexTransformWater, fWaterHeightCurrent, fTimeSec, fFogDistance);

	if (bLegacyDraws)
	{
		STATEMANAGER.SetTexture(0, NULL);
		STATEMANAGER.SetRenderState(D3DRS_ALPHABLENDENABLE, FALSE);
	}

	for (auto i = itFogBound; i != m_PatchVector.end(); ++i)
		DrawWater(i->second, true, hWaterTexture, pD3DTexture, matTexTransformWater, fWaterHeightCurrent, fTimeSec, fFogDistance);"""

if old_loop in content:
    content = content.replace(old_loop, new_loop)
else:
    print("Could not find loop to replace. Check line endings.")

with open("src/GameLib/MapOutdoorWater.cpp", "w") as f:
    f.write(content)

