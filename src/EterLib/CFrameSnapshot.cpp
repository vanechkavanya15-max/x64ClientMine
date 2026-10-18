#include "StdAfx.h"
#include "CFrameSnapshot.h"
#include "Camera.h"
#include "StateManager.h"
#include "RenderQueueManager.h"
#include "RenderOwnership.h"
#include <cstring>
#include <cmath>

alignas(64) TImmutableFrameSnapshot CFrameSnapshot::ms_buffers[CFrameSnapshot::BUFFER_COUNT]{};
alignas(64) std::atomic<const TImmutableFrameSnapshot*> CFrameSnapshot::ms_pFrontBuffer{ &CFrameSnapshot::ms_buffers[0] };
uint32_t CFrameSnapshot::ms_uWriteIndex = 1;
uint64_t CFrameSnapshot::ms_uSequenceCounter = 0;

void CFrameSnapshot::InitBufferDefaults(TImmutableFrameSnapshot& outSnap) noexcept
{
	std::memset(&outSnap, 0, sizeof(TImmutableFrameSnapshot));
	outSnap.matView._11 = outSnap.matView._22 = outSnap.matView._33 = outSnap.matView._44 = 1.0f;
	outSnap.matProj._11 = outSnap.matProj._22 = outSnap.matProj._33 = outSnap.matProj._44 = 1.0f;
	outSnap.matViewProj._11 = outSnap.matViewProj._22 = outSnap.matViewProj._33 = outSnap.matViewProj._44 = 1.0f;
	outSnap.matInvView._11 = outSnap.matInvView._22 = outSnap.matInvView._33 = outSnap.matInvView._44 = 1.0f;
}

void CFrameSnapshot::PackD3DLight(const D3DLIGHT9& src, bool bEnabled, TFrameLightPod& dst) noexcept
{
	dst = src;
	dst.isEnabled = bEnabled ? 1u : 0u;
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
	const D3DLIGHT9* pLights) noexcept
{
	TImmutableFrameSnapshot& backBuf = ms_buffers[ms_uWriteIndex];
	InitBufferDefaults(backBuf);

	// 1. View Matrix i Camera Eye Position
	backBuf.matView = camera.GetViewMatrix();
	backBuf.v3CameraPos = camera.GetEye();

	// 2. Inverse View Matrix
	const float fDet = D3DXMatrixDeterminant(&backBuf.matView);
	if (std::abs(fDet) > 1e-6f)
	{
		D3DXMatrixInverse(&backBuf.matInvView, nullptr, &backBuf.matView);
	}
	else
	{
		backBuf.matInvView = camera.GetInverseViewMatrix();
	}

	// 3. Perspective Projection i View-Projection
	D3DXMatrixPerspectiveFovRH(&backBuf.matProj, D3DXToRadian(fFov), fAspect, fNear, fFar);
	D3DXMatrixMultiply(&backBuf.matViewProj, &backBuf.matView, &backBuf.matProj);

	// 4. Time i Frame Index
	backBuf.fTime = fTime;
	backBuf.dwFrameIndex = dwFrameIndex;

	// 5. Fog Parameters
	if (pFogParams)
	{
		backBuf.v4FogParams = *pFogParams;
	}
	else
	{
		const float fInvRange = (fFar > fNear) ? (1.0f / (fFar - fNear)) : 0.0f;
		backBuf.v4FogParams = D3DXVECTOR4(fNear, fFar, fInvRange, 0.0f);
	}

	// 6. Lights
	if (pLights)
	{
		for (size_t i = 0; i < 3; ++i)
		{
			PackD3DLight(pLights[i], true, backBuf.lights[i]);
		}
	}
	else if (CStateManager::InstancePtr())
	{
		for (DWORD i = 0; i < 3; ++i)
		{
			D3DLIGHT9 d3dLight{};
			CStateManager::Instance().GetLight(i, &d3dLight);
			PackD3DLight(d3dLight, true, backBuf.lights[i]);
		}
	}

	// 7. Zatrzasniecie maski wlasnosci kategorii dla klatki
	uint32_t ownershipMask = 0;
	for (size_t i = 0; i < static_cast<size_t>(ERenderCategory::Count); ++i)
	{
		if (RenderOwnership::Owner(static_cast<ERenderCategory>(i)) != ERenderOwner::Legacy)
		{
			ownershipMask |= (1u << i);
		}
	}
	backBuf.dwOwnershipMask = ownershipMask;
	backBuf.uSequenceNumber = ++ms_uSequenceCounter;

	// 8. ATOMIC SWAP: Publikacja dla watku Rust WGPU z barier Release
	ms_pFrontBuffer.store(&backBuf, std::memory_order_release);
	ms_uWriteIndex ^= 1u;
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
	const D3DLIGHT9* pLights) noexcept
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

const TImmutableFrameSnapshot* CFrameSnapshot::GetPublishedPtr() noexcept
{
	return ms_pFrontBuffer.load(std::memory_order_acquire);
}

const TImmutableFrameSnapshot& CFrameSnapshot::Current() noexcept
{
	return *GetPublishedPtr();
}

void CFrameSnapshot::SetCurrent(const TImmutableFrameSnapshot& snapshot) noexcept
{
	TImmutableFrameSnapshot& backBuf = ms_buffers[ms_uWriteIndex];
	backBuf = snapshot;
	backBuf.uSequenceNumber = ++ms_uSequenceCounter;
	ms_pFrontBuffer.store(&backBuf, std::memory_order_release);
	ms_uWriteIndex ^= 1u;
}

void CFrameSnapshot::Reset() noexcept
{
	InitBufferDefaults(ms_buffers[0]);
	InitBufferDefaults(ms_buffers[1]);
	ms_uSequenceCounter = 0;
	ms_pFrontBuffer.store(&ms_buffers[0], std::memory_order_release);
	ms_uWriteIndex = 1;
}

extern "C" const TImmutableFrameSnapshot* m2_frame_snapshot_get_published() noexcept
{
	return CFrameSnapshot::GetPublishedPtr();
}
