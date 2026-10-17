#pragma once

#include "RenderPacketTypes.h"
#include "RenderQueueManager.h"
#include "RenderResourceRegistry.h"
#include "IRenderBackend.h"
#include <vector>
#include <string>
#include <memory>
#include <unordered_set>
#include <cstdint>

namespace FrameReplay
{
	// Struktura statystyk odtworzenia klatki
	struct SReplayStats
	{
		uint32_t dwDumpVersion{ 0 };
		uint32_t dwChunkCount{ 0 };
		uint32_t dwTotalPackets{ 0 };
		uint32_t dwRegisteredTextures{ 0 };
		bool bHasUniforms{ false };
		bool bSuccess{ false };
	};

	// Informacje o podsumowaniu zrzutu
	struct SDumpSummary
	{
		uint32_t dwMagic{ 0 };
		uint32_t dwVersion{ 0 };
		uint32_t dwChunkCount{ 0 };
		uint32_t dwSkyBoxPackets{ 0 };
		uint32_t dwTerrainPackets{ 0 };
		uint32_t dwDungeonPackets{ 0 };
		uint32_t dwWaterPackets{ 0 };
		uint32_t dwSpeedTreeBatches{ 0 };
		uint32_t dwSpeedTreeInstances{ 0 };
		uint32_t dwActorPackets{ 0 };
		uint32_t dwEffectBatches{ 0 };
		uint32_t dwEffectParticles{ 0 };
		uint32_t dwUIBatches{ 0 };
		uint32_t dwUIVertices{ 0 };
		uint32_t dwUIIndices{ 0 };
		float fTime{ 0.0f };
		uint32_t dwFrameIndex{ 0 };
	};
}

// Klasa odtwarzacza zrzutow klatki M2FR (v1 oraz v2)
class CFrameReplayPlayer
{
public:
	CFrameReplayPlayer();
	~CFrameReplayPlayer();

	// Ladowanie zrzutu z dysku (automatyczne rozpoznawanie v1 i v2)
	bool LoadDump(const char* c_pszFileName);

	// Pobranie zdeserializowanych danych klatki
	const FrameDumpV2::TDumpParsedFrame& GetParsedFrame() const { return m_parsedFrame; }
	FrameDumpV2::TDumpParsedFrame& GetParsedFrame() { return m_parsedFrame; }

	// Pobranie wersji formatu zrzutu (1 lub 2)
	uint32_t GetDumpVersion() const { return m_dwDumpVersion; }

	// Pobranie wersji ABI renderera
	uint32_t GetABIVersion() const { return CRenderQueueManager::ABI_VERSION; }

	// Pobranie liczby sekcji / chunkow
	uint32_t GetChunkCount() const { return static_cast<uint32_t>(m_parsedFrame.chunkHeaders.size()); }

	// Pobranie uniformow klatki (kamera, swiatla, mgla)
	const TRenderFrameUniforms& GetUniforms() const { return m_parsedFrame.uniforms; }

	// Pobranie podsumowania zaladowanego zrzutu
	FrameReplay::SDumpSummary GetSummary() const;

	// Odtworzenie stanu rejestru zasobow (CRenderResourceRegistry)
	bool ReconstructResourceRegistry(CRenderResourceRegistry* pRegistry = nullptr);

	// Rekonstrukcja pakietow i zgloszenie ich do CRenderQueueManager
	bool ReplayToQueue(CRenderQueueManager& queueManager);

	// Pelne odtworzenie klatki przez backend (np. CD3D9RenderBackend) z opcjonalnym zrzutem do PNG
	bool Play(const char* c_pszDumpFileName,
	          const char* c_pszOutputScreenshot = nullptr,
	          IRenderBackend* pBackend = nullptr,
	          CRenderQueueManager* pQueue = nullptr,
	          CRenderResourceRegistry* pRegistry = nullptr);

	// Statystyki ostatniego odtworzenia
	const FrameReplay::SReplayStats& GetStats() const { return m_stats; }

	// Czyszczenie stanu
	void Clear();

	// Eksport do starszego formatu v1 (uzyteczne w testach wstecznej kompatybilnosci)
	static bool ExportFrameDumpV1(const char* c_pszFileName, const CRenderQueueManager& queue);

private:
	bool LoadDumpV1(FILE* fp);
	bool LoadDumpV2(FILE* fp, const FrameDumpV2::THeader& hdr);

	uint32_t m_dwDumpVersion{ 0 };
	FrameDumpV2::TDumpParsedFrame m_parsedFrame{};
	FrameReplay::SReplayStats m_stats{};

	// Uchwyty i mocki zarejestrowanych zasobow
	std::vector<IDirect3DTexture9*> m_createdStubTextures;
	std::vector<TRenderTextureHandle> m_registeredHandles;
};
