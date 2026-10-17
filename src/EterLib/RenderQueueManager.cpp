#include "StdAfx.h"
#include "RenderQueueManager.h"
#include "IRenderBackend.h"
#include "SkyBox.h"
#include <cstdio>
#include <algorithm>

static constexpr size_t DEFAULT_PARTICLE_POOL_CAPACITY = 65536;
static constexpr size_t DEFAULT_SPEEDTREE_POOL_CAPACITY = 16384;
static constexpr size_t DEFAULT_WEAPON_TRACE_VERTEX_CAPACITY = 16384;

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
	m_weaponTracePackets.reserve(64);

	m_particleDataPool.resize(DEFAULT_PARTICLE_POOL_CAPACITY);
	m_particleDataOffset = 0;

	m_speedTreeInstancePool.resize(DEFAULT_SPEEDTREE_POOL_CAPACITY);
	m_speedTreeInstanceOffset = 0;

	m_weaponTraceVertexPool.resize(DEFAULT_WEAPON_TRACE_VERTEX_CAPACITY);
	m_weaponTraceVertexOffset = 0;

	m_droppedParticles = 0;
	m_droppedSpeedTree = 0;
	m_droppedWeaponTrace = 0;

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
	m_weaponTracePackets.clear();
	m_pSkyDescription.reset();
	m_flushedSpeedTreeOffset = 0;
	m_flushedUIOffset = 0;
	m_particleDataOffset = 0;
	m_speedTreeInstanceOffset = 0;
	m_weaponTraceVertexOffset = 0;
	m_droppedParticles = 0;
	m_droppedSpeedTree = 0;
	m_droppedWeaponTrace = 0;
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

void CRenderQueueManager::SubmitSkyDescription(const SSkyDescription& desc)
{
	m_pSkyDescription = std::make_unique<SSkyDescription>(desc);
}

const SSkyDescription* CRenderQueueManager::GetSkyDescription() const
{
	return m_pSkyDescription.get();
}

bool CRenderQueueManager::HasSkyDescription() const
{
	return m_pSkyDescription != nullptr;
}

void CRenderQueueManager::SubmitDungeonBlock(const TDungeonBlockPacket& packet)
{
	m_dungeonPackets.push_back(packet);
}

void CRenderQueueManager::SubmitWeaponTrace(const TWeaponTracePacket& packet)
{
	m_weaponTracePackets.push_back(packet);
}

const std::vector<TWeaponTracePacket>& CRenderQueueManager::GetWeaponTracePackets() const
{
	return m_weaponTracePackets;
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

	if (!m_weaponTracePackets.empty())
		pBackend->RenderWeaponTrace(m_weaponTracePackets.data(), static_cast<unsigned int>(m_weaponTracePackets.size()));

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

TPDTVertex* CRenderQueueManager::AllocateWeaponTraceVertexData(size_t count)
{
	if (count == 0)
		return nullptr;

	if (m_weaponTraceVertexOffset + count > m_weaponTraceVertexPool.size())
	{
		m_droppedWeaponTrace += count;
		TraceError("CRenderQueueManager::AllocateWeaponTraceVertexData: Pool overflow! requested=%zu, remaining=%zu, capacity=%zu, droppedTotal=%zu",
			count, m_weaponTraceVertexPool.size() - m_weaponTraceVertexOffset, m_weaponTraceVertexPool.size(), m_droppedWeaponTrace);
		return nullptr;
	}

	TPDTVertex* pResult = &m_weaponTraceVertexPool[m_weaponTraceVertexOffset];
	m_weaponTraceVertexOffset += count;
	return pResult;
}

const TPDTVertex* CRenderQueueManager::GetWeaponTraceVertexData() const
{
	return m_weaponTraceVertexPool.empty() ? nullptr : m_weaponTraceVertexPool.data();
}

bool CRenderQueueManager::ExportFrameToBinary(const char* c_pszFileName) const
{
	return ExportFrameToBinaryV2(c_pszFileName);
}

bool CRenderQueueManager::ExportFrameToBinaryV2(const char* c_pszFileName) const
{
	if (!c_pszFileName)
		return false;

	FILE* fp = nullptr;
	if (fopen_s(&fp, c_pszFileName, "wb") != 0 || !fp)
		return false;

	// 1. File Header
	FrameDumpV2::THeader fileHdr{};
	fileHdr.dwMagic = FrameDumpV2::M2FR_MAGIC;
	fileHdr.dwVersion = FrameDumpV2::M2FR_VERSION;
	fileHdr.dwChunkCount = 9;
	fileHdr.dwReserved = 0;
	fwrite(&fileHdr, sizeof(fileHdr), 1, fp);

	// 2. Chunk: UNIFORMS ('UNIF')
	{
		FrameDumpV2::TChunkHeader chunkHdr{};
		chunkHdr.dwTag = FrameDumpV2::CHUNK_UNIFORMS;
		chunkHdr.dwCount = 1;
		chunkHdr.dwElementSize = static_cast<uint32_t>(sizeof(TRenderFrameUniforms));
		chunkHdr.dwDataSize = chunkHdr.dwElementSize;
		fwrite(&chunkHdr, sizeof(chunkHdr), 1, fp);

		TRenderFrameUniforms sanitized{};
		std::memset(&sanitized, 0, sizeof(sanitized));
		sanitized = m_frameUniforms;
		fwrite(&sanitized, sizeof(sanitized), 1, fp);
	}

	// 3. Chunk: SKYBOX ('SKYB')
	{
		FrameDumpV2::TChunkHeader chunkHdr{};
		chunkHdr.dwTag = FrameDumpV2::CHUNK_SKYBOX;
		chunkHdr.dwCount = static_cast<uint32_t>(m_skyBoxPackets.size());
		chunkHdr.dwElementSize = static_cast<uint32_t>(sizeof(TSkyBoxPacket));
		chunkHdr.dwDataSize = chunkHdr.dwCount * chunkHdr.dwElementSize;
		fwrite(&chunkHdr, sizeof(chunkHdr), 1, fp);

		for (const auto& pkt : m_skyBoxPackets)
		{
			TSkyBoxPacket sanitized{};
			std::memset(&sanitized, 0, sizeof(sanitized));
			sanitized = pkt;
			sanitized.fPad = 0.0f;
			fwrite(&sanitized, sizeof(TSkyBoxPacket), 1, fp);
		}
	}

	// 4. Chunk: TERRAIN ('TERR')
	{
		FrameDumpV2::TChunkHeader chunkHdr{};
		chunkHdr.dwTag = FrameDumpV2::CHUNK_TERRAIN;
		chunkHdr.dwCount = static_cast<uint32_t>(m_terrainPackets.size());
		chunkHdr.dwElementSize = static_cast<uint32_t>(sizeof(TTerrainSectorPacket));
		chunkHdr.dwDataSize = chunkHdr.dwCount * chunkHdr.dwElementSize;
		fwrite(&chunkHdr, sizeof(chunkHdr), 1, fp);

		for (const auto& pkt : m_terrainPackets)
		{
			TTerrainSectorPacket sanitized{};
			std::memset(&sanitized, 0, sizeof(sanitized));
			sanitized = pkt;
			sanitized.pSplatTexture = nullptr;
			for (int l = 0; l < 4; ++l)
				sanitized.pLayerTextures[l] = nullptr;
			sanitized.pVertexData = nullptr;
			sanitized.pSplatWeights = nullptr;
			sanitized.pHeightMap = nullptr;
			sanitized.pTileMap = nullptr;
			sanitized.pNormalMap = nullptr;
			sanitized.fReserved = 0.0f;
			fwrite(&sanitized, sizeof(TTerrainSectorPacket), 1, fp);
		}
	}

	// 5. Chunk: DUNGEON ('DUNG')
	{
		FrameDumpV2::TChunkHeader chunkHdr{};
		chunkHdr.dwTag = FrameDumpV2::CHUNK_DUNGEON;
		chunkHdr.dwCount = static_cast<uint32_t>(m_dungeonPackets.size());
		chunkHdr.dwElementSize = static_cast<uint32_t>(sizeof(TDungeonBlockPacket));
		chunkHdr.dwDataSize = chunkHdr.dwCount * chunkHdr.dwElementSize;
		fwrite(&chunkHdr, sizeof(chunkHdr), 1, fp);

		for (const auto& pkt : m_dungeonPackets)
		{
			TDungeonBlockPacket sanitized{};
			std::memset(&sanitized, 0, sizeof(sanitized));
			sanitized = pkt;
			sanitized.pDiffuseTexture = nullptr;
			sanitized.pLightmapTexture = nullptr;
			sanitized.pVertexBuffer = nullptr;
			sanitized.pIndexBuffer = nullptr;
			sanitized.pVertices = nullptr;
			sanitized.pIndices = nullptr;
			sanitized.bPadDungeon[0] = 0;
			sanitized.bPadDungeon[1] = 0;
			sanitized.bPadDungeon[2] = 0;
			sanitized.fPad = 0.0f;
			fwrite(&sanitized, sizeof(TDungeonBlockPacket), 1, fp);
		}
	}

	// 6. Chunk: WATER ('WATR')
	{
		FrameDumpV2::TChunkHeader chunkHdr{};
		chunkHdr.dwTag = FrameDumpV2::CHUNK_WATER;
		chunkHdr.dwCount = static_cast<uint32_t>(m_waterPackets.size());
		chunkHdr.dwElementSize = static_cast<uint32_t>(sizeof(TWaterPacket));
		chunkHdr.dwDataSize = chunkHdr.dwCount * chunkHdr.dwElementSize;
		fwrite(&chunkHdr, sizeof(chunkHdr), 1, fp);

		for (const auto& pkt : m_waterPackets)
		{
			TWaterPacket sanitized{};
			std::memset(&sanitized, 0, sizeof(sanitized));
			sanitized = pkt;
			sanitized.pWaterTexture = nullptr;
			sanitized.pVertexBuffer = nullptr;
			sanitized.fPad = 0.0f;
			fwrite(&sanitized, sizeof(TWaterPacket), 1, fp);
		}
	}

	// 7. Chunk: SPEEDTREE ('TREE')
	{
		uint32_t treeDataSize = 0;
		for (const auto& pkt : m_speedTreePackets)
		{
			uint32_t instCount = (pkt.pInstances != nullptr) ? pkt.dwInstanceCount : 0;
			treeDataSize += static_cast<uint32_t>(sizeof(FrameDumpV2::TSpeedTreeBatchHeader) + instCount * sizeof(TSpeedTreeInstanceData));
		}

		FrameDumpV2::TChunkHeader chunkHdr{};
		chunkHdr.dwTag = FrameDumpV2::CHUNK_SPEEDTREE;
		chunkHdr.dwCount = static_cast<uint32_t>(m_speedTreePackets.size());
		chunkHdr.dwElementSize = 0;
		chunkHdr.dwDataSize = treeDataSize;
		fwrite(&chunkHdr, sizeof(chunkHdr), 1, fp);

		for (const auto& pkt : m_speedTreePackets)
		{
			FrameDumpV2::TSpeedTreeBatchHeader batchHdr{};
			batchHdr.dwTreeCRC = pkt.dwTreeCRC;
			batchHdr.dwFlags = (pkt.bRenderBranches ? 1 : 0) |
				((pkt.bRenderFronds ? 1 : 0) << 1) |
				((pkt.bRenderLeaves ? 1 : 0) << 2) |
				((pkt.bRenderBillboards ? 1 : 0) << 3);
			batchHdr.uTreeTypeID = static_cast<uint64_t>(pkt.dwTreeTypeID);
			batchHdr.uTrunkTextureID = static_cast<uint64_t>(pkt.dwTrunkTextureID);
			batchHdr.uLeafTextureID = static_cast<uint64_t>(pkt.dwLeafTextureID);
			uint32_t instCount = (pkt.pInstances != nullptr) ? pkt.dwInstanceCount : 0;
			batchHdr.dwInstanceCount = instCount;
			batchHdr.dwPad = 0;
			fwrite(&batchHdr, sizeof(batchHdr), 1, fp);

			for (uint32_t idx = 0; idx < instCount; ++idx)
			{
				TSpeedTreeInstanceData sanitized{};
				std::memset(&sanitized, 0, sizeof(sanitized));
				sanitized = pkt.pInstances[idx];
				sanitized.pTreeInstance = nullptr;
				sanitized.fPad[0] = 0.0f;
				sanitized.fPad[1] = 0.0f;
				sanitized.fPad[2] = 0.0f;
				fwrite(&sanitized, sizeof(TSpeedTreeInstanceData), 1, fp);
			}
		}
	}

	// 8. Chunk: ACTORS ('ACTR')
	{
		FrameDumpV2::TChunkHeader chunkHdr{};
		chunkHdr.dwTag = FrameDumpV2::CHUNK_ACTORS;
		chunkHdr.dwCount = static_cast<uint32_t>(m_actorPackets.size());
		chunkHdr.dwElementSize = static_cast<uint32_t>(sizeof(TActorRenderPacket));
		chunkHdr.dwDataSize = chunkHdr.dwCount * chunkHdr.dwElementSize;
		fwrite(&chunkHdr, sizeof(chunkHdr), 1, fp);

		for (const auto& pkt : m_actorPackets)
		{
			TActorRenderPacket sanitized{};
			std::memset(&sanitized, 0, sizeof(sanitized));
			sanitized = pkt;
			sanitized.pBoneMatrices = nullptr;
			fwrite(&sanitized, sizeof(TActorRenderPacket), 1, fp);
		}
	}

	// 9. Chunk: EFFECTS ('EFCT')
	{
		uint32_t effectDataSize = 0;
		for (const auto& pkt : m_effectPackets)
		{
			uint32_t partCount = (pkt.pParticles != nullptr) ? pkt.dwParticleCount : 0;
			effectDataSize += static_cast<uint32_t>(sizeof(FrameDumpV2::TEffectBatchHeader) + partCount * sizeof(TParticleInstanceData));
		}

		FrameDumpV2::TChunkHeader chunkHdr{};
		chunkHdr.dwTag = FrameDumpV2::CHUNK_EFFECTS;
		chunkHdr.dwCount = static_cast<uint32_t>(m_effectPackets.size());
		chunkHdr.dwElementSize = 0;
		chunkHdr.dwDataSize = effectDataSize;
		fwrite(&chunkHdr, sizeof(chunkHdr), 1, fp);

		for (const auto& pkt : m_effectPackets)
		{
			FrameDumpV2::TEffectBatchHeader batchHdr{};
			batchHdr.dwEffectID = pkt.dwEffectID;
			batchHdr.dwBlendMode = pkt.dwBlendMode;
			batchHdr.uTextureID = static_cast<uint64_t>(pkt.dwTextureID);
			uint32_t partCount = (pkt.pParticles != nullptr) ? pkt.dwParticleCount : 0;
			batchHdr.dwParticleCount = partCount;
			batchHdr.dwPad = 0;
			fwrite(&batchHdr, sizeof(batchHdr), 1, fp);

			for (uint32_t idx = 0; idx < partCount; ++idx)
			{
				TParticleInstanceData sanitized{};
				std::memset(&sanitized, 0, sizeof(sanitized));
				sanitized = pkt.pParticles[idx];
				fwrite(&sanitized, sizeof(TParticleInstanceData), 1, fp);
			}
		}
	}

	// 10. Chunk: UI ('UIBD')
	{
		uint32_t uiDataSize = 0;
		for (const auto& pkt : m_uiPackets)
		{
			uint32_t vtxCount = (pkt.pVertices != nullptr) ? pkt.dwVertexCount : 0;
			uint32_t idxCount = 0;
			uiDataSize += static_cast<uint32_t>(sizeof(FrameDumpV2::TUIBatchHeader) + vtxCount * sizeof(TUIVertex) + idxCount * sizeof(uint16_t));
		}

		FrameDumpV2::TChunkHeader chunkHdr{};
		chunkHdr.dwTag = FrameDumpV2::CHUNK_UI;
		chunkHdr.dwCount = static_cast<uint32_t>(m_uiPackets.size());
		chunkHdr.dwElementSize = 0;
		chunkHdr.dwDataSize = uiDataSize;
		fwrite(&chunkHdr, sizeof(chunkHdr), 1, fp);

		for (const auto& pkt : m_uiPackets)
		{
			FrameDumpV2::TUIBatchHeader batchHdr{};
			batchHdr.uTextureID = static_cast<uint64_t>(pkt.dwTextureID);
			batchHdr.bUseScissor = pkt.bUseScissor ? 1 : 0;
			batchHdr.scissorRect.left = pkt.scissorRect.left;
			batchHdr.scissorRect.top = pkt.scissorRect.top;
			batchHdr.scissorRect.right = pkt.scissorRect.right;
			batchHdr.scissorRect.bottom = pkt.scissorRect.bottom;
			batchHdr.dwBlendMode = pkt.dwBlendMode;
			uint32_t vtxCount = (pkt.pVertices != nullptr) ? pkt.dwVertexCount : 0;
			batchHdr.dwVertexCount = vtxCount;
			batchHdr.dwIndexCount = 0;
			fwrite(&batchHdr, sizeof(batchHdr), 1, fp);

			for (uint32_t idx = 0; idx < vtxCount; ++idx)
			{
				TUIVertex sanitized{};
				std::memset(&sanitized, 0, sizeof(sanitized));
				sanitized = pkt.pVertices[idx];
				fwrite(&sanitized, sizeof(TUIVertex), 1, fp);
			}
		}
	}

	fclose(fp);
	return true;
}

namespace FrameDumpV2
{
	bool ParseFrameDumpV2(const char* c_pszFileName, TDumpParsedFrame& outFrame)
	{
		if (!c_pszFileName)
			return false;

		FILE* fp = nullptr;
		if (fopen_s(&fp, c_pszFileName, "rb") != 0 || !fp)
			return false;

		THeader hdr{};
		if (fread(&hdr, sizeof(THeader), 1, fp) != 1)
		{
			fclose(fp);
			return false;
		}

		if (hdr.dwMagic != M2FR_MAGIC || hdr.dwVersion != M2FR_VERSION)
		{
			fclose(fp);
			return false;
		}

		outFrame = TDumpParsedFrame{};
		outFrame.header = hdr;

		for (uint32_t c = 0; c < hdr.dwChunkCount; ++c)
		{
			TChunkHeader chunkHdr{};
			if (fread(&chunkHdr, sizeof(TChunkHeader), 1, fp) != 1)
				break;

			outFrame.chunkHeaders.push_back(chunkHdr);

			switch (chunkHdr.dwTag)
			{
			case CHUNK_UNIFORMS:
				if (chunkHdr.dwCount >= 1 && chunkHdr.dwDataSize >= sizeof(TRenderFrameUniforms))
				{
					fread(&outFrame.uniforms, sizeof(TRenderFrameUniforms), 1, fp);
				}
				break;

			case CHUNK_SKYBOX:
				outFrame.skyBoxPackets.resize(chunkHdr.dwCount);
				for (uint32_t i = 0; i < chunkHdr.dwCount; ++i)
				{
					fread(&outFrame.skyBoxPackets[i], sizeof(TSkyBoxPacket), 1, fp);
				}
				break;

			case CHUNK_TERRAIN:
				outFrame.terrainPackets.resize(chunkHdr.dwCount);
				for (uint32_t i = 0; i < chunkHdr.dwCount; ++i)
				{
					fread(&outFrame.terrainPackets[i], sizeof(TTerrainSectorPacket), 1, fp);
				}
				break;

			case CHUNK_DUNGEON:
				outFrame.dungeonPackets.resize(chunkHdr.dwCount);
				for (uint32_t i = 0; i < chunkHdr.dwCount; ++i)
				{
					fread(&outFrame.dungeonPackets[i], sizeof(TDungeonBlockPacket), 1, fp);
				}
				break;

			case CHUNK_WATER:
				outFrame.waterPackets.resize(chunkHdr.dwCount);
				for (uint32_t i = 0; i < chunkHdr.dwCount; ++i)
				{
					fread(&outFrame.waterPackets[i], sizeof(TWaterPacket), 1, fp);
				}
				break;

			case CHUNK_SPEEDTREE:
				outFrame.speedTreeBatches.resize(chunkHdr.dwCount);
				for (uint32_t i = 0; i < chunkHdr.dwCount; ++i)
				{
					fread(&outFrame.speedTreeBatches[i].header, sizeof(TSpeedTreeBatchHeader), 1, fp);
					uint32_t instCount = outFrame.speedTreeBatches[i].header.dwInstanceCount;
					outFrame.speedTreeBatches[i].instances.resize(instCount);
					for (uint32_t j = 0; j < instCount; ++j)
					{
						fread(&outFrame.speedTreeBatches[i].instances[j], sizeof(TSpeedTreeInstanceData), 1, fp);
					}
				}
				break;

			case CHUNK_ACTORS:
				outFrame.actorPackets.resize(chunkHdr.dwCount);
				for (uint32_t i = 0; i < chunkHdr.dwCount; ++i)
				{
					fread(&outFrame.actorPackets[i], sizeof(TActorRenderPacket), 1, fp);
				}
				break;

			case CHUNK_EFFECTS:
				outFrame.effectBatches.resize(chunkHdr.dwCount);
				for (uint32_t i = 0; i < chunkHdr.dwCount; ++i)
				{
					fread(&outFrame.effectBatches[i].header, sizeof(TEffectBatchHeader), 1, fp);
					uint32_t partCount = outFrame.effectBatches[i].header.dwParticleCount;
					outFrame.effectBatches[i].particles.resize(partCount);
					for (uint32_t j = 0; j < partCount; ++j)
					{
						fread(&outFrame.effectBatches[i].particles[j], sizeof(TParticleInstanceData), 1, fp);
					}
				}
				break;

			case CHUNK_UI:
				outFrame.uiBatches.resize(chunkHdr.dwCount);
				for (uint32_t i = 0; i < chunkHdr.dwCount; ++i)
				{
					fread(&outFrame.uiBatches[i].header, sizeof(TUIBatchHeader), 1, fp);
					uint32_t vtxCount = outFrame.uiBatches[i].header.dwVertexCount;
					outFrame.uiBatches[i].vertices.resize(vtxCount);
					for (uint32_t j = 0; j < vtxCount; ++j)
					{
						fread(&outFrame.uiBatches[i].vertices[j], sizeof(TUIVertex), 1, fp);
					}
					uint32_t idxCount = outFrame.uiBatches[i].header.dwIndexCount;
					outFrame.uiBatches[i].indices.resize(idxCount);
					for (uint32_t j = 0; j < idxCount; ++j)
					{
						fread(&outFrame.uiBatches[i].indices[j], sizeof(uint16_t), 1, fp);
					}
				}
				break;

			default:
				if (chunkHdr.dwDataSize > 0)
				{
					fseek(fp, chunkHdr.dwDataSize, SEEK_CUR);
				}
				break;
			}
		}

		fclose(fp);
		return true;
	}
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
