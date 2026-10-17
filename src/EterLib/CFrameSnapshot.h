#pragma once

#include <d3d9.h>
#include <d3dx9.h>
#include <cstdint>

class CCamera;

struct TFrameSnapshot
{
	D3DXMATRIX matView;
	D3DXMATRIX matProj;
	D3DXMATRIX matViewProj;
	D3DXMATRIX matInvView;
	D3DXVECTOR3 v3CameraPos;
	D3DXVECTOR4 v4FogParams;
	D3DLIGHT9 lights[3];
	float fTime;
	uint32_t dwFrameIndex;

	TFrameSnapshot();
};

class CFrameSnapshot
{
public:
	static void Capture(
		const CCamera& camera,
		float fFov,
		float fAspect,
		float fNear,
		float fFar,
		float fTime = 0.0f,
		uint32_t dwFrameIndex = 0,
		const D3DXVECTOR4* pFogParams = nullptr,
		const D3DLIGHT9* pLights = nullptr
	);

	static void Capture(
		const CCamera* pCamera,
		float fFov,
		float fAspect,
		float fNear,
		float fFar,
		float fTime = 0.0f,
		uint32_t dwFrameIndex = 0,
		const D3DXVECTOR4* pFogParams = nullptr,
		const D3DLIGHT9* pLights = nullptr
	);

	static const TFrameSnapshot& Current();
	static void SetCurrent(const TFrameSnapshot& snapshot);
	static void Reset();

private:
	static TFrameSnapshot ms_snapshot;
};
