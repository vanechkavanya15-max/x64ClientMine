#include "StdAfx.h"
#include "ParticleQuad.h"

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
	const D3DXMATRIX* pMatLocal,
	bool bAttach,
	float fZRotationRad
)
{
	D3DXVECTOR3 v3Up(0.0f, 0.0f, 0.0f);
	D3DXVECTOR3 v3Cross(0.0f, 0.0f, 0.0f);

	if (!bStretch)
	{
		// Obsluga trybow: Type.h (ALL=1, Y=2, LIE=3, 2FACE=4, 3FACE=5)
		// oraz RenderAbi.h (ALL=0 lub 1, Y=2, LIE=3, 2FACE=4, 3FACE=5)
		const bool isLie = (byBillboardType == 3);
		const bool isY = (byBillboardType == 2 || byBillboardType == 4 || byBillboardType == 5);

		if (isLie)
		{
			const float rad = D3DXToRadian(fRotationDeg);
			const float fCos = cosf(rad);
			const float fSin = sinf(rad);

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
			if (v3Up.x * camView.y - v3Up.y * camView.x < 0.0f)
			{
				v3Up *= -1.0f;
			}

			const D3DXVECTOR3 d3dd(camView.x, camView.y, 0.0f);
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
				const float fCos = -sinf(rad); // + 90 deg w legacy
				const float fSin = cosf(rad);

				const D3DXVECTOR3 v3Temp = v3Up * fCos - v3Cross * fSin;
				v3Cross = v3Cross * fCos + v3Up * fSin;
				v3Up = v3Temp;
			}
		}
		else // BILLBOARD_TYPE_ALL i default
		{
			if (fRotationDeg == 0.0f)
			{
				v3Up = -camCross;
				v3Cross = camUp;
			}
			else
			{
				D3DXMATRIX matRotation;
				D3DXMatrixRotationAxis(&matRotation, &camView, D3DXToRadian(fRotationDeg));
				const D3DXVECTOR3 negCross = -camCross;
				D3DXVec3TransformCoord(&v3Up, &negCross, &matRotation);
				D3DXVec3TransformCoord(&v3Cross, &camUp, &matRotation);
			}
		}
	}
	else // Stretch mode
	{
		v3Up = velocity;

		if (pMatLocal)
		{
			D3DXVec3TransformNormal(&v3Up, &v3Up, pMatLocal);
		}

		const float length = D3DXVec3Length(&v3Up);
		if (length == 0.0f)
		{
			v3Up = D3DXVECTOR3(0.0f, 0.0f, 1.0f);
		}
		else
		{
			v3Up *= (1.0f + logf(1.0f + length)) / length;
		}

		D3DXVec3Cross(&v3Cross, &v3Up, &camView);
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
		const float fCos = cosf(fZRotationRad);
		const float fSin = sinf(fZRotationRad);

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

	outPositions[0] = centerPos - v3Up + v3Cross;
	outPositions[1] = centerPos - v3Up - v3Cross;
	outPositions[2] = centerPos + v3Up + v3Cross;
	outPositions[3] = centerPos + v3Up - v3Cross;
}
