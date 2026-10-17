#include "StdAfx.h"
#include "D3D9RenderBackend.h"
#include "StateManager.h"
#include "GrpBase.h"
#include "SpeedTreeLib/SpeedTreeWrapper.h"
#include "Render/TerrainShaderPipeline.h"

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
	if (m_pTerrainPipeline)
	{
		m_pTerrainPipeline->Shutdown();
		m_pTerrainPipeline.reset();
	}
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
		// Zgodnosc 1:1 z 24-bajtowym TUIVertex / TPDTVertex oraz macierza ortograficzna SetInterfaceRenderState
		const D3DVERTEXELEMENT9 uiDeclElements[] = {
			{ 0, 0,  D3DDECLTYPE_FLOAT3,   D3DDECLMETHOD_DEFAULT, D3DDECLUSAGE_POSITION, 0 },
			{ 0, 12, D3DDECLTYPE_D3DCOLOR, D3DDECLMETHOD_DEFAULT, D3DDECLUSAGE_COLOR,    0 },
			{ 0, 16, D3DDECLTYPE_FLOAT2,   D3DDECLMETHOD_DEFAULT, D3DDECLUSAGE_TEXCOORD, 0 },
			D3DDECL_END()
		};
		dev->CreateVertexDeclaration(uiDeclElements, &m_pUIVertexDecl);
	}

	if (!m_pDungeonVertexDecl)
	{
		// Zgodnosc 1:1 z 40-bajtowym TPNT2Vertex (pozycja, normalna, uv0_baza, uv1_lightmap)
		const D3DVERTEXELEMENT9 dungeonDeclElements[] = {
			{ 0, 0,  D3DDECLTYPE_FLOAT3, D3DDECLMETHOD_DEFAULT, D3DDECLUSAGE_POSITION, 0 },
			{ 0, 12, D3DDECLTYPE_FLOAT3, D3DDECLMETHOD_DEFAULT, D3DDECLUSAGE_NORMAL,   0 },
			{ 0, 24, D3DDECLTYPE_FLOAT2, D3DDECLMETHOD_DEFAULT, D3DDECLUSAGE_TEXCOORD, 0 },
			{ 0, 32, D3DDECLTYPE_FLOAT2, D3DDECLMETHOD_DEFAULT, D3DDECLUSAGE_TEXCOORD, 1 },
			D3DDECL_END()
		};
		dev->CreateVertexDeclaration(dungeonDeclElements, &m_pDungeonVertexDecl);
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
	if (m_pDungeonVertexDecl)
	{
		m_pDungeonVertexDecl->Release();
		m_pDungeonVertexDecl = nullptr;
	}
}

void CD3D9RenderBackend::OnLostDevice()
{
	if (m_pTerrainPipeline)
		m_pTerrainPipeline->Shutdown();
}

void CD3D9RenderBackend::OnResetDevice()
{
}

void CD3D9RenderBackend::BeginFrame(const TRenderFrameUniforms& uniforms)
{
	m_currentUniforms = uniforms;
	if (uniforms.matView._44 != 0.0f || uniforms.matView._11 != 0.0f || uniforms.matView._22 != 0.0f)
	{
		STATEMANAGER.SetTransform(D3DTS_VIEW, &uniforms.matView);
	}
	else
	{
		STATEMANAGER.GetTransform(D3DTS_VIEW, &m_currentUniforms.matView);
	}

	if (uniforms.matProj._34 != 0.0f || uniforms.matProj._11 != 0.0f || uniforms.matProj._22 != 0.0f)
	{
		STATEMANAGER.SetTransform(D3DTS_PROJECTION, &uniforms.matProj);
	}
	else
	{
		STATEMANAGER.GetTransform(D3DTS_PROJECTION, &m_currentUniforms.matProj);
	}

	D3DXMatrixMultiply(&m_currentUniforms.matViewProj, &m_currentUniforms.matView, &m_currentUniforms.matProj);
}

void CD3D9RenderBackend::RenderSkyBox(const TSkyBoxPacket* pPackets, unsigned int count)
{
	if (!pPackets || count == 0)
		return;

	// W trybie hybrydowym (Strangler Fig) CSkyBox::Render() oraz CSkyBox::RenderCloud()
	// odpowiadaja za bezposrednie rysowanie nieba i chmur w Direct3D 9 z pelnymi teksturami 64-bit.
	// Pakiety TSkyBoxPacket w kolejce sluza wylacznie do eksportu Golden Master oraz dla backendu WGPU.
	return;
}

void CD3D9RenderBackend::RenderTerrain(const TTerrainSectorPacket* pPackets, unsigned int count)
{
	if (!pPackets || count == 0)
		return;

	LPDIRECT3DDEVICE9 dev = STATEMANAGER.GetDevice();
	if (!dev)
		return;

	if (!m_pTerrainPipeline)
		m_pTerrainPipeline = std::make_unique<EterLib::Render::TerrainShaderPipeline>();

	if (!m_pTerrainPipeline->IsReady())
	{
		if (!m_pTerrainPipeline->Initialize(dev))
			return;
	}

	// W trybie hybrydowym (Strangler Fig) teren wraz z mapa cieni statycznych i dynamicznych
	// jest juz wyrenderowany w CMapOutdoor::RenderTerrain(). Pakiety TTerrainSectorPacket
	// sluza do eksportu Golden Master (frame_dump.bin) oraz dla backendu WGPU.
	if (pPackets[0].bPad == 0)
		return;

	const float fogColorAndRange[4] = {
		m_currentUniforms.vFogColor.r,
		m_currentUniforms.vFogColor.g,
		m_currentUniforms.vFogColor.b,
		(m_currentUniforms.fFogFar > m_currentUniforms.fFogNear) ? (1.0f / (m_currentUniforms.fFogFar - m_currentUniforms.fFogNear)) : 0.0001f
	};

	const D3DVECTOR lightDir = { m_currentUniforms.vSunLightDir.x, m_currentUniforms.vSunLightDir.y, m_currentUniforms.vSunLightDir.z };
	const D3DVECTOR ambientColor = { m_currentUniforms.vAmbientColor.r, m_currentUniforms.vAmbientColor.g, m_currentUniforms.vAmbientColor.b };

	m_pTerrainPipeline->BeginFrame(dev, m_currentUniforms.matViewProj, lightDir, ambientColor, fogColorAndRange);

	for (unsigned int i = 0; i < count; ++i)
	{
		const TTerrainSectorPacket& pkt = pPackets[i];
		if (!pkt.pVertexData || pkt.dwVertexCount == 0)
			continue;

		EterLib::Render::TerrainSectorBindDesc desc{};
		desc.sectorOriginX = pkt.fWorldPosX;
		desc.sectorOriginY = pkt.fWorldPosY;
		desc.invSectorSize = 1.0f / 25600.0f;
		desc.detailTileScale = 64.0f;
		desc.splatTexture = reinterpret_cast<LPDIRECT3DTEXTURE9>(pkt.pSplatTexture);

		for (int l = 0; l < 4; ++l)
		{
			desc.layerTextures[l] = reinterpret_cast<LPDIRECT3DTEXTURE9>(pkt.pLayerTextures[l]);
		}

		m_pTerrainPipeline->BindSector(dev, desc);

		auto* pD3DVB = reinterpret_cast<LPDIRECT3DVERTEXBUFFER9>(pkt.pVertexData);
		dev->SetStreamSource(0, pD3DVB, 0, sizeof(float) * 6);

		dev->DrawPrimitive(D3DPT_TRIANGLESTRIP, 0, pkt.dwVertexCount > 2 ? pkt.dwVertexCount - 2 : 0);
	}

	m_pTerrainPipeline->EndFrame(dev);
}

void CD3D9RenderBackend::RenderActors(const TActorRenderPacket* pPackets, unsigned int count)
{
	if (!pPackets || count == 0)
		return;

	LPDIRECT3DDEVICE9 dev = STATEMANAGER.GetDevice();
	if (!dev)
		return;

	D3DXMATRIX matSavedWorld;
	STATEMANAGER.GetTransform(D3DTS_WORLD, &matSavedWorld);

	for (unsigned int i = 0; i < count; ++i)
	{
		const TActorRenderPacket& pkt = pPackets[i];
		if (pkt.dwBoneCount > 0)
		{
			const D3DXMATRIX* pBones = pkt.GetBoneMatrices();
			UINT matrixCount = (std::min)(pkt.dwBoneCount, 60u);
			dev->SetVertexShaderConstantF(4, reinterpret_cast<const float*>(pBones), matrixCount * 4);
		}
	}

	STATEMANAGER.SetTransform(D3DTS_WORLD, &matSavedWorld);
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

	// Tryb hybrydowy (Strangler Fig): CSpeedTreeForestDirectX::Render() rysuje juz drzewa
	// z poprawnymi shaderami wiatru/lisci. Ponowne odtworzenie pakietow rysowalo kazde drzewo
	// DWA RAZY (bez stanu shaderow). Pakiety sluza tylko dla WGPU i frame_dump.bin.
	if (!m_bReplaySpeedTreePackets)
		return;

	for (unsigned int i = 0; i < count; ++i)
	{
		const TSpeedTreePacket& packet = pPackets[i];
		if (!packet.pMainTree || !packet.pInstances || packet.dwInstanceCount == 0)
			continue;

		if (packet.bRenderBranches)
		{
			packet.pMainTree->SetupBranchForTreeType();
			for (uint32_t idx = 0; idx < packet.dwInstanceCount; ++idx)
			{
				const auto& inst = packet.pInstances[idx];
				if (inst.pTreeInstance)
					const_cast<CSpeedTreeWrapper*>(inst.pTreeInstance)->RenderBranches();
			}
		}

		if (packet.bRenderFronds)
		{
			packet.pMainTree->SetupFrondForTreeType();
			for (uint32_t idx = 0; idx < packet.dwInstanceCount; ++idx)
			{
				const auto& inst = packet.pInstances[idx];
				if (inst.pTreeInstance)
					const_cast<CSpeedTreeWrapper*>(inst.pTreeInstance)->RenderFronds();
			}
		}

		if (packet.bRenderLeaves)
		{
			packet.pMainTree->SetupLeafForTreeType();
			for (uint32_t idx = 0; idx < packet.dwInstanceCount; ++idx)
			{
				const auto& inst = packet.pInstances[idx];
				if (inst.pTreeInstance)
					const_cast<CSpeedTreeWrapper*>(inst.pTreeInstance)->RenderLeaves();
			}
			const_cast<CSpeedTreeWrapper*>(packet.pMainTree)->EndLeafForTreeType();
		}

		if (packet.bRenderBillboards)
		{
			packet.pMainTree->SetupLeafForTreeType();
			for (uint32_t idx = 0; idx < packet.dwInstanceCount; ++idx)
			{
				const auto& inst = packet.pInstances[idx];
				if (inst.pTreeInstance)
					const_cast<CSpeedTreeWrapper*>(inst.pTreeInstance)->RenderBillboards();
			}
		}
	}
}

static void EnsureSharedQuadIndices(std::vector<uint16_t>& indices, uint32_t requiredQuads)
{
	if (indices.size() >= requiredQuads * 6)
		return;

	size_t oldQuads = indices.size() / 6;
	size_t newQuads = (std::max)(static_cast<size_t>(requiredQuads) + 512, static_cast<size_t>(2048));
	indices.resize(newQuads * 6);
	for (size_t q = oldQuads; q < newQuads; ++q)
	{
		uint16_t base = static_cast<uint16_t>(q * 4);
		indices[q * 6 + 0] = base + 0;
		indices[q * 6 + 1] = base + 2;
		indices[q * 6 + 2] = base + 1;
		indices[q * 6 + 3] = base + 2;
		indices[q * 6 + 4] = base + 3;
		indices[q * 6 + 5] = base + 1;
	}
}

void CD3D9RenderBackend::RenderEffects(const TEffectInstancePacket* pPackets, unsigned int count)
{
	if (!pPackets || count == 0)
		return;

	LPDIRECT3DDEVICE9 dev = STATEMANAGER.GetDevice();
	if (!dev)
		return;

	if (!m_pUIVertexDecl)
		InitVertexDeclarations();

	D3DXMATRIX matView;
	STATEMANAGER.GetTransform(D3DTS_VIEW, &matView);
	const D3DXVECTOR3 vRight(matView._11, matView._21, matView._31);
	const D3DXVECTOR3 vUp(matView._12, matView._22, matView._32);

	D3DXMATRIX matIdentity;
	D3DXMatrixIdentity(&matIdentity);
	STATEMANAGER.SaveTransform(D3DTS_WORLD, &matIdentity);

	STATEMANAGER.SaveRenderState(D3DRS_ALPHABLENDENABLE, TRUE);
	STATEMANAGER.SaveRenderState(D3DRS_ZWRITEENABLE, FALSE);
	STATEMANAGER.SaveRenderState(D3DRS_CULLMODE, D3DCULL_NONE);
	STATEMANAGER.SaveRenderState(D3DRS_LIGHTING, FALSE);

	if (m_pUIVertexDecl)
		STATEMANAGER.SetVertexDeclaration(m_pUIVertexDecl);

	static std::vector<TUIVertex> s_particleVertices;
	static std::vector<uint16_t> s_sharedEffectIndices;

	for (unsigned int i = 0; i < count; ++i)
	{
		const TEffectInstancePacket& pkt = pPackets[i];
		if (!pkt.pParticles || pkt.dwParticleCount == 0)
			continue;

		if (pkt.dwBlendMode == EFFECT_BLEND_ADDITIVE)
		{
			STATEMANAGER.SetRenderState(D3DRS_SRCBLEND, D3DBLEND_SRCALPHA);
			STATEMANAGER.SetRenderState(D3DRS_DESTBLEND, D3DBLEND_ONE);
		}
		else
		{
			STATEMANAGER.SetRenderState(D3DRS_SRCBLEND, D3DBLEND_SRCALPHA);
			STATEMANAGER.SetRenderState(D3DRS_DESTBLEND, D3DBLEND_INVSRCALPHA);
		}

		LPDIRECT3DBASETEXTURE9 pEffectTexture = reinterpret_cast<LPDIRECT3DBASETEXTURE9>(pkt.dwTextureID);
		bool bValidEffectTex = (pkt.dwTextureID != 0) && CStateManager::IsValidTexturePointer(pEffectTexture);

		if (bValidEffectTex)
		{
			STATEMANAGER.SetTexture(0, pEffectTexture);
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

		const uint32_t totalQuads = pkt.dwParticleCount;
		uint32_t particleOffset = 0;
		while (particleOffset < totalQuads)
		{
			const uint32_t numQuads = (std::min)(totalQuads - particleOffset, 16380u);
			if (s_particleVertices.size() < numQuads * 4)
				s_particleVertices.resize(numQuads * 4);

			EnsureSharedQuadIndices(s_sharedEffectIndices, numQuads);

			for (uint32_t pIdx = 0; pIdx < numQuads; ++pIdx)
			{
				const TParticleInstanceData& p = pkt.pParticles[particleOffset + pIdx];
				const D3DXVECTOR3 center(p.vWorldPos.x, p.vWorldPos.y, p.vWorldPos.z);
				const float sx = (p.fPad[0] > 0.0f) ? p.fPad[0] : p.vWorldPos.w;
				const float sy = (p.fPad[1] > 0.0f) ? p.fPad[1] : p.vWorldPos.w;

				D3DXVECTOR3 axisX = vRight * sx;
				D3DXVECTOR3 axisY = vUp * sy;
				if (p.fRotation != 0.0f)
				{
					const float rad = D3DXToRadian(p.fRotation);
					const float c = cosf(rad);
					const float s = sinf(rad);
					axisX = (vRight * c + vUp * s) * sx;
					axisY = (-vRight * s + vUp * c) * sy;
				}

				const uint32_t packedColor = static_cast<DWORD>(p.vColor);
				TUIVertex* v = &s_particleVertices[pIdx * 4];

				v[0].position = center - axisX + axisY;
				v[0].color = packedColor;
				v[0].texCoord = D3DXVECTOR2(p.vTexCoordRect.x, p.vTexCoordRect.y);

				v[1].position = center + axisX + axisY;
				v[1].color = packedColor;
				v[1].texCoord = D3DXVECTOR2(p.vTexCoordRect.z, p.vTexCoordRect.y);

				v[2].position = center - axisX - axisY;
				v[2].color = packedColor;
				v[2].texCoord = D3DXVECTOR2(p.vTexCoordRect.x, p.vTexCoordRect.w);

				v[3].position = center + axisX - axisY;
				v[3].color = packedColor;
				v[3].texCoord = D3DXVECTOR2(p.vTexCoordRect.z, p.vTexCoordRect.w);
			}

			STATEMANAGER.DrawIndexedPrimitiveUP(
				D3DPT_TRIANGLELIST,
				0,
				numQuads * 4,
				numQuads * 2,
				s_sharedEffectIndices.data(),
				D3DFMT_INDEX16,
				s_particleVertices.data(),
				static_cast<UINT>(sizeof(TUIVertex))
			);

			particleOffset += numQuads;
		}
	}

	STATEMANAGER.RestoreRenderState(D3DRS_LIGHTING);
	STATEMANAGER.RestoreRenderState(D3DRS_CULLMODE);
	STATEMANAGER.RestoreRenderState(D3DRS_ZWRITEENABLE);
	STATEMANAGER.RestoreRenderState(D3DRS_ALPHABLENDENABLE);
	STATEMANAGER.RestoreTransform(D3DTS_WORLD);
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

	D3DVIEWPORT9 vp;
	float screenW = static_cast<float>(m_width);
	float screenH = static_cast<float>(m_height);
	if (screenW <= 0.0f || screenH <= 0.0f)
	{
		if (SUCCEEDED(dev->GetViewport(&vp)))
		{
			screenW = static_cast<float>(vp.Width);
			screenH = static_cast<float>(vp.Height);
		}
	}

	D3DXMATRIX matIdentity;
	D3DXMatrixIdentity(&matIdentity);

	D3DXMATRIX matOrthoProj;
	D3DXMatrixOrthoOffCenterRH(&matOrthoProj, 0.0f, screenW, screenH, 0.0f, -1.0f, 1.0f);

	STATEMANAGER.SaveTransform(D3DTS_WORLD, &matIdentity);
	STATEMANAGER.SaveTransform(D3DTS_VIEW, &matIdentity);
	STATEMANAGER.SaveTransform(D3DTS_PROJECTION, &matOrthoProj);

	STATEMANAGER.SaveRenderState(D3DRS_ALPHABLENDENABLE, TRUE);
	STATEMANAGER.SaveRenderState(D3DRS_SRCBLEND, D3DBLEND_SRCALPHA);
	STATEMANAGER.SaveRenderState(D3DRS_DESTBLEND, D3DBLEND_INVSRCALPHA);
	STATEMANAGER.SaveRenderState(D3DRS_ZENABLE, FALSE);
	STATEMANAGER.SaveRenderState(D3DRS_ZWRITEENABLE, FALSE);
	STATEMANAGER.SaveRenderState(D3DRS_CULLMODE, D3DCULL_NONE);
	STATEMANAGER.SaveRenderState(D3DRS_LIGHTING, FALSE);

	if (m_pUIVertexDecl)
		STATEMANAGER.SetVertexDeclaration(m_pUIVertexDecl);

	static std::vector<uint16_t> s_sharedQuadIndices;

	for (unsigned int i = 0; i < count; ++i)
	{
		const TUIBatchPacket& packet = pPackets[i];
		const TUIVertex* pVtx = packet.pVertices;
		size_t vtxCount = packet.dwVertexCount;

		if (!pVtx || vtxCount == 0)
			continue;

		if (packet.bUseScissor)
		{
			STATEMANAGER.SetRenderState(D3DRS_SCISSORTESTENABLE, TRUE);
			STATEMANAGER.SetScissorRect(packet.scissorRect);
		}
		else
		{
			STATEMANAGER.SetRenderState(D3DRS_SCISSORTESTENABLE, FALSE);
		}

		if (packet.dwBlendMode == UI_BLEND_SCREEN)
		{
			STATEMANAGER.SetRenderState(D3DRS_SRCBLEND, D3DBLEND_ONE);
			STATEMANAGER.SetRenderState(D3DRS_DESTBLEND, D3DBLEND_INVSRCCOLOR);
		}
		else if (packet.dwBlendMode == UI_BLEND_ADDITIVE)
		{
			STATEMANAGER.SetRenderState(D3DRS_SRCBLEND, D3DBLEND_SRCALPHA);
			STATEMANAGER.SetRenderState(D3DRS_DESTBLEND, D3DBLEND_ONE);
		}
		else
		{
			STATEMANAGER.SetRenderState(D3DRS_SRCBLEND, D3DBLEND_SRCALPHA);
			STATEMANAGER.SetRenderState(D3DRS_DESTBLEND, D3DBLEND_INVSRCALPHA);
		}

		uintptr_t texId = packet.dwTextureID;
		LPDIRECT3DBASETEXTURE9 pD3DTexture = reinterpret_cast<LPDIRECT3DBASETEXTURE9>(texId);
		bool bValidTexture = (texId != 0) && CStateManager::IsValidTexturePointer(pD3DTexture);

		if (bValidTexture)
		{
			STATEMANAGER.SetTexture(0, pD3DTexture);
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

		const uint32_t totalQuads = static_cast<uint32_t>(vtxCount / 4);
		uint32_t quadsDrawn = 0;
		while (quadsDrawn < totalQuads)
		{
			const uint32_t numQuads = (std::min)(totalQuads - quadsDrawn, 16380u);
			EnsureSharedQuadIndices(s_sharedQuadIndices, numQuads);

			STATEMANAGER.DrawIndexedPrimitiveUP(
				D3DPT_TRIANGLELIST,
				0,
				numQuads * 4,
				numQuads * 2,
				s_sharedQuadIndices.data(),
				D3DFMT_INDEX16,
				pVtx + (quadsDrawn * 4),
				static_cast<UINT>(sizeof(TUIVertex))
			);

			quadsDrawn += numQuads;
		}
	}

	STATEMANAGER.RestoreRenderState(D3DRS_LIGHTING);
	STATEMANAGER.RestoreRenderState(D3DRS_CULLMODE);
	STATEMANAGER.RestoreRenderState(D3DRS_ZWRITEENABLE);
	STATEMANAGER.RestoreRenderState(D3DRS_ZENABLE);
	STATEMANAGER.RestoreRenderState(D3DRS_DESTBLEND);
	STATEMANAGER.RestoreRenderState(D3DRS_SRCBLEND);
	STATEMANAGER.RestoreRenderState(D3DRS_ALPHABLENDENABLE);

	STATEMANAGER.RestoreTransform(D3DTS_PROJECTION);
	STATEMANAGER.RestoreTransform(D3DTS_VIEW);
	STATEMANAGER.RestoreTransform(D3DTS_WORLD);
}

void CD3D9RenderBackend::RenderDungeonBlocks(const TDungeonBlockPacket* pPackets, unsigned int count)
{
	if (!pPackets || count == 0)
		return;

	LPDIRECT3DDEVICE9 dev = STATEMANAGER.GetDevice();
	if (!dev)
		return;

	if (!m_pDungeonVertexDecl)
		InitVertexDeclarations();

	D3DXMATRIX matSavedWorld;
	STATEMANAGER.GetTransform(D3DTS_WORLD, &matSavedWorld);

	STATEMANAGER.SaveRenderState(D3DRS_LIGHTING, FALSE);
	STATEMANAGER.SaveRenderState(D3DRS_CULLMODE, D3DCULL_CCW);
	STATEMANAGER.SaveRenderState(D3DRS_ALPHABLENDENABLE, TRUE);
	STATEMANAGER.SaveRenderState(D3DRS_SRCBLEND, D3DBLEND_SRCALPHA);
	STATEMANAGER.SaveRenderState(D3DRS_DESTBLEND, D3DBLEND_INVSRCALPHA);
	STATEMANAGER.SaveRenderState(D3DRS_ZENABLE, TRUE);
	STATEMANAGER.SaveRenderState(D3DRS_ZWRITEENABLE, TRUE);

	// Uzycie deklaracji wierzcholka TPNT2Vertex (pozycja, normalna, uv0_baza, uv1_lightmap)
	if (m_pDungeonVertexDecl)
		STATEMANAGER.SetVertexDeclaration(m_pDungeonVertexDecl);

	// Konfiguracja kaskady 2-teksturowej dla bezpiecznego fallbacku D3D9
	// Stopien 0: Baza diffuse
	STATEMANAGER.SetTextureStageState(0, D3DTSS_COLOROP, D3DTOP_MODULATE);
	STATEMANAGER.SetTextureStageState(0, D3DTSS_COLORARG1, D3DTA_TEXTURE);
	STATEMANAGER.SetTextureStageState(0, D3DTSS_COLORARG2, D3DTA_CURRENT);
	STATEMANAGER.SetTextureStageState(0, D3DTSS_ALPHAOP, D3DTOP_MODULATE);
	STATEMANAGER.SetTextureStageState(0, D3DTSS_ALPHAARG1, D3DTA_TEXTURE);
	STATEMANAGER.SetTextureStageState(0, D3DTSS_ALPHAARG2, D3DTA_CURRENT);
	STATEMANAGER.SetTextureStageState(0, D3DTSS_TEXCOORDINDEX, 0);

	// Stopien 1: Lightmap z podwojnym mnozeniem (MODULATE2X prebaked lighting)
	STATEMANAGER.SetTextureStageState(1, D3DTSS_COLOROP, D3DTOP_MODULATE2X);
	STATEMANAGER.SetTextureStageState(1, D3DTSS_COLORARG1, D3DTA_TEXTURE);
	STATEMANAGER.SetTextureStageState(1, D3DTSS_COLORARG2, D3DTA_CURRENT);
	STATEMANAGER.SetTextureStageState(1, D3DTSS_ALPHAOP, D3DTOP_SELECTARG2);
	STATEMANAGER.SetTextureStageState(1, D3DTSS_ALPHAARG1, D3DTA_TEXTURE);
	STATEMANAGER.SetTextureStageState(1, D3DTSS_ALPHAARG2, D3DTA_CURRENT);
	STATEMANAGER.SetTextureStageState(1, D3DTSS_TEXCOORDINDEX, 1);

	for (unsigned int i = 0; i < count; ++i)
	{
		const TDungeonBlockPacket& pkt = pPackets[i];
		STATEMANAGER.SetTransform(D3DTS_WORLD, &pkt.matWorld);

		// Ustawienie tekstury bazowej (diffuse)
		LPDIRECT3DBASETEXTURE9 pBaseTex = reinterpret_cast<LPDIRECT3DBASETEXTURE9>(pkt.pDiffuseTexture);
		if (!pBaseTex && pkt.dwDiffuseTextureID >= 0x10000)
			pBaseTex = reinterpret_cast<LPDIRECT3DBASETEXTURE9>(pkt.dwDiffuseTextureID);
		STATEMANAGER.SetTexture(0, pBaseTex);

		// Ustawienie tekstury mapy swiatla (lightmap)
		LPDIRECT3DBASETEXTURE9 pLightmapTex = reinterpret_cast<LPDIRECT3DBASETEXTURE9>(pkt.pLightmapTexture);
		if (!pLightmapTex && pkt.dwLightmapTextureID >= 0x10000)
			pLightmapTex = reinterpret_cast<LPDIRECT3DBASETEXTURE9>(pkt.dwLightmapTextureID);
		STATEMANAGER.SetTexture(1, pLightmapTex);

		// Wariant 1: Rysowanie z buforow Direct3D9 (VertexBuffer / IndexBuffer)
		if (pkt.pVertexBuffer)
		{
			auto* pVB = reinterpret_cast<LPDIRECT3DVERTEXBUFFER9>(pkt.pVertexBuffer);
			STATEMANAGER.SetStreamSource(0, pVB, sizeof(TPNT2Vertex));

			if (pkt.pIndexBuffer)
			{
				auto* pIB = reinterpret_cast<LPDIRECT3DINDEXBUFFER9>(pkt.pIndexBuffer);
				STATEMANAGER.SetIndices(pIB, pkt.nBaseVertexIndex);
				const uint32_t primCount = pkt.dwPrimitiveCount > 0 ? pkt.dwPrimitiveCount : (pkt.dwIndexCount / 3);
				if (primCount > 0)
				{
					STATEMANAGER.DrawIndexedPrimitive(
						D3DPT_TRIANGLELIST,
						pkt.nBaseVertexIndex,
						0,
						pkt.dwVertexCount,
						pkt.dwStartIndex,
						primCount
					);
				}
			}
			else
			{
				const uint32_t primCount = pkt.dwPrimitiveCount > 0 ? pkt.dwPrimitiveCount : (pkt.dwVertexCount / 3);
				if (primCount > 0)
				{
					STATEMANAGER.DrawPrimitive(D3DPT_TRIANGLELIST, pkt.dwStartIndex, primCount);
				}
			}
		}
		// Wariant 2: Rysowanie ze wskaznikow pamieci CPU (User Pointer)
		else if (pkt.pVertices && pkt.dwVertexCount > 0)
		{
			if (pkt.pIndices && pkt.dwIndexCount > 0)
			{
				const uint32_t primCount = pkt.dwPrimitiveCount > 0 ? pkt.dwPrimitiveCount : (pkt.dwIndexCount / 3);
				if (primCount > 0)
				{
					STATEMANAGER.DrawIndexedPrimitiveUP(
						D3DPT_TRIANGLELIST,
						0,
						pkt.dwVertexCount,
						primCount,
						pkt.pIndices + pkt.dwStartIndex,
						D3DFMT_INDEX16,
						pkt.pVertices,
						sizeof(TPNT2Vertex)
					);
				}
			}
			else
			{
				const uint32_t primCount = pkt.dwPrimitiveCount > 0 ? pkt.dwPrimitiveCount : (pkt.dwVertexCount / 3);
				if (primCount > 0)
				{
					STATEMANAGER.DrawPrimitiveUP(
						D3DPT_TRIANGLELIST,
						primCount,
						pkt.pVertices + pkt.dwStartIndex,
						sizeof(TPNT2Vertex)
					);
				}
			}
		}
	}

	// Przywrocenie domyslnego stanu stopnia 1 (wylaczenie Lightmap Stage 1)
	STATEMANAGER.SetTexture(1, nullptr);
	STATEMANAGER.SetTextureStageState(1, D3DTSS_COLOROP, D3DTOP_DISABLE);
	STATEMANAGER.SetTextureStageState(1, D3DTSS_ALPHAOP, D3DTOP_DISABLE);
	STATEMANAGER.SetTexture(0, nullptr);

	STATEMANAGER.RestoreRenderState(D3DRS_ZWRITEENABLE);
	STATEMANAGER.RestoreRenderState(D3DRS_ZENABLE);
	STATEMANAGER.RestoreRenderState(D3DRS_DESTBLEND);
	STATEMANAGER.RestoreRenderState(D3DRS_SRCBLEND);
	STATEMANAGER.RestoreRenderState(D3DRS_ALPHABLENDENABLE);
	STATEMANAGER.RestoreRenderState(D3DRS_CULLMODE);
	STATEMANAGER.RestoreRenderState(D3DRS_LIGHTING);
	STATEMANAGER.SetTransform(D3DTS_WORLD, &matSavedWorld);
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

bool CD3D9RenderBackend::CaptureFrame(const char* szFilename)
{
	if (!szFilename)
		return false;

	LPDIRECT3DDEVICE9 dev = STATEMANAGER.GetDevice();
	if (!dev)
		return false;

	LPDIRECT3DSURFACE9 pBackBuffer = nullptr;
	if (FAILED(dev->GetBackBuffer(0, 0, D3DBACKBUFFER_TYPE_MONO, &pBackBuffer)) || !pBackBuffer)
		return false;

	std::string fileName = szFilename;
	if (fileName.length() < 4 || (fileName.compare(fileName.length() - 4, 4, ".png") != 0 && fileName.compare(fileName.length() - 4, 4, ".PNG") != 0))
	{
		fileName += ".png";
	}

	HRESULT hr = D3DXSaveSurfaceToFileA(fileName.c_str(), D3DXIFF_PNG, pBackBuffer, nullptr, nullptr);
	pBackBuffer->Release();
	return SUCCEEDED(hr);
}
