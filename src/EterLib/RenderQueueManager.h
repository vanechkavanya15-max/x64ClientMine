#pragma once

#include "RenderPacketTypes.h"
#include <vector>
#include <string>
#include <cstddef>

class IRenderBackend;

class CRenderQueueManager
{
public:
	static CRenderQueueManager& Instance()
	{
		static CRenderQueueManager s_instance;
		return s_instance;
	}

	CRenderQueueManager();
	~CRenderQueueManager();

	void BeginFrame();
	void EndFrame();
	void Clear();

	void SetFrameUniforms(const TRenderFrameUniforms& uniforms);
	const TRenderFrameUniforms& GetFrameUniforms() const;

	void SubmitSpeedTree(const TSpeedTreePacket& packet);
	void SubmitUI(const TUIBatchPacket& packet);
	void SubmitActor(const TActorRenderPacket& packet);
	void SubmitTerrain(const TTerrainSectorPacket& packet);
	void SubmitEffect(const TEffectInstancePacket& packet);
	void SubmitWater(const TWaterPacket& packet);

	void Flush(IRenderBackend* pBackend);
	void FlushSpeedTree();
	void FlushUI();

	const std::vector<TSpeedTreePacket>& GetSpeedTreePackets() const;
	const std::vector<TUIBatchPacket>& GetUIPackets() const;
	const std::vector<TActorRenderPacket>& GetActorPackets() const;
	const std::vector<TTerrainSectorPacket>& GetTerrainPackets() const;
	const std::vector<TEffectInstancePacket>& GetEffectPackets() const;
	const std::vector<TWaterPacket>& GetWaterPackets() const;

	size_t GetTotalPacketCount() const;

	TParticleInstanceData* AllocateParticleData(size_t count);
	void ClearParticleData();

	// Golden Master Frame Dumper (T036)
	bool ExportFrameToBinary(const char* c_pszFileName) const;
	void RequestFrameDump(const std::string& filename = "frame_dump.bin");
	bool HasPendingFrameDump() const;
	const std::string& GetPendingFrameDumpPath() const;
	void ClearPendingFrameDump();

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
	std::vector<TParticleInstanceData> m_particleDataPool;

	bool m_bPendingDump{ false };
	std::string m_strPendingDumpPath;
};
