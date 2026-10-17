#pragma once

#include <d3d9.h>
#include <d3dx9.h>
#include <cmath>

const D3DXVECTOR3 kDefaultCharacterShadowLightDir(1.732f, 1.0f, -3.464f);

inline void CalculateLightViewMatrix(D3DXMATRIX* pOutMat, const D3DXVECTOR3& v3LightDir, const D3DXVECTOR3& v3TargetPos, float fRange = 5000.0f)
{
	if (!pOutMat)
		return;

	D3DXVECTOR3 v3NormLightDir;
	float fLengthSq = D3DXVec3LengthSq(&v3LightDir);
	if (fLengthSq > 1e-6f)
	{
		D3DXVec3Normalize(&v3NormLightDir, &v3LightDir);
	}
	else
	{
		const D3DXVECTOR3 v3DefaultLight(1.732f, 1.0f, -3.464f);
		D3DXVec3Normalize(&v3NormLightDir, &v3DefaultLight);
	}

	D3DXVECTOR3 v3Eye = v3TargetPos - v3NormLightDir * fRange;

	D3DXVECTOR3 v3Up(0.0f, 0.0f, 1.0f);
	if (fabsf(D3DXVec3Dot(&v3NormLightDir, &v3Up)) > 0.999f)
	{
		v3Up = D3DXVECTOR3(0.0f, 1.0f, 0.0f);
	}

	D3DXMatrixLookAtRH(pOutMat, &v3Eye, &v3TargetPos, &v3Up);
}

inline void BuildCharacterShadowMatrix(D3DXMATRIX* pOutMat, const D3DXVECTOR3& v3LightDir, const D3DXVECTOR3& v3TargetPos, float fRange = 5000.0f)
{
	CalculateLightViewMatrix(pOutMat, v3LightDir, v3TargetPos, fRange);
}

inline void CalculateLightProjMatrix(D3DXMATRIX* pOutMat, float fWidth = 2550.0f, float fHeight = 2550.0f, float fNear = 1.0f, float fFar = 15000.0f)
{
	if (!pOutMat)
		return;
	D3DXMatrixOrthoRH(pOutMat, fWidth, fHeight, fNear, fFar);
}

inline void CalculateDynamicShadowMatrix(D3DXMATRIX* pOutDynamicShadow, const D3DXMATRIX& matViewInverse, const D3DXMATRIX& matLightView, const D3DXMATRIX& matShadowScale)
{
	if (!pOutDynamicShadow)
		return;
	*pOutDynamicShadow = matViewInverse * matLightView * matShadowScale;
}
