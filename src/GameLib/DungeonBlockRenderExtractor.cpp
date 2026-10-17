#include "StdAfx.h"
#include "DungeonBlockRenderExtractor.h"
#include "DungeonBlock.h"
#include "EterGrnLib/Model.h"
#include "EterGrnLib/ModelInstance.h"
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
