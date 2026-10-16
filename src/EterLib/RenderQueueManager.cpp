#include "StdAfx.h"
#include "RenderQueueManager.h"
#include "StateManager.h"
#include "SpeedTreeLib/SpeedTreeWrapper.h"

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
		if (packet.vertices.empty())
			continue;

		if (packet.bScissorEnable)
		{
			STATEMANAGER.SetRenderState(D3DRS_SCISSORTESTENABLE, TRUE);
			STATEMANAGER.SetScissorRect(packet.scissorRect);
		}
		else
		{
			STATEMANAGER.SetRenderState(D3DRS_SCISSORTESTENABLE, FALSE);
		}

		if (packet.dwTextureId != 0)
		{
			STATEMANAGER.SetTexture(0, reinterpret_cast<LPDIRECT3DBASETEXTURE9>(static_cast<uintptr_t>(packet.dwTextureId)));
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
				static_cast<UINT>(packet.vertices.size()),
				static_cast<UINT>(packet.indices.size() / 3),
				packet.indices.data(),
				D3DFMT_INDEX16,
				packet.vertices.data(),
				static_cast<UINT>(sizeof(TUIVertex))
			);
		}
		else
		{
			STATEMANAGER.DrawPrimitiveUP(
				D3DPT_TRIANGLELIST,
				static_cast<UINT>(packet.vertices.size() / 3),
				packet.vertices.data(),
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
