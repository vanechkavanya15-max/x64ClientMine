#include "StdAfx.h"
#include "TerrainRenderExtractor.h"
#include "MapOutdoor.h"
#include "AreaTerrain.h"
#include "TerrainPatch.h"
#include "EterLib/RenderQueueManager.h"
#include "EterLib/RenderResourceRegistry.h"
#include <algorithm>

CTerrainRenderExtractor::CTerrainRenderExtractor()
{
	m_splatPassPool.reserve(4096);
	m_passPacketPool.reserve(4096);
	m_lastPatchPasses.reserve(16);
}

void CTerrainRenderExtractor::BeginFrame()
{
	m_lastExtractedCount = 0;
	m_totalExtractedPasses = 0;
	m_splatPassPool.clear();
	m_passPacketPool.clear();
	m_lastPatchPasses.clear();
}

bool CTerrainRenderExtractor::ExtractAndSubmitPatch(
	CMapOutdoor* pMapOutdoor,
	uint32_t patchNum,
	uint8_t byLODLevel,
	uint8_t byBand,
	WORD wPrimitiveCount,
	D3DPRIMITIVETYPE ePrimitiveType,
	float fDistance,
	TTerrainSectorPacket& outPacket)
{
	if (!ExtractPatch(pMapOutdoor, patchNum, byLODLevel, byBand, wPrimitiveCount, ePrimitiveType, fDistance, outPacket))
		return false;

	CRenderQueueManager::Instance().SubmitTerrain(outPacket);
	++m_lastExtractedCount;
	return true;
}

bool CTerrainRenderExtractor::ExtractPatch(
	CMapOutdoor* pMapOutdoor,
	uint32_t patchNum,
	uint8_t byLODLevel,
	uint8_t byBand,
	WORD wPrimitiveCount,
	D3DPRIMITIVETYPE ePrimitiveType,
	float fDistance,
	TTerrainSectorPacket& outPacket)
{
	if (!pMapOutdoor)
		return false;

	CTerrainPatchProxy* pProxy = pMapOutdoor->GetTerrainPatchProxy(patchNum);
	if (!pProxy || !pProxy->isUsed())
		return false;

	const short sPatchNum = pProxy->GetPatchNum();
	if (sPatchNum < 0)
		return false;

	const BYTE ucTerrainNum = pProxy->GetTerrainNum();
	if (0xFF == ucTerrainNum)
		return false;

	CTerrain* pTerrain = nullptr;
	if (!pMapOutdoor->GetTerrainPointer(ucTerrainNum, &pTerrain) || !pTerrain)
		return false;

	CGraphicVertexBuffer* pkVB = pProxy->HardwareTransformPatch_GetVertexBufferPtr();
	if (!pkVB)
		return false;

	LPDIRECT3DVERTEXBUFFER9 pD3DVB = pkVB->GetD3DVertexBuffer();
	if (!pD3DVB)
		return false;

	WORD wCoordX = 0, wCoordY = 0;
	pTerrain->GetCoordinate(&wCoordX, &wCoordY);

	const WORD wPatchCount = pMapOutdoor->GetPatchCount();
	const uint16_t uSectorX = static_cast<uint16_t>(wPatchCount > 0 ? (patchNum % wPatchCount) : 0);
	const uint16_t uSectorY = static_cast<uint16_t>(wPatchCount > 0 ? (patchNum / wPatchCount) : 0);

	outPacket = TTerrainSectorPacket{};
	outPacket.nCoordX = static_cast<int16_t>(wCoordX);
	outPacket.nCoordY = static_cast<int16_t>(wCoordY);
	outPacket.dwSectorX = wCoordX;
	outPacket.dwSectorY = wCoordY;
	outPacket.sectorX = uSectorX;
	outPacket.sectorY = uSectorY;
	outPacket.patchNum = patchNum;
	outPacket.fWorldPosX = static_cast<float>(wCoordX * CTerrainImpl::TERRAIN_XSIZE);
	outPacket.fWorldPosY = static_cast<float>(wCoordY * CTerrainImpl::TERRAIN_YSIZE);
	outPacket.pVertexData = static_cast<void*>(pD3DVB);
	outPacket.dwVertexCount = CTerrainPatch::TERRAIN_VERTEX_COUNT;
	outPacket.pIndexBuffer = (byLODLevel < TERRAINPATCH_LODMAX) ?
		static_cast<void*>(pMapOutdoor->m_IndexBuffer[byLODLevel].GetD3DIndexBuffer()) : nullptr;
	outPacket.dwPrimitiveType = static_cast<uint32_t>(ePrimitiveType);
	outPacket.dwPrimitiveCount = static_cast<uint32_t>(wPrimitiveCount);
	outPacket.byLODLevel = byLODLevel;
	outPacket.byBand = byBand;
	outPacket.bDrawShadow = pMapOutdoor->m_bDrawShadow ? 1 : 0;
	outPacket.SetCharacterShadow(pMapOutdoor->m_bDrawShadow && pMapOutdoor->m_bDrawChrShadow);
	outPacket.pShadowTexture = static_cast<void*>(pTerrain->GetShadowTexture());
	outPacket.pChrShadowTexture = static_cast<void*>(pMapOutdoor->m_lpCharacterShadowMapTexture);
	outPacket.bHasWater = pProxy->isWaterExists();
	outPacket.fWaterHeight = pProxy->GetWaterHeight();

	// Macierze HTP transformacji
	D3DXMATRIX matWorldCommon = pMapOutdoor->m_matWorldForCommonUse;
	matWorldCommon._41 = -static_cast<float>(wCoordX * CTerrainImpl::TERRAIN_XSIZE);
	matWorldCommon._42 = static_cast<float>(wCoordY * CTerrainImpl::TERRAIN_YSIZE);
	D3DXMATRIX matTexTransform;
	D3DXMatrixMultiply(&matTexTransform, &pMapOutdoor->m_matViewInverse, &matWorldCommon);
	D3DXMATRIX matSplatAlphaTexTransform;
	D3DXMatrixMultiply(&matSplatAlphaTexTransform, &matTexTransform, &pMapOutdoor->m_matSplatAlpha);

	m_lastPatchPasses.clear();
	const size_t passStartIndex = m_splatPassPool.size();
	const size_t passPktStartIndex = m_passPacketPool.size();

	// Ekstrakcja przebiegow splattingu dla pasm Near i Fog (pasmo Far to None - 0 przebiegow)
	if (byBand != TERRAIN_BAND_FAR)
	{
		TTerrainSplatPatch& rSplatPatch = pTerrain->GetTerrainSplatPatch();
		const DWORD dwNumTextures = pTerrain->GetNumTextures();
		bool isFirst = true;
		uint32_t activeLayerCount = 0;

		for (DWORD j = 1; j < dwNumTextures; ++j)
		{
			TTerainSplat& rSplat = rSplatPatch.Splats[j];
			if (!rSplat.Active)
				continue;

			if (sPatchNum < TERRAIN_PATCHCOUNT * TERRAIN_PATCHCOUNT && rSplatPatch.PatchTileCount[sPatchNum][j] == 0)
				continue;

			const TTerrainTexture& rTexture = pMapOutdoor->m_TextureSet.GetTexture(j);

			D3DXMATRIX matSplatColorTexTransform;
			D3DXMatrixMultiply(&matSplatColorTexTransform, &pMapOutdoor->m_matViewInverse, &rTexture.m_matTransform);

			STerrainSplatPass pass{};
			pass.layerIndex = j;
			pass.pLayerTexture = static_cast<void*>(rTexture.pd3dTexture);
			pass.textureId = (rTexture.pd3dTexture && CRenderResourceRegistry::InstancePtr())
				? CRenderResourceRegistry::Instance().RegisterTexture(rTexture.pd3dTexture, "Terrain_Layer")
				: static_cast<uint32_t>(j);
			pass.pAlphaTexture = static_cast<void*>(rSplat.pd3dTexture);
			pass.alphaTextureId = (rSplat.pd3dTexture && CRenderResourceRegistry::InstancePtr())
				? CRenderResourceRegistry::Instance().RegisterTexture(rSplat.pd3dTexture, "Terrain_Alpha")
				: kInvalidTextureHandle;
			pass.matTextureTransform = matSplatColorTexTransform;
			pass.matAlphaTransform = matSplatAlphaTexTransform;
			pass.bDisableAlphaOp = isFirst;
			pass.bShadowPass = false;

			m_splatPassPool.push_back(pass);
			m_lastPatchPasses.push_back(pass);

			TTerrainPassPacket passPkt{};
			passPkt.layerIndex = j;
			passPkt.alphaSplatHandle = pass.alphaTextureId;
			passPkt.lodLevel = byLODLevel;
			passPkt.indexBufferHandle = 0;
			passPkt.matLayer = ToAbi(matSplatColorTexTransform);
			passPkt.nearDistance = pMapOutdoor->mc_pEnvironmentData ? pMapOutdoor->mc_pEnvironmentData->GetFogNearDistance() : 5000.0f;
			passPkt.fogDistance = fDistance;
			passPkt.farDistance = pMapOutdoor->mc_pEnvironmentData ? pMapOutdoor->mc_pEnvironmentData->GetFogFarDistance() : 10000.0f;
			passPkt.shadowTextureHandle = 0;
			m_passPacketPool.push_back(passPkt);

			if (activeLayerCount < 4)
			{
				outPacket.pLayerTextures[activeLayerCount] = rTexture.pd3dTexture;
				outPacket.layerTextureIds[activeLayerCount] = pass.textureId;
				if (activeLayerCount == 0)
				{
					outPacket.pSplatTexture = rSplat.pd3dTexture;
					outPacket.splatTextureId = pass.alphaTextureId;
				}
				++activeLayerCount;
			}

			isFirst = false;
		}

		const uint32_t numPasses = static_cast<uint32_t>(m_splatPassPool.size() - passStartIndex);
		outPacket.dwPassCount = numPasses;
		outPacket.pPassPackets = (numPasses > 0) ? &m_passPacketPool[passPktStartIndex] : nullptr;
		outPacket.pPassData = (numPasses > 0) ? &m_splatPassPool[passStartIndex] : nullptr;
		m_totalExtractedPasses += numPasses;
	}

	return true;
}

bool CTerrainRenderExtractor::ExtractPatch(CMapOutdoor* pMapOutdoor, uint32_t patchNum, TTerrainSectorPacket& outPacket)
{
	WORD wPrimCount = 0;
	D3DPRIMITIVETYPE ePrimType = D3DPT_TRIANGLELIST;
	if (pMapOutdoor)
		pMapOutdoor->SelectIndexBuffer(0, &wPrimCount, &ePrimType);

	return ExtractPatch(pMapOutdoor, patchNum, 0, TERRAIN_BAND_NEAR, wPrimCount, ePrimType, 0.0f, outPacket);
}

void CTerrainRenderExtractor::ExtractAndSubmitVisibleNodes(CMapOutdoor* pMapOutdoor, std::span<const uint32_t> visibleNodes)
{
	if (!pMapOutdoor)
		return;

	for (const uint32_t patchNum : visibleNodes)
	{
		TTerrainSectorPacket packet{};
		ExtractAndSubmitPatch(pMapOutdoor, patchNum, 0, TERRAIN_BAND_NEAR, 0, D3DPT_TRIANGLELIST, 0.0f, packet);
	}
}

void CTerrainRenderExtractor::ExtractAndSubmitFromOutdoor(CMapOutdoor* pMapOutdoor)
{
	if (!pMapOutdoor)
		return;

	ExtractAndSubmitVisibleNodes(pMapOutdoor, pMapOutdoor->GetVisibleQuadtreeNodes());
}
