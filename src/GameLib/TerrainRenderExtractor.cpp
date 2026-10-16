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

	outPacket.pVertexData = static_cast<void*>(pD3DVB);
	outPacket.dwVertexCount = CTerrainPatch::TERRAIN_VERTEX_COUNT;

	// Warstwy tekstur i splatting
	const short sPatchNum = pProxy->GetPatchNum();
	TTerrainSplatPatch& rSplatPatch = pTerrain->GetTerrainSplatPatch();
	const DWORD dwNumTextures = pTerrain->GetNumTextures();

	outPacket.splatTextureId = 0;
	for (int i = 0; i < 4; ++i)
		outPacket.layerTextureIds[i] = 0;

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
			outPacket.splatTextureId = static_cast<uint32_t>(reinterpret_cast<uintptr_t>(rSplat.pd3dTexture));
		}

		outPacket.layerTextureIds[activeLayerCount++] = static_cast<uint32_t>(j);
	}

	if (!m_splatWeightBuffer.empty())
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
	TTerrainSectorPacket packet{};

	for (const uint32_t patchNum : visibleNodes)
	{
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
