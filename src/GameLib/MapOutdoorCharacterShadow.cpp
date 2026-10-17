#include "StdAfx.h"
#include "EterLib/StateManager.h"
#include "EterLib/Camera.h"
#include "EterLib/RenderOwnership.h"
#include "EterLib/RenderResourceRegistry.h"
#include "EterLib/RenderPacketTypes.h"

#include "MapOutdoor.h"
#include "MapOutdoorCharacterShadow.h"

void CMapOutdoor::SetShadowTextureSize(WORD size)
{
	if (m_wShadowMapSize != size)
	{
		m_bRecreateShadowTexture = true;
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

	LPDIRECT3DDEVICE9EX pDevice = STATEMANAGER.GetDevice();
	if (!pDevice)
	{
		TraceError("CMapOutdoor: D3D Device unavailable for Character Shadow RT\n");
		return;
	}

	if (FAILED(pDevice->CreateTexture(m_wShadowMapSize, m_wShadowMapSize, 1, D3DUSAGE_RENDERTARGET, D3DFMT_R5G6B5, D3DPOOL_DEFAULT, &m_lpCharacterShadowMapTexture, nullptr)))
	{
		TraceError("CMapOutdoor Unable to create Character Shadow render target texture\n");
		return;
	}

	if (FAILED(m_lpCharacterShadowMapTexture->GetSurfaceLevel(0, &m_lpCharacterShadowMapRenderTargetSurface)))
	{
		TraceError("CMapOutdoor Unable to GetSurfaceLevel Character Shadow render target texture\n");
		return;
	}

	if (FAILED(pDevice->CreateDepthStencilSurface(m_wShadowMapSize, m_wShadowMapSize, D3DFMT_D16, D3DMULTISAMPLE_NONE, 0, TRUE, &m_lpCharacterShadowMapDepthSurface, nullptr)))
	{
		TraceError("CMapOutdoor Unable to create Character Shadow depth Surface\n");
		return;
	}

	if (m_lpCharacterShadowMapTexture && CRenderResourceRegistry::InstancePtr())
	{
		m_hCharacterShadowTexture = CRenderResourceRegistry::Instance().RegisterTextureWithInfo(
			m_lpCharacterShadowMapTexture, m_wShadowMapSize, m_wShadowMapSize, D3DFMT_R5G6B5, "CharacterShadowRT");
	}
}

void CMapOutdoor::ReleaseCharacterShadowTexture()
{
	if (m_hCharacterShadowTexture != 0 && CRenderResourceRegistry::InstancePtr())
	{
		CRenderResourceRegistry::Instance().UnregisterTexture(m_hCharacterShadowTexture);
		m_hCharacterShadowTexture = 0;
	}

	SAFE_RELEASE(m_lpCharacterShadowMapRenderTargetSurface);
	SAFE_RELEASE(m_lpCharacterShadowMapDepthSurface);
	SAFE_RELEASE(m_lpCharacterShadowMapTexture);
}

bool CMapOutdoor::BeginRenderCharacterShadowToTexture()
{
	if (!RenderOwnership::LegacyDraws(ERenderCategory::Shadow))
		return false;

	CCamera* pCurrentCamera = CCameraManager::Instance().GetCurrentCamera();

	if (!pCurrentCamera)
		return false;

	if (m_bRecreateShadowTexture)
	{
		CreateCharacterShadowTexture();
		m_bRecreateShadowTexture = false;
	}

	D3DXVECTOR3 v3Target = pCurrentCamera->GetTarget();
	D3DXMATRIX matLightView, matLightProj;
	CalculateLightViewMatrix(&matLightView, kDefaultCharacterShadowLightDir, v3Target, 5000.0f);
	CalculateLightProjMatrix(&matLightProj, 2550.0f, 2550.0f, 1.0f, 15000.0f);

	m_characterShadowPassDesc.passType = RENDER_PASS_CHARACTER_SHADOW;
	m_characterShadowPassDesc.targetHandle = m_hCharacterShadowTexture;
	m_characterShadowPassDesc.clearColor = 0xFFFFFFFF;
	m_characterShadowPassDesc.bClearTarget = 1;
	m_characterShadowPassDesc.matView = ToAbi(matLightView);
	m_characterShadowPassDesc.matProj = ToAbi(matLightProj);

	STATEMANAGER.SaveTransform(D3DTS_VIEW, &matLightView);
	STATEMANAGER.SaveTransform(D3DTS_PROJECTION, &matLightProj);

	m_dwShadowLightEnable = STATEMANAGER.GetRenderState(D3DRS_LIGHTING);
	STATEMANAGER.SetRenderState(D3DRS_LIGHTING, FALSE);
	
	STATEMANAGER.SaveRenderState(D3DRS_TEXTUREFACTOR, 0xFF808080);
	STATEMANAGER.SaveTextureStageState(0, D3DTSS_COLOROP, D3DTOP_SELECTARG1);
	STATEMANAGER.SaveTextureStageState(0, D3DTSS_COLORARG1, D3DTA_TFACTOR);
	STATEMANAGER.SaveTextureStageState(0, D3DTSS_ALPHAOP, D3DTOP_DISABLE);
	STATEMANAGER.SaveTextureStageState(1, D3DTSS_COLOROP, D3DTOP_DISABLE);
	
	LPDIRECT3DDEVICE9EX pDevice = STATEMANAGER.GetDevice();
	if (!pDevice)
		return false;

	pDevice->GetDepthStencilSurface(&m_lpBackupDepthSurface);
	pDevice->GetRenderTarget(0, &m_lpBackupRenderTargetSurface);

	pDevice->SetRenderTarget(0, m_lpCharacterShadowMapRenderTargetSurface);
	pDevice->SetDepthStencilSurface(m_lpCharacterShadowMapDepthSurface);
	
	pDevice->Clear(0, NULL, D3DCLEAR_TARGET | D3DCLEAR_ZBUFFER, m_characterShadowPassDesc.clearColor, 1.0f, 0);
	
	STATEMANAGER.GetViewport(&m_BackupViewport);
	STATEMANAGER.SetViewport(&m_ShadowMapViewport);
	
	return true;
}

void CMapOutdoor::EndRenderCharacterShadowToTexture()
{
	if (!RenderOwnership::LegacyDraws(ERenderCategory::Shadow))
		return;

	STATEMANAGER.SetViewport(&m_BackupViewport);

	LPDIRECT3DDEVICE9EX pDevice = STATEMANAGER.GetDevice();
	if (pDevice)
	{
		pDevice->SetDepthStencilSurface(m_lpBackupDepthSurface);
		pDevice->SetRenderTarget(0, m_lpBackupRenderTargetSurface);
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
	STATEMANAGER.SetRenderState(D3DRS_LIGHTING, m_dwShadowLightEnable);
	STATEMANAGER.RestoreRenderState(D3DRS_TEXTUREFACTOR);
}

void CMapOutdoor::GetShadowPassData(TShadowPassData* pOutData) const
{
	if (!pOutData)
		return;

	pOutData->passDesc = m_characterShadowPassDesc;
	pOutData->matDynamicShadow = ToAbi(m_matDynamicShadow);
	pOutData->v3LightDir = ToAbi(kDefaultCharacterShadowLightDir);
	pOutData->fRange = 5000.0f;

	CCamera* pCurrentCamera = CCameraManager::Instance().GetCurrentCamera();
	if (pCurrentCamera)
	{
		D3DXVECTOR3 v3Target = pCurrentCamera->GetTarget();
		pOutData->v3TargetPos = ToAbi(v3Target);
	}
	else
	{
		pOutData->v3TargetPos = TRenderVec3{ 0.0f, 0.0f, 0.0f };
	}

	pOutData->wShadowMapSize = m_wShadowMapSize;
	pOutData->wPad = 0;
	pOutData->hRenderTarget = m_hCharacterShadowTexture;
	pOutData->viewport.left = m_ShadowMapViewport.X;
	pOutData->viewport.top = m_ShadowMapViewport.Y;
	pOutData->viewport.right = m_ShadowMapViewport.X + m_ShadowMapViewport.Width;
	pOutData->viewport.bottom = m_ShadowMapViewport.Y + m_ShadowMapViewport.Height;
}
