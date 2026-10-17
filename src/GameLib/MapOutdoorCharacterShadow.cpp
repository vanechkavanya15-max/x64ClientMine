#include "StdAfx.h"
#include "EterLib/StateManager.h"
#include "EterLib/Camera.h"
#include "EterLib/RenderOwnership.h"

#include "MapOutdoor.h"
#include "MapOutdoorCharacterShadow.h"

static int recreate = false;

void CMapOutdoor::SetShadowTextureSize(WORD size)
{
	if (m_wShadowMapSize != size)
	{
		recreate = true;
		Tracenf("ShadowTextureSize changed %d -> %d", m_wShadowMapSize, size);
	}

	m_wShadowMapSize = size;
}

void CMapOutdoor::CreateCharacterShadowTexture()
{
	extern bool GRAPHICS_CAPS_CAN_NOT_DRAW_SHADOW;

	if (GRAPHICS_CAPS_CAN_NOT_DRAW_SHADOW)
		return;

	ReleaseCharacterShadowTexture();

	if (IsLowTextureMemory())
		SetShadowTextureSize(128);

	m_ShadowMapViewport.X = 1;
	m_ShadowMapViewport.Y = 1;
	m_ShadowMapViewport.Width = m_wShadowMapSize - 2;
	m_ShadowMapViewport.Height = m_wShadowMapSize - 2;
	m_ShadowMapViewport.MinZ = 0.0f;
	m_ShadowMapViewport.MaxZ = 1.0f;

	LPDIRECT3DDEVICE9 lpDevice = STATEMANAGER.GetDevice();
	if (!lpDevice)
		return;

	if (FAILED(lpDevice->CreateTexture(m_wShadowMapSize, m_wShadowMapSize, 1, D3DUSAGE_RENDERTARGET, D3DFMT_R5G6B5, D3DPOOL_DEFAULT, &m_lpCharacterShadowMapTexture, nullptr)))
	{
		TraceError("CMapOutdoor Unable to create Character Shadow render target texture\n");
		return;
	}

	if (FAILED(m_lpCharacterShadowMapTexture->GetSurfaceLevel(0, &m_lpCharacterShadowMapRenderTargetSurface)))
	{
		TraceError("CMapOutdoor Unable to GetSurfaceLevel Character Shadow render target texture\n");
		return;
	}

	if (FAILED(lpDevice->CreateDepthStencilSurface(m_wShadowMapSize, m_wShadowMapSize, D3DFMT_D16, D3DMULTISAMPLE_NONE, 0, TRUE, &m_lpCharacterShadowMapDepthSurface, nullptr)))
	{
		TraceError("CMapOutdoor Unable to create Character Shadow depth Surface\n");
		return;
	}
}

void CMapOutdoor::ReleaseCharacterShadowTexture()
{
	SAFE_RELEASE(m_lpCharacterShadowMapRenderTargetSurface);
	SAFE_RELEASE(m_lpCharacterShadowMapDepthSurface);
	SAFE_RELEASE(m_lpCharacterShadowMapTexture);
}

DWORD dwLightEnable = FALSE;

bool CMapOutdoor::BeginRenderCharacterShadowToTexture()
{
	if (!RenderOwnership::LegacyDraws(ERenderCategory::Shadow))
		return false;

	D3DXMATRIX matLightView, matLightProj;
	
	CCamera* pCurrentCamera = CCameraManager::Instance().GetCurrentCamera();

	if (!pCurrentCamera)
		return false;

	if (recreate)
	{
		CreateCharacterShadowTexture();
		recreate = false;
	}

	D3DXVECTOR3 v3Target = pCurrentCamera->GetTarget();
	BuildCharacterShadowMatrix(&matLightView, kDefaultCharacterShadowLightDir, v3Target, 5000.0f);
	
	D3DXMatrixOrthoRH(&matLightProj, 2550.0f, 2550.0f, 1.0f, 15000.0f);

	STATEMANAGER.SaveTransform(D3DTS_VIEW, &matLightView);
	STATEMANAGER.SaveTransform(D3DTS_PROJECTION, &matLightProj);

	dwLightEnable = STATEMANAGER.GetRenderState(D3DRS_LIGHTING);
	STATEMANAGER.SetRenderState(D3DRS_LIGHTING, FALSE);
	
	STATEMANAGER.SaveRenderState(D3DRS_TEXTUREFACTOR, 0xFF808080);
	STATEMANAGER.SaveTextureStageState(0, D3DTSS_COLOROP, D3DTOP_SELECTARG1);
	STATEMANAGER.SaveTextureStageState(0, D3DTSS_COLORARG1, D3DTA_TFACTOR);
	STATEMANAGER.SaveTextureStageState(0, D3DTSS_ALPHAOP, D3DTOP_DISABLE);
	STATEMANAGER.SaveTextureStageState(1, D3DTSS_COLOROP, D3DTOP_DISABLE);
	
	LPDIRECT3DDEVICE9 lpDevice = STATEMANAGER.GetDevice();
	if (!lpDevice)
		return false;

	lpDevice->GetDepthStencilSurface(&m_lpBackupDepthSurface);
	lpDevice->GetRenderTarget(0, &m_lpBackupRenderTargetSurface);

	lpDevice->SetRenderTarget(0, m_lpCharacterShadowMapRenderTargetSurface);
	lpDevice->SetDepthStencilSurface(m_lpCharacterShadowMapDepthSurface);
	
	lpDevice->Clear(0, NULL, D3DCLEAR_TARGET | D3DCLEAR_ZBUFFER, 0xFFFFFFFF, 1.0f, 0);
	
	STATEMANAGER.GetViewport(&m_BackupViewport);
	STATEMANAGER.SetViewport(&m_ShadowMapViewport);
	
	return true;
}

void CMapOutdoor::EndRenderCharacterShadowToTexture()
{
	if (!RenderOwnership::LegacyDraws(ERenderCategory::Shadow))
		return;

	STATEMANAGER.SetViewport(&m_BackupViewport);

	LPDIRECT3DDEVICE9 lpDevice = STATEMANAGER.GetDevice();
	if (lpDevice)
	{
		lpDevice->SetDepthStencilSurface(m_lpBackupDepthSurface);
		lpDevice->SetRenderTarget(0, m_lpBackupRenderTargetSurface);
	}

	SAFE_RELEASE(m_lpBackupRenderTargetSurface);
	SAFE_RELEASE(m_lpBackupDepthSurface);

	STATEMANAGER.RestoreTransform(D3DTS_VIEW);
	STATEMANAGER.RestoreTransform(D3DTS_PROJECTION);

	// Restore Device Context
	STATEMANAGER.RestoreTextureStageState(1, D3DTSS_COLOROP);
	STATEMANAGER.RestoreTextureStageState(0, D3DTSS_ALPHAOP);
	STATEMANAGER.RestoreTextureStageState(0, D3DTSS_COLORARG1);
	STATEMANAGER.RestoreTextureStageState(0, D3DTSS_COLOROP);
	STATEMANAGER.SetRenderState(D3DRS_LIGHTING, dwLightEnable);
	STATEMANAGER.RestoreRenderState(D3DRS_TEXTUREFACTOR);
}
