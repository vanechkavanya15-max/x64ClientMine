#include "StdAfx.h"
#include "D3D9RenderBackend.h"
#include "StateManager.h"
#include "GrpBase.h"
#include "RenderOwnership.h"
#include "SpeedTreeLib/SpeedTreeWrapper.h"
#include "Render/TerrainShaderPipeline.h"
#include "RenderResourceRegistry.h"
#include "CFrameSnapshot.h"
#include "RenderQueueManager.h"
#include "SkyBox.h"
#include "Camera.h"
#include "ParticleQuad.h"

CD3D9RenderBackend::CD3D9RenderBackend()
{
}

CD3D9RenderBackend::~CD3D9RenderBackend()
{
	Shutdown();
}

bool CD3D9RenderBackend::Initialize(HWND /*hWnd*/, unsigned int width, unsigned int height)
{
	m_width = width;
	m_height = height;
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
}

void CD3D9RenderBackend::Resize(unsigned int width, unsigned int height)
{
	m_width = width;
	m_height = height;
}

void CD3D9RenderBackend::InitVertexDeclarations()
{
	LPDIRECT3DDEVICE9 pD3DDevice = STATEMANAGER.GetDevice();
	if (!pD3DDevice)
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
		pD3DDevice->CreateVertexDeclaration(uiDeclElements, &m_pUIVertexDecl);
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
		pD3DDevice->CreateVertexDeclaration(dungeonDeclElements, &m_pDungeonVertexDecl);
	}
}

void CD3D9RenderBackend::ReleaseVertexDeclarations()
{
	if (m_pUIVertexDecl)
	{
		m_pUIVertexDecl->Release();
		m_pUIVertexDecl = nullptr;
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
	const TFrameSnapshot& snapshot = CFrameSnapshot::Current();

	if (m_currentUniforms.matView._44 == 0.0f && m_currentUniforms.matView._11 == 0.0f && m_currentUniforms.matView._22 == 0.0f)
	{
		m_currentUniforms.matView = snapshot.matView;
	}
	STATEMANAGER.SetTransform(D3DTS_VIEW, &m_currentUniforms.matView);

	if (m_currentUniforms.matProj._34 == 0.0f && m_currentUniforms.matProj._11 == 0.0f && m_currentUniforms.matProj._22 == 0.0f)
	{
		m_currentUniforms.matProj = snapshot.matProj;
	}
	STATEMANAGER.SetTransform(D3DTS_PROJECTION, &m_currentUniforms.matProj);

	if (m_currentUniforms.matViewProj._44 == 0.0f && m_currentUniforms.matViewProj._11 == 0.0f)
	{
		m_currentUniforms.matViewProj = snapshot.matViewProj;
	}
	else
	{
		D3DXMatrixMultiply(&m_currentUniforms.matViewProj, &m_currentUniforms.matView, &m_currentUniforms.matProj);
	}
}

void CD3D9RenderBackend::RenderSkyBox(const TSkyBoxPacket* pPackets, unsigned int count)
{
	if (RenderOwnership::LegacyDraws(ERenderCategory::SkyBox))
		return;

	if (RenderOwnership::Owner(ERenderCategory::SkyBox) != ERenderOwner::D3D9Backend)
		return;

	if (!pPackets || count == 0)
		return;

	if (!CStateManager::InstancePtr())
		return;

	LPDIRECT3DDEVICE9 pD3DDevice = STATEMANAGER.GetDevice();
	if (!pD3DDevice)
		return;

	const SSkyDescription* pSkyDesc = CRenderQueueManager::Instance().GetSkyDescription();

	// Geometria 6 scian standardowego skyboxa
	struct SSkyBoxVertexFace
	{
		D3DXVECTOR3 pos[4];
	};

	static const SSkyBoxVertexFace s_faces[6] = {
		// 0: FRONT
		{ { D3DXVECTOR3(1.0f, -1.0f, -1.0f), D3DXVECTOR3(1.0f, -1.0f, 1.0f), D3DXVECTOR3(-1.0f, -1.0f, -1.0f), D3DXVECTOR3(-1.0f, -1.0f, 1.0f) } },
		// 1: BACK
		{ { D3DXVECTOR3(-1.0f, 1.0f, -1.0f), D3DXVECTOR3(-1.0f, 1.0f, 1.0f), D3DXVECTOR3(1.0f, 1.0f, -1.0f), D3DXVECTOR3(1.0f, 1.0f, 1.0f) } },
		// 2: LEFT
		{ { D3DXVECTOR3(1.0f, 1.0f, -1.0f), D3DXVECTOR3(1.0f, 1.0f, 1.0f), D3DXVECTOR3(1.0f, -1.0f, -1.0f), D3DXVECTOR3(1.0f, -1.0f, 1.0f) } },
		// 3: RIGHT
		{ { D3DXVECTOR3(-1.0f, -1.0f, -1.0f), D3DXVECTOR3(-1.0f, -1.0f, 1.0f), D3DXVECTOR3(-1.0f, 1.0f, -1.0f), D3DXVECTOR3(-1.0f, 1.0f, 1.0f) } },
		// 4: TOP
		{ { D3DXVECTOR3(1.0f, -1.0f, 1.0f), D3DXVECTOR3(1.0f, 1.0f, 1.0f), D3DXVECTOR3(-1.0f, -1.0f, 1.0f), D3DXVECTOR3(-1.0f, 1.0f, 1.0f) } },
		// 5: BOTTOM
		{ { D3DXVECTOR3(1.0f, -1.0f, -1.0f), D3DXVECTOR3(1.0f, 1.0f, -1.0f), D3DXVECTOR3(-1.0f, -1.0f, -1.0f), D3DXVECTOR3(-1.0f, 1.0f, -1.0f) } }
	};

	for (unsigned int i = 0; i < count; ++i)
	{
		const TSkyBoxPacket& pkt = pPackets[i];

		// 1. Odtworzenie pelnego stanu nieba (SELECTARG1 / SELECTARG2)
		STATEMANAGER.SaveRenderState(D3DRS_ZENABLE, TRUE);
		STATEMANAGER.SaveRenderState(D3DRS_ZWRITEENABLE, FALSE);
		STATEMANAGER.SaveRenderState(D3DRS_LIGHTING, FALSE);
		STATEMANAGER.SaveRenderState(D3DRS_FOGENABLE, FALSE);
		STATEMANAGER.SaveRenderState(D3DRS_ALPHABLENDENABLE, FALSE);

		STATEMANAGER.SaveTextureStageState(0, D3DTSS_COLOROP, D3DTOP_SELECTARG2);
		STATEMANAGER.SaveTextureStageState(0, D3DTSS_COLORARG1, D3DTA_TEXTURE);
		STATEMANAGER.SaveTextureStageState(0, D3DTSS_COLORARG2, D3DTA_DIFFUSE);
		STATEMANAGER.SaveTextureStageState(0, D3DTSS_ALPHAOP, D3DTOP_DISABLE);

		STATEMANAGER.SaveTexture(1, NULL);
		STATEMANAGER.SaveTextureStageState(1, D3DTSS_COLOROP, D3DTOP_DISABLE);
		STATEMANAGER.SaveTextureStageState(1, D3DTSS_ALPHAOP, D3DTOP_DISABLE);

		STATEMANAGER.SetFVF(D3DFVF_XYZ | D3DFVF_DIFFUSE | D3DFVF_TEX1);

		const D3DXMATRIX& matSky = (pSkyDesc != nullptr) ? pSkyDesc->matSkyWorld : pkt.matWorld;
		STATEMANAGER.SaveTransform(D3DTS_WORLD, &matSky);

		const bool bTextureMode = (pSkyDesc != nullptr && pSkyDesc->ucRenderMode == CSkyObject::SKY_RENDER_MODE_TEXTURE) ||
			(pkt.dwSkyTextureID != kInvalidTextureHandle);

		if (bTextureMode)
		{
			STATEMANAGER.SetTextureStageState(0, D3DTSS_COLOROP, D3DTOP_SELECTARG1);
			STATEMANAGER.SaveSamplerState(0, D3DSAMP_ADDRESSU, D3DTADDRESS_CLAMP);
			STATEMANAGER.SaveSamplerState(0, D3DSAMP_ADDRESSV, D3DTADDRESS_CLAMP);

			for (unsigned int f = 0; f < 6; ++f)
			{
				TRenderTextureHandle hTex = kInvalidTextureHandle;
				if (pSkyDesc && pSkyDesc->faceTextures[f] != kInvalidTextureHandle)
					hTex = pSkyDesc->faceTextures[f];
				else
					hTex = pkt.dwSkyTextureID;

				LPDIRECT3DTEXTURE9 pD3DTex = (hTex != kInvalidTextureHandle && CRenderResourceRegistry::InstancePtr())
					? CRenderResourceRegistry::Instance().ResolveTexture(hTex)
					: nullptr;

				STATEMANAGER.SetTexture(0, pD3DTex);

				if (pSkyDesc && f < 6 && !pSkyDesc->faces[f].vertices.empty())
				{
					const auto& verts = pSkyDesc->faces[f].vertices;
					for (size_t q = 0; q + 4 <= verts.size(); q += 4)
					{
						if (CGraphicBase::SetPDTStream(const_cast<TPDTVertex*>(&verts[q]), 4))
							STATEMANAGER.DrawPrimitive(D3DPT_TRIANGLESTRIP, 0, 2);
					}
				}
				else
				{
					TPDTVertex quad[4];
					quad[0].position = s_faces[f].pos[0];
					quad[0].texCoord = D3DXVECTOR2(0.0f, 1.0f);
					quad[0].diffuse = 0xFFFFFFFF;

					quad[1].position = s_faces[f].pos[1];
					quad[1].texCoord = D3DXVECTOR2(0.0f, 0.0f);
					quad[1].diffuse = 0xFFFFFFFF;

					quad[2].position = s_faces[f].pos[2];
					quad[2].texCoord = D3DXVECTOR2(1.0f, 1.0f);
					quad[2].diffuse = 0xFFFFFFFF;

					quad[3].position = s_faces[f].pos[3];
					quad[3].texCoord = D3DXVECTOR2(1.0f, 0.0f);
					quad[3].diffuse = 0xFFFFFFFF;

					if (CGraphicBase::SetPDTStream(quad, 4))
						STATEMANAGER.DrawPrimitive(D3DPT_TRIANGLESTRIP, 0, 2);
				}
			}

			STATEMANAGER.RestoreSamplerState(0, D3DSAMP_ADDRESSU);
			STATEMANAGER.RestoreSamplerState(0, D3DSAMP_ADDRESSV);
		}
		else
		{
			// Tryb diffuse / gradientowy (SELECTARG2)
			D3DCOLOR dwDiffuse = D3DCOLOR_COLORVALUE(pkt.vSkyColor.r, pkt.vSkyColor.g, pkt.vSkyColor.b, pkt.vSkyColor.a);
			if (dwDiffuse == 0)
				dwDiffuse = 0xFFFFFFFF;

			if (pSkyDesc && !pSkyDesc->gradientVertices.empty())
			{
				const auto& gverts = pSkyDesc->gradientVertices;
				for (size_t q = 0; q + 4 <= gverts.size(); q += 4)
				{
					if (CGraphicBase::SetPDTStream(const_cast<TPDTVertex*>(&gverts[q]), 4))
						STATEMANAGER.DrawPrimitive(D3DPT_TRIANGLESTRIP, 0, 2);
				}
			}
			else
			{
				for (unsigned int f = 0; f < 6; ++f)
				{
					TPDTVertex quad[4];
					quad[0].position = s_faces[f].pos[0];
					quad[0].texCoord = D3DXVECTOR2(0.0f, 1.0f);
					quad[0].diffuse = dwDiffuse;

					quad[1].position = s_faces[f].pos[1];
					quad[1].texCoord = D3DXVECTOR2(0.0f, 0.0f);
					quad[1].diffuse = dwDiffuse;

					quad[2].position = s_faces[f].pos[2];
					quad[2].texCoord = D3DXVECTOR2(1.0f, 1.0f);
					quad[2].diffuse = dwDiffuse;

					quad[3].position = s_faces[f].pos[3];
					quad[3].texCoord = D3DXVECTOR2(1.0f, 0.0f);
					quad[3].diffuse = dwDiffuse;

					if (CGraphicBase::SetPDTStream(quad, 4))
						STATEMANAGER.DrawPrimitive(D3DPT_TRIANGLESTRIP, 0, 2);
				}
			}
		}

		STATEMANAGER.RestoreTransform(D3DTS_WORLD);

		STATEMANAGER.RestoreTextureStageState(1, D3DTSS_ALPHAOP);
		STATEMANAGER.RestoreTextureStageState(1, D3DTSS_COLOROP);
		STATEMANAGER.RestoreTexture(1);

		STATEMANAGER.RestoreTextureStageState(0, D3DTSS_ALPHAOP);
		STATEMANAGER.RestoreTextureStageState(0, D3DTSS_COLORARG2);
		STATEMANAGER.RestoreTextureStageState(0, D3DTSS_COLORARG1);
		STATEMANAGER.RestoreTextureStageState(0, D3DTSS_COLOROP);

		STATEMANAGER.RestoreRenderState(D3DRS_ALPHABLENDENABLE);
		STATEMANAGER.RestoreRenderState(D3DRS_FOGENABLE);
		STATEMANAGER.RestoreRenderState(D3DRS_LIGHTING);
		STATEMANAGER.RestoreRenderState(D3DRS_ZWRITEENABLE);
		STATEMANAGER.RestoreRenderState(D3DRS_ZENABLE);

		// 2. Odtworzenie pelnego stanu chmur:
		// (D3DTS_TEXTURE0 + COUNT2, MODULATEINVALPHA_ADDCOLOR, ONE/INVSRCCOLOR)
		const TRenderTextureHandle hCloudTex = (pSkyDesc && pSkyDesc->cloudTextureHandle != kInvalidTextureHandle)
			? pSkyDesc->cloudTextureHandle
			: pkt.dwCloudTextureID;

		if (hCloudTex != kInvalidTextureHandle)
		{
			LPDIRECT3DTEXTURE9 pCloudTex = CRenderResourceRegistry::InstancePtr()
				? CRenderResourceRegistry::Instance().ResolveTexture(hCloudTex)
				: nullptr;

			if (pCloudTex)
			{
				STATEMANAGER.SaveRenderState(D3DRS_ZENABLE, TRUE);
				STATEMANAGER.SaveRenderState(D3DRS_ZWRITEENABLE, FALSE);
				STATEMANAGER.SaveRenderState(D3DRS_LIGHTING, FALSE);
				STATEMANAGER.SaveRenderState(D3DRS_FOGENABLE, FALSE);
				STATEMANAGER.SaveRenderState(D3DRS_ALPHABLENDENABLE, TRUE);
				STATEMANAGER.SaveRenderState(D3DRS_SRCBLEND, D3DBLEND_ONE);
				STATEMANAGER.SaveRenderState(D3DRS_DESTBLEND, D3DBLEND_INVSRCCOLOR);

				STATEMANAGER.SaveTextureStageState(0, D3DTSS_TEXTURETRANSFORMFLAGS, D3DTTFF_COUNT2);

				D3DXMATRIX matTextureCloud;
				if (pSkyDesc)
				{
					matTextureCloud = pSkyDesc->matCloudTexture;
				}
				else
				{
					D3DXMatrixIdentity(&matTextureCloud);
					matTextureCloud._11 = (pkt.fCloudScale != 0.0f) ? pkt.fCloudScale : 1.0f;
					matTextureCloud._22 = (pkt.fCloudScale != 0.0f) ? pkt.fCloudScale : 1.0f;
				}

				STATEMANAGER.SaveTransform(D3DTS_TEXTURE0, &matTextureCloud);

				STATEMANAGER.SaveTextureStageState(0, D3DTSS_COLOROP, D3DTOP_MODULATEINVALPHA_ADDCOLOR);
				STATEMANAGER.SaveTextureStageState(0, D3DTSS_COLORARG1, D3DTA_TEXTURE);
				STATEMANAGER.SaveTextureStageState(0, D3DTSS_COLORARG2, D3DTA_DIFFUSE);
				STATEMANAGER.SaveTextureStageState(0, D3DTSS_ALPHAOP, D3DTOP_SELECTARG1);
				STATEMANAGER.SaveTextureStageState(0, D3DTSS_ALPHAARG1, D3DTA_TEXTURE);

				D3DXMATRIX matProjCloud;
				D3DXMatrixPerspectiveFovRH(&matProjCloud, D3DX_PI * 0.25f, 1.33333f, 50.0f, 999999.0f);

				D3DXMATRIX matWorldCloud;
				if (pSkyDesc)
				{
					matWorldCloud = pSkyDesc->matCloudWorld;
				}
				else
				{
					float fScale = (pkt.fCloudScale != 0.0f) ? pkt.fCloudScale : 1.0f;
					D3DXMatrixScaling(&matWorldCloud, fScale, fScale, 1.0f);
					matWorldCloud._43 = pkt.fCloudHeight;
				}

				STATEMANAGER.SaveTransform(D3DTS_WORLD, &matWorldCloud);
				STATEMANAGER.SaveTransform(D3DTS_PROJECTION, &matProjCloud);
				STATEMANAGER.SetTexture(0, pCloudTex);

				if (pSkyDesc && !pSkyDesc->faceCloud.vertices.empty())
				{
					const auto& cverts = pSkyDesc->faceCloud.vertices;
					for (size_t q = 0; q + 4 <= cverts.size(); q += 4)
					{
						if (CGraphicBase::SetPDTStream(const_cast<TPDTVertex*>(&cverts[q]), 4))
							STATEMANAGER.DrawPrimitive(D3DPT_TRIANGLESTRIP, 0, 2);
					}
				}
				else
				{
					TPDTVertex cloudQuad[4];
					cloudQuad[0].position = D3DXVECTOR3(1.0f, 1.0f, 0.0f);
					cloudQuad[0].texCoord = D3DXVECTOR2(0.0f, 1.0f);
					cloudQuad[0].diffuse = 0xFFFFFFFF;

					cloudQuad[1].position = D3DXVECTOR3(-1.0f, 1.0f, 0.0f);
					cloudQuad[1].texCoord = D3DXVECTOR2(0.0f, 0.0f);
					cloudQuad[1].diffuse = 0xFFFFFFFF;

					cloudQuad[2].position = D3DXVECTOR3(1.0f, -1.0f, 0.0f);
					cloudQuad[2].texCoord = D3DXVECTOR2(1.0f, 1.0f);
					cloudQuad[2].diffuse = 0xFFFFFFFF;

					cloudQuad[3].position = D3DXVECTOR3(-1.0f, -1.0f, 0.0f);
					cloudQuad[3].texCoord = D3DXVECTOR2(1.0f, 0.0f);
					cloudQuad[3].diffuse = 0xFFFFFFFF;

					if (CGraphicBase::SetPDTStream(cloudQuad, 4))
						STATEMANAGER.DrawPrimitive(D3DPT_TRIANGLESTRIP, 0, 2);
				}

				STATEMANAGER.RestoreTransform(D3DTS_PROJECTION);
				STATEMANAGER.RestoreTransform(D3DTS_WORLD);

				STATEMANAGER.RestoreTextureStageState(0, D3DTSS_ALPHAARG1);
				STATEMANAGER.RestoreTextureStageState(0, D3DTSS_ALPHAOP);
				STATEMANAGER.RestoreTextureStageState(0, D3DTSS_COLORARG2);
				STATEMANAGER.RestoreTextureStageState(0, D3DTSS_COLORARG1);
				STATEMANAGER.RestoreTextureStageState(0, D3DTSS_COLOROP);

				STATEMANAGER.RestoreTransform(D3DTS_TEXTURE0);
				STATEMANAGER.RestoreTextureStageState(0, D3DTSS_TEXTURETRANSFORMFLAGS);

				STATEMANAGER.RestoreRenderState(D3DRS_LIGHTING);
				STATEMANAGER.RestoreRenderState(D3DRS_ZENABLE);
				STATEMANAGER.RestoreRenderState(D3DRS_ZWRITEENABLE);
				STATEMANAGER.RestoreRenderState(D3DRS_FOGENABLE);
				STATEMANAGER.RestoreRenderState(D3DRS_ALPHABLENDENABLE);
				STATEMANAGER.RestoreRenderState(D3DRS_SRCBLEND);
				STATEMANAGER.RestoreRenderState(D3DRS_DESTBLEND);
			}
		}
	}
}

void CD3D9RenderBackend::RenderTerrain(const TTerrainSectorPacket* pPackets, unsigned int count)
{
	if (RenderOwnership::Owner(ERenderCategory::Terrain) != ERenderOwner::D3D9Backend)
		return;

	if (!pPackets || count == 0)
		return;

	LPDIRECT3DDEVICE9 pD3DDevice = STATEMANAGER.GetDevice();
	if (!pD3DDevice)
		return;

	if (!m_pTerrainPipeline)
		m_pTerrainPipeline = std::make_unique<EterLib::Render::TerrainShaderPipeline>();

	if (!m_pTerrainPipeline->IsReady())
	{
		if (!m_pTerrainPipeline->Initialize(pD3DDevice))
			return;
	}

	const float fogColorAndRange[4] = {
		m_currentUniforms.vFogColor.r,
		m_currentUniforms.vFogColor.g,
		m_currentUniforms.vFogColor.b,
		(m_currentUniforms.fFogFar > m_currentUniforms.fFogNear) ? (1.0f / (m_currentUniforms.fFogFar - m_currentUniforms.fFogNear)) : 0.0001f
	};

	const D3DVECTOR lightDir = { m_currentUniforms.vSunLightDir.x, m_currentUniforms.vSunLightDir.y, m_currentUniforms.vSunLightDir.z };
	const D3DVECTOR ambientColor = { m_currentUniforms.vAmbientColor.r, m_currentUniforms.vAmbientColor.g, m_currentUniforms.vAmbientColor.b };

	m_pTerrainPipeline->BeginFrame(pD3DDevice, m_currentUniforms.matViewProj, lightDir, ambientColor, fogColorAndRange);

	for (unsigned int i = 0; i < count; ++i)
	{
		const TTerrainSectorPacket& pkt = pPackets[i];
		if (!pkt.pVertexData || pkt.dwVertexCount == 0)
			continue;

		EterLib::Render::TerrainSectorBindDesc desc{};
		desc.sectorOriginX = pkt.fWorldPosX;
		desc.sectorOriginY = pkt.fWorldPosY;
		desc.invSectorSize = 1.0f / kSectorSize;
		desc.detailTileScale = 64.0f;
		desc.splatTexture = reinterpret_cast<LPDIRECT3DTEXTURE9>(pkt.pSplatTexture);

		for (int l = 0; l < 4; ++l)
		{
			desc.layerTextures[l] = reinterpret_cast<LPDIRECT3DTEXTURE9>(pkt.pLayerTextures[l]);
		}

		m_pTerrainPipeline->BindSector(pD3DDevice, desc);

		auto* pD3DVB = reinterpret_cast<LPDIRECT3DVERTEXBUFFER9>(pkt.pVertexData);
		STATEMANAGER.SetStreamSource(0, pD3DVB, sizeof(float) * 6);

		STATEMANAGER.DrawPrimitive(D3DPT_TRIANGLESTRIP, 0, pkt.dwVertexCount > 2 ? pkt.dwVertexCount - 2 : 0);
	}

	m_pTerrainPipeline->EndFrame(pD3DDevice);
}

void CD3D9RenderBackend::RenderActors(const TActorRenderPacket* pPackets, unsigned int count)
{
	if (RenderOwnership::Owner(ERenderCategory::Model) != ERenderOwner::D3D9Backend)
		return;

	if (!pPackets || count == 0)
		return;

	for (unsigned int i = 0; i < count; ++i)
	{
		const TActorRenderPacket& pkt = pPackets[i];
		if (pkt.dwBoneCount > 0)
		{
			const D3DXMATRIX* pBones = pkt.GetBoneMatrices();
			UINT matrixCount = (std::min)(pkt.dwBoneCount, kMaxShaderBones);
			STATEMANAGER.SetVertexShaderConstant(4, pBones, matrixCount * 4);
		}
	}
}

void CD3D9RenderBackend::RenderWater(const TWaterPacket* pPackets, unsigned int count)
{
	if (!pPackets || count == 0)
		return;

	if (RenderOwnership::Owner(ERenderCategory::Water) != ERenderOwner::D3D9Backend)
		return;

	LPDIRECT3DDEVICE9 pD3DDevice = STATEMANAGER.GetDevice();
	if (!pD3DDevice)
		return;

	STATEMANAGER.SaveRenderState(D3DRS_ZWRITEENABLE, FALSE);
	STATEMANAGER.SaveRenderState(D3DRS_ALPHABLENDENABLE, TRUE);
	STATEMANAGER.SaveRenderState(D3DRS_CULLMODE, D3DCULL_NONE);
	STATEMANAGER.SaveRenderState(D3DRS_DIFFUSEMATERIALSOURCE, D3DMCS_COLOR1);
	STATEMANAGER.SaveRenderState(D3DRS_COLORVERTEX, TRUE);

	STATEMANAGER.SetFVF(D3DFVF_XYZ | D3DFVF_DIFFUSE);

	STATEMANAGER.SaveTextureStageState(0, D3DTSS_TEXCOORDINDEX, D3DTSS_TCI_CAMERASPACEPOSITION);
	STATEMANAGER.SaveTextureStageState(0, D3DTSS_TEXTURETRANSFORMFLAGS, D3DTTFF_COUNT2);

	STATEMANAGER.SaveSamplerState(0, D3DSAMP_MINFILTER, D3DTEXF_ANISOTROPIC);
	STATEMANAGER.SaveSamplerState(0, D3DSAMP_MAGFILTER, D3DTEXF_ANISOTROPIC);
	STATEMANAGER.SaveSamplerState(0, D3DSAMP_MIPFILTER, D3DTEXF_LINEAR);
	STATEMANAGER.SaveSamplerState(0, D3DSAMP_ADDRESSU, D3DTADDRESS_WRAP);
	STATEMANAGER.SaveSamplerState(0, D3DSAMP_ADDRESSV, D3DTADDRESS_WRAP);

	STATEMANAGER.SetTextureStageState(0, D3DTSS_COLORARG1, D3DTA_TEXTURE);
	STATEMANAGER.SetTextureStageState(0, D3DTSS_COLOROP, D3DTOP_SELECTARG1);
	STATEMANAGER.SetTextureStageState(0, D3DTSS_ALPHAARG1, D3DTA_DIFFUSE);
	STATEMANAGER.SetTextureStageState(0, D3DTSS_ALPHAOP, D3DTOP_SELECTARG1);

	STATEMANAGER.SetTexture(1, NULL);
	STATEMANAGER.SetTextureStageState(1, D3DTSS_COLOROP, D3DTOP_DISABLE);
	STATEMANAGER.SetTextureStageState(1, D3DTSS_ALPHAOP, D3DTOP_DISABLE);

	int currentPass = -1; // 0: Near, 1: Far
	float currentHeight = 0.0f;
	D3DXMATRIX matWorldWater;
	D3DXMatrixIdentity(&matWorldWater);

	for (unsigned int idx = 0; idx < count; ++idx)
	{
		const TWaterPacket& pkt = pPackets[idx];
		if (!pkt.pVertexBuffer || pkt.dwFaceCount == 0)
			continue;

		int packetPass = pkt.bFarPass ? 1 : 0;
		if (packetPass != currentPass)
		{
			currentPass = packetPass;
			if (currentPass == 0)
			{
				LPDIRECT3DTEXTURE9 pTex = reinterpret_cast<LPDIRECT3DTEXTURE9>(pkt.pWaterTexture);
				if (!pTex && pkt.dwWaterTextureID != kInvalidTextureHandle)
				{
					pTex = CRenderResourceRegistry::SafeResolveTexture(pkt.dwWaterTextureID);
				}
				STATEMANAGER.SetTexture(0, pTex);
				STATEMANAGER.SetRenderState(D3DRS_ALPHABLENDENABLE, TRUE);
				STATEMANAGER.SaveTransform(D3DTS_TEXTURE0, &pkt.matTexTransformWater);
			}
			else
			{
				STATEMANAGER.SetTexture(0, NULL);
				STATEMANAGER.SetRenderState(D3DRS_ALPHABLENDENABLE, FALSE);
			}
		}

		if (idx == 0 || pkt.fWaterHeight != currentHeight)
		{
			currentHeight = pkt.fWaterHeight;
			matWorldWater._43 = currentHeight;
			STATEMANAGER.SetTransform(D3DTS_WORLD, &matWorldWater);
		}

		auto* pVB = reinterpret_cast<LPDIRECT3DVERTEXBUFFER9>(pkt.pVertexBuffer);
		STATEMANAGER.SetStreamSource(0, pVB, sizeof(float) * 4);
		STATEMANAGER.DrawPrimitive(D3DPT_TRIANGLELIST, 0, pkt.dwFaceCount);
	}

	D3DXMatrixIdentity(&matWorldWater);
	STATEMANAGER.SetTransform(D3DTS_WORLD, &matWorldWater);
	STATEMANAGER.SetStreamSource(0, NULL, 0);
	STATEMANAGER.SetTexture(0, NULL);

	if (currentPass == 0)
	{
		STATEMANAGER.RestoreTransform(D3DTS_TEXTURE0);
	}

	STATEMANAGER.RestoreSamplerState(0, D3DSAMP_MINFILTER);
	STATEMANAGER.RestoreSamplerState(0, D3DSAMP_MAGFILTER);
	STATEMANAGER.RestoreSamplerState(0, D3DSAMP_MIPFILTER);
	STATEMANAGER.RestoreSamplerState(0, D3DSAMP_ADDRESSU);
	STATEMANAGER.RestoreSamplerState(0, D3DSAMP_ADDRESSV);
	STATEMANAGER.RestoreTextureStageState(0, D3DTSS_TEXCOORDINDEX);
	STATEMANAGER.RestoreTextureStageState(0, D3DTSS_TEXTURETRANSFORMFLAGS);

	STATEMANAGER.RestoreRenderState(D3DRS_DIFFUSEMATERIALSOURCE);
	STATEMANAGER.RestoreRenderState(D3DRS_COLORVERTEX);
	STATEMANAGER.RestoreRenderState(D3DRS_ZWRITEENABLE);
	STATEMANAGER.RestoreRenderState(D3DRS_ALPHABLENDENABLE);
	STATEMANAGER.RestoreRenderState(D3DRS_CULLMODE);
}

void CD3D9RenderBackend::RenderSpeedTree(const TSpeedTreePacket* pPackets, unsigned int count)
{
	if (RenderOwnership::LegacyDraws(ERenderCategory::SpeedTree))
		return;

	if (RenderOwnership::Owner(ERenderCategory::SpeedTree) != ERenderOwner::D3D9Backend)
		return;

	if (!pPackets || count == 0)
		return;

	if (!CStateManager::InstancePtr() || !STATEMANAGER.GetDevice())
		return;

	ScopedD3DVertexDeclGuard speedTreeDeclGuard(nullptr, false);

	STATEMANAGER.SaveRenderState(D3DRS_LIGHTING, FALSE);
	STATEMANAGER.SaveRenderState(D3DRS_COLORVERTEX, TRUE);
	STATEMANAGER.SaveRenderState(D3DRS_ALPHATESTENABLE, TRUE);
	STATEMANAGER.SaveRenderState(D3DRS_ALPHAFUNC, D3DCMP_GREATER);
	STATEMANAGER.SaveRenderState(D3DRS_CULLMODE, D3DCULL_CW);

	for (unsigned int i = 0; i < count; ++i)
	{
		const TSpeedTreePacket& packet = pPackets[i];
		if (!packet.pMainTree || !packet.pInstances || packet.dwInstanceCount == 0)
			continue;

		if (packet.bRenderBranches)
		{
			if (CSpeedTreeWrapper::GetBranchVertexShader())
				STATEMANAGER.SetVertexDeclaration(CSpeedTreeWrapper::GetBranchVertexShader());

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
			STATEMANAGER.SetRenderState(D3DRS_CULLMODE, D3DCULL_NONE);

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
			if (CSpeedTreeWrapper::GetLeafVertexShaderDecl() && CSpeedTreeWrapper::GetLeafVertexShader())
			{
				STATEMANAGER.SetVertexDeclaration(CSpeedTreeWrapper::GetLeafVertexShaderDecl());
				STATEMANAGER.SaveVertexShader(CSpeedTreeWrapper::GetLeafVertexShader());
			}

			packet.pMainTree->SetupLeafForTreeType();
			for (uint32_t idx = 0; idx < packet.dwInstanceCount; ++idx)
			{
				const auto& inst = packet.pInstances[idx];
				if (inst.pTreeInstance)
					const_cast<CSpeedTreeWrapper*>(inst.pTreeInstance)->RenderLeaves();
			}
			const_cast<CSpeedTreeWrapper*>(packet.pMainTree)->EndLeafForTreeType();

			if (CSpeedTreeWrapper::GetLeafVertexShaderDecl() && CSpeedTreeWrapper::GetLeafVertexShader())
			{
				STATEMANAGER.RestoreVertexShader();
			}
		}

		if (packet.bRenderBillboards)
		{
			STATEMANAGER.SetRenderState(D3DRS_LIGHTING, FALSE);
			STATEMANAGER.SetRenderState(D3DRS_COLORVERTEX, FALSE);

			packet.pMainTree->SetupLeafForTreeType();
			for (uint32_t idx = 0; idx < packet.dwInstanceCount; ++idx)
			{
				const auto& inst = packet.pInstances[idx];
				if (inst.pTreeInstance)
					const_cast<CSpeedTreeWrapper*>(inst.pTreeInstance)->RenderBillboards();
			}
		}
	}

	STATEMANAGER.SetTextureStageState(1, D3DTSS_COLOROP, D3DTOP_DISABLE);
	STATEMANAGER.SetTextureStageState(1, D3DTSS_ALPHAOP, D3DTOP_DISABLE);

	STATEMANAGER.RestoreRenderState(D3DRS_CULLMODE);
	STATEMANAGER.RestoreRenderState(D3DRS_ALPHAFUNC);
	STATEMANAGER.RestoreRenderState(D3DRS_ALPHATESTENABLE);
	STATEMANAGER.RestoreRenderState(D3DRS_COLORVERTEX);
	STATEMANAGER.RestoreRenderState(D3DRS_LIGHTING);
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
	RenderParticles(pPackets, count);
}

void CD3D9RenderBackend::RenderParticles(const TEffectInstancePacket* pPackets, unsigned int count)
{
	if (RenderOwnership::Owner(ERenderCategory::Effect) != ERenderOwner::D3D9Backend)
		return;

	if (!pPackets || count == 0)
		return;
	if (!m_pUIVertexDecl)
		InitVertexDeclarations();

	const D3DXMATRIX& matView = (m_currentUniforms.matView._11 != 0.0f || m_currentUniforms.matView._44 != 0.0f)
		? m_currentUniforms.matView
		: CFrameSnapshot::Current().matView;
	D3DXVECTOR3 vCross(matView._11, matView._21, matView._31);
	D3DXVECTOR3 vUp(matView._12, matView._22, matView._32);
	D3DXVECTOR3 vView(matView._13, matView._23, matView._33);

	CCamera* pCamera = CCameraManager::Instance().GetCurrentCamera();
	if (pCamera)
	{
		vUp = pCamera->GetUp();
		vCross = pCamera->GetCross();
		vView = pCamera->GetView();
	}

	D3DXMATRIX matIdentity;
	D3DXMatrixIdentity(&matIdentity);
	STATEMANAGER.SaveTransform(D3DTS_WORLD, &matIdentity);

	STATEMANAGER.SaveRenderState(D3DRS_ALPHABLENDENABLE, TRUE);
	STATEMANAGER.SaveRenderState(D3DRS_ZWRITEENABLE, FALSE);
	STATEMANAGER.SaveRenderState(D3DRS_CULLMODE, D3DCULL_NONE);
	STATEMANAGER.SaveRenderState(D3DRS_LIGHTING, FALSE);

	if (m_pUIVertexDecl)
		STATEMANAGER.SetVertexDeclaration(m_pUIVertexDecl);

	for (unsigned int i = 0; i < count; ++i)
	{
		const TEffectInstancePacket& pkt = pPackets[i];
		if (!pkt.pParticles || pkt.dwParticleCount == 0)
			continue;

		if (pkt.srcBlend != 0 && pkt.destBlend != 0)
		{
			STATEMANAGER.SetRenderState(D3DRS_SRCBLEND, pkt.srcBlend);
			STATEMANAGER.SetRenderState(D3DRS_DESTBLEND, pkt.destBlend);
		}
		else if (pkt.dwBlendMode == EFFECT_BLEND_ADDITIVE)
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
			const uint32_t numQuads = (std::min)(totalQuads - particleOffset, kMaxQuadsPerDraw);
			if (m_particleVertices.size() < numQuads * 4)
				m_particleVertices.resize(numQuads * 4);

			EnsureSharedQuadIndices(m_sharedEffectIndices, numQuads);

			for (uint32_t pIdx = 0; pIdx < numQuads; ++pIdx)
			{
				const TParticleInstanceData& p = pkt.pParticles[particleOffset + pIdx];
				const D3DXVECTOR3 center(p.vWorldPos.x, p.vWorldPos.y, p.vWorldPos.z);
				const float sx = (p.fHalfWidth > 0.0f) ? p.fHalfWidth : p.vWorldPos.w;
				const float sy = (p.fHalfHeight > 0.0f) ? p.fHalfHeight : p.vWorldPos.w;

				D3DXVECTOR3 quadPositions[4];
				const D3DXVECTOR3 velocity(0.0f, 0.0f, 0.0f);

				BuildParticleQuad(
					pkt.billboardMode,
					pkt.bStretch != 0,
					center,
					sx,
					sy,
					p.fRotation,
					vUp,
					vCross,
					vView,
					velocity,
					quadPositions
				);

				const uint32_t packedColor = static_cast<DWORD>(p.vColor);
				TUIVertex* v = &m_particleVertices[pIdx * 4];

				v[0].position = quadPositions[0];
				v[0].color = packedColor;
				v[0].texCoord = D3DXVECTOR2(p.vTexCoordRect.x, p.vTexCoordRect.w);

				v[1].position = quadPositions[1];
				v[1].color = packedColor;
				v[1].texCoord = D3DXVECTOR2(p.vTexCoordRect.x, p.vTexCoordRect.y);

				v[2].position = quadPositions[2];
				v[2].color = packedColor;
				v[2].texCoord = D3DXVECTOR2(p.vTexCoordRect.z, p.vTexCoordRect.w);

				v[3].position = quadPositions[3];
				v[3].color = packedColor;
				v[3].texCoord = D3DXVECTOR2(p.vTexCoordRect.z, p.vTexCoordRect.y);
			}

			STATEMANAGER.DrawIndexedPrimitiveUP(
				D3DPT_TRIANGLELIST,
				0,
				numQuads * 4,
				numQuads * 2,
				m_sharedEffectIndices.data(),
				D3DFMT_INDEX16,
				m_particleVertices.data(),
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

void CD3D9RenderBackend::RenderParticles(const TParticlePacket* pPackets, unsigned int count)
{
	if (RenderOwnership::Owner(ERenderCategory::Effect) != ERenderOwner::D3D9Backend)
		return;

	if (!pPackets || count == 0)
		return;
	if (!m_pUIVertexDecl)
		InitVertexDeclarations();

	const D3DXMATRIX& matView = (m_currentUniforms.matView._11 != 0.0f || m_currentUniforms.matView._44 != 0.0f)
		? m_currentUniforms.matView
		: CFrameSnapshot::Current().matView;
	D3DXVECTOR3 vCross(matView._11, matView._21, matView._31);
	D3DXVECTOR3 vUp(matView._12, matView._22, matView._32);
	D3DXVECTOR3 vView(matView._13, matView._23, matView._33);

	CCamera* pCamera = CCameraManager::Instance().GetCurrentCamera();
	if (pCamera)
	{
		vUp = pCamera->GetUp();
		vCross = pCamera->GetCross();
		vView = pCamera->GetView();
	}

	D3DXMATRIX matIdentity;
	D3DXMatrixIdentity(&matIdentity);
	STATEMANAGER.SaveTransform(D3DTS_WORLD, &matIdentity);

	STATEMANAGER.SaveRenderState(D3DRS_ALPHABLENDENABLE, TRUE);
	STATEMANAGER.SaveRenderState(D3DRS_ZWRITEENABLE, FALSE);
	STATEMANAGER.SaveRenderState(D3DRS_CULLMODE, D3DCULL_NONE);
	STATEMANAGER.SaveRenderState(D3DRS_LIGHTING, FALSE);

	if (m_pUIVertexDecl)
		STATEMANAGER.SetVertexDeclaration(m_pUIVertexDecl);

	for (unsigned int i = 0; i < count; ++i)
	{
		const TParticlePacket& pkt = pPackets[i];

		if (pkt.srcBlend != 0 && pkt.destBlend != 0)
		{
			STATEMANAGER.SetRenderState(D3DRS_SRCBLEND, pkt.srcBlend);
			STATEMANAGER.SetRenderState(D3DRS_DESTBLEND, pkt.destBlend);
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

		if (m_particleVertices.size() < 4)
			m_particleVertices.resize(4);

		EnsureSharedQuadIndices(m_sharedEffectIndices, 1);

		D3DXVECTOR3 quadPositions[4];
		const D3DXVECTOR3 center(pkt.vPosition.x, pkt.vPosition.y, pkt.vPosition.z);
		const D3DXVECTOR3 velocity(pkt.vVelocity.x, pkt.vVelocity.y, pkt.vVelocity.z);

		BuildParticleQuad(
			pkt.billboardMode,
			pkt.bStretch != 0,
			center,
			pkt.fHalfWidth,
			pkt.fHalfHeight,
			pkt.fRotation,
			vUp,
			vCross,
			vView,
			velocity,
			quadPositions
		);

		const uint32_t packedColor = static_cast<DWORD>(pkt.vColor);
		TUIVertex* v = m_particleVertices.data();

		v[0].position = quadPositions[0];
		v[0].color = packedColor;
		v[0].texCoord = D3DXVECTOR2(pkt.vTexCoordRect.x, pkt.vTexCoordRect.w);

		v[1].position = quadPositions[1];
		v[1].color = packedColor;
		v[1].texCoord = D3DXVECTOR2(pkt.vTexCoordRect.x, pkt.vTexCoordRect.y);

		v[2].position = quadPositions[2];
		v[2].color = packedColor;
		v[2].texCoord = D3DXVECTOR2(pkt.vTexCoordRect.z, pkt.vTexCoordRect.w);

		v[3].position = quadPositions[3];
		v[3].color = packedColor;
		v[3].texCoord = D3DXVECTOR2(pkt.vTexCoordRect.z, pkt.vTexCoordRect.y);

		STATEMANAGER.DrawIndexedPrimitiveUP(
			D3DPT_TRIANGLELIST,
			0,
			4,
			2,
			m_sharedEffectIndices.data(),
			D3DFMT_INDEX16,
			m_particleVertices.data(),
			static_cast<UINT>(sizeof(TUIVertex))
		);
	}

	STATEMANAGER.RestoreRenderState(D3DRS_LIGHTING);
	STATEMANAGER.RestoreRenderState(D3DRS_CULLMODE);
	STATEMANAGER.RestoreRenderState(D3DRS_ZWRITEENABLE);
	STATEMANAGER.RestoreRenderState(D3DRS_ALPHABLENDENABLE);
	STATEMANAGER.RestoreTransform(D3DTS_WORLD);
}

void CD3D9RenderBackend::RenderUI(const TUIBatchPacket* pPackets, unsigned int count)
{
	if (RenderOwnership::Owner(ERenderCategory::UI) != ERenderOwner::D3D9Backend)
		return;

	if (!pPackets || count == 0)
		return;

	LPDIRECT3DDEVICE9 pD3DDevice = STATEMANAGER.GetDevice();
	if (!pD3DDevice)
		return;

	if (!m_pUIVertexDecl)
		InitVertexDeclarations();

	D3DVIEWPORT9 vp;
	float screenW = static_cast<float>(m_width);
	float screenH = static_cast<float>(m_height);
	if (screenW <= 0.0f || screenH <= 0.0f)
	{
		if (SUCCEEDED(pD3DDevice->GetViewport(&vp)))
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
			const uint32_t numQuads = (std::min)(totalQuads - quadsDrawn, kMaxQuadsPerDraw);
			EnsureSharedQuadIndices(m_sharedQuadIndices, numQuads);

			STATEMANAGER.DrawIndexedPrimitiveUP(
				D3DPT_TRIANGLELIST,
				0,
				numQuads * 4,
				numQuads * 2,
				m_sharedQuadIndices.data(),
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
	if (RenderOwnership::Owner(ERenderCategory::Dungeon) != ERenderOwner::D3D9Backend)
		return;

	if (!pPackets || count == 0)
		return;

	if (!m_pDungeonVertexDecl)
		InitVertexDeclarations();

	D3DXMATRIX matIdentity;
	D3DXMatrixIdentity(&matIdentity);
	STATEMANAGER.SaveTransform(D3DTS_WORLD, &matIdentity);

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
	STATEMANAGER.RestoreTransform(D3DTS_WORLD);
}

void CD3D9RenderBackend::RenderWeaponTrace(const TWeaponTracePacket* pPackets, unsigned int count)
{
	if (RenderOwnership::Owner(ERenderCategory::WeaponTrace) != ERenderOwner::D3D9Backend)
		return;

	if (!pPackets || count == 0)
		return;

	const TPDTVertex* pAllVertices = CRenderQueueManager::Instance().GetWeaponTraceVertexData();
	if (!pAllVertices)
		return;

	D3DXMATRIX matIdentity;
	D3DXMatrixIdentity(&matIdentity);
	STATEMANAGER.SaveTransform(D3DTS_WORLD, &matIdentity);
	STATEMANAGER.SaveFVF(D3DFVF_XYZ | D3DFVF_DIFFUSE | D3DFVF_TEX1);

	STATEMANAGER.SaveRenderState(D3DRS_CULLMODE, D3DCULL_NONE);
	STATEMANAGER.SaveRenderState(D3DRS_ALPHABLENDENABLE, TRUE);
	STATEMANAGER.SaveRenderState(D3DRS_SRCBLEND, D3DBLEND_SRCALPHA);
	STATEMANAGER.SaveRenderState(D3DRS_DESTBLEND, D3DBLEND_INVSRCALPHA);

	STATEMANAGER.SaveRenderState(D3DRS_ALPHATESTENABLE, FALSE);
	STATEMANAGER.SaveRenderState(D3DRS_ALPHAREF, 0x00000011);
	STATEMANAGER.SaveRenderState(D3DRS_ALPHAFUNC, D3DCMP_GREATER);

	STATEMANAGER.SaveRenderState(D3DRS_ZENABLE, TRUE);
	STATEMANAGER.SaveRenderState(D3DRS_ZFUNC, D3DCMP_LESSEQUAL);
	STATEMANAGER.SaveRenderState(D3DRS_ZWRITEENABLE, FALSE);
	STATEMANAGER.SaveRenderState(D3DRS_LIGHTING, FALSE);

	for (unsigned int i = 0; i < count; ++i)
	{
		const TWeaponTracePacket& pkt = pPackets[i];
		if (pkt.vertexCount < 4)
			continue;

		// Ustawienia stanow potoku z TPipelineStateKey
		STATEMANAGER.SetRenderState(D3DRS_SRCBLEND, pkt.stateKey.srcBlend ? pkt.stateKey.srcBlend : D3DBLEND_SRCALPHA);
		STATEMANAGER.SetRenderState(D3DRS_DESTBLEND, pkt.stateKey.destBlend ? pkt.stateKey.destBlend : D3DBLEND_INVSRCALPHA);
		STATEMANAGER.SetRenderState(D3DRS_CULLMODE, pkt.stateKey.cullMode ? pkt.stateKey.cullMode : D3DCULL_NONE);
		STATEMANAGER.SetRenderState(D3DRS_ZENABLE, pkt.stateKey.zTest ? TRUE : FALSE);
		STATEMANAGER.SetRenderState(D3DRS_ZWRITEENABLE, pkt.stateKey.zWrite ? TRUE : FALSE);

		// Rozwiazanie tekstury z rejestru zasobow
		IDirect3DTexture9* pTexture = nullptr;
		if (pkt.textureHandle != kInvalidTextureHandle && CRenderResourceRegistry::InstancePtr())
		{
			pTexture = CRenderResourceRegistry::Instance().ResolveTexture(pkt.textureHandle);
		}

		if (pTexture)
		{
			STATEMANAGER.SetTexture(0, pTexture);
			STATEMANAGER.SetTextureStageState(0, D3DTSS_COLORARG1, D3DTA_DIFFUSE);
			STATEMANAGER.SetTextureStageState(0, D3DTSS_COLORARG2, D3DTA_TEXTURE);
			STATEMANAGER.SetTextureStageState(0, D3DTSS_COLOROP, D3DTOP_SELECTARG2);
			STATEMANAGER.SetTextureStageState(0, D3DTSS_ALPHAARG1, D3DTA_DIFFUSE);
			STATEMANAGER.SetTextureStageState(0, D3DTSS_ALPHAARG2, D3DTA_TEXTURE);
			STATEMANAGER.SetTextureStageState(0, D3DTSS_ALPHAOP, D3DTOP_SELECTARG2);
		}
		else
		{
			STATEMANAGER.SetTexture(0, nullptr);
			STATEMANAGER.SetTextureStageState(0, D3DTSS_COLORARG1, D3DTA_DIFFUSE);
			STATEMANAGER.SetTextureStageState(0, D3DTSS_COLOROP, D3DTOP_SELECTARG1);
			STATEMANAGER.SetTextureStageState(0, D3DTSS_ALPHAARG1, D3DTA_DIFFUSE);
			STATEMANAGER.SetTextureStageState(0, D3DTSS_ALPHAOP, D3DTOP_SELECTARG1);
		}

		STATEMANAGER.SetTextureStageState(1, D3DTSS_COLOROP, D3DTOP_DISABLE);
		STATEMANAGER.SetTextureStageState(1, D3DTSS_ALPHAOP, D3DTOP_DISABLE);
		STATEMANAGER.SetTexture(1, nullptr);

		const uint32_t primitiveCount = pkt.vertexCount - 2;
		const TPDTVertex* pVertices = pAllVertices + pkt.vertexOffset;

		STATEMANAGER.DrawPrimitiveUP(
			D3DPT_TRIANGLESTRIP,
			primitiveCount,
			pVertices,
			sizeof(TPDTVertex)
		);
	}

	STATEMANAGER.SetTexture(0, nullptr);
	STATEMANAGER.SetTexture(1, nullptr);

	STATEMANAGER.RestoreRenderState(D3DRS_LIGHTING);
	STATEMANAGER.RestoreRenderState(D3DRS_ZWRITEENABLE);
	STATEMANAGER.RestoreRenderState(D3DRS_ZFUNC);
	STATEMANAGER.RestoreRenderState(D3DRS_ZENABLE);

	STATEMANAGER.RestoreRenderState(D3DRS_ALPHAFUNC);
	STATEMANAGER.RestoreRenderState(D3DRS_ALPHAREF);
	STATEMANAGER.RestoreRenderState(D3DRS_ALPHATESTENABLE);

	STATEMANAGER.RestoreRenderState(D3DRS_DESTBLEND);
	STATEMANAGER.RestoreRenderState(D3DRS_SRCBLEND);
	STATEMANAGER.RestoreRenderState(D3DRS_ALPHABLENDENABLE);
	STATEMANAGER.RestoreRenderState(D3DRS_CULLMODE);

	STATEMANAGER.RestoreFVF();
	STATEMANAGER.RestoreTransform(D3DTS_WORLD);
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
	LPDIRECT3DDEVICE9 pD3DDevice = STATEMANAGER.GetDevice();
	if (!pD3DDevice)
		return true;
	HRESULT hr = pD3DDevice->TestCooperativeLevel();
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

void CD3D9RenderBackend::RenderDebugLines(const TDebugLinePacket* pPackets, unsigned int count)
{
	RenderDebugLinesWithDevice(pPackets, count, nullptr);
}

namespace
{
	struct SDebugVertex
	{
		float x, y, z;
		DWORD color;
		static constexpr DWORD FVF = D3DFVF_XYZ | D3DFVF_DIFFUSE;
	};
}

void CD3D9RenderBackend::RenderDebugLinesWithDevice(const TDebugLinePacket* pPackets, unsigned int count, void* pCustomDevice)
{
	if (!pPackets || count == 0)
		return;

	IDirect3DDevice9* pD3DDevice = pCustomDevice ? static_cast<IDirect3DDevice9*>(pCustomDevice) : STATEMANAGER.GetDevice();
	if (!pD3DDevice)
		return;

	std::vector<SDebugVertex> stdVerts;
	std::vector<SDebugVertex> xrayVerts;
	stdVerts.reserve(count * 2);
	xrayVerts.reserve(count * 2);

	for (unsigned int i = 0; i < count; ++i)
	{
		const auto& p = pPackets[i];
		auto& target = p.bXRay ? xrayVerts : stdVerts;
		target.push_back({ p.v3Start.x, p.v3Start.y, p.v3Start.z, p.dwColor });
		target.push_back({ p.v3End.x, p.v3End.y, p.v3End.z, p.dwColor });
	}

	auto drawBatched = [pD3DDevice](const std::vector<SDebugVertex>& verts) {
		constexpr size_t MAX_BATCH_PRIMITIVES = 4096;
		const size_t maxVerticesPerChunk = MAX_BATCH_PRIMITIVES * 2;
		for (size_t offset = 0; offset < verts.size(); offset += maxVerticesPerChunk)
		{
			const size_t chunkVerts = std::min(maxVerticesPerChunk, verts.size() - offset);
			const auto primitiveCount = static_cast<UINT>(chunkVerts / 2);
			if (primitiveCount > 0)
			{
				pD3DDevice->DrawPrimitiveUP(D3DPT_LINELIST, primitiveCount, &verts[offset], sizeof(SDebugVertex));
			}
		}
	};

	// Rysowanie wierzcholkow standardowych (ze sprawdzaniem bufora Z)
	if (!stdVerts.empty())
	{
		pD3DDevice->SetFVF(SDebugVertex::FVF);
		drawBatched(stdVerts);
	}

	// Rysowanie wierzcholkow X-Ray (przez sciany) z zachowaniem biezacego stanu ZENABLE
	if (!xrayVerts.empty())
	{
		DWORD dwSavedZEnable = D3DZB_TRUE;
		pD3DDevice->GetRenderState(D3DRS_ZENABLE, &dwSavedZEnable);

		pD3DDevice->SetRenderState(D3DRS_ZENABLE, FALSE);
		pD3DDevice->SetFVF(SDebugVertex::FVF);
		drawBatched(xrayVerts);

		pD3DDevice->SetRenderState(D3DRS_ZENABLE, dwSavedZEnable);
	}
}

