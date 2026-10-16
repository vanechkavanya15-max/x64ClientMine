#pragma once

#include <span>
#include <cstdint>
#include <vector>
#include "EterLib/RenderPacketTypes.h"

class CMapOutdoor;
class CTerrain;
class CTerrainPatchProxy;

/**
 * @brief Zero-allocation terrain sector render extractor and submitter.
 * Extracts visible terrain sectors and splatting data after frustum culling
 * and submits TTerrainSectorPacket to CRenderQueueManager.
 */
class CTerrainRenderExtractor
{
public:
	CTerrainRenderExtractor() = default;
	~CTerrainRenderExtractor() = default;

	CTerrainRenderExtractor(const CTerrainRenderExtractor&) = delete;
	CTerrainRenderExtractor& operator=(const CTerrainRenderExtractor&) = delete;

	CTerrainRenderExtractor(CTerrainRenderExtractor&&) noexcept = default;
	CTerrainRenderExtractor& operator=(CTerrainRenderExtractor&&) noexcept = default;

	// Inicjalizacja z prealokacja struktur pomocniczych
	void Initialize(size_t maxSectors = 256);

	// Ekstrakcja pojedynczego patcha do struktury TTerrainSectorPacket
	bool ExtractPatch(CMapOutdoor* pMapOutdoor, uint32_t patchNum, TTerrainSectorPacket& outPacket);

	// Masowa ekstrakcja i emisja widocznych wezlow quadtree
	void ExtractAndSubmitVisibleNodes(CMapOutdoor* pMapOutdoor, std::span<const uint32_t> visibleNodes);

	// Ekstrakcja i emisja bezposrednio z aktywnego stanu CMapOutdoor
	void ExtractAndSubmitFromOutdoor(CMapOutdoor* pMapOutdoor);

	// Statystyki ostatniej klatki
	[[nodiscard]] uint32_t GetLastExtractedSectorCount() const noexcept { return m_lastExtractedCount; }

private:
	std::vector<uint32_t> m_splatWeightBuffer;
	uint32_t m_lastExtractedCount{ 0 };
};

using TerrainRenderExtractor = CTerrainRenderExtractor;
