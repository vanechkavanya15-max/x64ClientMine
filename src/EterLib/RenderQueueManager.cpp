#include "StdAfx.h"
#include "RenderQueueManager.h"
#include "IRenderBackend.h"
#include "StateManager.h"
#include "SpeedTreeLib/SpeedTreeWrapper.h"
#include <cstdio>

CRenderQueueManager::CRenderQueueManager()
{
	m_speedTreePackets.reserve(256);
	m_uiPackets.reserve(128);
	m_actorPackets.reserve(512);
	m_terrainPackets.reserve(64);
	m_effectPackets.reserve(256);
	m_waterPackets.reserve(32);
	memset(&m_frameUniforms, 0, sizeof(TRenderFrameUniforms));
}

CRenderQueueManager::~CRenderQueueManager()
{
	Clear();
}

void CRenderQueueManager::BeginFrame()
{
	m_speedTreePackets.clear();
	m_uiPackets.clear();
	m_actorPackets.clear();
	m_terrainPackets.clear();
	m_effectPackets.clear();
	m_waterPackets.clear();
	m_particleDataPool.clear();
}

void CRenderQueueManager::EndFrame()
{
}

void CRenderQueueManager::Clear()
{
	m_speedTreePackets.clear();
	m_uiPackets.clear();
	m_actorPackets.clear();
	m_terrainPackets.clear();
	m_effectPackets.clear();
	m_waterPackets.clear();
	m_particleDataPool.clear();
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

void CRenderQueueManager::Flush(IRenderBackend* pBackend)
{
	if (!pBackend)
		return;

	if (HasPendingFrameDump())
	{
		ExportFrameToBinary(GetPendingFrameDumpPath().c_str());
		ClearPendingFrameDump();
	}

	pBackend->BeginFrame(m_frameUniforms);

	if (!m_terrainPackets.empty())
		pBackend->RenderTerrain(m_terrainPackets.data(), static_cast<unsigned int>(m_terrainPackets.size()));

	if (!m_waterPackets.empty())
		pBackend->RenderWater(m_waterPackets.data(), static_cast<unsigned int>(m_waterPackets.size()));

	if (!m_speedTreePackets.empty())
		pBackend->RenderSpeedTree(m_speedTreePackets.data(), static_cast<unsigned int>(m_speedTreePackets.size()));

	if (!m_actorPackets.empty())
		pBackend->RenderActors(m_actorPackets.data(), static_cast<unsigned int>(m_actorPackets.size()));

	if (!m_effectPackets.empty())
		pBackend->RenderEffects(m_effectPackets.data(), static_cast<unsigned int>(m_effectPackets.size()));

	if (!m_uiPackets.empty())
		pBackend->RenderUI(m_uiPackets.data(), static_cast<unsigned int>(m_uiPackets.size()));

	pBackend->EndFrame();

	Clear();
}

void CRenderQueueManager::FlushSpeedTree()
{
	for (const auto& packet : m_speedTreePackets)
	{
		if (!packet.pMainTree || packet.instances.empty())
			continue;

		if (packet.bRenderBranches)
		{
			packet.pMainTree->SetupBranchForTreeType();
			for (const auto& inst : packet.instances)
			{
				if (inst.pTreeInstance)
					const_cast<CSpeedTreeWrapper*>(inst.pTreeInstance)->RenderBranches();
			}
		}

		if (packet.bRenderFronds)
		{
			packet.pMainTree->SetupFrondForTreeType();
			for (const auto& inst : packet.instances)
			{
				if (inst.pTreeInstance)
					const_cast<CSpeedTreeWrapper*>(inst.pTreeInstance)->RenderFronds();
			}
		}

		if (packet.bRenderLeaves)
		{
			packet.pMainTree->SetupLeafForTreeType();
			for (const auto& inst : packet.instances)
			{
				if (inst.pTreeInstance)
					const_cast<CSpeedTreeWrapper*>(inst.pTreeInstance)->RenderLeaves();
			}
			const_cast<CSpeedTreeWrapper*>(packet.pMainTree)->EndLeafForTreeType();
		}

		if (packet.bRenderBillboards)
		{
			packet.pMainTree->SetupLeafForTreeType();
			for (const auto& inst : packet.instances)
			{
				if (inst.pTreeInstance)
					const_cast<CSpeedTreeWrapper*>(inst.pTreeInstance)->RenderBillboards();
			}
		}
	}
}

void CRenderQueueManager::FlushUI()
{
	if (m_uiPackets.empty())
		return;

	LPDIRECT3DDEVICE9 dev = STATEMANAGER.GetDevice();
	if (!dev)
	{
		m_uiPackets.clear();
		return;
	}

	STATEMANAGER.SaveRenderState(D3DRS_ALPHABLENDENABLE, TRUE);
	STATEMANAGER.SaveRenderState(D3DRS_SRCBLEND, D3DBLEND_SRCALPHA);
	STATEMANAGER.SaveRenderState(D3DRS_DESTBLEND, D3DBLEND_INVSRCALPHA);
	STATEMANAGER.SaveRenderState(D3DRS_ZENABLE, FALSE);
	STATEMANAGER.SaveRenderState(D3DRS_ZWRITEENABLE, FALSE);
	STATEMANAGER.SaveRenderState(D3DRS_CULLMODE, D3DCULL_NONE);
	STATEMANAGER.SaveRenderState(D3DRS_LIGHTING, FALSE);

	static LPDIRECT3DVERTEXDECLARATION9 s_pUIVertexDecl = nullptr;
	if (!s_pUIVertexDecl)
	{
		const D3DVERTEXELEMENT9 declElements[] = {
			{ 0, 0,  D3DDECLTYPE_FLOAT4,   D3DDECLMETHOD_DEFAULT, D3DDECLUSAGE_POSITIONT, 0 },
			{ 0, 16, D3DDECLTYPE_D3DCOLOR, D3DDECLMETHOD_DEFAULT, D3DDECLUSAGE_COLOR,     0 },
			{ 0, 20, D3DDECLTYPE_FLOAT2,   D3DDECLMETHOD_DEFAULT, D3DDECLUSAGE_TEXCOORD,  0 },
			D3DDECL_END()
		};
		dev->CreateVertexDeclaration(declElements, &s_pUIVertexDecl);
	}

	if (s_pUIVertexDecl)
		STATEMANAGER.SetVertexDeclaration(s_pUIVertexDecl);

	for (const auto& packet : m_uiPackets)
	{
		const TUIVertex* pVtx = packet.pVertices ? packet.pVertices : (packet.vertices.empty() ? nullptr : packet.vertices.data());
		size_t vtxCount = packet.dwVertexCount > 0 ? packet.dwVertexCount : packet.vertices.size();

		if (!pVtx || vtxCount == 0)
			continue;

		if (packet.bScissorEnable || packet.bUseScissor)
		{
			STATEMANAGER.SetRenderState(D3DRS_SCISSORTESTENABLE, TRUE);
			STATEMANAGER.SetScissorRect(packet.scissorRect);
		}
		else
		{
			STATEMANAGER.SetRenderState(D3DRS_SCISSORTESTENABLE, FALSE);
		}

		uint32_t texId = packet.dwTextureId != 0 ? packet.dwTextureId : packet.dwTextureID;
		if (texId != 0)
		{
			STATEMANAGER.SetTexture(0, reinterpret_cast<LPDIRECT3DBASETEXTURE9>(static_cast<uintptr_t>(texId)));
			STATEMANAGER.SetTextureStageState(0, D3DTSS_COLOROP, D3DTOP_MODULATE);
			STATEMANAGER.SetTextureStageState(0, D3DTSS_COLORARG1, D3DTA_TEXTURE);
			STATEMANAGER.SetTextureStageState(0, D3DTSS_COLORARG2, D3DTA_DIFFUSE);
			STATEMANAGER.SetTextureStageState(0, D3DTSS_ALPHAOP, D3DTOP_MODULATE);
			STATEMANAGER.SetTextureStageState(0, D3DTSS_ALPHAARG1, D3DTA_TEXTURE);
			STATEMANAGER.SetTextureStageState(0, D3DTSS_ALPHAARG2, D3DTA_DIFFUSE);
		}
		else
		{
			STATEMANAGER.SetTexture(0, nullptr);
			STATEMANAGER.SetTextureStageState(0, D3DTSS_COLOROP, D3DTOP_SELECTARG1);
			STATEMANAGER.SetTextureStageState(0, D3DTSS_COLORARG1, D3DTA_DIFFUSE);
			STATEMANAGER.SetTextureStageState(0, D3DTSS_ALPHAOP, D3DTOP_SELECTARG1);
			STATEMANAGER.SetTextureStageState(0, D3DTSS_ALPHAARG1, D3DTA_DIFFUSE);
		}

		if (!packet.indices.empty())
		{
			STATEMANAGER.DrawIndexedPrimitiveUP(
				D3DPT_TRIANGLELIST,
				0,
				static_cast<UINT>(vtxCount),
				static_cast<UINT>(packet.indices.size() / 3),
				packet.indices.data(),
				D3DFMT_INDEX16,
				pVtx,
				static_cast<UINT>(sizeof(TUIVertex))
			);
		}
		else
		{
			STATEMANAGER.DrawPrimitiveUP(
				D3DPT_TRIANGLELIST,
				static_cast<UINT>(vtxCount / 3),
				pVtx,
				static_cast<UINT>(sizeof(TUIVertex))
			);
		}
	}

	STATEMANAGER.RestoreRenderState(D3DRS_LIGHTING);
	STATEMANAGER.RestoreRenderState(D3DRS_CULLMODE);
	STATEMANAGER.RestoreRenderState(D3DRS_ZWRITEENABLE);
	STATEMANAGER.RestoreRenderState(D3DRS_ZENABLE);
	STATEMANAGER.RestoreRenderState(D3DRS_DESTBLEND);
	STATEMANAGER.RestoreRenderState(D3DRS_SRCBLEND);
	STATEMANAGER.RestoreRenderState(D3DRS_ALPHABLENDENABLE);

	m_uiPackets.clear();
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

size_t CRenderQueueManager::GetTotalPacketCount() const
{
	return m_speedTreePackets.size() + m_uiPackets.size() + m_actorPackets.size() +
		m_terrainPackets.size() + m_effectPackets.size() + m_waterPackets.size();
}

TParticleInstanceData* CRenderQueueManager::AllocateParticleData(size_t count)
{
	if (count == 0)
		return nullptr;

	size_t currentSize = m_particleDataPool.size();
	m_particleDataPool.resize(currentSize + count);
	return &m_particleDataPool[currentSize];
}

void CRenderQueueManager::ClearParticleData()
{
	m_particleDataPool.clear();
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

		uint32_t instCount = static_cast<uint32_t>(pkt.instances.size());
		fwrite(&instCount, sizeof(uint32_t), 1, fp);
		for (const auto& inst : pkt.instances)
		{
			TSpeedTreeInstanceData sanitized = inst;
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
		fwrite(&pkt.dwTextureID, sizeof(uint32_t), 1, fp);
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
		uint32_t texId = pkt.dwTextureId != 0 ? pkt.dwTextureId : pkt.dwTextureID;
		fwrite(&texId, sizeof(uint32_t), 1, fp);
		uint32_t scissor = (pkt.bScissorEnable || pkt.bUseScissor) ? 1 : 0;
		fwrite(&scissor, sizeof(uint32_t), 1, fp);
		fwrite(&pkt.scissorRect, sizeof(RECT), 1, fp);
		fwrite(&pkt.dwBlendMode, sizeof(uint32_t), 1, fp);

		uint32_t vtxCount = pkt.dwVertexCount > 0 ? pkt.dwVertexCount : static_cast<uint32_t>(pkt.vertices.size());
		const TUIVertex* pVtx = pkt.pVertices ? pkt.pVertices : (pkt.vertices.empty() ? nullptr : pkt.vertices.data());
		fwrite(&vtxCount, sizeof(uint32_t), 1, fp);
		if (vtxCount > 0 && pVtx)
		{
			fwrite(pVtx, sizeof(TUIVertex), vtxCount, fp);
		}

		uint32_t idxCount = static_cast<uint32_t>(pkt.indices.size());
		fwrite(&idxCount, sizeof(uint32_t), 1, fp);
		if (idxCount > 0)
		{
			fwrite(pkt.indices.data(), sizeof(uint16_t), idxCount, fp);
		}
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
