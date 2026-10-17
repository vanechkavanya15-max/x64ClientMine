#include "StdAfx.h"
#include "CFrameSnapshot.h"
#include "Camera.h"
#include "StateManager.h"
#include "RenderQueueManager.h"
#include <cstring>
#include <cmath>

TFrameSnapshot CFrameSnapshot::ms_snapshot;

TFrameSnapshot::TFrameSnapshot()
{
	D3DXMatrixIdentity(&matView);
	D3DXMatrixIdentity(&matProj);
	D3DXMatrixIdentity(&matViewProj);
	D3DXMatrixIdentity(&matInvView);
	v3CameraPos = D3DXVECTOR3(0.0f, 0.0f, 0.0f);
	v4FogParams = D3DXVECTOR4(0.0f, 0.0f, 0.0f, 0.0f);
	std::memset(lights, 0, sizeof(lights));
	fTime = 0.0f;
	dwFrameIndex = 0;
}

void CFrameSnapshot::Capture(
	const CCamera& camera,
	float fFov,
	float fAspect,
	float fNear,
	float fFar,
	float fTime,
	uint32_t dwFrameIndex,
	const D3DXVECTOR4* pFogParams,
	const D3DLIGHT9* pLights)
{
	// 1. Camera View Matrix and Eye Position
	ms_snapshot.matView = camera.GetViewMatrix();
	ms_snapshot.v3CameraPos = camera.GetEye();

	// 2. Inverse View Matrix
	float fDet = D3DXMatrixDeterminant(&ms_snapshot.matView);
	if (std::abs(fDet) > 1e-6f)
	{
		D3DXMatrixInverse(&ms_snapshot.matInvView, nullptr, &ms_snapshot.matView);
	}
	else
	{
		ms_snapshot.matInvView = camera.GetInverseViewMatrix();
	}

	// 3. Perspective Projection Matrix
	D3DXMatrixPerspectiveFovRH(&ms_snapshot.matProj, D3DXToRadian(fFov), fAspect, fNear, fFar);

	// 4. Combined View-Projection Matrix
	D3DXMatrixMultiply(&ms_snapshot.matViewProj, &ms_snapshot.matView, &ms_snapshot.matProj);

	// 5. Time and Frame Index
	ms_snapshot.fTime = fTime;
	ms_snapshot.dwFrameIndex = dwFrameIndex;

	// 6. Fog Parameters
	if (pFogParams)
	{
		ms_snapshot.v4FogParams = *pFogParams;
	}
	else
	{
		ms_snapshot.v4FogParams = CRenderQueueManager::Instance().GetFrameUniforms().v4FogParams;
		if (ms_snapshot.v4FogParams.x == 0.0f && ms_snapshot.v4FogParams.y == 0.0f)
		{
			const float fInvRange = (fFar > fNear) ? (1.0f / (fFar - fNear)) : 0.0f;
			ms_snapshot.v4FogParams = D3DXVECTOR4(fNear, fFar, fInvRange, 0.0f);
		}
	}

	// 7. Lights
	if (pLights)
	{
		std::memcpy(ms_snapshot.lights, pLights, sizeof(ms_snapshot.lights));
	}
	else if (CStateManager::InstancePtr())
	{
		std::memset(ms_snapshot.lights, 0, sizeof(ms_snapshot.lights));
		for (DWORD i = 0; i < 3; ++i)
		{
			CStateManager::Instance().GetLight(i, &ms_snapshot.lights[i]);
		}
	}
	else
	{
		std::memset(ms_snapshot.lights, 0, sizeof(ms_snapshot.lights));
	}
}

void CFrameSnapshot::Capture(
	const CCamera* pCamera,
	float fFov,
	float fAspect,
	float fNear,
	float fFar,
	float fTime,
	uint32_t dwFrameIndex,
	const D3DXVECTOR4* pFogParams,
	const D3DLIGHT9* pLights)
{
	if (pCamera)
	{
		Capture(*pCamera, fFov, fAspect, fNear, fFar, fTime, dwFrameIndex, pFogParams, pLights);
	}
	else
	{
		CCamera dummyCam;
		Capture(dummyCam, fFov, fAspect, fNear, fFar, fTime, dwFrameIndex, pFogParams, pLights);
	}
}

const TFrameSnapshot& CFrameSnapshot::Current()
{
	return ms_snapshot;
}

void CFrameSnapshot::SetCurrent(const TFrameSnapshot& snapshot)
{
	ms_snapshot = snapshot;
}

void CFrameSnapshot::Reset()
{
	ms_snapshot = TFrameSnapshot();
}
