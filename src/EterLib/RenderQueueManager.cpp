#include "StdAfx.h"
#include "RenderQueueManager.h"
#include "IRenderBackend.h"
#include <cstdio>
#include <algorithm>

static constexpr size_t DEFAULT_PARTICLE_POOL_CAPACITY = 65536;
static constexpr size_t DEFAULT_SPEEDTREE_POOL_CAPACITY = 16384;

std::vector<std::pair<std::string, size_t>> CRenderQueueManager::GetPacketSizes()
{
	return {
		{ "TRenderFrameUniforms", sizeof(TRenderFrameUniforms) },
		{ "TTerrainSectorPacket", sizeof(TTerrainSectorPacket) },
		{ "TActorRenderPacket", sizeof(TActorRenderPacket) },
		{ "TWaterPacket", sizeof(TWaterPacket) },
		{ "TSkyBoxPacket", sizeof(TSkyBoxPacket) },
		{ "TSpeedTreeInstanceData", sizeof(TSpeedTreeInstanceData) },
		{ "TSpeedTreePacket", sizeof(TSpeedTreePacket) },
		{ "TParticleInstanceData", sizeof(TParticleInstanceData) },
		{ "TEffectInstancePacket", sizeof(TEffectInstancePacket) },
		{ "TUIVertex", sizeof(TUIVertex) },
		{ "TUIBatchPacket", sizeof(TUIBatchPacket) },
		{ "TDungeonBlockPacket", sizeof(TDungeonBlockPacket) },
		{ "TRenderFramePacket", sizeof(TRenderFramePacket) }
	};
}

std::map<std::string, size_t> CRenderQueueManager::GetPacketSizeMap()
{
	const auto list = GetPacketSizes();
	std::map<std::string, size_t> mapSizes;
	for (const auto& kv : list)
	{
		mapSizes[kv.first] = kv.second;
	}
	return mapSizes;
}

CRenderQueueManager::CRenderQueueManager()
{
	m_speedTreePackets.reserve(256);
	m_uiPackets.reserve(128);
	m_actorPackets.reserve(512);
	m_terrainPackets.reserve(64);
	m_effectPackets.reserve(256);
	m_waterPackets.reserve(32);
	m_skyBoxPackets.reserve(8);
	m_dungeonPackets.reserve(256);

	m_particleDataPool.resize(DEFAULT_PARTICLE_POOL_CAPACITY);
	m_particleDataOffset = 0;

	m_speedTreeInstancePool.resize(DEFAULT_SPEEDTREE_POOL_CAPACITY);
	m_speedTreeInstanceOffset = 0;

	m_droppedParticles = 0;
	m_droppedSpeedTree = 0;

	memset(&m_frameUniforms, 0, sizeof(TRenderFrameUniforms));
}

CRenderQueueManager::~CRenderQueueManager() = default;

void CRenderQueueManager::BeginFrame()
{
	m_speedTreePackets.clear();
	m_uiPackets.clear();
	m_actorPackets.clear();
	m_terrainPackets.clear();
	m_effectPackets.clear();
	m_waterPackets.clear();
	m_skyBoxPackets.clear();
	m_dungeonPackets.clear();
	m_flushedSpeedTreeOffset = 0;
	m_flushedUIOffset = 0;
	m_particleDataOffset = 0;
	m_speedTreeInstanceOffset = 0;
	m_droppedParticles = 0;
	m_droppedSpeedTree = 0;
}

void CRenderQueueManager::EndFrame()
{
	if (HasPendingFrameDump())
	{
		ExportFrameToBinary(GetPendingFrameDumpPath().c_str());
		CaptureReferenceScreenshot("cpp_reference.png");
		ClearPendingFrameDump();
	}
}

void CRenderQueueManager::SetFrameUniforms(const TRenderFrameUniforms& uniforms)
{
	m_frameUniforms = uniforms;
}

const TRenderFrameUniforms& CRenderQueueManager::GetFrameUniforms() const
{
	return m_frameUniforms;
}

void CRenderQueueManager::SubmitSpeedTree(const TSpeedTreePacket& packet)
{
	m_speedTreePackets.push_back(packet);
}

void CRenderQueueManager::SubmitUI(const TUIBatchPacket& packet)
{
	m_uiPackets.push_back(packet);
}

void CRenderQueueManager::SubmitActor(const TActorRenderPacket& packet)
{
	m_actorPackets.push_back(packet);
}

void CRenderQueueManager::SubmitTerrain(const TTerrainSectorPacket& packet)
{
	m_terrainPackets.push_back(packet);
}

void CRenderQueueManager::SubmitEffect(const TEffectInstancePacket& packet)
{
	m_effectPackets.push_back(packet);
}

void CRenderQueueManager::SubmitWater(const TWaterPacket& packet)
{
	m_waterPackets.push_back(packet);
}

void CRenderQueueManager::SubmitSkyBox(const TSkyBoxPacket& packet)
{
	m_skyBoxPackets.push_back(packet);
}

void CRenderQueueManager::SubmitDungeonBlock(const TDungeonBlockPacket& packet)
{
	m_dungeonPackets.push_back(packet);
}

void CRenderQueueManager::Flush(IRenderBackend* pBackend)
{
	if (!pBackend)
		pBackend = GetActiveBackend();
	if (!pBackend)
		return;

	pBackend->BeginFrame(m_frameUniforms);

	if (!m_skyBoxPackets.empty())
		pBackend->RenderSkyBox(m_skyBoxPackets.data(), static_cast<unsigned int>(m_skyBoxPackets.size()));

	if (!m_terrainPackets.empty())
		pBackend->RenderTerrain(m_terrainPackets.data(), static_cast<unsigned int>(m_terrainPackets.size()));

	if (!m_dungeonPackets.empty())
		pBackend->RenderDungeonBlocks(m_dungeonPackets.data(), static_cast<unsigned int>(m_dungeonPackets.size()));

	if (!m_waterPackets.empty())
		pBackend->RenderWater(m_waterPackets.data(), static_cast<unsigned int>(m_waterPackets.size()));

	if (m_flushedSpeedTreeOffset < m_speedTreePackets.size())
	{
		pBackend->RenderSpeedTree(
			m_speedTreePackets.data() + m_flushedSpeedTreeOffset,
			static_cast<unsigned int>(m_speedTreePackets.size() - m_flushedSpeedTreeOffset)
		);
		m_flushedSpeedTreeOffset = m_speedTreePackets.size();
	}

	if (!m_actorPackets.empty())
		pBackend->RenderActors(m_actorPackets.data(), static_cast<unsigned int>(m_actorPackets.size()));

	if (!m_effectPackets.empty())
		pBackend->RenderEffects(m_effectPackets.data(), static_cast<unsigned int>(m_effectPackets.size()));

	if (m_flushedUIOffset < m_uiPackets.size())
	{
		pBackend->RenderUI(
			m_uiPackets.data() + m_flushedUIOffset,
			static_cast<unsigned int>(m_uiPackets.size() - m_flushedUIOffset)
		);
		m_flushedUIOffset = m_uiPackets.size();
	}

	pBackend->EndFrame();
}

void CRenderQueueManager::FlushUI()
{
	if (m_flushedUIOffset >= m_uiPackets.size())
		return;

	IRenderBackend* pBackend = GetActiveBackend();
	if (pBackend)
	{
		pBackend->RenderUI(
			m_uiPackets.data() + m_flushedUIOffset,
			static_cast<unsigned int>(m_uiPackets.size() - m_flushedUIOffset)
		);
	}
	m_flushedUIOffset = m_uiPackets.size();
}

const std::vector<TSpeedTreePacket>& CRenderQueueManager::GetSpeedTreePackets() const
{
	return m_speedTreePackets;
}

const std::vector<TUIBatchPacket>& CRenderQueueManager::GetUIPackets() const
{
	return m_uiPackets;
}

const std::vector<TActorRenderPacket>& CRenderQueueManager::GetActorPackets() const
{
	return m_actorPackets;
}

const std::vector<TTerrainSectorPacket>& CRenderQueueManager::GetTerrainPackets() const
{
	return m_terrainPackets;
}

const std::vector<TEffectInstancePacket>& CRenderQueueManager::GetEffectPackets() const
{
	return m_effectPackets;
}

const std::vector<TWaterPacket>& CRenderQueueManager::GetWaterPackets() const
{
	return m_waterPackets;
}

const std::vector<TSkyBoxPacket>& CRenderQueueManager::GetSkyBoxPackets() const
{
	return m_skyBoxPackets;
}

const std::vector<TDungeonBlockPacket>& CRenderQueueManager::GetDungeonPackets() const
{
	return m_dungeonPackets;
}

size_t CRenderQueueManager::GetTotalPacketCount() const
{
	return m_speedTreePackets.size() + m_uiPackets.size() + m_actorPackets.size() +
		m_terrainPackets.size() + m_effectPackets.size() + m_waterPackets.size() +
		m_skyBoxPackets.size() + m_dungeonPackets.size();
}

TParticleInstanceData* CRenderQueueManager::AllocateParticleData(size_t count)
{
	if (count == 0)
		return nullptr;

	if (m_particleDataOffset + count > m_particleDataPool.size())
	{
		m_droppedParticles += count;
		TraceError("CRenderQueueManager::AllocateParticleData: Pool overflow! requested=%zu, remaining=%zu, capacity=%zu, droppedTotal=%zu",
			count, m_particleDataPool.size() - m_particleDataOffset, m_particleDataPool.size(), m_droppedParticles);
		return nullptr;
	}

	TParticleInstanceData* pResult = &m_particleDataPool[m_particleDataOffset];
	m_particleDataOffset += count;
	return pResult;
}

TSpeedTreeInstanceData* CRenderQueueManager::AllocateSpeedTreeInstanceData(size_t count)
{
	if (count == 0)
		return nullptr;

	if (m_speedTreeInstanceOffset + count > m_speedTreeInstancePool.size())
	{
		m_droppedSpeedTree += count;
		TraceError("CRenderQueueManager::AllocateSpeedTreeInstanceData: Pool overflow! requested=%zu, remaining=%zu, capacity=%zu, droppedTotal=%zu",
			count, m_speedTreeInstancePool.size() - m_speedTreeInstanceOffset, m_speedTreeInstancePool.size(), m_droppedSpeedTree);
		return nullptr;
	}

	TSpeedTreeInstanceData* pResult = &m_speedTreeInstancePool[m_speedTreeInstanceOffset];
	m_speedTreeInstanceOffset += count;
	return pResult;
}

bool CRenderQueueManager::ExportFrameToBinary(const char* c_pszFileName) const
{
	if (!c_pszFileName)
		return false;

	FILE* fp = nullptr;
	if (fopen_s(&fp, c_pszFileName, "wb") != 0 || !fp)
		return false;

	uint32_t magic = 0x5246324D; // "M2FR"
	uint32_t version = 1;
	uint32_t reserved[2] = { 0, 0 };
	fwrite(&magic, sizeof(uint32_t), 1, fp);
	fwrite(&version, sizeof(uint32_t), 1, fp);
	fwrite(reserved, sizeof(uint32_t), 2, fp);

	fwrite(&m_frameUniforms, sizeof(TRenderFrameUniforms), 1, fp);

	uint32_t terrainCount = static_cast<uint32_t>(m_terrainPackets.size());
	fwrite(&terrainCount, sizeof(uint32_t), 1, fp);
	for (const auto& pkt : m_terrainPackets)
	{
		TTerrainSectorPacket sanitized = pkt;
		sanitized.pSplatTexture = nullptr;
		for (int l = 0; l < 4; ++l)
			sanitized.pLayerTextures[l] = nullptr;
		sanitized.pVertexData = nullptr;
		sanitized.pSplatWeights = nullptr;
		sanitized.pHeightMap = nullptr;
		sanitized.pTileMap = nullptr;
		sanitized.pNormalMap = nullptr;
		fwrite(&sanitized, sizeof(TTerrainSectorPacket), 1, fp);
	}

	uint32_t waterCount = static_cast<uint32_t>(m_waterPackets.size());
	fwrite(&waterCount, sizeof(uint32_t), 1, fp);
	for (const auto& pkt : m_waterPackets)
	{
		TWaterPacket sanitized = pkt;
		sanitized.pWaterTexture = nullptr;
		sanitized.pVertexBuffer = nullptr;
		fwrite(&sanitized, sizeof(TWaterPacket), 1, fp);
	}

	uint32_t treeCount = static_cast<uint32_t>(m_speedTreePackets.size());
	fwrite(&treeCount, sizeof(uint32_t), 1, fp);
	for (const auto& pkt : m_speedTreePackets)
	{
		fwrite(&pkt.dwTreeCRC, sizeof(uint32_t), 1, fp);
		uint32_t flags = (pkt.bRenderBranches ? 1 : 0) |
			((pkt.bRenderFronds ? 1 : 0) << 1) |
			((pkt.bRenderLeaves ? 1 : 0) << 2) |
			((pkt.bRenderBillboards ? 1 : 0) << 3);
		fwrite(&flags, sizeof(uint32_t), 1, fp);
		fwrite(&pkt.dwTreeTypeID, sizeof(uint32_t), 1, fp);
		fwrite(&pkt.dwTrunkTextureID, sizeof(uint32_t), 1, fp);
		fwrite(&pkt.dwLeafTextureID, sizeof(uint32_t), 1, fp);

		uint32_t instCount = (pkt.pInstances != nullptr) ? pkt.dwInstanceCount : 0;
		fwrite(&instCount, sizeof(uint32_t), 1, fp);
		for (uint32_t idx = 0; idx < instCount; ++idx)
		{
			TSpeedTreeInstanceData sanitized = pkt.pInstances[idx];
			sanitized.pTreeInstance = nullptr;
			fwrite(&sanitized, sizeof(TSpeedTreeInstanceData), 1, fp);
		}
	}

	uint32_t actorCount = static_cast<uint32_t>(m_actorPackets.size());
	fwrite(&actorCount, sizeof(uint32_t), 1, fp);
	for (const auto& pkt : m_actorPackets)
	{
		TActorRenderPacket sanitized = pkt;
		sanitized.pBoneMatrices = nullptr;
		fwrite(&sanitized, sizeof(TActorRenderPacket), 1, fp);
	}

	uint32_t effectCount = static_cast<uint32_t>(m_effectPackets.size());
	fwrite(&effectCount, sizeof(uint32_t), 1, fp);
	for (const auto& pkt : m_effectPackets)
	{
		fwrite(&pkt.dwEffectID, sizeof(uint32_t), 1, fp);
		uint32_t texId32 = static_cast<uint32_t>(pkt.dwTextureID & 0xFFFFFFFF);
		fwrite(&texId32, sizeof(uint32_t), 1, fp);
		fwrite(&pkt.dwBlendMode, sizeof(uint32_t), 1, fp);
		fwrite(&pkt.dwParticleCount, sizeof(uint32_t), 1, fp);
		if (pkt.dwParticleCount > 0 && pkt.pParticles)
		{
			fwrite(pkt.pParticles, sizeof(TParticleInstanceData), pkt.dwParticleCount, fp);
		}
	}

	uint32_t uiCount = static_cast<uint32_t>(m_uiPackets.size());
	fwrite(&uiCount, sizeof(uint32_t), 1, fp);
	for (const auto& pkt : m_uiPackets)
	{
		uint32_t texId = static_cast<uint32_t>(pkt.dwTextureID & 0xFFFFFFFF);
		fwrite(&texId, sizeof(uint32_t), 1, fp);
		uint32_t scissor = pkt.bUseScissor ? 1 : 0;
		fwrite(&scissor, sizeof(uint32_t), 1, fp);
		fwrite(&pkt.scissorRect, sizeof(RECT), 1, fp);
		fwrite(&pkt.dwBlendMode, sizeof(uint32_t), 1, fp);

		uint32_t vtxCount = pkt.dwVertexCount;
		const TUIVertex* pVtx = pkt.pVertices;
		fwrite(&vtxCount, sizeof(uint32_t), 1, fp);
		if (vtxCount > 0 && pVtx)
		{
			fwrite(pVtx, sizeof(TUIVertex), vtxCount, fp);
		}

		uint32_t idxCount = 0;
		fwrite(&idxCount, sizeof(uint32_t), 1, fp);
	}

	fclose(fp);
	return true;
}

void CRenderQueueManager::RequestFrameDump(const std::string& filename)
{
	m_bPendingDump = true;
	m_strPendingDumpPath = filename;
}

bool CRenderQueueManager::HasPendingFrameDump() const
{
	return m_bPendingDump;
}

const std::string& CRenderQueueManager::GetPendingFrameDumpPath() const
{
	return m_strPendingDumpPath;
}

void CRenderQueueManager::ClearPendingFrameDump()
{
	m_bPendingDump = false;
	m_strPendingDumpPath.clear();
}

bool CRenderQueueManager::CaptureReferenceScreenshot(const char* c_pszFileName) const
{
	if (!c_pszFileName)
		return false;

	IRenderBackend* pBackend = GetActiveBackend();
	if (!pBackend)
		return false;

	return pBackend->CaptureFrame(c_pszFileName);
}

void CRenderQueueManager::SetActiveBackend(IRenderBackend* pBackend)
{
	m_pActiveBackend = pBackend;
}

IRenderBackend* CRenderQueueManager::GetActiveBackend() const
{
	return m_pActiveBackend;
}
