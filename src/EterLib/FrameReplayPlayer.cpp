#include "StdAfx.h"
#include "FrameReplayPlayer.h"
#include <cstdio>

CFrameReplayPlayer::CFrameReplayPlayer()
{
}

CFrameReplayPlayer::~CFrameReplayPlayer()
{
	Clear();
}

void CFrameReplayPlayer::Clear()
{
	m_dwDumpVersion = 0;
	m_parsedFrame = FrameDumpV2::TDumpParsedFrame{};
	m_stats = FrameReplay::SReplayStats{};
	m_registeredHandles.clear();
	m_createdStubTextures.clear();
}

bool CFrameReplayPlayer::LoadDump(const char* c_pszFileName)
{
	Clear();

	if (!c_pszFileName)
		return false;

	if (!FrameDumpV2::ParseFrameDumpV2(c_pszFileName, m_parsedFrame))
		return false;

	m_dwDumpVersion = 2;
	m_stats.dwDumpVersion = 2;
	m_stats.dwChunkCount = static_cast<uint32_t>(m_parsedFrame.chunkHeaders.size());
	m_stats.bHasUniforms = true;

	m_stats.dwTotalPackets = static_cast<uint32_t>(
		m_parsedFrame.skyBoxPackets.size() +
		m_parsedFrame.terrainPackets.size() +
		m_parsedFrame.dungeonPackets.size() +
		m_parsedFrame.waterPackets.size() +
		m_parsedFrame.speedTreeBatches.size() +
		m_parsedFrame.actorPackets.size() +
		m_parsedFrame.effectBatches.size() +
		m_parsedFrame.uiBatches.size()
	);

	m_stats.bSuccess = true;
	return true;
}

FrameReplay::SDumpSummary CFrameReplayPlayer::GetSummary() const
{
	FrameReplay::SDumpSummary summary{};
	summary.dwMagic = m_parsedFrame.header.dwMagic;
	summary.dwVersion = m_parsedFrame.header.dwVersion;
	summary.dwChunkCount = static_cast<uint32_t>(m_parsedFrame.chunkHeaders.size());
	summary.dwSkyBoxPackets = static_cast<uint32_t>(m_parsedFrame.skyBoxPackets.size());
	summary.dwTerrainPackets = static_cast<uint32_t>(m_parsedFrame.terrainPackets.size());
	summary.dwDungeonPackets = static_cast<uint32_t>(m_parsedFrame.dungeonPackets.size());
	summary.dwWaterPackets = static_cast<uint32_t>(m_parsedFrame.waterPackets.size());
	summary.dwSpeedTreeBatches = static_cast<uint32_t>(m_parsedFrame.speedTreeBatches.size());
	summary.dwActorPackets = static_cast<uint32_t>(m_parsedFrame.actorPackets.size());
	summary.dwEffectBatches = static_cast<uint32_t>(m_parsedFrame.effectBatches.size());
	summary.dwUIBatches = static_cast<uint32_t>(m_parsedFrame.uiBatches.size());
	summary.fTime = m_parsedFrame.uniforms.fTime;
	summary.dwFrameIndex = m_parsedFrame.uniforms.dwFrameIndex;
	return summary;
}

bool CFrameReplayPlayer::ReplayToQueue(CRenderQueueManager& queueManager)
{
	queueManager.BeginFrame();
	queueManager.SetFrameUniforms(m_parsedFrame.uniforms);

	// 1. SkyBox
	for (const auto& pkt : m_parsedFrame.skyBoxPackets)
		queueManager.SubmitSkyBox(pkt);

	// 2. Terrain
	for (const auto& pkt : m_parsedFrame.terrainPackets)
		queueManager.SubmitTerrain(pkt);

	// 3. Dungeon
	for (const auto& pkt : m_parsedFrame.dungeonPackets)
		queueManager.SubmitDungeonBlock(pkt);

	// 4. Water
	for (const auto& pkt : m_parsedFrame.waterPackets)
		queueManager.SubmitWater(pkt);

	// 5. SpeedTree
	for (const auto& batch : m_parsedFrame.speedTreeBatches)
	{
		TSpeedTreeInstanceData* pInstData = queueManager.AllocateSpeedTreeInstanceData(batch.instances.size());
		if (pInstData && !batch.instances.empty())
		{
			memcpy(pInstData, batch.instances.data(), batch.instances.size() * sizeof(TSpeedTreeInstanceData));
		}
		TSpeedTreePacket pkt{};
		pkt.dwTreeCRC = batch.header.dwTreeCRC;
		pkt.dwFlags = batch.header.dwFlags;
		pkt.dwTreeTypeID = static_cast<uint32_t>(batch.header.uTreeTypeID);
		pkt.dwTrunkTextureID = static_cast<uint32_t>(batch.header.uTrunkTextureID);
		pkt.dwLeafTextureID = static_cast<uint32_t>(batch.header.uLeafTextureID);
		pkt.dwInstanceCount = batch.header.dwInstanceCount;
		pkt.pInstances = pInstData;
		queueManager.SubmitSpeedTree(pkt);
	}

	// 6. Actors
	for (const auto& pkt : m_parsedFrame.actorPackets)
		queueManager.SubmitActor(pkt);

	// 7. Effects
	for (const auto& batch : m_parsedFrame.effectBatches)
	{
		TParticleInstanceData* pPartData = queueManager.AllocateParticleData(batch.particles.size());
		if (pPartData && !batch.particles.empty())
		{
			memcpy(pPartData, batch.particles.data(), batch.particles.size() * sizeof(TParticleInstanceData));
		}
		TEffectInstancePacket pkt{};
		pkt.dwEffectID = batch.header.dwEffectID;
		pkt.dwTextureID = static_cast<uintptr_t>(batch.header.uTextureID);
		pkt.dwBlendMode = batch.header.dwBlendMode;
		pkt.dwParticleCount = batch.header.dwParticleCount;
		pkt.pParticles = pPartData;
		queueManager.SubmitEffect(pkt);
	}

	// 8. UI Batches
	for (const auto& batch : m_parsedFrame.uiBatches)
	{
		TUIBatchPacket pkt{};
		pkt.dwTextureID = static_cast<uintptr_t>(batch.header.uTextureID);
		pkt.dwBlendMode = batch.header.dwBlendMode;
		pkt.dwVertexCount = batch.header.dwVertexCount;
		pkt.bUseScissor = batch.header.bUseScissor;
		pkt.scissorRect.left = batch.header.scissorRect.left;
		pkt.scissorRect.top = batch.header.scissorRect.top;
		pkt.scissorRect.right = batch.header.scissorRect.right;
		TUIVertex* pUIVerts = nullptr;
		if (!batch.vertices.empty())
		{
			size_t sz = batch.vertices.size() * sizeof(TUIVertex);
			pUIVerts = static_cast<TUIVertex*>(queueManager.AllocateFrameData(sz));
			if (pUIVerts)
			{
				memcpy(pUIVerts, batch.vertices.data(), sz);
			}
		}
		pkt.pVertices = pUIVerts;
		queueManager.SubmitUI(pkt);
	}

	return true;
}

bool CFrameReplayPlayer::Play(const char* c_pszDumpFileName,
                             const char* c_pszOutputScreenshot,
                             IRenderBackend* pBackend,
                             CRenderQueueManager* pQueue,
                             CRenderResourceRegistry* /*pRegistry*/)
{
	if (!LoadDump(c_pszDumpFileName))
		return false;

	CRenderQueueManager* pTargetQueue = pQueue ? pQueue : &CRenderQueueManager::Instance();
	if (!ReplayToQueue(*pTargetQueue))
		return false;

	if (pBackend)
	{
		pTargetQueue->Flush(pBackend);

		if (c_pszOutputScreenshot)
		{
			pBackend->CaptureFrame(c_pszOutputScreenshot);
		}
	}

	return true;
}
