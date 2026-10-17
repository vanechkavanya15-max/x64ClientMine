#include "StdAfx.h"
#include "MapOutdoor.h"

#include "EterLib/StateManager.h"
#include "EterLib/RenderOwnership.h"
#include "EterLib/RenderQueueManager.h"

void CMapOutdoor::__RenderTerrain_RenderHardwareTransformPatch()
{
	const bool bLegacyDraws = RenderOwnership::LegacyDraws(ERenderCategory::Terrain);

	// Bezwzgledna gwarancja RAII: czyszczenie shaderow/deklaracji oraz ustawienie FVF gdy legacy
	std::unique_ptr<ScopedD3DFFPGuard> htpFFPGuard;
	if (bLegacyDraws)
		htpFFPGuard = std::make_unique<ScopedD3DFFPGuard>(D3DFVF_XYZ | D3DFVF_NORMAL, false);

	const TRenderFrameUniforms& uniforms = CRenderQueueManager::Instance().GetFrameUniforms();
	DWORD dwFogColor;
	float fFogFarDistance;
	float fFogNearDistance;
	if (uniforms.fFogFar > uniforms.fFogNear && uniforms.fFogFar > 0.0f)
	{
		dwFogColor = D3DCOLOR_COLORVALUE(uniforms.vFogColor.r, uniforms.vFogColor.g, uniforms.vFogColor.b, uniforms.vFogColor.a);
		fFogNearDistance = uniforms.fFogNear;
		fFogFarDistance = uniforms.fFogFar;
	}
	else if (mc_pEnvironmentData)
	{
		dwFogColor = mc_pEnvironmentData->FogColor;
		fFogNearDistance = mc_pEnvironmentData->GetFogNearDistance();
		fFogFarDistance = mc_pEnvironmentData->GetFogFarDistance();
	}
	else
	{
		dwFogColor = 0xffffffff;
		fFogNearDistance = 5000.0f;
		fFogFarDistance = 10000.0f;
	}

	if (bLegacyDraws)
	{
		//////////////////////////////////////////////////////////////////////////
		// Render State & TextureStageState	

		STATEMANAGER.SaveTextureStageState(0, D3DTSS_TEXCOORDINDEX, D3DTSS_TCI_CAMERASPACEPOSITION);
		STATEMANAGER.SaveTextureStageState(0, D3DTSS_TEXTURETRANSFORMFLAGS, D3DTTFF_COUNT2);
		STATEMANAGER.SaveTextureStageState(1, D3DTSS_TEXCOORDINDEX, D3DTSS_TCI_CAMERASPACEPOSITION);
		STATEMANAGER.SaveTextureStageState(1, D3DTSS_TEXTURETRANSFORMFLAGS, D3DTTFF_COUNT2);

		STATEMANAGER.SaveRenderState(D3DRS_ALPHABLENDENABLE, TRUE);
		STATEMANAGER.SaveRenderState(D3DRS_ALPHATESTENABLE, TRUE);
		STATEMANAGER.SaveRenderState(D3DRS_ALPHAREF, 0x00000000);
		STATEMANAGER.SaveRenderState(D3DRS_ALPHAFUNC, D3DCMP_GREATER);

		STATEMANAGER.SaveRenderState(D3DRS_TEXTUREFACTOR, dwFogColor);

		STATEMANAGER.SetTextureStageState(0, D3DTSS_COLORARG1, D3DTA_TEXTURE);
		STATEMANAGER.SetTextureStageState(0, D3DTSS_COLORARG2, D3DTA_CURRENT);
		STATEMANAGER.SetTextureStageState(0, D3DTSS_COLOROP,   D3DTOP_MODULATE);
		STATEMANAGER.SetTextureStageState(0, D3DTSS_ALPHAARG1, D3DTA_TEXTURE);
		STATEMANAGER.SetTextureStageState(0, D3DTSS_ALPHAOP,   D3DTOP_SELECTARG1);
		STATEMANAGER.SetSamplerState(0, D3DSAMP_ADDRESSU, D3DTADDRESS_WRAP);
		STATEMANAGER.SetSamplerState(0, D3DSAMP_ADDRESSV, D3DTADDRESS_WRAP);

		STATEMANAGER.SetTextureStageState(1, D3DTSS_COLORARG1, D3DTA_CURRENT);
		STATEMANAGER.SetTextureStageState(1, D3DTSS_COLOROP,   D3DTOP_SELECTARG1);
		STATEMANAGER.SetTextureStageState(1, D3DTSS_ALPHAARG1, D3DTA_TEXTURE);
		STATEMANAGER.SetTextureStageState(1, D3DTSS_ALPHAOP,   D3DTOP_SELECTARG1);
		STATEMANAGER.SetSamplerState(1, D3DSAMP_ADDRESSU, D3DTADDRESS_CLAMP);
		STATEMANAGER.SetSamplerState(1, D3DSAMP_ADDRESSV, D3DTADDRESS_CLAMP);

		CSpeedTreeWrapper::ms_bSelfShadowOn = true;
		STATEMANAGER.SetBestFiltering(0);
		STATEMANAGER.SetBestFiltering(1);

		m_matWorldForCommonUse._41 = 0.0f;
		m_matWorldForCommonUse._42 = 0.0f;
		STATEMANAGER.SetTransform(D3DTS_WORLD, &m_matWorldForCommonUse);

		STATEMANAGER.SaveTransform(D3DTS_TEXTURE0, &m_matWorldForCommonUse);
		STATEMANAGER.SaveTransform(D3DTS_TEXTURE1, &m_matWorldForCommonUse);

		STATEMANAGER.SetFVF(D3DFVF_XYZ | D3DFVF_NORMAL);
	}

	m_TerrainRenderExtractor.BeginFrame();

	m_iRenderedSplatNumSqSum = 0;
	m_iRenderedPatchNum = 0;
	m_iRenderedSplatNum = 0;
	m_RenderedTextureNumVector.clear();

	std::pair<float, long> fog_far(fFogFarDistance + 1600.0f, 0);
	std::pair<float, long> fog_near(fFogNearDistance - 3200.0f, 0);

	if (mc_pEnvironmentData && mc_pEnvironmentData->bDensityFog)
		fog_far.first = 1e10f;

	auto far_it = std::upper_bound(m_PatchVector.begin(), m_PatchVector.end(), fog_far);
	auto near_it = std::upper_bound(m_PatchVector.begin(), m_PatchVector.end(), fog_near);

	WORD wPrimitiveCount = 0;
	D3DPRIMITIVETYPE ePrimitiveType = D3DPT_TRIANGLELIST;

	BYTE byCUrrentLODLevel = 0;

	float fLODLevel1Distance = __GetNoFogDistance();
	float fLODLevel2Distance = __GetFogDistance();

	SelectIndexBuffer(0, &wPrimitiveCount, &ePrimitiveType);

	auto it = m_PatchVector.begin();

	// Pasmo 1: Near Band (odleglosc ponizej fog_near)
	for (; it != near_it; ++it)
	{
		if (byCUrrentLODLevel == 0 && fLODLevel1Distance <= it->first)
		{
			byCUrrentLODLevel = 1;
			SelectIndexBuffer(1, &wPrimitiveCount, &ePrimitiveType);
		}
		else if (byCUrrentLODLevel == 1 && fLODLevel2Distance <= it->first)
		{
			byCUrrentLODLevel = 2;
			SelectIndexBuffer(2, &wPrimitiveCount, &ePrimitiveType);
		}

		__HardwareTransformPatch_RenderPatchSplat(it->second, wPrimitiveCount, ePrimitiveType, byCUrrentLODLevel, TERRAIN_BAND_NEAR, it->first);

		if (m_iRenderedSplatNum >= m_iSplatLimit)
			break;

		if (bLegacyDraws && m_bDrawWireFrame)
			DrawWireFrame(it->second, wPrimitiveCount, ePrimitiveType);
	}

	// Pasmo 2: Fog Band (odleglosc miedzy fog_near a fog_far)
	if (m_iRenderedSplatNum < m_iSplatLimit)
	{
		for (it = near_it; it != far_it; ++it)
		{
			if (byCUrrentLODLevel == 0 && fLODLevel1Distance <= it->first)
			{
				byCUrrentLODLevel = 1;
				SelectIndexBuffer(1, &wPrimitiveCount, &ePrimitiveType);
			}
			else if (byCUrrentLODLevel == 1 && fLODLevel2Distance <= it->first)
			{
				byCUrrentLODLevel = 2;
				SelectIndexBuffer(2, &wPrimitiveCount, &ePrimitiveType);
			}

			__HardwareTransformPatch_RenderPatchSplat(it->second, wPrimitiveCount, ePrimitiveType, byCUrrentLODLevel, TERRAIN_BAND_FOG, it->first);

			if (m_iRenderedSplatNum >= m_iSplatLimit)
				break;

			if (bLegacyDraws && m_bDrawWireFrame)
				DrawWireFrame(it->second, wPrimitiveCount, ePrimitiveType);
		}
	}

	if (bLegacyDraws)
	{
		STATEMANAGER.SetRenderState(D3DRS_LIGHTING, FALSE);

		STATEMANAGER.SetTexture(0, NULL);
		STATEMANAGER.SetTextureStageState(0, D3DTSS_TEXTURETRANSFORMFLAGS, FALSE);

		STATEMANAGER.SetTexture(1, NULL);
		STATEMANAGER.SetTextureStageState(1, D3DTSS_TEXTURETRANSFORMFLAGS, FALSE);

		STATEMANAGER.SetTextureStageState(0, D3DTSS_COLORARG1, D3DTA_TFACTOR);
		STATEMANAGER.SetTextureStageState(0, D3DTSS_COLOROP,   D3DTOP_SELECTARG1);
		STATEMANAGER.SetTextureStageState(0, D3DTSS_ALPHAOP,   D3DTOP_DISABLE);

		STATEMANAGER.SetTextureStageState(1, D3DTSS_COLOROP,   D3DTOP_DISABLE);
		STATEMANAGER.SetTextureStageState(1, D3DTSS_ALPHAOP,   D3DTOP_DISABLE);
	}

	// Pasmo 3: Far Band (powyzej fog_far) - rysowanie / ekstrakcja bez splattingu (RenderPatchNone)
	if (m_iRenderedSplatNum < m_iSplatLimit)
	{
		for (it = far_it; it != m_PatchVector.end(); ++it)
		{
			if (byCUrrentLODLevel == 0 && fLODLevel1Distance <= it->first)
			{
				byCUrrentLODLevel = 1;
				SelectIndexBuffer(1, &wPrimitiveCount, &ePrimitiveType);
			}
			else if (byCUrrentLODLevel == 1 && fLODLevel2Distance <= it->first)
			{
				byCUrrentLODLevel = 2;
				SelectIndexBuffer(2, &wPrimitiveCount, &ePrimitiveType);
			}

			__HardwareTransformPatch_RenderPatchNone(it->second, wPrimitiveCount, ePrimitiveType, byCUrrentLODLevel, it->first);

			if (m_iRenderedSplatNum >= m_iSplatLimit)
				break;

			if (bLegacyDraws && m_bDrawWireFrame)
				DrawWireFrame(it->second, wPrimitiveCount, ePrimitiveType);
		}
	}

	if (bLegacyDraws)
	{
		STATEMANAGER.SetTextureStageState(0, D3DTSS_COLORARG1, D3DTA_TEXTURE);
		STATEMANAGER.SetTextureStageState(0, D3DTSS_COLORARG2, D3DTA_CURRENT);
		STATEMANAGER.SetTextureStageState(0, D3DTSS_COLOROP,   D3DTOP_MODULATE);
		STATEMANAGER.SetTextureStageState(0, D3DTSS_ALPHAARG1, D3DTA_TEXTURE);
		STATEMANAGER.SetTextureStageState(0, D3DTSS_ALPHAOP,   D3DTOP_SELECTARG1);

		STATEMANAGER.SetTextureStageState(1, D3DTSS_COLORARG1, D3DTA_CURRENT);
		STATEMANAGER.SetTextureStageState(1, D3DTSS_COLOROP,   D3DTOP_SELECTARG1);
		STATEMANAGER.SetTextureStageState(1, D3DTSS_ALPHAARG1, D3DTA_TEXTURE);
		STATEMANAGER.SetTextureStageState(1, D3DTSS_ALPHAOP,   D3DTOP_SELECTARG1);

		STATEMANAGER.SetRenderState(D3DRS_LIGHTING, TRUE);

		std::sort(m_RenderedTextureNumVector.begin(), m_RenderedTextureNumVector.end());

		//////////////////////////////////////////////////////////////////////////
		// Render State & TextureStageState

		STATEMANAGER.RestoreRenderState(D3DRS_TEXTUREFACTOR);

		STATEMANAGER.RestoreTransform(D3DTS_TEXTURE0);
		STATEMANAGER.RestoreTransform(D3DTS_TEXTURE1);

		STATEMANAGER.RestoreTextureStageState(0, D3DTSS_TEXCOORDINDEX);
		STATEMANAGER.RestoreTextureStageState(0, D3DTSS_TEXTURETRANSFORMFLAGS);
		STATEMANAGER.RestoreTextureStageState(1, D3DTSS_TEXCOORDINDEX);
		STATEMANAGER.RestoreTextureStageState(1, D3DTSS_TEXTURETRANSFORMFLAGS);

		STATEMANAGER.RestoreRenderState(D3DRS_ALPHABLENDENABLE);
		STATEMANAGER.RestoreRenderState(D3DRS_ALPHATESTENABLE);
		STATEMANAGER.RestoreRenderState(D3DRS_ALPHAREF);
		STATEMANAGER.RestoreRenderState(D3DRS_ALPHAFUNC);
	}
}

void CMapOutdoor::__HardwareTransformPatch_RenderPatchSplat(
	long patchnum,
	WORD wPrimitiveCount,
	D3DPRIMITIVETYPE ePrimitiveType,
	BYTE byLODLevel,
	BYTE byBand,
	float fDistance)
{
	assert(NULL != m_pTerrainPatchProxyList && "__HardwareTransformPatch_RenderPatchSplat");
	CTerrainPatchProxy* pTerrainPatchProxy = &m_pTerrainPatchProxyList[patchnum];

	if (!pTerrainPatchProxy->isUsed())
		return;

	long sPatchNum = pTerrainPatchProxy->GetPatchNum();
	if (sPatchNum < 0)
		return;

	BYTE ucTerrainNum = pTerrainPatchProxy->GetTerrainNum();
	if (0xFF == ucTerrainNum)
		return;

	CTerrain* pTerrain = nullptr;
	if (!GetTerrainPointer(ucTerrainNum, &pTerrain) || !pTerrain)
		return;

	CGraphicVertexBuffer* pkVB = pTerrainPatchProxy->HardwareTransformPatch_GetVertexBufferPtr();
	if (!pkVB)
		return;

	LPDIRECT3DVERTEXBUFFER9 pD3DVB = pkVB->GetD3DVertexBuffer();
	if (!pD3DVB)
		return;

	// Ekstrakcja TTerrainSectorPacket do CRenderQueueManager (zero-allocation)
	TTerrainSectorPacket sectorPacket{};
	m_TerrainRenderExtractor.ExtractAndSubmitPatch(
		this,
		static_cast<uint32_t>(patchnum),
		byLODLevel,
		byBand,
		wPrimitiveCount,
		ePrimitiveType,
		fDistance,
		sectorPacket
	);

	const bool bLegacyDraws = RenderOwnership::LegacyDraws(ERenderCategory::Terrain);

	WORD wCoordX, wCoordY;
	pTerrain->GetCoordinate(&wCoordX, &wCoordY);

	TTerrainSplatPatch& rTerrainSplatPatch = pTerrain->GetTerrainSplatPatch();

	D3DXMATRIX matTexTransform, matSplatAlphaTexTransform, matSplatColorTexTransform;
	m_matWorldForCommonUse._41 = -(float)(wCoordX * CTerrainImpl::TERRAIN_XSIZE);
	m_matWorldForCommonUse._42 = (float)(wCoordY * CTerrainImpl::TERRAIN_YSIZE);
	D3DXMatrixMultiply(&matTexTransform, &m_matViewInverse, &m_matWorldForCommonUse);
	D3DXMatrixMultiply(&matSplatAlphaTexTransform, &matTexTransform, &m_matSplatAlpha);

	D3DXMATRIX matTiling;
	D3DXMatrixScaling(&matTiling, 1.0f / 640.0f, -1.0f / 640.0f, 0.0f);
	matTiling._41 = 0.0f;
	matTiling._42 = 0.0f;

	if (bLegacyDraws)
	{
		STATEMANAGER.SetTransform(D3DTS_TEXTURE1, &matSplatAlphaTexTransform);
		D3DXMatrixMultiply(&matSplatColorTexTransform, &m_matViewInverse, &matTiling);
		STATEMANAGER.SetTransform(D3DTS_TEXTURE0, &matSplatColorTexTransform);
		STATEMANAGER.SetStreamSource(0, pD3DVB, m_iPatchTerrainVertexSize);
		STATEMANAGER.SetRenderState(D3DRS_LIGHTING, FALSE);
	}

	int iPrevRenderedSplatNum = m_iRenderedSplatNum;
	bool isFirst = true;

	for (DWORD j = 1; j < pTerrain->GetNumTextures(); ++j)
	{
		TTerainSplat& rSplat = rTerrainSplatPatch.Splats[j];
		if (!rSplat.Active)
			continue;

		if (rTerrainSplatPatch.PatchTileCount[sPatchNum][j] == 0)
			continue;

		const TTerrainTexture& rTexture = m_TextureSet.GetTexture(j);

		if (bLegacyDraws)
		{
			D3DXMatrixMultiply(&matSplatColorTexTransform, &m_matViewInverse, &rTexture.m_matTransform);
			STATEMANAGER.SetTransform(D3DTS_TEXTURE0, &matSplatColorTexTransform);
			if (isFirst)
			{
				STATEMANAGER.SetTextureStageState(1, D3DTSS_ALPHAOP, D3DTOP_DISABLE);
				STATEMANAGER.SetTexture(0, rTexture.pd3dTexture);
				STATEMANAGER.SetTexture(1, rSplat.pd3dTexture);
				STATEMANAGER.DrawIndexedPrimitive(ePrimitiveType, 0, m_iPatchTerrainVertexCount, 0, wPrimitiveCount);
				STATEMANAGER.SetTextureStageState(1, D3DTSS_ALPHAOP, D3DTOP_SELECTARG1);
				isFirst = false;
			}
			else
			{
				STATEMANAGER.SetTexture(0, rTexture.pd3dTexture);
				STATEMANAGER.SetTexture(1, rSplat.pd3dTexture);
				STATEMANAGER.DrawIndexedPrimitive(ePrimitiveType, 0, m_iPatchTerrainVertexCount, 0, wPrimitiveCount);
			}
		}

		auto aIterator = std::find(m_RenderedTextureNumVector.begin(), m_RenderedTextureNumVector.end(), (int)j);
		if (aIterator == m_RenderedTextureNumVector.end())
			m_RenderedTextureNumVector.push_back(j);

		++m_iRenderedSplatNum;
		if (m_iRenderedSplatNum >= m_iSplatLimit)
			break;
	}

	// Cien
	if (m_bDrawShadow)
	{
		D3DXMATRIX matShadowTexTransform;
		D3DXMatrixMultiply(&matShadowTexTransform, &matTexTransform, &m_matStaticShadow);

		if (bLegacyDraws)
		{
			DWORD dwFogColor = mc_pEnvironmentData ? static_cast<DWORD>(mc_pEnvironmentData->FogColor) : 0xffffffff;
			STATEMANAGER.SetRenderState(D3DRS_LIGHTING, TRUE);

			STATEMANAGER.SetRenderState(D3DRS_FOGCOLOR, 0xFFFFFFFF);
			STATEMANAGER.SetRenderState(D3DRS_SRCBLEND, D3DBLEND_ZERO);
			STATEMANAGER.SetRenderState(D3DRS_DESTBLEND, D3DBLEND_SRCCOLOR);

			STATEMANAGER.SetTransform(D3DTS_TEXTURE0, &matShadowTexTransform);
			STATEMANAGER.SetTexture(0, pTerrain->GetShadowTexture());

			STATEMANAGER.SetTextureStageState(0, D3DTSS_COLORARG1, D3DTA_TEXTURE);
			STATEMANAGER.SetTextureStageState(0, D3DTSS_COLORARG2, D3DTA_CURRENT);
			STATEMANAGER.SetTextureStageState(0, D3DTSS_COLOROP,   D3DTOP_MODULATE);
			STATEMANAGER.SetTextureStageState(0, D3DTSS_ALPHAARG1, D3DTA_TEXTURE);
			STATEMANAGER.SetTextureStageState(0, D3DTSS_ALPHAARG2, D3DTA_CURRENT);
			STATEMANAGER.SetTextureStageState(0, D3DTSS_ALPHAOP,   D3DTOP_DISABLE);
			STATEMANAGER.SetSamplerState(0, D3DSAMP_ADDRESSU, D3DTADDRESS_CLAMP);
			STATEMANAGER.SetSamplerState(0, D3DSAMP_ADDRESSV, D3DTADDRESS_CLAMP);

			if (m_bDrawChrShadow)
			{
				STATEMANAGER.SetTransform(D3DTS_TEXTURE1, &m_matDynamicShadow);
				STATEMANAGER.SetTexture(1, m_lpCharacterShadowMapTexture);
				STATEMANAGER.SetTextureStageState(1, D3DTSS_COLORARG1, D3DTA_TEXTURE);
				STATEMANAGER.SetTextureStageState(1, D3DTSS_COLORARG2, D3DTA_CURRENT);
				STATEMANAGER.SetTextureStageState(1, D3DTSS_COLOROP,   D3DTOP_MODULATE);
				STATEMANAGER.SetTextureStageState(1, D3DTSS_ALPHAOP,   D3DTOP_DISABLE);
				STATEMANAGER.SetSamplerState(1, D3DSAMP_ADDRESSU, D3DTADDRESS_CLAMP);
				STATEMANAGER.SetSamplerState(1, D3DSAMP_ADDRESSV, D3DTADDRESS_CLAMP);
			}
			else
			{
				STATEMANAGER.SetTexture(1, NULL);
			}

			ms_faceCount += wPrimitiveCount;
			STATEMANAGER.DrawIndexedPrimitive(ePrimitiveType, 0, m_iPatchTerrainVertexCount, 0, wPrimitiveCount);

			if (m_bDrawChrShadow)
			{
				STATEMANAGER.SetTextureStageState(1, D3DTSS_COLORARG1, D3DTA_CURRENT);
				STATEMANAGER.SetTextureStageState(1, D3DTSS_COLOROP,   D3DTOP_SELECTARG1);
				STATEMANAGER.SetTextureStageState(1, D3DTSS_ALPHAARG1, D3DTA_TEXTURE);
				STATEMANAGER.SetTextureStageState(1, D3DTSS_ALPHAOP,   D3DTOP_SELECTARG1);
			}

			STATEMANAGER.SetTextureStageState(0, D3DTSS_COLORARG1, D3DTA_TEXTURE);
			STATEMANAGER.SetTextureStageState(0, D3DTSS_COLORARG2, D3DTA_CURRENT);
			STATEMANAGER.SetTextureStageState(0, D3DTSS_COLOROP,   D3DTOP_MODULATE);
			STATEMANAGER.SetTextureStageState(0, D3DTSS_ALPHAARG1, D3DTA_TEXTURE);
			STATEMANAGER.SetTextureStageState(0, D3DTSS_ALPHAOP,   D3DTOP_SELECTARG1);
			STATEMANAGER.SetSamplerState(0, D3DSAMP_ADDRESSU, D3DTADDRESS_WRAP);
			STATEMANAGER.SetSamplerState(0, D3DSAMP_ADDRESSV, D3DTADDRESS_WRAP);

			STATEMANAGER.SetRenderState(D3DRS_SRCBLEND, D3DBLEND_SRCALPHA);
			STATEMANAGER.SetRenderState(D3DRS_DESTBLEND, D3DBLEND_INVSRCALPHA);
			STATEMANAGER.SetRenderState(D3DRS_FOGCOLOR, dwFogColor);
			STATEMANAGER.SetRenderState(D3DRS_LIGHTING, FALSE);
		}

		++m_iRenderedSplatNum;
	}

	++m_iRenderedPatchNum;
	int iCurRenderedSplatNum = m_iRenderedSplatNum - iPrevRenderedSplatNum;
	m_iRenderedSplatNumSqSum += iCurRenderedSplatNum * iCurRenderedSplatNum;
}

void CMapOutdoor::__HardwareTransformPatch_RenderPatchNone(
	long patchnum,
	WORD wPrimitiveCount,
	D3DPRIMITIVETYPE ePrimitiveType,
	BYTE byLODLevel,
	float fDistance)
{
	assert(NULL != m_pTerrainPatchProxyList && "__HardwareTransformPatch_RenderPatchNone");
	CTerrainPatchProxy* pTerrainPatchProxy = &m_pTerrainPatchProxyList[patchnum];

	if (!pTerrainPatchProxy->isUsed())
		return;

	CGraphicVertexBuffer* pkVB = pTerrainPatchProxy->HardwareTransformPatch_GetVertexBufferPtr();
	if (!pkVB)
		return;

	// Ekstrakcja TTerrainSectorPacket dla pasma far (None)
	TTerrainSectorPacket sectorPacket{};
	m_TerrainRenderExtractor.ExtractAndSubmitPatch(
		this,
		static_cast<uint32_t>(patchnum),
		byLODLevel,
		TERRAIN_BAND_FAR,
		wPrimitiveCount,
		ePrimitiveType,
		fDistance,
		sectorPacket
	);

	if (RenderOwnership::LegacyDraws(ERenderCategory::Terrain))
	{
		STATEMANAGER.SetStreamSource(0, pkVB->GetD3DVertexBuffer(), m_iPatchTerrainVertexSize);
		STATEMANAGER.DrawIndexedPrimitive(ePrimitiveType, 0, m_iPatchTerrainVertexCount, 0, wPrimitiveCount);
	}
}
