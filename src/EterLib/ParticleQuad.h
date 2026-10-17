#pragma once

#include "RenderPacketTypes.h"
#include <d3dx9.h>
#include <cmath>
#include <cstdint>
#include <algorithm>

namespace EterLib::Render
{
	enum EBillboardMode : uint8_t
	{
		BILLBOARD_ALL = 0,
		BILLBOARD_Y = 1,
		BILLBOARD_LIE = 2,
		BILLBOARD_2FACE = 3,
		BILLBOARD_3FACE = 4,
		BILLBOARD_STRETCH = 5
	};
}

struct SParticleQuad
{
	D3DXVECTOR3 positions[4];
};

void BuildParticleQuad(
	uint8_t byBillboardType,
	bool bStretch,
	const D3DXVECTOR3& center,
	float halfWidth,
	float halfHeight,
	float fRotationDeg,
	const D3DXVECTOR3& camUp,
	const D3DXVECTOR3& camCross,
	const D3DXVECTOR3& camView,
	const D3DXVECTOR3& velocity,
	D3DXVECTOR3 outPositions[4],
	const D3DXMATRIX* pMatLocal = nullptr,
	bool bAttach = false,
	float fZRotationRad = 0.0f
);

inline SParticleQuad BuildParticleQuad(
	uint8_t byBillboardType,
	bool bStretch,
	const D3DXVECTOR3& center,
	float halfWidth,
	float halfHeight,
	float fRotationDeg,
	const D3DXVECTOR3& camUp,
	const D3DXVECTOR3& camCross,
	const D3DXVECTOR3& camView,
	const D3DXVECTOR3& velocity = D3DXVECTOR3(0.0f, 0.0f, 0.0f),
	const D3DXMATRIX* pMatLocal = nullptr,
	bool bAttach = false,
	float fZRotationRad = 0.0f
)
{
	SParticleQuad quad{};
	BuildParticleQuad(byBillboardType, bStretch, center, halfWidth, halfHeight, fRotationDeg,
		camUp, camCross, camView, velocity, quad.positions, pMatLocal, bAttach, fZRotationRad);
	return quad;
}
