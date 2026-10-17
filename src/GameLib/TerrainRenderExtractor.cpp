#include "StdAfx.h"
#include "TerrainRenderExtractor.h"
#include "MapOutdoor.h"
#include "AreaTerrain.h"
#include "TerrainPatch.h"
#include "EterLib/RenderQueueManager.h"
#include <algorithm>

void CTerrainRenderExtractor::Initialize(size_t maxSectors)
{
	if (maxSectors == 0)
		maxSectors = 256;

	m_splatWeightBuffer.assign(maxSectors, 0xFFFFFFFF);
	m_lastExtractedCount = 0;
}

bool CTerrainRenderExtractor::ExtractPatch(CMapOutdoor* pMapOutdoor, uint32_t patchNum, TTerrainSectorPacket& outPacket)
{
	if (!pMapOutdoor)
		return false;

	CTerrainPatchProxy* pProxy = pMapOutdoor->GetTerrainPatchProxy(patchNum);
	if (!pProxy || !pProxy->isUsed())
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

	outPacket.dwSectorX = wCoordX;
	outPacket.dwSectorY = wCoordY;
	outPacket.sectorX = uSectorX;
	outPacket.sectorY = uSectorY;
	outPacket.patchNum = patchNum;
	outPacket.fWorldPosX = static_cast<float>(wCoordX * CTerrainImpl::TERRAIN_XSIZE);
	outPacket.fWorldPosY = static_cast<float>(wCoordY * CTerrainImpl::TERRAIN_YSIZE);

	outPacket.pVertexData = static_cast<void*>(pD3DVB);
	outPacket.dwVertexCount = CTerrainPatch::TERRAIN_VERTEX_COUNT;

	// Warstwy tekstur i 4-kanalowy splatting
	const short sPatchNum = pProxy->GetPatchNum();
	TTerrainSplatPatch& rSplatPatch = pTerrain->GetTerrainSplatPatch();
	const DWORD dwNumTextures = pTerrain->GetNumTextures();

	outPacket.splatTextureId = 0;
	outPacket.pSplatTexture = nullptr;
	for (int i = 0; i < 4; ++i)
	{
		outPacket.layerTextureIds[i] = 0;
		outPacket.pLayerTextures[i] = nullptr;
	}

	const auto& weightMap = pTerrain->GetSplatWeightMap();
	if (pTerrain->GetSplatTexture())
	{
		outPacket.pSplatTexture = pTerrain->GetSplatTexture();
		outPacket.splatTextureId = static_cast<uint32_t>(reinterpret_cast<uintptr_t>(outPacket.pSplatTexture) & 0xFFFFFFFF);

		const auto& domLayers = weightMap.GetDominantLayers();
		for (int l = 0; l < 4; ++l)
		{
			const uint32_t texIdx = domLayers[l];
			if (texIdx > 0 && texIdx < dwNumTextures)
			{
				const TTerrainTexture& rTex = pTerrain->GetTexture(static_cast<long>(texIdx));
				outPacket.layerTextureIds[l] = texIdx;
				outPacket.pLayerTextures[l] = rTex.pd3dTexture;
			}
		}
	}
	else
	{
		uint32_t activeLayerCount = 0;
		for (DWORD j = 1; j < dwNumTextures && activeLayerCount < 4; ++j)
		{
			TTerainSplat& rSplat = rSplatPatch.Splats[j];
			if (!rSplat.Active)
				continue;

			if (sPatchNum >= 0 && sPatchNum < TERRAIN_PATCHCOUNT * TERRAIN_PATCHCOUNT)
			{
				if (rSplatPatch.PatchTileCount[sPatchNum][j] == 0)
					continue;
			}

			if (activeLayerCount == 0 && rSplat.pd3dTexture)
			{
				outPacket.pSplatTexture = rSplat.pd3dTexture;
				outPacket.splatTextureId = static_cast<uint32_t>(reinterpret_cast<uintptr_t>(rSplat.pd3dTexture) & 0xFFFFFFFF);
			}

			const TTerrainTexture& rTex = pTerrain->GetTexture(static_cast<long>(j));
			outPacket.layerTextureIds[activeLayerCount] = static_cast<uint32_t>(j);
			outPacket.pLayerTextures[activeLayerCount] = rTex.pd3dTexture;
			++activeLayerCount;
		}
	}

	const auto rawWeights = weightMap.GetRawData();
	if (!rawWeights.empty())
	{
		outPacket.pSplatWeights = rawWeights.data();
		outPacket.dwSplatWeightCount = static_cast<uint32_t>(rawWeights.size());
	}
	else if (!m_splatWeightBuffer.empty())
	{
		outPacket.pSplatWeights = m_splatWeightBuffer.data();
		outPacket.dwSplatWeightCount = static_cast<uint32_t>(m_splatWeightBuffer.size());
	}
	else
	{
		outPacket.pSplatWeights = nullptr;
		outPacket.dwSplatWeightCount = 0;
	}

	return true;
}

void CTerrainRenderExtractor::ExtractAndSubmitVisibleNodes(CMapOutdoor* pMapOutdoor, std::span<const uint32_t> visibleNodes)
{
	if (!pMapOutdoor)
		return;

	uint32_t count = 0;

	for (const uint32_t patchNum : visibleNodes)
	{
		TTerrainSectorPacket packet{};
		if (ExtractPatch(pMapOutdoor, patchNum, packet))
		{
			CRenderQueueManager::Instance().SubmitTerrain(packet);
			++count;
		}
	}

	m_lastExtractedCount = count;
}

void CTerrainRenderExtractor::ExtractAndSubmitFromOutdoor(CMapOutdoor* pMapOutdoor)
{
	if (!pMapOutdoor)
		return;

	ExtractAndSubmitVisibleNodes(pMapOutdoor, pMapOutdoor->GetVisibleQuadtreeNodes());
}
