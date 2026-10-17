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
