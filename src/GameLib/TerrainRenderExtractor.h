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
 * Extracts visible terrain sectors and splatting passes in parity with
 * __RenderTerrain_RenderHardwareTransformPatch and submits to CRenderQueueManager.
 */
class CTerrainRenderExtractor
{
public:
	CTerrainRenderExtractor();
	~CTerrainRenderExtractor() = default;

	CTerrainRenderExtractor(const CTerrainRenderExtractor&) = delete;
	CTerrainRenderExtractor& operator=(const CTerrainRenderExtractor&) = delete;

	CTerrainRenderExtractor(CTerrainRenderExtractor&&) noexcept = default;
	CTerrainRenderExtractor& operator=(CTerrainRenderExtractor&&) noexcept = default;

	// Inicjalizacja / reset stanu na poczatku klatki renderowania
	void BeginFrame();

	// Ekstrakcja pojedynczego patcha do TTerrainSectorPacket i emisja do kolejki renderowania
	bool ExtractAndSubmitPatch(
		CMapOutdoor* pMapOutdoor,
		uint32_t patchNum,
		uint8_t byLODLevel,
		uint8_t byBand,
		WORD wPrimitiveCount,
		D3DPRIMITIVETYPE ePrimitiveType,
		float fDistance,
		TTerrainSectorPacket& outPacket
	);

	// Ekstrakcja pojedynczego patcha z pelnymi przebiegami splattingu
	bool ExtractPatch(
		CMapOutdoor* pMapOutdoor,
		uint32_t patchNum,
		uint8_t byLODLevel,
		uint8_t byBand,
		WORD wPrimitiveCount,
		D3DPRIMITIVETYPE ePrimitiveType,
		float fDistance,
		TTerrainSectorPacket& outPacket
	);

	// Ekstrakcja pojedynczego patcha do struktury TTerrainSectorPacket (przeciazenie bazowe)
	bool ExtractPatch(CMapOutdoor* pMapOutdoor, uint32_t patchNum, TTerrainSectorPacket& outPacket);

	// Masowa ekstrakcja i emisja widocznych wezlow quadtree
	void ExtractAndSubmitVisibleNodes(CMapOutdoor* pMapOutdoor, std::span<const uint32_t> visibleNodes);

	// Ekstrakcja i emisja bezposrednio z aktywnego stanu CMapOutdoor
	void ExtractAndSubmitFromOutdoor(CMapOutdoor* pMapOutdoor);

	// Pobranie listy przebiegow dla ostatniego wyekstrahowanego patcha
	[[nodiscard]] const std::vector<STerrainSplatPass>& GetLastPatchPasses() const noexcept { return m_lastPatchPasses; }

	// Pobranie bufora przebiegow w postaci pakietow ABI
	[[nodiscard]] const std::vector<TTerrainPassPacket>& GetPassPacketPool() const noexcept { return m_passPacketPool; }

	// Statystyki ostatniej klatki
	[[nodiscard]] uint32_t GetLastExtractedSectorCount() const noexcept { return m_lastExtractedCount; }
	[[nodiscard]] uint32_t GetLastExtractedPassCount() const noexcept { return m_totalExtractedPasses; }

private:
	uint32_t m_lastExtractedCount{ 0 };
	uint32_t m_totalExtractedPasses{ 0 };
	std::vector<STerrainSplatPass> m_splatPassPool;
	std::vector<TTerrainPassPacket> m_passPacketPool;
	std::vector<STerrainSplatPass> m_lastPatchPasses;
};

using TerrainRenderExtractor = CTerrainRenderExtractor;

