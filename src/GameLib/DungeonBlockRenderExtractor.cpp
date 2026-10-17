#include "StdAfx.h"
#include "DungeonBlockRenderExtractor.h"
#include "DungeonBlock.h"
#include "EterGrnLib/Model.h"
#include "EterGrnLib/ModelInstance.h"
#include "EterGrnLib/LODController.h"
#include "EterModelLib/GltfModel.h"
#include "EterModelLib/GltfModelInstance.h"
#include "EterLib/Render/ActorSkinningPipeline.h"
#include "EterLib/RenderQueueManager.h"
#include <algorithm>

bool CDungeonBlockRenderExtractor::ExtractAndSubmit(CDungeonModelInstance* pInstance)
{
	if (!pInstance || pInstance->IsEmpty())
		return false;

	CGrannyModel* pModel = pInstance->GetModel();
	if (!pModel)
		return false;

	LPDIRECT3DVERTEXBUFFER9 lpd3dRigidPNTVtxBuf = pModel->GetPNTD3DVertexBuffer();
	LPDIRECT3DINDEXBUFFER9 lpd3dIdxBuf = pModel->GetD3DIndexBuffer();
	if (!lpd3dRigidPNTVtxBuf || !lpd3dIdxBuf)
		return false;

	const D3DXMATRIX* pActiveMatrices = pInstance->GetActiveMeshMatrices();
	if (!pActiveMatrices)
		return false;

	bool bSubmittedAny = false;

	// Przetwarzanie obu typow materialow lochu: BLEND_PNT oraz DIFFUSE_PNT
	const CGrannyMaterial::EType materialTypes[] = {
		CGrannyMaterial::TYPE_BLEND_PNT,
		CGrannyMaterial::TYPE_DIFFUSE_PNT
	};

	CRenderQueueManager& rqm = CRenderQueueManager::Instance();

	for (CGrannyMaterial::EType eMtrlType : materialTypes)
	{
		const CGrannyModel::TMeshNode* pMeshNode = pModel->GetMeshNodeList(CGrannyMesh::TYPE_RIGID, eMtrlType);

		while (pMeshNode)
		{
			const CGrannyMesh* pMesh = pMeshNode->pMesh;
			if (pMesh)
			{
				const D3DXMATRIX& matWorld = pActiveMatrices[pMeshNode->iMesh];
				int vtxMeshBasePos = pMesh->GetVertexBasePosition();
				int vtxCount = pMesh->GetVertexCount();
				const CGrannyMesh::TTriGroupNode* pTriGroupNode = pMesh->GetTriGroupNodeList(eMtrlType);

				while (pTriGroupNode)
				{
					const CGrannyMaterial& rkMtrl = pInstance->GetMaterialPalette().GetMaterialRef(pTriGroupNode->mtrlIndex);

					TDungeonBlockPacket packet{};
					packet.matWorld = matWorld;

					// Tekstura 0: baza (diffuse)
					LPDIRECT3DTEXTURE9 pTex0 = rkMtrl.GetD3DTexture(0);
					packet.pDiffuseTexture = pTex0;
					packet.dwDiffuseTextureID = reinterpret_cast<uintptr_t>(pTex0);

					// Tekstura 1: mapa swiatla (lightmap)
					LPDIRECT3DTEXTURE9 pTex1 = rkMtrl.GetD3DTexture(1);
					packet.pLightmapTexture = pTex1;
					packet.dwLightmapTextureID = reinterpret_cast<uintptr_t>(pTex1);

					// Bufory Direct3D 9
					packet.pVertexBuffer = lpd3dRigidPNTVtxBuf;
					packet.pIndexBuffer = lpd3dIdxBuf;

					packet.dwVertexCount = static_cast<uint32_t>(vtxCount);
					packet.dwIndexCount = static_cast<uint32_t>(pTriGroupNode->triCount * 3);
					packet.dwPrimitiveCount = static_cast<uint32_t>(pTriGroupNode->triCount);
					packet.dwStartIndex = static_cast<uint32_t>(pTriGroupNode->idxPos);
					packet.nBaseVertexIndex = vtxMeshBasePos;

					packet.fAlpha = (eMtrlType == CGrannyMaterial::TYPE_BLEND_PNT) ? 0.99f : 1.0f;
					packet.bAlphaBlend = (eMtrlType == CGrannyMaterial::TYPE_BLEND_PNT) ? 1 : 0;
					packet.fPad = 0.0f;

					rqm.SubmitDungeonBlock(packet);
					bSubmittedAny = true;

					pTriGroupNode = pTriGroupNode->pNextTriGroupNode;
				}
			}

			pMeshNode = pMeshNode->pNextMeshNode;
		}
	}

	return bSubmittedAny;
}

bool CDungeonBlockRenderExtractor::ExtractAndSubmit(CGrannyLODController* pController, const D3DXMATRIX* pmatWorld)
{
	if (!pController || !pController->isGltfModelInstance())
		return false;

	pController->EnsureGltfStaticGpuBuffers();

	EterModelLib::CGltfModelInstance* pGltfInst = pController->GetGltfModelInstance();
	if (!pGltfInst)
		return false;

	const EterModelLib::CGltfModel* pModel = pGltfInst->GetModel();
	if (!pModel || !pModel->IsLoaded())
		return false;

	const GltfModelData& modelData = pModel->GetModelData();
	if (modelData.submeshes.empty())
		return false;

	LPDIRECT3DVERTEXBUFFER9 pVB = pController->GetGltfStaticVertexBuffer();
	LPDIRECT3DINDEXBUFFER9 pIB = pController->GetGltfStaticIndexBuffer();
	if (!pVB || !pIB)
		return false;

	const auto& matStates = pController->GetGltfMaterialStates();
	CRenderQueueManager& rqm = CRenderQueueManager::Instance();
	bool bSubmittedAny = false;

	D3DXMATRIX matIdentity;
	D3DXMatrixIdentity(&matIdentity);
	const D3DXMATRIX& rmatWorld = pmatWorld ? *pmatWorld : matIdentity;

	for (size_t s = 0; s < modelData.submeshes.size(); ++s)
	{
		const GltfSubmesh& submesh = modelData.submeshes[s];
		if (submesh.indexCount == 0)
			continue;

		TDungeonBlockPacket packet{};
		packet.matWorld = rmatWorld;
		packet.pVertexBuffer = pVB;
		packet.pIndexBuffer = pIB;

		packet.dwStartIndex = static_cast<uint32_t>(submesh.indexOffset);
		packet.dwIndexCount = static_cast<uint32_t>(submesh.indexCount);
		packet.dwPrimitiveCount = static_cast<uint32_t>(submesh.indexCount / 3);
		packet.nBaseVertexIndex = static_cast<int32_t>(submesh.vertexOffset);
		packet.dwVertexCount = static_cast<uint32_t>(submesh.vertexCount);

		bool isBlend = false;
		LPDIRECT3DTEXTURE9 pD3DTexture = nullptr;

		if (submesh.materialIndex >= 0 && submesh.materialIndex < static_cast<int>(matStates.size()))
		{
			const CGrannyLODController::SGltfMaterialState& state = matStates[submesh.materialIndex];
			isBlend = state.isBlend;
			if (!state.roDiffuseImage.IsNull())
			{
				CGraphicImage* pImg = state.roDiffuseImage.GetPointer();
				if (pImg && !pImg->IsEmpty() && pImg->GetTexturePointer())
				{
					pD3DTexture = pImg->GetTexturePointer()->GetD3DTexture();
				}
			}
		}

		packet.pDiffuseTexture = pD3DTexture;
		packet.dwDiffuseTextureID = reinterpret_cast<uintptr_t>(pD3DTexture);
		packet.pLightmapTexture = nullptr;
		packet.dwLightmapTextureID = 0;

		packet.fAlpha = isBlend ? 0.99f : 1.0f;
		packet.bAlphaBlend = isBlend ? 1 : 0;
		packet.bPadDungeon[0] = static_cast<uint8_t>(sizeof(EterLib::Render::GpuSkinnedVertex));

		rqm.SubmitDungeonBlock(packet);
		bSubmittedAny = true;
	}

	return bSubmittedAny;
}
