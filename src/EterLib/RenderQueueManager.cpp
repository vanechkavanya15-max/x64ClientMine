#include "StdAfx.h"
#include "RenderQueueManager.h"
#include "IRenderBackend.h"
#include "SkyBox.h"
#include <cstdio>
#include <algorithm>

static constexpr size_t DEFAULT_PARTICLE_POOL_CAPACITY = 65536;
static constexpr size_t DEFAULT_SPEEDTREE_POOL_CAPACITY = 16384;
static constexpr size_t DEFAULT_WEAPON_TRACE_VERTEX_CAPACITY = 16384;
static constexpr size_t DEFAULT_EFFECT_MESH_VERTEX_CAPACITY = 32768;
static constexpr size_t DEFAULT_BONE_MATRIX_POOL_CAPACITY = 32768;
static constexpr size_t DEFAULT_FRAME_DATA_POOL_CAPACITY = 4 * 1024 * 1024;

void SFrameBuffer::Clear()
{
	speedTreePackets.clear();
	uiPackets.clear();
	actorPackets.clear();
	terrainPackets.clear();
	effectPackets.clear();
	waterPackets.clear();
	skyBoxPackets.clear();
	dungeonPackets.clear();
	weaponTracePackets.clear();
	effectMeshPackets.clear();
	pSkyDescription.reset();
	debugLinePackets.clear();

	flushedSpeedTreeOffset = 0;
	flushedUIOffset = 0;
	particleDataOffset = 0;
	speedTreeInstanceOffset = 0;
	weaponTraceVertexOffset = 0;
	effectMeshVertexOffset = 0;
	boneMatrixOffset = 0;
	frameDataOffset = 0;

	droppedParticles = 0;
	droppedSpeedTree = 0;
	droppedWeaponTrace = 0;
	droppedEffectMesh = 0;
	droppedBones = 0;
	droppedFrameData = 0;

	memset(&frameUniforms, 0, sizeof(TRenderFrameUniforms));
}

size_t SFrameBuffer::GetTotalPacketCount() const
{
	return speedTreePackets.size() + uiPackets.size() + actorPackets.size() +
		terrainPackets.size() + effectPackets.size() + waterPackets.size() +
		skyBoxPackets.size() + dungeonPackets.size() + weaponTracePackets.size() +
		effectMeshPackets.size() + debugLinePackets.size();
}

TParticleInstanceData* SFrameBuffer::AllocateParticleData(size_t count)
{
	if (count == 0)
		return nullptr;

	if (particleDataOffset + count > particleDataPool.size())
	{
		droppedParticles += count;
		TraceError("CRenderQueueManager::AllocateParticleData: Pool overflow! requested=%zu, remaining=%zu, capacity=%zu, droppedTotal=%zu",
			count, particleDataPool.size() - particleDataOffset, particleDataPool.size(), droppedParticles);
		return nullptr;
	}

	TParticleInstanceData* pResult = &particleDataPool[particleDataOffset];
	particleDataOffset += count;
	return pResult;
}

TSpeedTreeInstanceData* SFrameBuffer::AllocateSpeedTreeInstanceData(size_t count)
{
	if (count == 0)
		return nullptr;

	if (speedTreeInstanceOffset + count > speedTreeInstancePool.size())
	{
		droppedSpeedTree += count;
		TraceError("CRenderQueueManager::AllocateSpeedTreeInstanceData: Pool overflow! requested=%zu, remaining=%zu, capacity=%zu, droppedTotal=%zu",
			count, speedTreeInstancePool.size() - speedTreeInstanceOffset, speedTreeInstancePool.size(), droppedSpeedTree);
		return nullptr;
	}

	TSpeedTreeInstanceData* pResult = &speedTreeInstancePool[speedTreeInstanceOffset];
	speedTreeInstanceOffset += count;
	return pResult;
}

TPDTVertex* SFrameBuffer::AllocateWeaponTraceVertexData(size_t count)
{
	if (count == 0)
		return nullptr;

	if (weaponTraceVertexOffset + count > weaponTraceVertexPool.size())
	{
		droppedWeaponTrace += count;
		TraceError("CRenderQueueManager::AllocateWeaponTraceVertexData: Pool overflow! requested=%zu, remaining=%zu, capacity=%zu, droppedTotal=%zu",
			count, weaponTraceVertexPool.size() - weaponTraceVertexOffset, weaponTraceVertexPool.size(), droppedWeaponTrace);
		return nullptr;
	}

	TPDTVertex* pResult = &weaponTraceVertexPool[weaponTraceVertexOffset];
	weaponTraceVertexOffset += count;
	return pResult;
}

TPTVertex* SFrameBuffer::AllocateEffectMeshVertexData(size_t count)
{
	if (count == 0)
		return nullptr;

	if (effectMeshVertexOffset + count > effectMeshVertexPool.size())
	{
		droppedEffectMesh += count;
		TraceError("CRenderQueueManager::AllocateEffectMeshVertexData: Pool overflow! requested=%zu, remaining=%zu, capacity=%zu, droppedTotal=%zu",
			count, effectMeshVertexPool.size() - effectMeshVertexOffset, effectMeshVertexPool.size(), droppedEffectMesh);
		return nullptr;
	}

	TPTVertex* pResult = &effectMeshVertexPool[effectMeshVertexOffset];
	effectMeshVertexOffset += count;
	return pResult;
}

D3DXMATRIX* SFrameBuffer::AllocateBoneMatrices(size_t count)
{
	if (count == 0)
		return nullptr;

	if (boneMatrixOffset + count > boneMatrixPool.size())
	{
		droppedBones += count;
		TraceError("CRenderQueueManager::AllocateBoneMatrices: Pool overflow! requested=%zu, remaining=%zu, capacity=%zu, droppedTotal=%zu",
			count, boneMatrixPool.size() - boneMatrixOffset, boneMatrixPool.size(), droppedBones);
		return nullptr;
	}

	D3DXMATRIX* pResult = &boneMatrixPool[boneMatrixOffset];
	boneMatrixOffset += count;
	return pResult;
}

void* SFrameBuffer::AllocateFrameData(size_t sizeInBytes, size_t alignment)
{
	if (sizeInBytes == 0)
		return nullptr;

	if (alignment == 0)
		alignment = 16;

	const size_t alignedOffset = (frameDataOffset + (alignment - 1)) & ~(alignment - 1);
	if (alignedOffset + sizeInBytes > frameDataPool.size())
	{
		droppedFrameData += sizeInBytes;
		TraceError("CRenderQueueManager::AllocateFrameData: Pool overflow! requested=%zu, remaining=%zu, capacity=%zu, droppedTotal=%zu",
			sizeInBytes, frameDataPool.size() - frameDataOffset, frameDataPool.size(), droppedFrameData);
		return nullptr;
	}

	void* pResult = &frameDataPool[alignedOffset];
	frameDataOffset = alignedOffset + sizeInBytes;
	return pResult;
}

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
	m_activeFrameIndex = 0;
	m_bHasSwappedFrame = false;

	for (size_t i = 0; i < FRAME_BUFFER_COUNT; ++i)
	{
		m_frameBuffers[i].speedTreePackets.reserve(256);
		m_frameBuffers[i].uiPackets.reserve(128);
		m_frameBuffers[i].actorPackets.reserve(512);
		m_frameBuffers[i].terrainPackets.reserve(64);
		m_frameBuffers[i].effectPackets.reserve(256);
		m_frameBuffers[i].waterPackets.reserve(32);
		m_frameBuffers[i].skyBoxPackets.reserve(8);
		m_frameBuffers[i].dungeonPackets.reserve(256);
		m_frameBuffers[i].weaponTracePackets.reserve(64);
		m_frameBuffers[i].effectMeshPackets.reserve(256);
		m_frameBuffers[i].debugLinePackets.reserve(512);

		m_frameBuffers[i].particleDataPool.resize(DEFAULT_PARTICLE_POOL_CAPACITY);
		m_frameBuffers[i].speedTreeInstancePool.resize(DEFAULT_SPEEDTREE_POOL_CAPACITY);
		m_frameBuffers[i].weaponTraceVertexPool.resize(DEFAULT_WEAPON_TRACE_VERTEX_CAPACITY);
		m_frameBuffers[i].effectMeshVertexPool.resize(DEFAULT_EFFECT_MESH_VERTEX_CAPACITY);
		m_frameBuffers[i].boneMatrixPool.resize(DEFAULT_BONE_MATRIX_POOL_CAPACITY);
		m_frameBuffers[i].frameDataPool.resize(DEFAULT_FRAME_DATA_POOL_CAPACITY);

		m_frameBuffers[i].Clear();
	}
	ResetFrameStats();
}

CRenderQueueManager::~CRenderQueueManager() = default;

void CRenderQueueManager::ResetFrameStats()
{
	m_frameStats.Reset();
}

void CRenderQueueManager::RecordDrawCall(ERenderCategory category, uint32_t faceCount)
{
	m_frameStats.totalDrawCalls++;
	m_frameStats.totalFaceCount += faceCount;

	size_t idx = static_cast<size_t>(category);
	if (idx < static_cast<size_t>(ERenderCategory::Count))
	{
		m_frameStats.categories[idx].drawCallCount++;
		m_frameStats.categories[idx].faceCount += faceCount;
	}
}

void CRenderQueueManager::RecordPacket(ERenderCategory category, uint32_t count)
{
	m_frameStats.totalPackets += count;

	size_t idx = static_cast<size_t>(category);
	if (idx < static_cast<size_t>(ERenderCategory::Count))
	{
		m_frameStats.categories[idx].packetCount += count;
	}
}

void CRenderQueueManager::RecordDroppedPacket(ERenderCategory category, uint32_t count)
{
	m_frameStats.totalDropped += count;

	size_t idx = static_cast<size_t>(category);
	if (idx < static_cast<size_t>(ERenderCategory::Count))
	{
		m_frameStats.categories[idx].droppedCount += count;
	}
}

void CRenderQueueManager::BeginFrame()
{
	m_frameBuffers[GetWorkFrameIndex()].Clear();
	ResetFrameStats();
}

void CRenderQueueManager::SwapFrameBuffers()
{
	m_activeFrameIndex ^= 1;
	m_bHasSwappedFrame = true;
	m_frameBuffers[GetWorkFrameIndex()].Clear();
}

void CRenderQueueManager::EndFrame()
{
	if (HasPendingFrameDump())
	{
		ExportFrameToBinary(GetPendingFrameDumpPath().c_str());
		CaptureReferenceScreenshot("cpp_reference.png");
		ClearPendingFrameDump();
	}
	SwapFrameBuffers();
}

void CRenderQueueManager::SetFrameUniforms(const TRenderFrameUniforms& uniforms)
{
	m_frameBuffers[GetWorkFrameIndex()].frameUniforms = uniforms;
}

const TRenderFrameUniforms& CRenderQueueManager::GetFrameUniforms() const
{
	if (m_bHasSwappedFrame)
		return m_frameBuffers[m_activeFrameIndex].frameUniforms;
	return m_frameBuffers[GetWorkFrameIndex()].frameUniforms;
}

void CRenderQueueManager::SubmitSpeedTree(const TSpeedTreePacket& packet)
{
	m_frameBuffers[GetWorkFrameIndex()].speedTreePackets.push_back(packet);
	RecordPacket(ERenderCategory::SpeedTree);
}

void CRenderQueueManager::SubmitUI(const TUIBatchPacket& packet)
{
	m_frameBuffers[GetWorkFrameIndex()].uiPackets.push_back(packet);
	RecordPacket(ERenderCategory::UI);
}

void CRenderQueueManager::SubmitActor(const TActorRenderPacket& packet)
{
	m_frameBuffers[GetWorkFrameIndex()].actorPackets.push_back(packet);
	RecordPacket(ERenderCategory::Model);
}

void CRenderQueueManager::SubmitTerrain(const TTerrainSectorPacket& packet)
{
	m_frameBuffers[GetWorkFrameIndex()].terrainPackets.push_back(packet);
	RecordPacket(ERenderCategory::Terrain);
}

void CRenderQueueManager::SubmitEffect(const TEffectInstancePacket& packet)
{
	m_frameBuffers[GetWorkFrameIndex()].effectPackets.push_back(packet);
	RecordPacket(ERenderCategory::Effect);
}

void CRenderQueueManager::SubmitWater(const TWaterPacket& packet)
{
	m_frameBuffers[GetWorkFrameIndex()].waterPackets.push_back(packet);
	RecordPacket(ERenderCategory::Water);
}

void CRenderQueueManager::SubmitSkyBox(const TSkyBoxPacket& packet)
{
	m_frameBuffers[GetWorkFrameIndex()].skyBoxPackets.push_back(packet);
	RecordPacket(ERenderCategory::SkyBox);
}

void CRenderQueueManager::SubmitSkyDescription(const SSkyDescription& desc)
{
	m_frameBuffers[GetWorkFrameIndex()].pSkyDescription = std::make_unique<SSkyDescription>(desc);
	RecordPacket(ERenderCategory::SkyBox);
}

const SSkyDescription* CRenderQueueManager::GetSkyDescription() const
{
	if (m_frameBuffers[m_activeFrameIndex].pSkyDescription)
		return m_frameBuffers[m_activeFrameIndex].pSkyDescription.get();
	if (m_frameBuffers[GetWorkFrameIndex()].pSkyDescription)
		return m_frameBuffers[GetWorkFrameIndex()].pSkyDescription.get();
	return nullptr;
}

bool CRenderQueueManager::HasSkyDescription() const
{
	return GetSkyDescription() != nullptr;
}

void CRenderQueueManager::SubmitDungeonBlock(const TDungeonBlockPacket& packet)
{
	m_frameBuffers[GetWorkFrameIndex()].dungeonPackets.push_back(packet);
	RecordPacket(ERenderCategory::Dungeon);
}

void CRenderQueueManager::SubmitWeaponTrace(const TWeaponTracePacket& packet)
{
	m_frameBuffers[GetWorkFrameIndex()].weaponTracePackets.push_back(packet);
	RecordPacket(ERenderCategory::WeaponTrace);
}

const std::vector<TWeaponTracePacket>& CRenderQueueManager::GetWeaponTracePackets() const
{
	if (!m_frameBuffers[GetWorkFrameIndex()].weaponTracePackets.empty())
		return m_frameBuffers[GetWorkFrameIndex()].weaponTracePackets;
	return m_frameBuffers[m_activeFrameIndex].weaponTracePackets;
}

void CRenderQueueManager::SubmitEffectMesh(const TEffectMeshPacket& packet)
{
	m_frameBuffers[GetWorkFrameIndex()].effectMeshPackets.push_back(packet);
	RecordPacket(ERenderCategory::EffectMesh);
}

const std::vector<TEffectMeshPacket>& CRenderQueueManager::GetEffectMeshPackets() const
{
	if (!m_frameBuffers[GetWorkFrameIndex()].effectMeshPackets.empty())
		return m_frameBuffers[GetWorkFrameIndex()].effectMeshPackets;
	return m_frameBuffers[m_activeFrameIndex].effectMeshPackets;
}

void CRenderQueueManager::SubmitDebugLine(const TDebugLinePacket& packet)
{
	m_frameBuffers[GetWorkFrameIndex()].debugLinePackets.push_back(packet);
	RecordPacket(ERenderCategory::Debug);
}

void CRenderQueueManager::SubmitDebugLines(const TDebugLinePacket* pPackets, size_t count)
{
	if (!pPackets || count == 0)
		return;
	auto& debugPackets = m_frameBuffers[GetWorkFrameIndex()].debugLinePackets;
	debugPackets.insert(debugPackets.end(), pPackets, pPackets + count);
	RecordPacket(ERenderCategory::Debug, static_cast<uint32_t>(count));
}



const std::vector<TDebugLinePacket>& CRenderQueueManager::GetDebugLinePackets() const
{
	if (!m_frameBuffers[GetWorkFrameIndex()].debugLinePackets.empty())
		return m_frameBuffers[GetWorkFrameIndex()].debugLinePackets;
	return m_frameBuffers[m_activeFrameIndex].debugLinePackets;
}

void CRenderQueueManager::Flush(IRenderBackend* pBackend)
{
	if (!pBackend)
		pBackend = GetActiveBackend();
	if (!pBackend)
		return;

	SFrameBuffer& renderFrame = (m_bHasSwappedFrame || m_frameBuffers[m_activeFrameIndex].GetTotalPacketCount() > 0)
		? m_frameBuffers[m_activeFrameIndex]
		: m_frameBuffers[GetWorkFrameIndex()];

	pBackend->BeginFrame(renderFrame.frameUniforms);

	if (!renderFrame.skyBoxPackets.empty())
		pBackend->RenderSkyBox(renderFrame.skyBoxPackets.data(), static_cast<unsigned int>(renderFrame.skyBoxPackets.size()));

	if (!renderFrame.terrainPackets.empty())
		pBackend->RenderTerrain(renderFrame.terrainPackets.data(), static_cast<unsigned int>(renderFrame.terrainPackets.size()));

	if (!renderFrame.dungeonPackets.empty())
		pBackend->RenderDungeonBlocks(renderFrame.dungeonPackets.data(), static_cast<unsigned int>(renderFrame.dungeonPackets.size()));

	if (!renderFrame.waterPackets.empty())
		pBackend->RenderWater(renderFrame.waterPackets.data(), static_cast<unsigned int>(renderFrame.waterPackets.size()));

	if (renderFrame.flushedSpeedTreeOffset < renderFrame.speedTreePackets.size())
	{
		pBackend->RenderSpeedTree(
			renderFrame.speedTreePackets.data() + renderFrame.flushedSpeedTreeOffset,
			static_cast<unsigned int>(renderFrame.speedTreePackets.size() - renderFrame.flushedSpeedTreeOffset)
		);
		renderFrame.flushedSpeedTreeOffset = renderFrame.speedTreePackets.size();
	}

	if (!renderFrame.actorPackets.empty())
		pBackend->RenderActors(renderFrame.actorPackets.data(), static_cast<unsigned int>(renderFrame.actorPackets.size()));

	if (!renderFrame.weaponTracePackets.empty())
		pBackend->RenderWeaponTrace(renderFrame.weaponTracePackets.data(), static_cast<unsigned int>(renderFrame.weaponTracePackets.size()));

	if (!renderFrame.effectPackets.empty())
		pBackend->RenderEffects(renderFrame.effectPackets.data(), static_cast<unsigned int>(renderFrame.effectPackets.size()));

	if (!renderFrame.effectMeshPackets.empty())
		pBackend->RenderEffectMesh(renderFrame.effectMeshPackets.data(), static_cast<unsigned int>(renderFrame.effectMeshPackets.size()));

	if (!renderFrame.debugLinePackets.empty())
		pBackend->RenderDebugLines(renderFrame.debugLinePackets.data(), static_cast<unsigned int>(renderFrame.debugLinePackets.size()));

	if (renderFrame.flushedUIOffset < renderFrame.uiPackets.size())
	{
		pBackend->RenderUI(
			renderFrame.uiPackets.data() + renderFrame.flushedUIOffset,
			static_cast<unsigned int>(renderFrame.uiPackets.size() - renderFrame.flushedUIOffset)
		);
		renderFrame.flushedUIOffset = renderFrame.uiPackets.size();
	}

	pBackend->EndFrame();
}

void CRenderQueueManager::FlushUI()
{
	SFrameBuffer& targetFrame = (m_bHasSwappedFrame || m_frameBuffers[m_activeFrameIndex].GetTotalPacketCount() > 0)
		? m_frameBuffers[m_activeFrameIndex]
		: m_frameBuffers[GetWorkFrameIndex()];

	if (targetFrame.flushedUIOffset >= targetFrame.uiPackets.size())
		return;

	IRenderBackend* pBackend = GetActiveBackend();
	if (pBackend)
	{
		pBackend->RenderUI(
			targetFrame.uiPackets.data() + targetFrame.flushedUIOffset,
			static_cast<unsigned int>(targetFrame.uiPackets.size() - targetFrame.flushedUIOffset)
		);
	}
	targetFrame.flushedUIOffset = targetFrame.uiPackets.size();
}

const std::vector<TSpeedTreePacket>& CRenderQueueManager::GetSpeedTreePackets() const
{
	if (!m_frameBuffers[GetWorkFrameIndex()].speedTreePackets.empty())
		return m_frameBuffers[GetWorkFrameIndex()].speedTreePackets;
	return m_frameBuffers[m_activeFrameIndex].speedTreePackets;
}

const std::vector<TUIBatchPacket>& CRenderQueueManager::GetUIPackets() const
{
	if (!m_frameBuffers[GetWorkFrameIndex()].uiPackets.empty())
		return m_frameBuffers[GetWorkFrameIndex()].uiPackets;
	return m_frameBuffers[m_activeFrameIndex].uiPackets;
}

const std::vector<TActorRenderPacket>& CRenderQueueManager::GetActorPackets() const
{
	if (!m_frameBuffers[GetWorkFrameIndex()].actorPackets.empty())
		return m_frameBuffers[GetWorkFrameIndex()].actorPackets;
	return m_frameBuffers[m_activeFrameIndex].actorPackets;
}

const std::vector<TTerrainSectorPacket>& CRenderQueueManager::GetTerrainPackets() const
{
	if (!m_frameBuffers[GetWorkFrameIndex()].terrainPackets.empty())
		return m_frameBuffers[GetWorkFrameIndex()].terrainPackets;
	return m_frameBuffers[m_activeFrameIndex].terrainPackets;
}

const std::vector<TEffectInstancePacket>& CRenderQueueManager::GetEffectPackets() const
{
	if (!m_frameBuffers[GetWorkFrameIndex()].effectPackets.empty())
		return m_frameBuffers[GetWorkFrameIndex()].effectPackets;
	return m_frameBuffers[m_activeFrameIndex].effectPackets;
}

const std::vector<TWaterPacket>& CRenderQueueManager::GetWaterPackets() const
{
	if (!m_frameBuffers[GetWorkFrameIndex()].waterPackets.empty())
		return m_frameBuffers[GetWorkFrameIndex()].waterPackets;
	return m_frameBuffers[m_activeFrameIndex].waterPackets;
}

const std::vector<TSkyBoxPacket>& CRenderQueueManager::GetSkyBoxPackets() const
{
	if (!m_frameBuffers[GetWorkFrameIndex()].skyBoxPackets.empty())
		return m_frameBuffers[GetWorkFrameIndex()].skyBoxPackets;
	return m_frameBuffers[m_activeFrameIndex].skyBoxPackets;
}

const std::vector<TDungeonBlockPacket>& CRenderQueueManager::GetDungeonPackets() const
{
	if (!m_frameBuffers[GetWorkFrameIndex()].dungeonPackets.empty())
		return m_frameBuffers[GetWorkFrameIndex()].dungeonPackets;
	return m_frameBuffers[m_activeFrameIndex].dungeonPackets;
}

size_t CRenderQueueManager::GetTotalPacketCount() const
{
	if (!m_bHasSwappedFrame && m_frameBuffers[m_activeFrameIndex].GetTotalPacketCount() == 0)
		return m_frameBuffers[GetWorkFrameIndex()].GetTotalPacketCount();
	return m_frameBuffers[m_activeFrameIndex].GetTotalPacketCount();
}

TParticleInstanceData* CRenderQueueManager::AllocateParticleData(size_t count)
{
	TParticleInstanceData* pResult = m_frameBuffers[GetWorkFrameIndex()].AllocateParticleData(count);
	if (!pResult && count > 0)
		RecordDroppedPacket(ERenderCategory::Effect, static_cast<uint32_t>(count));
	return pResult;
}

TSpeedTreeInstanceData* CRenderQueueManager::AllocateSpeedTreeInstanceData(size_t count)
{
	TSpeedTreeInstanceData* pResult = m_frameBuffers[GetWorkFrameIndex()].AllocateSpeedTreeInstanceData(count);
	if (!pResult && count > 0)
		RecordDroppedPacket(ERenderCategory::SpeedTree, static_cast<uint32_t>(count));
	return pResult;
}

TPDTVertex* CRenderQueueManager::AllocateWeaponTraceVertexData(size_t count)
{
	TPDTVertex* pResult = m_frameBuffers[GetWorkFrameIndex()].AllocateWeaponTraceVertexData(count);
	if (!pResult && count > 0)
		RecordDroppedPacket(ERenderCategory::WeaponTrace, static_cast<uint32_t>(count));
	return pResult;
}

const TPDTVertex* CRenderQueueManager::GetWeaponTraceVertexData() const
{
	if (m_frameBuffers[m_activeFrameIndex].weaponTraceVertexOffset > 0)
		return m_frameBuffers[m_activeFrameIndex].weaponTraceVertexPool.data();
	if (m_frameBuffers[GetWorkFrameIndex()].weaponTraceVertexOffset > 0)
		return m_frameBuffers[GetWorkFrameIndex()].weaponTraceVertexPool.data();
	return m_frameBuffers[m_activeFrameIndex].weaponTraceVertexPool.data();
}

TPTVertex* CRenderQueueManager::AllocateEffectMeshVertexData(size_t count)
{
	TPTVertex* pResult = m_frameBuffers[GetWorkFrameIndex()].AllocateEffectMeshVertexData(count);
	if (!pResult && count > 0)
		RecordDroppedPacket(ERenderCategory::EffectMesh, static_cast<uint32_t>(count));
	return pResult;
}

const TPTVertex* CRenderQueueManager::GetEffectMeshVertexData() const
{
	if (m_frameBuffers[m_activeFrameIndex].effectMeshVertexOffset > 0)
		return m_frameBuffers[m_activeFrameIndex].effectMeshVertexPool.data();
	if (m_frameBuffers[GetWorkFrameIndex()].effectMeshVertexOffset > 0)
		return m_frameBuffers[GetWorkFrameIndex()].effectMeshVertexPool.data();
	return m_frameBuffers[m_activeFrameIndex].effectMeshVertexPool.data();
}

D3DXMATRIX* CRenderQueueManager::AllocateBoneMatrices(size_t count)
{
	D3DXMATRIX* pResult = m_frameBuffers[GetWorkFrameIndex()].AllocateBoneMatrices(count);
	if (!pResult && count > 0)
		RecordDroppedPacket(ERenderCategory::Model, static_cast<uint32_t>(count));
	return pResult;
}

const D3DXMATRIX* CRenderQueueManager::GetBoneMatrixData() const
{
	if (m_frameBuffers[m_activeFrameIndex].boneMatrixOffset > 0)
		return m_frameBuffers[m_activeFrameIndex].boneMatrixPool.data();
	if (m_frameBuffers[GetWorkFrameIndex()].boneMatrixOffset > 0)
		return m_frameBuffers[GetWorkFrameIndex()].boneMatrixPool.data();
	return m_frameBuffers[m_activeFrameIndex].boneMatrixPool.data();
}

void* CRenderQueueManager::AllocateFrameData(size_t sizeInBytes, size_t alignment)
{
	return m_frameBuffers[GetWorkFrameIndex()].AllocateFrameData(sizeInBytes, alignment);
}

const void* CRenderQueueManager::GetActiveFrameData() const
{
	return m_frameBuffers[m_activeFrameIndex].frameDataPool.empty() ? nullptr : m_frameBuffers[m_activeFrameIndex].frameDataPool.data();
}

const void* CRenderQueueManager::GetWorkFrameData() const
{
	return m_frameBuffers[GetWorkFrameIndex()].frameDataPool.empty() ? nullptr : m_frameBuffers[GetWorkFrameIndex()].frameDataPool.data();
}

size_t CRenderQueueManager::GetDroppedParticles() const
{
	return m_frameBuffers[GetWorkFrameIndex()].droppedParticles;
}

size_t CRenderQueueManager::GetDroppedSpeedTree() const
{
	return m_frameBuffers[GetWorkFrameIndex()].droppedSpeedTree;
}

size_t CRenderQueueManager::GetDroppedWeaponTrace() const
{
	return m_frameBuffers[GetWorkFrameIndex()].droppedWeaponTrace;
}

size_t CRenderQueueManager::GetDroppedEffectMesh() const
{
	return m_frameBuffers[GetWorkFrameIndex()].droppedEffectMesh;
}

size_t CRenderQueueManager::GetDroppedBones() const
{
	return m_frameBuffers[GetWorkFrameIndex()].droppedBones;
}

size_t CRenderQueueManager::GetDroppedFrameData() const
{
	return m_frameBuffers[GetWorkFrameIndex()].droppedFrameData;
}

void CRenderQueueManager::ResetDroppedCounters()
{
	for (size_t i = 0; i < FRAME_BUFFER_COUNT; ++i)
	{
		m_frameBuffers[i].droppedParticles = 0;
		m_frameBuffers[i].droppedSpeedTree = 0;
		m_frameBuffers[i].droppedWeaponTrace = 0;
		m_frameBuffers[i].droppedEffectMesh = 0;
		m_frameBuffers[i].droppedBones = 0;
		m_frameBuffers[i].droppedFrameData = 0;
	}
}

size_t CRenderQueueManager::GetParticlePoolCapacity() const
{
	return m_frameBuffers[GetWorkFrameIndex()].particleDataPool.size();
}

size_t CRenderQueueManager::GetSpeedTreePoolCapacity() const
{
	return m_frameBuffers[GetWorkFrameIndex()].speedTreeInstancePool.size();
}

size_t CRenderQueueManager::GetWeaponTracePoolCapacity() const
{
	return m_frameBuffers[GetWorkFrameIndex()].weaponTraceVertexPool.size();
}

size_t CRenderQueueManager::GetEffectMeshPoolCapacity() const
{
	return m_frameBuffers[GetWorkFrameIndex()].effectMeshVertexPool.size();
}

size_t CRenderQueueManager::GetBoneMatrixPoolCapacity() const
{
	return m_frameBuffers[GetWorkFrameIndex()].boneMatrixPool.size();
}

size_t CRenderQueueManager::GetFrameDataPoolCapacity() const
{
	return m_frameBuffers[GetWorkFrameIndex()].frameDataPool.size();
}

size_t CRenderQueueManager::GetParticleDataOffset() const
{
	return m_frameBuffers[GetWorkFrameIndex()].particleDataOffset;
}

size_t CRenderQueueManager::GetSpeedTreeInstanceOffset() const
{
	return m_frameBuffers[GetWorkFrameIndex()].speedTreeInstanceOffset;
}

size_t CRenderQueueManager::GetWeaponTraceVertexOffset() const
{
	return m_frameBuffers[GetWorkFrameIndex()].weaponTraceVertexOffset;
}

size_t CRenderQueueManager::GetEffectMeshVertexOffset() const
{
	return m_frameBuffers[GetWorkFrameIndex()].effectMeshVertexOffset;
}

size_t CRenderQueueManager::GetBoneMatrixOffset() const
{
	return m_frameBuffers[GetWorkFrameIndex()].boneMatrixOffset;
}

size_t CRenderQueueManager::GetFrameDataOffset() const
{
	return m_frameBuffers[GetWorkFrameIndex()].frameDataOffset;
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

	const SFrameBuffer& frame = (m_frameBuffers[GetWorkFrameIndex()].GetTotalPacketCount() > 0)
		? m_frameBuffers[GetWorkFrameIndex()]
		: m_frameBuffers[m_activeFrameIndex];

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
		sanitized = frame.frameUniforms;
		fwrite(&sanitized, sizeof(sanitized), 1, fp);
	}

	// 3. Chunk: SKYBOX ('SKYB')
	{
		FrameDumpV2::TChunkHeader chunkHdr{};
		chunkHdr.dwTag = FrameDumpV2::CHUNK_SKYBOX;
		chunkHdr.dwCount = static_cast<uint32_t>(frame.skyBoxPackets.size());
		chunkHdr.dwElementSize = static_cast<uint32_t>(sizeof(TSkyBoxPacket));
		chunkHdr.dwDataSize = chunkHdr.dwCount * chunkHdr.dwElementSize;
		fwrite(&chunkHdr, sizeof(chunkHdr), 1, fp);

		for (const auto& pkt : frame.skyBoxPackets)
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
		chunkHdr.dwCount = static_cast<uint32_t>(frame.terrainPackets.size());
		chunkHdr.dwElementSize = static_cast<uint32_t>(sizeof(TTerrainSectorPacket));
		chunkHdr.dwDataSize = chunkHdr.dwCount * chunkHdr.dwElementSize;
		fwrite(&chunkHdr, sizeof(chunkHdr), 1, fp);

		for (const auto& pkt : frame.terrainPackets)
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
		chunkHdr.dwCount = static_cast<uint32_t>(frame.dungeonPackets.size());
		chunkHdr.dwElementSize = static_cast<uint32_t>(sizeof(TDungeonBlockPacket));
		chunkHdr.dwDataSize = chunkHdr.dwCount * chunkHdr.dwElementSize;
		fwrite(&chunkHdr, sizeof(chunkHdr), 1, fp);

		for (const auto& pkt : frame.dungeonPackets)
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
		chunkHdr.dwCount = static_cast<uint32_t>(frame.waterPackets.size());
		chunkHdr.dwElementSize = static_cast<uint32_t>(sizeof(TWaterPacket));
		chunkHdr.dwDataSize = chunkHdr.dwCount * chunkHdr.dwElementSize;
		fwrite(&chunkHdr, sizeof(chunkHdr), 1, fp);

		for (const auto& pkt : frame.waterPackets)
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
		for (const auto& pkt : frame.speedTreePackets)
		{
			uint32_t instCount = (pkt.pInstances != nullptr) ? pkt.dwInstanceCount : 0;
			treeDataSize += static_cast<uint32_t>(sizeof(FrameDumpV2::TSpeedTreeBatchHeader) + instCount * sizeof(TSpeedTreeInstanceData));
		}

		FrameDumpV2::TChunkHeader chunkHdr{};
		chunkHdr.dwTag = FrameDumpV2::CHUNK_SPEEDTREE;
		chunkHdr.dwCount = static_cast<uint32_t>(frame.speedTreePackets.size());
		chunkHdr.dwElementSize = 0;
		chunkHdr.dwDataSize = treeDataSize;
		fwrite(&chunkHdr, sizeof(chunkHdr), 1, fp);

		for (const auto& pkt : frame.speedTreePackets)
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
				fwrite(&sanitized, sizeof(TSpeedTreeInstanceData), 1, fp);
			}
		}
	}

	// 8. Chunk: ACTORS ('ACTR')
	{
		FrameDumpV2::TChunkHeader chunkHdr{};
		chunkHdr.dwTag = FrameDumpV2::CHUNK_ACTORS;
		chunkHdr.dwCount = static_cast<uint32_t>(frame.actorPackets.size());
		chunkHdr.dwElementSize = static_cast<uint32_t>(sizeof(TActorRenderPacket));
		chunkHdr.dwDataSize = chunkHdr.dwCount * chunkHdr.dwElementSize;
		fwrite(&chunkHdr, sizeof(chunkHdr), 1, fp);

		for (const auto& pkt : frame.actorPackets)
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
		for (const auto& pkt : frame.effectPackets)
		{
			uint32_t partCount = (pkt.pParticles != nullptr) ? pkt.dwParticleCount : 0;
			effectDataSize += static_cast<uint32_t>(sizeof(FrameDumpV2::TEffectBatchHeader) + partCount * sizeof(TParticleInstanceData));
		}

		FrameDumpV2::TChunkHeader chunkHdr{};
		chunkHdr.dwTag = FrameDumpV2::CHUNK_EFFECTS;
		chunkHdr.dwCount = static_cast<uint32_t>(frame.effectPackets.size());
		chunkHdr.dwElementSize = 0;
		chunkHdr.dwDataSize = effectDataSize;
		fwrite(&chunkHdr, sizeof(chunkHdr), 1, fp);

		for (const auto& pkt : frame.effectPackets)
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
		for (const auto& pkt : frame.uiPackets)
		{
			uint32_t vtxCount = (pkt.pVertices != nullptr) ? pkt.dwVertexCount : 0;
			uint32_t idxCount = 0;
			uiDataSize += static_cast<uint32_t>(sizeof(FrameDumpV2::TUIBatchHeader) + vtxCount * sizeof(TUIVertex) + idxCount * sizeof(uint16_t));
		}

		FrameDumpV2::TChunkHeader chunkHdr{};
		chunkHdr.dwTag = FrameDumpV2::CHUNK_UI;
		chunkHdr.dwCount = static_cast<uint32_t>(frame.uiPackets.size());
		chunkHdr.dwElementSize = 0;
		chunkHdr.dwDataSize = uiDataSize;
		fwrite(&chunkHdr, sizeof(chunkHdr), 1, fp);

		for (const auto& pkt : frame.uiPackets)
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
