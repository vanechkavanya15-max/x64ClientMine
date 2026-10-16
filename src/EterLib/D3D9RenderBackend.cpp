#include "StdAfx.h"
#include "D3D9RenderBackend.h"
#include "StateManager.h"
#include "SpeedTreeLib/SpeedTreeWrapper.h"

CD3D9RenderBackend::CD3D9RenderBackend()
{
}

CD3D9RenderBackend::~CD3D9RenderBackend()
{
	Shutdown();
}

bool CD3D9RenderBackend::Initialize(HWND hWnd, unsigned int width, unsigned int height)
{
	m_hWnd = hWnd;
	m_width = width;
	m_height = height;
	m_bInitialized = true;
	InitVertexDeclarations();
	return true;
}

void CD3D9RenderBackend::Shutdown()
{
	ReleaseVertexDeclarations();
	m_bInitialized = false;
}

void CD3D9RenderBackend::Resize(unsigned int width, unsigned int height)
{
	m_width = width;
	m_height = height;
}

void CD3D9RenderBackend::InitVertexDeclarations()
{
	LPDIRECT3DDEVICE9 dev = STATEMANAGER.GetDevice();
	if (!dev)
		return;

	if (!m_pUIVertexDecl)
	{
		const D3DVERTEXELEMENT9 uiDeclElements[] = {
			{ 0, 0,  D3DDECLTYPE_FLOAT4,   D3DDECLMETHOD_DEFAULT, D3DDECLUSAGE_POSITIONT, 0 },
			{ 0, 16, D3DDECLTYPE_D3DCOLOR, D3DDECLMETHOD_DEFAULT, D3DDECLUSAGE_COLOR,     0 },
			{ 0, 20, D3DDECLTYPE_FLOAT2,   D3DDECLMETHOD_DEFAULT, D3DDECLUSAGE_TEXCOORD,  0 },
			D3DDECL_END()
		};
		dev->CreateVertexDeclaration(uiDeclElements, &m_pUIVertexDecl);
	}
}

void CD3D9RenderBackend::ReleaseVertexDeclarations()
{
	if (m_pUIVertexDecl)
	{
		m_pUIVertexDecl->Release();
		m_pUIVertexDecl = nullptr;
	}
	if (m_pActorVertexDecl)
	{
		m_pActorVertexDecl->Release();
		m_pActorVertexDecl = nullptr;
	}
}

void CD3D9RenderBackend::BeginFrame(const TRenderFrameUniforms& uniforms)
{
	STATEMANAGER.SetTransform(D3DTS_VIEW, &uniforms.matView);
	STATEMANAGER.SetTransform(D3DTS_PROJECTION, &uniforms.matProj);
}

void CD3D9RenderBackend::RenderTerrain(const TTerrainSectorPacket* pPackets, unsigned int count)
{
	if (!pPackets || count == 0)
		return;
}

void CD3D9RenderBackend::RenderActors(const TActorRenderPacket* pPackets, unsigned int count)
{
	if (!pPackets || count == 0)
		return;

	LPDIRECT3DDEVICE9 dev = STATEMANAGER.GetDevice();
	if (!dev)
		return;

	for (unsigned int i = 0; i < count; ++i)
	{
		const TActorRenderPacket& pkt = pPackets[i];
		STATEMANAGER.SetTransform(D3DTS_WORLD, &pkt.matWorld);

		if (pkt.dwBoneCount > 0)
		{
			const D3DXMATRIX* pBones = pkt.pBoneMatrices ? pkt.pBoneMatrices : pkt.boneMatrices;
			UINT matrixCount = (std::min)(pkt.dwBoneCount, 60u);
			dev->SetVertexShaderConstantF(4, reinterpret_cast<const float*>(pBones), matrixCount * 4);
		}
	}
}

void CD3D9RenderBackend::RenderWater(const TWaterPacket* pPackets, unsigned int count)
{
	if (!pPackets || count == 0)
		return;
}

void CD3D9RenderBackend::RenderSpeedTree(const TSpeedTreePacket* pPackets, unsigned int count)
{
	if (!pPackets || count == 0)
		return;

	for (unsigned int i = 0; i < count; ++i)
	{
		const TSpeedTreePacket& packet = pPackets[i];
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

void CD3D9RenderBackend::RenderEffects(const TEffectInstancePacket* pPackets, unsigned int count)
{
	if (!pPackets || count == 0)
		return;
}

void CD3D9RenderBackend::RenderUI(const TUIBatchPacket* pPackets, unsigned int count)
{
	if (!pPackets || count == 0)
		return;

	LPDIRECT3DDEVICE9 dev = STATEMANAGER.GetDevice();
	if (!dev)
		return;

	if (!m_pUIVertexDecl)
		InitVertexDeclarations();

	STATEMANAGER.SaveRenderState(D3DRS_ALPHABLENDENABLE, TRUE);
	STATEMANAGER.SaveRenderState(D3DRS_SRCBLEND, D3DBLEND_SRCALPHA);
	STATEMANAGER.SaveRenderState(D3DRS_DESTBLEND, D3DBLEND_INVSRCALPHA);
	STATEMANAGER.SaveRenderState(D3DRS_ZENABLE, FALSE);
	STATEMANAGER.SaveRenderState(D3DRS_ZWRITEENABLE, FALSE);
	STATEMANAGER.SaveRenderState(D3DRS_CULLMODE, D3DCULL_NONE);
	STATEMANAGER.SaveRenderState(D3DRS_LIGHTING, FALSE);

	if (m_pUIVertexDecl)
		STATEMANAGER.SetVertexDeclaration(m_pUIVertexDecl);

	for (unsigned int i = 0; i < count; ++i)
	{
		const TUIBatchPacket& packet = pPackets[i];
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
}

void CD3D9RenderBackend::EndFrame()
{
}

const char* CD3D9RenderBackend::GetBackendName() const
{
	return "Direct3D9 Fixed/Programmable Hybrid";
}

bool CD3D9RenderBackend::IsDeviceLost() const
{
	LPDIRECT3DDEVICE9 dev = STATEMANAGER.GetDevice();
	if (!dev)
		return true;
	HRESULT hr = dev->TestCooperativeLevel();
	return (hr == D3DERR_DEVICELOST || hr == D3DERR_DEVICENOTRESET);
}
