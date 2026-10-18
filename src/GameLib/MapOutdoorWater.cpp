#include "StdAfx.h"
#include "EterLib/StateManager.h"
#include "EterLib/ResourceManager.h"
#include "EterLib/RenderQueueManager.h"
#include "EterLib/RenderOwnership.h"
#include "EterLib/RenderResourceRegistry.h"
#include "EterLib/RenderConventions.h"

#include "MapOutdoor.h"
#include "MapOutdoorWater.h"
#include "TerrainPatch.h"

void CMapOutdoor::LoadWaterTexture()
{
	UnloadWaterTexture();
	char buf[256];
	for (int i = 0; i < 30; ++i)
	{
		sprintf(buf, "d:/ymir Work/special/water/%02d.dds", i+1);
		m_WaterInstances[i].SetImagePointer((CGraphicImage *) CResourceManager::Instance().GetResourcePointer(buf));
	}
}

void CMapOutdoor::UnloadWaterTexture()
{
	for (int i = 0; i < 30; ++i)
		m_WaterInstances[i].Destroy();

	STATEMANAGER.SetTexture(0, NULL);
	STATEMANAGER.SetStreamSource(0, NULL, 0);
}

void CMapOutdoor::RenderWater()
{
	if (m_PatchVector.empty())
		return;

	if (!IsVisiblePart(PART_WATER))
		return;

	const DWORD dwCurTime = CTimer::Instance().GetCurrentMillisecond();
	const float fTimeSec = static_cast<float>(dwCurTime) * 0.001f;
	const float fWaterHeightCurrent = ComputeWaterWaveHeight(0.0f, 0.0f, fTimeSec, 1.5f, 5.0f);

	const DWORD dwWaterFrame = CalculateWaterAnimationFrameIndex(ELTimer_GetMSec());
	CGraphicImageInstance* pWaterInstance = &m_WaterInstances[dwWaterFrame];
	CGraphicTexture* pWaterTexture = pWaterInstance ? pWaterInstance->GetTexturePointer() : NULL;
	LPDIRECT3DTEXTURE9 pD3DTexture = (pWaterTexture && pWaterTexture->GetD3DTexture()) ? pWaterTexture->GetD3DTexture() : NULL;
	const TRenderTextureHandle hWaterTexture = CRenderResourceRegistry::SafeRegisterTexture(pD3DTexture, "water_anim");

	D3DXMATRIX matTexTransformWater;
	BuildWaterUVTransformMatrix(&matTexTransformWater, m_matViewInverse, m_fWaterTexCoordBase);

	const TRenderFrameUniforms& uniforms = CRenderQueueManager::Instance().GetFrameUniforms();
	const float fFogDistance = (uniforms.fFogFar > uniforms.fFogNear && uniforms.fFogNear > 0.0f) ? uniforms.fFogNear : __GetFogDistance();

	const bool bLegacyDraws = RenderOwnership::LegacyDraws(ERenderCategory::Water);

	if (bLegacyDraws)
	{
		STATEMANAGER.SaveRenderState(D3DRS_ZWRITEENABLE, FALSE);
		STATEMANAGER.SaveRenderState(D3DRS_ALPHABLENDENABLE, TRUE);
		STATEMANAGER.SaveRenderState(D3DRS_CULLMODE, D3DCULL_NONE);
		STATEMANAGER.SaveRenderState(D3DRS_DIFFUSEMATERIALSOURCE, D3DMCS_COLOR1);
		STATEMANAGER.SaveRenderState(D3DRS_COLORVERTEX, TRUE);

		if (pD3DTexture)
			STATEMANAGER.SetTexture(0, pD3DTexture);
		else
			STATEMANAGER.SetTexture(0, NULL);

		STATEMANAGER.SaveTransform(D3DTS_TEXTURE0, &matTexTransformWater);
		STATEMANAGER.SetFVF(D3DFVF_XYZ | D3DFVF_DIFFUSE);

		STATEMANAGER.SaveTextureStageState(0, D3DTSS_TEXCOORDINDEX, D3DTSS_TCI_CAMERASPACEPOSITION);
		STATEMANAGER.SaveTextureStageState(0, D3DTSS_TEXTURETRANSFORMFLAGS, D3DTTFF_COUNT2);

		STATEMANAGER.SaveSamplerState(0, D3DSAMP_MINFILTER, D3DTEXF_ANISOTROPIC);
		STATEMANAGER.SaveSamplerState(0, D3DSAMP_MAGFILTER, D3DTEXF_ANISOTROPIC);
		STATEMANAGER.SaveSamplerState(0, D3DSAMP_MIPFILTER, D3DTEXF_LINEAR);
		STATEMANAGER.SaveSamplerState(0, D3DSAMP_ADDRESSU, D3DTADDRESS_WRAP);
		STATEMANAGER.SaveSamplerState(0, D3DSAMP_ADDRESSV, D3DTADDRESS_WRAP);

		STATEMANAGER.SetTextureStageState(0, D3DTSS_COLORARG1, D3DTA_TEXTURE);
		STATEMANAGER.SetTextureStageState(0, D3DTSS_COLOROP, D3DTOP_SELECTARG1);
		STATEMANAGER.SetTextureStageState(0, D3DTSS_ALPHAARG1, D3DTA_DIFFUSE);
		STATEMANAGER.SetTextureStageState(0, D3DTSS_ALPHAOP, D3DTOP_SELECTARG1);

		STATEMANAGER.SetTexture(1, NULL);
		STATEMANAGER.SetTextureStageState(1, D3DTSS_COLOROP, D3DTOP_DISABLE);
		STATEMANAGER.SetTextureStageState(1, D3DTSS_ALPHAOP, D3DTOP_DISABLE);

		m_matWorldForCommonUse._43 = fWaterHeightCurrent;
		m_matWorldForCommonUse._41 = 0.0f;
		m_matWorldForCommonUse._42 = 0.0f;
		STATEMANAGER.SetTransform(D3DTS_WORLD, &m_matWorldForCommonUse);
	}

	for (const auto& patch : m_PatchVector)
	{
		if (patch.first < fFogDistance)
			DrawWater(patch.second, false, hWaterTexture, pD3DTexture, matTexTransformWater, fWaterHeightCurrent, fTimeSec, fFogDistance);
	}

	if (bLegacyDraws)
	{
		STATEMANAGER.SetTexture(0, NULL);
		STATEMANAGER.SetRenderState(D3DRS_ALPHABLENDENABLE, FALSE);
	}

	for (const auto& patch : m_PatchVector)
	{
		if (patch.first >= fFogDistance)
			DrawWater(patch.second, true, hWaterTexture, pD3DTexture, matTexTransformWater, fWaterHeightCurrent, fTimeSec, fFogDistance);
	}

	if (bLegacyDraws)
	{
		m_matWorldForCommonUse._43 = 0.0f;
		STATEMANAGER.SetStreamSource(0, NULL, 0);

		STATEMANAGER.RestoreTransform(D3DTS_TEXTURE0);
		STATEMANAGER.RestoreSamplerState(0, D3DSAMP_MINFILTER);
		STATEMANAGER.RestoreSamplerState(0, D3DSAMP_MAGFILTER);
		STATEMANAGER.RestoreSamplerState(0, D3DSAMP_MIPFILTER);
		STATEMANAGER.RestoreSamplerState(0, D3DSAMP_ADDRESSU);
		STATEMANAGER.RestoreSamplerState(0, D3DSAMP_ADDRESSV);
		STATEMANAGER.RestoreTextureStageState(0, D3DTSS_TEXCOORDINDEX);
		STATEMANAGER.RestoreTextureStageState(0, D3DTSS_TEXTURETRANSFORMFLAGS);

		STATEMANAGER.RestoreRenderState(D3DRS_DIFFUSEMATERIALSOURCE);
		STATEMANAGER.RestoreRenderState(D3DRS_COLORVERTEX);
		STATEMANAGER.RestoreRenderState(D3DRS_ZWRITEENABLE);
		STATEMANAGER.RestoreRenderState(D3DRS_ALPHABLENDENABLE);
		STATEMANAGER.RestoreRenderState(D3DRS_CULLMODE);
	}
}

void CMapOutdoor::DrawWater(long patchnum)
{
	if (!m_pTerrainPatchProxyList)
		return;

	const long lMaxPatches = static_cast<long>(m_wPatchCount * m_wPatchCount);
	if (patchnum < 0 || patchnum >= lMaxPatches)
		return;

	const DWORD dwCurTime = CTimer::Instance().GetCurrentMillisecond();
	const float fTimeSec = static_cast<float>(dwCurTime) * 0.001f;
	const float fWaterHeightCurrent = ComputeWaterWaveHeight(0.0f, 0.0f, fTimeSec, 1.5f, 5.0f);

	const DWORD dwWaterFrame = CalculateWaterAnimationFrameIndex(ELTimer_GetMSec());
	CGraphicImageInstance* pWaterInstance = &m_WaterInstances[dwWaterFrame];
	CGraphicTexture* pWaterTexture = pWaterInstance ? pWaterInstance->GetTexturePointer() : NULL;
	LPDIRECT3DTEXTURE9 pD3DTexture = (pWaterTexture && pWaterTexture->GetD3DTexture()) ? pWaterTexture->GetD3DTexture() : NULL;
	const TRenderTextureHandle hWaterTexture = CRenderResourceRegistry::SafeRegisterTexture(pD3DTexture, "water_anim");

	D3DXMATRIX matTexTransformWater;
	BuildWaterUVTransformMatrix(&matTexTransformWater, m_matViewInverse, m_fWaterTexCoordBase);

	DrawWater(patchnum, false, hWaterTexture, pD3DTexture, matTexTransformWater, fWaterHeightCurrent, fTimeSec, __GetFogDistance());
}

void CMapOutdoor::DrawWater(
	long patchnum,
	bool bFarPass,
	uint32_t hWaterTexture,
	LPDIRECT3DTEXTURE9 pD3DTexture,
	const D3DXMATRIX& matTexTransformWater,
	float fWaterHeightCurrent,
	float fTimeSec,
	float fFogDistance
)
{
	if (!m_pTerrainPatchProxyList)
		return;

	const long lMaxPatches = static_cast<long>(m_wPatchCount * m_wPatchCount);
	if (patchnum < 0 || patchnum >= lMaxPatches)
		return;

	CTerrainPatchProxy& rkTerrainPatchProxy = m_pTerrainPatchProxyList[patchnum];

	if (!rkTerrainPatchProxy.isUsed() || !rkTerrainPatchProxy.isWaterExists())
		return;

	CGraphicVertexBuffer* pkVB = rkTerrainPatchProxy.GetWaterVertexBufferPointer();
	if (!pkVB)
		return;

	LPDIRECT3DVERTEXBUFFER9 lpd3dVB = pkVB->GetD3DVertexBuffer();
	if (!lpd3dVB)
		return;

	UINT uPriCount = rkTerrainPatchProxy.GetWaterFaceCount();
	if (!uPriCount)
		return;

	// Emisja pakietu do CRenderQueueManager
	TWaterPacket kWaterPacket{};
	kWaterPacket.patchNum = static_cast<uint32_t>(patchnum);
	kWaterPacket.sectorX = static_cast<uint16_t>(m_wPatchCount > 0 ? (kWaterPacket.patchNum % m_wPatchCount) : 0);
	kWaterPacket.sectorY = static_cast<uint16_t>(m_wPatchCount > 0 ? (kWaterPacket.patchNum / m_wPatchCount) : 0);

	const BYTE ucTerrainNum = rkTerrainPatchProxy.GetTerrainNum();
	CTerrain* pTerrain = nullptr;
	if (GetTerrainPointer(ucTerrainNum, &pTerrain) && pTerrain)
	{
		WORD wCoordX = 0, wCoordY = 0;
		pTerrain->GetCoordinate(&wCoordX, &wCoordY);
		kWaterPacket.dwSectorX = wCoordX;
		kWaterPacket.dwSectorY = wCoordY;
	}

	kWaterPacket.fWaveTime = fTimeSec;
	kWaterPacket.fWaterHeight = fWaterHeightCurrent;
	kWaterPacket.fWaveFrequency = 1.5f;
	kWaterPacket.fWaveAmplitude = 5.0f;
	kWaterPacket.pVertexBuffer = static_cast<void*>(lpd3dVB);
	kWaterPacket.dwVertexCount = uPriCount * 3;
	kWaterPacket.dwFaceCount = uPriCount;

	kWaterPacket.bFarPass = bFarPass ? 1 : 0;
	kWaterPacket.bAlphaBlend = bFarPass ? 0 : 1;
	kWaterPacket.fFogDistance = fFogDistance;
	kWaterPacket.dwWaterTextureID = bFarPass ? kInvalidTextureHandle : hWaterTexture;
	kWaterPacket.pWaterTexture = bFarPass ? nullptr : static_cast<void*>(pD3DTexture);
	kWaterPacket.matTexTransformWater = matTexTransformWater;

	CRenderQueueManager::Instance().SubmitWater(kWaterPacket);

	if (RenderOwnership::LegacyDraws(ERenderCategory::Water))
	{
		STATEMANAGER.SetStreamSource(0, lpd3dVB, sizeof(SWaterVertex));
		STATEMANAGER.DrawPrimitive(D3DPT_TRIANGLELIST, 0, uPriCount);
	}
}


