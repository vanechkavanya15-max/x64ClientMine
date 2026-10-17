#pragma once

#include "RenderPacketTypes.h"
#include <vector>
#include <string>
#include <map>
#include <utility>
#include <cstddef>
#include <cstdint>

class IRenderBackend;

constexpr uint32_t M2_RENDER_ABI_VERSION = 1;

namespace FrameDumpV2
{
	constexpr uint32_t M2FR_MAGIC = 0x5246324D; // "M2FR"
	constexpr uint32_t M2FR_VERSION = 2;

	constexpr uint32_t MakeTag(char a, char b, char c, char d) noexcept
	{
		return static_cast<uint32_t>(static_cast<uint8_t>(a)) |
			(static_cast<uint32_t>(static_cast<uint8_t>(b)) << 8) |
			(static_cast<uint32_t>(static_cast<uint8_t>(c)) << 16) |
			(static_cast<uint32_t>(static_cast<uint8_t>(d)) << 24);
	}

	constexpr uint32_t CHUNK_UNIFORMS  = MakeTag('U', 'N', 'I', 'F'); // 0x46494E55
	constexpr uint32_t CHUNK_SKYBOX    = MakeTag('S', 'K', 'Y', 'B'); // 0x42594B53
	constexpr uint32_t CHUNK_TERRAIN   = MakeTag('T', 'E', 'R', 'R'); // 0x52524554
	constexpr uint32_t CHUNK_DUNGEON   = MakeTag('D', 'U', 'N', 'G'); // 0x474E5544
	constexpr uint32_t CHUNK_WATER     = MakeTag('W', 'A', 'T', 'R'); // 0x52544157
	constexpr uint32_t CHUNK_SPEEDTREE = MakeTag('T', 'R', 'E', 'E'); // 0x45455254
	constexpr uint32_t CHUNK_ACTORS    = MakeTag('A', 'C', 'T', 'R'); // 0x52544341
	constexpr uint32_t CHUNK_EFFECTS   = MakeTag('E', 'F', 'C', 'T'); // 0x54434645
	constexpr uint32_t CHUNK_UI        = MakeTag('U', 'I', 'B', 'D'); // 0x44424955

#pragma pack(push, 1)
	struct THeader
	{
		uint32_t dwMagic{ M2FR_MAGIC };
		uint32_t dwVersion{ M2FR_VERSION };
		uint32_t dwChunkCount{ 9 };
		uint32_t dwReserved{ 0 };
	};

	struct TChunkHeader
	{
		uint32_t dwTag{ 0 };
		uint32_t dwCount{ 0 };
		uint32_t dwElementSize{ 0 };
		uint32_t dwDataSize{ 0 };
	};

	struct TSpeedTreeBatchHeader
	{
		uint32_t dwTreeCRC{ 0 };
		uint32_t dwFlags{ 0 };
		uint64_t uTreeTypeID{ 0 };
		uint64_t uTrunkTextureID{ 0 };
		uint64_t uLeafTextureID{ 0 };
		uint32_t dwInstanceCount{ 0 };
		uint32_t dwPad{ 0 };
	};

	struct TEffectBatchHeader
	{
		uint32_t dwEffectID{ 0 };
		uint32_t dwBlendMode{ 0 };
		uint64_t uTextureID{ 0 };
		uint32_t dwParticleCount{ 0 };
		uint32_t dwPad{ 0 };
	};

	struct TUIBatchHeader
	{
		uint64_t uTextureID{ 0 };
		uint32_t bUseScissor{ 0 };
		TRect32 scissorRect{};
		uint32_t dwBlendMode{ 0 };
		uint32_t dwVertexCount{ 0 };
		uint32_t dwIndexCount{ 0 };
	};
#pragma pack(pop)

	static_assert(sizeof(THeader) == 16, "THeader size must be 16 bytes");
	static_assert(sizeof(TChunkHeader) == 16, "TChunkHeader size must be 16 bytes");
	static_assert(sizeof(TSpeedTreeBatchHeader) == 40, "TSpeedTreeBatchHeader size must be 40 bytes");
	static_assert(sizeof(TEffectBatchHeader) == 24, "TEffectBatchHeader size must be 24 bytes");
	static_assert(sizeof(TUIBatchHeader) == 40, "TUIBatchHeader size must be 40 bytes");

	struct TDumpSpeedTreeBatch
	{
		TSpeedTreeBatchHeader header{};
		std::vector<TSpeedTreeInstanceData> instances;
	};

	struct TDumpEffectBatch
	{
		TEffectBatchHeader header{};
		std::vector<TParticleInstanceData> particles;
	};

	struct TDumpUIBatch
	{
		TUIBatchHeader header{};
		std::vector<TUIVertex> vertices;
		std::vector<uint16_t> indices;
	};

	struct TDumpParsedFrame
	{
		THeader header{};
		std::vector<TChunkHeader> chunkHeaders;
		TRenderFrameUniforms uniforms{};
		std::vector<TSkyBoxPacket> skyBoxPackets;
		std::vector<TTerrainSectorPacket> terrainPackets;
		std::vector<TDungeonBlockPacket> dungeonPackets;
		std::vector<TWaterPacket> waterPackets;
		std::vector<TDumpSpeedTreeBatch> speedTreeBatches;
		std::vector<TActorRenderPacket> actorPackets;
		std::vector<TDumpEffectBatch> effectBatches;
		std::vector<TDumpUIBatch> uiBatches;
	};

	bool ParseFrameDumpV2(const char* c_pszFileName, TDumpParsedFrame& outFrame);
}

class CRenderQueueManager
{
public:
	static constexpr uint32_t ABI_VERSION = M2_RENDER_ABI_VERSION;
	static uint32_t GetABIVersion() { return M2_RENDER_ABI_VERSION; }

	static std::vector<std::pair<std::string, size_t>> GetPacketSizes();
	static std::map<std::string, size_t> GetPacketSizeMap();

	static CRenderQueueManager& Instance()
	{
		static CRenderQueueManager s_instance;
		return s_instance;
	}

	CRenderQueueManager();
	~CRenderQueueManager();

	void BeginFrame();
	void EndFrame();

	void SetFrameUniforms(const TRenderFrameUniforms& uniforms);
	const TRenderFrameUniforms& GetFrameUniforms() const;

	void SubmitSpeedTree(const TSpeedTreePacket& packet);
	void SubmitUI(const TUIBatchPacket& packet);
	void SubmitActor(const TActorRenderPacket& packet);
	void SubmitTerrain(const TTerrainSectorPacket& packet);
	void SubmitEffect(const TEffectInstancePacket& packet);
	void SubmitWater(const TWaterPacket& packet);
	void SubmitSkyBox(const TSkyBoxPacket& packet);
	void SubmitDungeonBlock(const TDungeonBlockPacket& packet);

	void Flush(IRenderBackend* pBackend = nullptr);
	void FlushUI();

	const std::vector<TSpeedTreePacket>& GetSpeedTreePackets() const;
	const std::vector<TUIBatchPacket>& GetUIPackets() const;
	const std::vector<TActorRenderPacket>& GetActorPackets() const;
	const std::vector<TTerrainSectorPacket>& GetTerrainPackets() const;
	const std::vector<TEffectInstancePacket>& GetEffectPackets() const;
	const std::vector<TWaterPacket>& GetWaterPackets() const;
	const std::vector<TSkyBoxPacket>& GetSkyBoxPackets() const;
	const std::vector<TDungeonBlockPacket>& GetDungeonPackets() const;

	size_t GetTotalPacketCount() const;

	TParticleInstanceData* AllocateParticleData(size_t count);
	TSpeedTreeInstanceData* AllocateSpeedTreeInstanceData(size_t count);

	size_t GetDroppedParticles() const { return m_droppedParticles; }
	size_t GetDroppedSpeedTree() const { return m_droppedSpeedTree; }
	void ResetDroppedCounters() { m_droppedParticles = 0; m_droppedSpeedTree = 0; }

	size_t GetParticlePoolCapacity() const { return m_particleDataPool.size(); }
	size_t GetSpeedTreePoolCapacity() const { return m_speedTreeInstancePool.size(); }
	size_t GetParticleDataOffset() const { return m_particleDataOffset; }
	size_t GetSpeedTreeInstanceOffset() const { return m_speedTreeInstanceOffset; }

	// Golden Master Frame Dumper (Faza 6) & Backend Switcher (Faza 8)
	bool ExportFrameToBinary(const char* c_pszFileName) const;
	bool ExportFrameToBinaryV2(const char* c_pszFileName) const;
	bool CaptureReferenceScreenshot(const char* c_pszFileName) const;
	void RequestFrameDump(const std::string& filename = "frame_dump.bin");
	bool HasPendingFrameDump() const;
	const std::string& GetPendingFrameDumpPath() const;
	void ClearPendingFrameDump();

	void SetActiveBackend(IRenderBackend* pBackend);
	IRenderBackend* GetActiveBackend() const;

private:
	CRenderQueueManager(const CRenderQueueManager&) = delete;
	CRenderQueueManager& operator=(const CRenderQueueManager&) = delete;

	TRenderFrameUniforms m_frameUniforms{};
	std::vector<TSpeedTreePacket> m_speedTreePackets;
	std::vector<TUIBatchPacket> m_uiPackets;
	std::vector<TActorRenderPacket> m_actorPackets;
	std::vector<TTerrainSectorPacket> m_terrainPackets;
	std::vector<TEffectInstancePacket> m_effectPackets;
	std::vector<TWaterPacket> m_waterPackets;
	std::vector<TSkyBoxPacket> m_skyBoxPackets;
	std::vector<TDungeonBlockPacket> m_dungeonPackets;

	size_t m_flushedSpeedTreeOffset{ 0 };
	size_t m_flushedUIOffset{ 0 };

	// Monotoniczne pule danych klatki (zero alokacji i brak inwalidacji wskaznikow w trakcie klatki)
	std::vector<TParticleInstanceData> m_particleDataPool;
	size_t m_particleDataOffset{ 0 };

	std::vector<TSpeedTreeInstanceData> m_speedTreeInstancePool;
	size_t m_speedTreeInstanceOffset{ 0 };

	size_t m_droppedParticles{ 0 };
	size_t m_droppedSpeedTree{ 0 };

	IRenderBackend* m_pActiveBackend{ nullptr };
	bool m_bPendingDump{ false };
	std::string m_strPendingDumpPath;
};

inline std::vector<std::pair<std::string, size_t>> GetRenderPacketSizes()
{
	return CRenderQueueManager::GetPacketSizes();
}

inline std::map<std::string, size_t> GetRenderPacketSizeMap()
{
	return CRenderQueueManager::GetPacketSizeMap();
}
