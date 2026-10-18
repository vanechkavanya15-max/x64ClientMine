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
		BILLBOARD_ALL = 1,
		BILLBOARD_Y = 2,
		BILLBOARD_LIE = 3,
		BILLBOARD_2FACE = 4,
		BILLBOARD_3FACE = 5,
		BILLBOARD_STRETCH = 6
	};
}

struct SCameraParameters
{
	D3DXVECTOR3 v3Eye{ 0.0f, 0.0f, 0.0f };
	D3DXVECTOR3 v3Up{ 0.0f, 0.0f, 1.0f };
	D3DXVECTOR3 v3Cross{ 1.0f, 0.0f, 0.0f };
	D3DXVECTOR3 v3View{ 0.0f, 1.0f, 0.0f };
};

struct SParticleQuad
{
	D3DXVECTOR3 positions[4];
};

[[nodiscard]] inline SParticleQuad ComputeParticleBillboardQuad(
	uint8_t byBillboardType,
	bool bStretch,
	const D3DXVECTOR3& center,
	float halfWidth,
	float halfHeight,
	float fRotationDeg,
	const SCameraParameters& camera,
	const D3DXVECTOR3& velocity = D3DXVECTOR3(0.0f, 0.0f, 0.0f),
	const D3DXMATRIX* pMatLocal = nullptr,
	bool bAttach = false,
	float fZRotationRad = 0.0f
) noexcept
{
	const D3DXVECTOR3 vToCam = camera.v3Eye - center;
	const float fDistSq = D3DXVec3LengthSq(&vToCam);
	if (fDistSq < 1e-12f)
	{
		const D3DXVECTOR3 safeUp = (D3DXVec3LengthSq(&camera.v3Up) > 1e-6f) ? camera.v3Up : D3DXVECTOR3(0.0f, 0.0f, 1.0f);
		const D3DXVECTOR3 safeCross = (D3DXVec3LengthSq(&camera.v3Cross) > 1e-6f) ? camera.v3Cross : D3DXVECTOR3(1.0f, 0.0f, 0.0f);

		const D3DXVECTOR3 vOffsetUp = halfHeight * safeUp;
		const D3DXVECTOR3 vOffsetCross = -halfWidth * safeCross;

		D3DXVECTOR3 centerPos = center;
		if (pMatLocal && bAttach)
		{
			D3DXVec3TransformCoord(&centerPos, &center, pMatLocal);
		}

		SParticleQuad quad{};
		quad.positions[0] = centerPos - vOffsetUp + vOffsetCross;
		quad.positions[1] = centerPos - vOffsetUp - vOffsetCross;
		quad.positions[2] = centerPos + vOffsetUp + vOffsetCross;
		quad.positions[3] = centerPos + vOffsetUp - vOffsetCross;
		return quad;
	}

	D3DXVECTOR3 safeCamView = camera.v3View;
	const float camViewLenSq = D3DXVec3LengthSq(&safeCamView);
	if (camViewLenSq < 1e-12f)
	{
		safeCamView = D3DXVECTOR3(0.0f, 1.0f, 0.0f);
	}
	else
	{
		safeCamView /= std::sqrt(camViewLenSq);
	}

	D3DXVECTOR3 v3Up(0.0f, 0.0f, 0.0f);
	D3DXVECTOR3 v3Cross(0.0f, 0.0f, 0.0f);

	if (byBillboardType == EterLib::Render::BILLBOARD_STRETCH || byBillboardType == 6)
	{
		bStretch = true;
	}

	if (!bStretch)
	{
		const bool isLie = (byBillboardType == EterLib::Render::BILLBOARD_LIE || byBillboardType == 3);
		const bool isY = (byBillboardType == EterLib::Render::BILLBOARD_Y ||
		                  byBillboardType == EterLib::Render::BILLBOARD_2FACE ||
		                  byBillboardType == EterLib::Render::BILLBOARD_3FACE ||
		                  byBillboardType == 2 || byBillboardType == 4 || byBillboardType == 5);

		if (isLie)
		{
			const float rad = D3DXToRadian(fRotationDeg);
			const float fCos = std::cos(rad);
			const float fSin = std::sin(rad);

			v3Up.x = fCos;
			v3Up.y = -fSin;
			v3Up.z = 0.0f;

			v3Cross.x = fSin;
			v3Cross.y = fCos;
			v3Cross.z = 0.0f;
		}
		else if (isY)
		{
			v3Up = D3DXVECTOR3(0.0f, 0.0f, 1.0f);
			if (v3Up.x * safeCamView.y - v3Up.y * safeCamView.x < 0.0f)
			{
				v3Up *= -1.0f;
			}

			const D3DXVECTOR3 d3dd(safeCamView.x, safeCamView.y, 0.0f);
			D3DXVec3Cross(&v3Cross, &v3Up, &d3dd);
			const float crossLen = D3DXVec3Length(&v3Cross);
			if (crossLen > 1e-6f)
			{
				v3Cross /= crossLen;
			}
			else
			{
				v3Cross = D3DXVECTOR3(1.0f, 0.0f, 0.0f);
			}

			if (fRotationDeg != 0.0f)
			{
				const float rad = D3DXToRadian(fRotationDeg);
				const float fCos = std::cos(rad);
				const float fSin = std::sin(rad);

				const D3DXVECTOR3 v3Temp = v3Up * fCos - v3Cross * fSin;
				v3Cross = v3Cross * fCos + v3Up * fSin;
				v3Up = v3Temp;
			}
		}
		else
		{
			if (fRotationDeg == 0.0f)
			{
				v3Up = -camera.v3Cross;
				v3Cross = camera.v3Up;
			}
			else
			{
				D3DXMATRIX matRotation;
				D3DXMatrixRotationAxis(&matRotation, &safeCamView, D3DXToRadian(fRotationDeg));
				const D3DXVECTOR3 negCross = -camera.v3Cross;
				D3DXVec3TransformCoord(&v3Up, &negCross, &matRotation);
				D3DXVec3TransformCoord(&v3Cross, &camera.v3Up, &matRotation);
			}
		}
	}
	else
	{
		v3Up = velocity;

		if (pMatLocal)
		{
			D3DXVec3TransformNormal(&v3Up, &v3Up, pMatLocal);
		}

		const float length = D3DXVec3Length(&v3Up);
		if (length < 1e-6f)
		{
			v3Up = D3DXVECTOR3(0.0f, 0.0f, 1.0f);
		}
		else
		{
			v3Up *= (1.0f + std::log(1.0f + length)) / length;
		}

		D3DXVec3Cross(&v3Cross, &v3Up, &safeCamView);
		const float crossLen = D3DXVec3Length(&v3Cross);
		if (crossLen > 1e-6f)
		{
			v3Cross /= crossLen;
		}
		else
		{
			v3Cross = D3DXVECTOR3(1.0f, 0.0f, 0.0f);
		}
	}

	if (fZRotationRad != 0.0f)
	{
		const float fCos = std::cos(fZRotationRad);
		const float fSin = std::sin(fZRotationRad);

		const float ux = v3Up.x;
		const float uy = v3Up.y;
		v3Up.x = ux * fCos - uy * fSin;
		v3Up.y = uy * fCos + ux * fSin;

		const float cx = v3Cross.x;
		const float cy = v3Cross.y;
		v3Cross.x = cx * fCos - cy * fSin;
		v3Cross.y = cy * fCos + cx * fSin;
	}

	v3Cross = -halfWidth * v3Cross;
	v3Up = halfHeight * v3Up;

	D3DXVECTOR3 centerPos = center;
	if (pMatLocal && bAttach)
	{
		D3DXVec3TransformCoord(&centerPos, &center, pMatLocal);
	}

	SParticleQuad quad{};
	quad.positions[0] = centerPos - v3Up + v3Cross;
	quad.positions[1] = centerPos - v3Up - v3Cross;
	quad.positions[2] = centerPos + v3Up + v3Cross;
	quad.positions[3] = centerPos + v3Up - v3Cross;
	return quad;
}

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
