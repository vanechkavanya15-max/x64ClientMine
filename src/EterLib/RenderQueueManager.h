#pragma once

#include "RenderPacketTypes.h"
#include "GrpBase.h"
#include "RenderOwnership.h"
#include <vector>
#include <string>
#include <map>
#include <utility>
#include <cstddef>
#include <cstdint>
#include <memory>

struct SRenderCategoryStats
{
	uint32_t packetCount{ 0 };
	uint32_t drawCallCount{ 0 };
	uint32_t faceCount{ 0 };
	uint32_t droppedCount{ 0 };

	uint32_t GetPacketCount() const { return packetCount; }
	uint32_t GetDrawCallCount() const { return drawCallCount; }
	uint32_t GetFaceCount() const { return faceCount; }
	uint32_t GetDroppedCount() const { return droppedCount; }
};

struct SRenderFrameStats
{
	static constexpr size_t CATEGORY_COUNT = static_cast<size_t>(ERenderCategory::Count);

	SRenderCategoryStats categories[CATEGORY_COUNT]{};

	uint32_t totalPackets{ 0 };
	uint32_t totalDrawCalls{ 0 };
	uint32_t totalFaceCount{ 0 };
	uint32_t totalDropped{ 0 };

	const SRenderCategoryStats& GetCategory(ERenderCategory cat) const
	{
		size_t idx = static_cast<size_t>(cat);
		if (idx < CATEGORY_COUNT)
			return categories[idx];
		static const SRenderCategoryStats s_empty{};
		return s_empty;
	}

	SRenderCategoryStats& GetCategory(ERenderCategory cat)
	{
		size_t idx = static_cast<size_t>(cat);
		if (idx < CATEGORY_COUNT)
			return categories[idx];
		static SRenderCategoryStats s_fallback{};
		return s_fallback;
	}

	void Reset()
	{
		*this = SRenderFrameStats{};
	}
};


class IRenderBackend;
struct SSkyDescription;

constexpr uint32_t M2_RENDER_ABI_VERSION = 2;

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

	struct TReplayStats
	{
		uint32_t skyBoxCount{ 0 };
		uint32_t terrainCount{ 0 };
		uint32_t dungeonCount{ 0 };
		uint32_t waterCount{ 0 };
		uint32_t speedTreeBatchCount{ 0 };
		uint32_t actorCount{ 0 };
		uint32_t effectBatchCount{ 0 };
		uint32_t uiBatchCount{ 0 };
		uint32_t totalPackets{ 0 };
		bool bSuccess{ false };
	};

	bool ParseFrameDumpV2(const char* c_pszFileName, TDumpParsedFrame& outFrame);
	bool ValidateFrameDump(const TDumpParsedFrame& frame);
	bool ReplayFrame(const TDumpParsedFrame& frame, IRenderBackend* pBackend, TReplayStats* pOutStats = nullptr);
	bool ReplayFrameDumpV2(const char* c_pszFileName, IRenderBackend* pBackend, TReplayStats* pOutStats = nullptr);
}

struct SFrameBuffer
{
	TRenderFrameUniforms frameUniforms{};
	std::vector<TSpeedTreePacket> speedTreePackets;
	std::vector<TUIBatchPacket> uiPackets;
	std::vector<TActorRenderPacket> actorPackets;
	std::vector<TTerrainSectorPacket> terrainPackets;
	std::vector<TEffectInstancePacket> effectPackets;
	std::vector<TWaterPacket> waterPackets;
	std::vector<TSkyBoxPacket> skyBoxPackets;
	std::vector<TDungeonBlockPacket> dungeonPackets;
	std::vector<TWeaponTracePacket> weaponTracePackets;
	std::vector<TEffectMeshPacket> effectMeshPackets;
	std::unique_ptr<SSkyDescription> pSkyDescription;
	std::vector<TDebugLinePacket> debugLinePackets;

	size_t flushedSpeedTreeOffset{ 0 };
	size_t flushedUIOffset{ 0 };

	// Monotoniczne pule danych klatki (zero alokacji w trakcie klatki)
	std::vector<TParticleInstanceData> particleDataPool;
	size_t particleDataOffset{ 0 };

	std::vector<TSpeedTreeInstanceData> speedTreeInstancePool;
	size_t speedTreeInstanceOffset{ 0 };

	std::vector<TPDTVertex> weaponTraceVertexPool;
	size_t weaponTraceVertexOffset{ 0 };

	std::vector<TPTVertex> effectMeshVertexPool;
	size_t effectMeshVertexOffset{ 0 };

	std::vector<D3DXMATRIX> boneMatrixPool;
	size_t boneMatrixOffset{ 0 };

	// Dynamiczna pula klatki pod dane wskazywane (np. splatting passes, wagi)
	std::vector<uint8_t> frameDataPool;
	size_t frameDataOffset{ 0 };

	size_t droppedParticles{ 0 };
	size_t droppedSpeedTree{ 0 };
	size_t droppedWeaponTrace{ 0 };
	size_t droppedEffectMesh{ 0 };
	size_t droppedBones{ 0 };
	size_t droppedFrameData{ 0 };

	void Clear();
	size_t GetTotalPacketCount() const;

	TParticleInstanceData* AllocateParticleData(size_t count);
	TSpeedTreeInstanceData* AllocateSpeedTreeInstanceData(size_t count);
	TPDTVertex* AllocateWeaponTraceVertexData(size_t count);
	TPTVertex* AllocateEffectMeshVertexData(size_t count);
	D3DXMATRIX* AllocateBoneMatrices(size_t count);
	void* AllocateFrameData(size_t sizeInBytes, size_t alignment = 16);
};

class CRenderQueueManager
{
public:
	static constexpr uint32_t ABI_VERSION = M2_RENDER_ABI_VERSION;
	static uint32_t GetABIVersion() { return M2_RENDER_ABI_VERSION; }
	static constexpr size_t FRAME_BUFFER_COUNT = 2;

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
	void SwapFrameBuffers();

	size_t GetActiveFrameIndex() const noexcept { return m_activeFrameIndex; }
	size_t GetWorkFrameIndex() const noexcept { return m_activeFrameIndex ^ 1; }
	size_t GetRenderFrameIndex() const noexcept { return m_activeFrameIndex; }
	size_t GetProducerFrameIndex() const noexcept { return m_activeFrameIndex ^ 1; }

	const SFrameBuffer& GetActiveFrame() const noexcept { return m_frameBuffers[m_activeFrameIndex]; }
	SFrameBuffer& GetActiveFrame() noexcept { return m_frameBuffers[m_activeFrameIndex]; }
	const SFrameBuffer& GetWorkFrame() const noexcept { return m_frameBuffers[GetWorkFrameIndex()]; }
	SFrameBuffer& GetWorkFrame() noexcept { return m_frameBuffers[GetWorkFrameIndex()]; }
	const SFrameBuffer& GetRenderFrame() const noexcept { return m_frameBuffers[m_activeFrameIndex]; }
	SFrameBuffer& GetRenderFrame() noexcept { return m_frameBuffers[m_activeFrameIndex]; }
	const SFrameBuffer& GetFrame(size_t index) const { return m_frameBuffers[index % FRAME_BUFFER_COUNT]; }
	SFrameBuffer& GetFrame(size_t index) { return m_frameBuffers[index % FRAME_BUFFER_COUNT]; }

	void SetFrameUniforms(const TRenderFrameUniforms& uniforms);
	const TRenderFrameUniforms& GetFrameUniforms() const;
	const TRenderFrameUniforms& GetActiveFrameUniforms() const { return m_frameBuffers[m_activeFrameIndex].frameUniforms; }
	const TRenderFrameUniforms& GetWorkFrameUniforms() const { return m_frameBuffers[GetWorkFrameIndex()].frameUniforms; }

	void SubmitSpeedTree(const TSpeedTreePacket& packet);
	void SubmitUI(const TUIBatchPacket& packet);
	void SubmitActor(const TActorRenderPacket& packet);
	void SubmitTerrain(const TTerrainSectorPacket& packet);
	void SubmitEffect(const TEffectInstancePacket& packet);
	void SubmitWater(const TWaterPacket& packet);
	void SubmitSkyBox(const TSkyBoxPacket& packet);
	void SubmitSkyDescription(const SSkyDescription& desc);
	const SSkyDescription* GetSkyDescription() const;
	bool HasSkyDescription() const;
	void SubmitDungeonBlock(const TDungeonBlockPacket& packet);
	void SubmitEffectMesh(const TEffectMeshPacket& packet);
	void SubmitWeaponTrace(const TWeaponTracePacket& packet);
	void SubmitDebugLine(const TDebugLinePacket& packet);
	void SubmitDebugLines(const TDebugLinePacket* pPackets, size_t count);

	void Flush(IRenderBackend* pBackend = nullptr);
	void FlushUI();

	const std::vector<TSpeedTreePacket>& GetSpeedTreePackets() const;
	const std::vector<TUIBatchPacket>& GetUIPackets() const;
	const std::vector<TActorRenderPacket>& GetActorPackets() const;
	const std::vector<TTerrainSectorPacket>& GetTerrainPackets() const;
	const std::vector<TEffectInstancePacket>& GetEffectPackets() const;
	const std::vector<TEffectMeshPacket>& GetEffectMeshPackets() const;
	const std::vector<TWaterPacket>& GetWaterPackets() const;
	const std::vector<TSkyBoxPacket>& GetSkyBoxPackets() const;
	const std::vector<TDungeonBlockPacket>& GetDungeonPackets() const;
	const std::vector<TWeaponTracePacket>& GetWeaponTracePackets() const;
	const std::vector<TDebugLinePacket>& GetDebugLinePackets() const;

	const std::vector<TSpeedTreePacket>& GetActiveSpeedTreePackets() const { return m_frameBuffers[m_activeFrameIndex].speedTreePackets; }
	const std::vector<TUIBatchPacket>& GetActiveUIPackets() const { return m_frameBuffers[m_activeFrameIndex].uiPackets; }
	const std::vector<TActorRenderPacket>& GetActiveActorPackets() const { return m_frameBuffers[m_activeFrameIndex].actorPackets; }
	const std::vector<TTerrainSectorPacket>& GetActiveTerrainPackets() const { return m_frameBuffers[m_activeFrameIndex].terrainPackets; }
	const std::vector<TEffectInstancePacket>& GetActiveEffectPackets() const { return m_frameBuffers[m_activeFrameIndex].effectPackets; }
	const std::vector<TEffectMeshPacket>& GetActiveEffectMeshPackets() const { return m_frameBuffers[m_activeFrameIndex].effectMeshPackets; }
	const std::vector<TWaterPacket>& GetActiveWaterPackets() const { return m_frameBuffers[m_activeFrameIndex].waterPackets; }
	const std::vector<TSkyBoxPacket>& GetActiveSkyBoxPackets() const { return m_frameBuffers[m_activeFrameIndex].skyBoxPackets; }
	const std::vector<TDungeonBlockPacket>& GetActiveDungeonPackets() const { return m_frameBuffers[m_activeFrameIndex].dungeonPackets; }
	const std::vector<TWeaponTracePacket>& GetActiveWeaponTracePackets() const { return m_frameBuffers[m_activeFrameIndex].weaponTracePackets; }
	const std::vector<TDebugLinePacket>& GetActiveDebugLinePackets() const { return m_frameBuffers[m_activeFrameIndex].debugLinePackets; }

	const std::vector<TSpeedTreePacket>& GetWorkSpeedTreePackets() const { return m_frameBuffers[GetWorkFrameIndex()].speedTreePackets; }
	const std::vector<TUIBatchPacket>& GetWorkUIPackets() const { return m_frameBuffers[GetWorkFrameIndex()].uiPackets; }
	const std::vector<TActorRenderPacket>& GetWorkActorPackets() const { return m_frameBuffers[GetWorkFrameIndex()].actorPackets; }
	const std::vector<TTerrainSectorPacket>& GetWorkTerrainPackets() const { return m_frameBuffers[GetWorkFrameIndex()].terrainPackets; }
	const std::vector<TEffectInstancePacket>& GetWorkEffectPackets() const { return m_frameBuffers[GetWorkFrameIndex()].effectPackets; }
	const std::vector<TEffectMeshPacket>& GetWorkEffectMeshPackets() const { return m_frameBuffers[GetWorkFrameIndex()].effectMeshPackets; }
	const std::vector<TWaterPacket>& GetWorkWaterPackets() const { return m_frameBuffers[GetWorkFrameIndex()].waterPackets; }
	const std::vector<TSkyBoxPacket>& GetWorkSkyBoxPackets() const { return m_frameBuffers[GetWorkFrameIndex()].skyBoxPackets; }
	const std::vector<TDungeonBlockPacket>& GetWorkDungeonPackets() const { return m_frameBuffers[GetWorkFrameIndex()].dungeonPackets; }
	const std::vector<TWeaponTracePacket>& GetWorkWeaponTracePackets() const { return m_frameBuffers[GetWorkFrameIndex()].weaponTracePackets; }
	const std::vector<TDebugLinePacket>& GetWorkDebugLinePackets() const { return m_frameBuffers[GetWorkFrameIndex()].debugLinePackets; }

	size_t GetTotalPacketCount() const;
	const SFrameBuffer& GetReadableFrameBuffer() const;

	TParticleInstanceData* AllocateParticleData(size_t count);
	TSpeedTreeInstanceData* AllocateSpeedTreeInstanceData(size_t count);
	TPDTVertex* AllocateWeaponTraceVertexData(size_t count);
	const TPDTVertex* GetWeaponTraceVertexData() const;
	TPTVertex* AllocateEffectMeshVertexData(size_t count);
	const TPTVertex* GetEffectMeshVertexData() const;
	D3DXMATRIX* AllocateBoneMatrices(size_t count);
	const D3DXMATRIX* GetBoneMatrixData() const;

	void* AllocateFrameData(size_t sizeInBytes, size_t alignment = 16);
	template<typename T>
	T* AllocateFrameData(size_t count = 1)
	{
		return static_cast<T*>(AllocateFrameData(sizeof(T) * count, alignof(T)));
	}

	const void* GetActiveFrameData() const;
	const void* GetWorkFrameData() const;

	size_t GetDroppedParticles() const;
	size_t GetDroppedSpeedTree() const;
	size_t GetDroppedWeaponTrace() const;
	size_t GetDroppedEffectMesh() const;
	size_t GetDroppedBones() const;
	size_t GetDroppedFrameData() const;
	void ResetDroppedCounters();

	size_t GetParticlePoolCapacity() const;
	size_t GetSpeedTreePoolCapacity() const;
	size_t GetWeaponTracePoolCapacity() const;
	size_t GetEffectMeshPoolCapacity() const;
	size_t GetBoneMatrixPoolCapacity() const;
	size_t GetFrameDataPoolCapacity() const;

	size_t GetParticleDataOffset() const;
	size_t GetSpeedTreeInstanceOffset() const;
	size_t GetWeaponTraceVertexOffset() const;
	size_t GetEffectMeshVertexOffset() const;
	size_t GetBoneMatrixOffset() const;
	size_t GetFrameDataOffset() const;

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

	// Render statistics (S7.4)
	void ResetFrameStats();
	void RecordDrawCall(ERenderCategory category, uint32_t faceCount);
	void RecordPacket(ERenderCategory category, uint32_t count = 1);
	void RecordDroppedPacket(ERenderCategory category, uint32_t count = 1);
	const SRenderFrameStats& GetFrameStats() const { return m_frameStats; }

private:
	CRenderQueueManager(const CRenderQueueManager&) = delete;
	CRenderQueueManager& operator=(const CRenderQueueManager&) = delete;

	SRenderFrameStats m_frameStats{};

	SFrameBuffer m_frameBuffers[FRAME_BUFFER_COUNT];
	size_t m_activeFrameIndex{ 0 };
	bool m_bHasSwappedFrame{ false };


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
