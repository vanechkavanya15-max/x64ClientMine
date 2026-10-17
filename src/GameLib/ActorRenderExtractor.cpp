#include "StdAfx.h"
#include "ActorRenderExtractor.h"
#include "ActorInstance.h"
#include "RaceData.h"
#include "EterGrnLib/LODController.h"
#include "EterGrnLib/ModelInstance.h"
#include "EterGrnLib/Model.h"
#include "EterModelLib/GltfModelInstance.h"
#include "EterLib/RenderQueueManager.h"

bool ActorRenderExtractor::Extract(CActorInstance& actor, TActorRenderPacket& outPacket)
{
	if (!actor.isShow())
		return false;

	// 1. Ekstrakcja podstawowych identyfikatorow i macierzy swiata aktora
	outPacket.dwActorID = actor.GetVirtualID();
	outPacket.matWorld = actor.GetWorldMatrix();

	// 2. Identyfikator siatki glownej (PART_MAIN) lub rasy bazowej
	outPacket.dwMeshID = actor.GetPartItemID(CRaceData::PART_MAIN);
	if (outPacket.dwMeshID == 0)
	{
		outPacket.dwMeshID = actor.GetRace();
	}

	// 3. Identyfikator wariantu tekstury / ksztaltu
	outPacket.dwTextureID = actor.GetShape();

	// 4. Stany cieniowania, koloru i alfy
	outPacket.fAlpha = actor.GetAlphaValue();
	outPacket.bAlphaBlend = (actor.GetRenderMode() == CActorInstance::RENDER_MODE_BLEND);
	outPacket.bIsDead = (actor.IsDead() != FALSE);
	outPacket.bRenderShadow = true;
	outPacket.bSelected = false;
	outPacket.fShadowSize = 0.0f;

	// 5. Ekstrakcja broni
	outPacket.dwWeaponMeshID = actor.GetPartItemID(CRaceData::PART_WEAPON);
	outPacket.dwWeaponTextureID = 0;

	// 6. Ekstrakcja palety macierzy kosci szkieletu
	if (!ExtractBonePalette(actor, outPacket))
	{
		for (uint32_t i = 0; i < 64; ++i)
		{
			D3DXMatrixIdentity(&outPacket.boneMatrices[i]);
		}
		outPacket.dwBoneCount = 1;
		outPacket.pBoneMatrices = outPacket.boneMatrices;
	}
	else if (!outPacket.pBoneMatrices)
	{
		outPacket.pBoneMatrices = outPacket.boneMatrices;
	}

	// 7. Ekstrakcja macierzy socketow (bron, helm, fryzura, aury)
	ExtractSockets(actor, outPacket);

	return true;
}

bool ActorRenderExtractor::ExtractBonePalette(CActorInstance& actor, TActorRenderPacket& outPacket)
{
	outPacket.dwBoneCount = 0;

	// Pobranie kontrolera LOD dla glownej czesci aktora
	CGrannyLODController* pController = actor.GetLODControllerPointer(CRaceData::PART_MAIN);
	if (pController)
	{
		if (pController->isGltfModelInstance())
		{
			EterModelLib::CGltfModelInstance* pGltfInst = pController->GetGltfModelInstance();
			if (pGltfInst)
			{
				const auto& skinMatrices = pGltfInst->GetSkinningMatrices();
				if (!skinMatrices.empty())
				{
					uint32_t copyCount = std::min<uint32_t>(static_cast<uint32_t>(skinMatrices.size()), 64);
					D3DXMATRIX* pContinuousPoolBones = CRenderQueueManager::Instance().AllocateBoneMatrices(copyCount);
					if (pContinuousPoolBones)
					{
						memcpy(pContinuousPoolBones, skinMatrices.data(), sizeof(D3DXMATRIX) * copyCount);
						outPacket.pBoneMatrices = pContinuousPoolBones;
					}
					else
					{
						outPacket.pBoneMatrices = outPacket.boneMatrices;
					}

					memcpy(outPacket.boneMatrices, skinMatrices.data(), sizeof(D3DXMATRIX) * copyCount);
					outPacket.dwBoneCount = copyCount;
					for (uint32_t i = copyCount; i < 64; ++i)
						D3DXMatrixIdentity(&outPacket.boneMatrices[i]);
					return true;
				}
			}
		}

		CGrannyModelInstance* pModelInst = pController->GetModelInstance();
		if (pModelInst)
		{
			// Wyciaganie bezposrednio koncowej tablicy macierzy kosci ze szkieletu Granny przez GrannyGetWorldPoseComposite4x4Array
			const D3DXMATRIX* pCompositeBones = pModelInst->GetCompositeBoneMatrices();
			int totalBones = pModelInst->GetBoneCount();
			if (pCompositeBones && totalBones > 0)
			{
				uint32_t copyCount = std::min<uint32_t>(static_cast<uint32_t>(totalBones), 64);
				D3DXMATRIX* pContinuousPoolBones = CRenderQueueManager::Instance().AllocateBoneMatrices(copyCount);
				if (pContinuousPoolBones)
				{
					memcpy(pContinuousPoolBones, pCompositeBones, sizeof(D3DXMATRIX) * copyCount);
					outPacket.pBoneMatrices = pContinuousPoolBones;
				}
				else
				{
					outPacket.pBoneMatrices = pCompositeBones;
				}

				memcpy(outPacket.boneMatrices, pCompositeBones, sizeof(D3DXMATRIX) * copyCount);
				outPacket.dwBoneCount = copyCount;
				for (uint32_t i = copyCount; i < 64; ++i)
					D3DXMatrixIdentity(&outPacket.boneMatrices[i]);
				return true;
			}
		}
	}

	// W przypadku braku kosci ustawiamy pojedyncza macierz jednostkowa w puli klatki i w buforze
	if (outPacket.dwBoneCount == 0)
	{
		for (uint32_t i = 0; i < 64; ++i)
		{
			D3DXMatrixIdentity(&outPacket.boneMatrices[i]);
		}
		D3DXMATRIX* pContinuousPoolBones = CRenderQueueManager::Instance().AllocateBoneMatrices(1);
		if (pContinuousPoolBones)
		{
			D3DXMatrixIdentity(pContinuousPoolBones);
			outPacket.pBoneMatrices = pContinuousPoolBones;
		}
		else
		{
			outPacket.pBoneMatrices = outPacket.boneMatrices;
		}
		outPacket.dwBoneCount = 1;
	}

	return true;
}

bool ActorRenderExtractor::ExtractSockets(CActorInstance& actor, TActorRenderPacket& outPacket)
{
	// 1. Socket prawej dloni dla broni glownej (PART_WEAPON)
	D3DXMATRIX* pWeaponMat = nullptr;
	if (actor.GetSocketMatrix(CRaceData::PART_WEAPON, &pWeaponMat) && pWeaponMat)
	{
		outPacket.matWeaponSocket = *pWeaponMat;
	}
	else
	{
		D3DXMatrixIdentity(&outPacket.matWeaponSocket);
	}

	// 2. Socket lewej dloni dla drugiej broni / tarczy (PART_WEAPON_LEFT)
	D3DXMATRIX* pWeaponLeftMat = nullptr;
	if (actor.GetSocketMatrix(CRaceData::PART_WEAPON_LEFT, &pWeaponLeftMat) && pWeaponLeftMat)
	{
		outPacket.matWeaponLeftSocket = *pWeaponLeftMat;
	}
	else
	{
		D3DXMatrixIdentity(&outPacket.matWeaponLeftSocket);
	}

	// 3. Socket helmu / glowy (PART_HEAD)
	D3DXMATRIX* pHelmetMat = nullptr;
	if (actor.GetSocketMatrix(CRaceData::PART_HEAD, &pHelmetMat) && pHelmetMat)
	{
		outPacket.matHelmetSocket = *pHelmetMat;
	}
	else
	{
		D3DXMatrixIdentity(&outPacket.matHelmetSocket);
	}

	// 4. Socket fryzury (PART_HAIR)
	D3DXMATRIX* pHairMat = nullptr;
	if (actor.GetSocketMatrix(CRaceData::PART_HAIR, &pHairMat) && pHairMat)
	{
		outPacket.matHairSocket = *pHairMat;
	}
	else
	{
		D3DXMatrixIdentity(&outPacket.matHairSocket);
	}

	// 5. Ekstrakcja macierzy socketow dla efektow aury
	outPacket.dwAuraEffectCount = 0;
	const auto& effectList = actor.GetAttachingEffectList();
	for (const auto& effect : effectList)
	{
		if (outPacket.dwAuraEffectCount >= 8)
			break;

		uint32_t idx = outPacket.dwAuraEffectCount;
		outPacket.dwAuraEffectIDs[idx] = effect.dwEffectIndex;

		if (effect.bTransformInitialized)
		{
			outPacket.matAuraEffectSockets[idx] = effect.matLastTransform;
		}
		else
		{
			D3DXMATRIX matTransform;
			D3DXMATRIX* pBoneMat = nullptr;
			if (effect.iBoneIndex >= 0 && actor.GetBoneMatrix(effect.dwModelIndex, effect.iBoneIndex, &pBoneMat) && pBoneMat)
			{
				D3DXMatrixMultiply(&matTransform, pBoneMat, &effect.matTranslation);
				D3DXMatrixMultiply(&matTransform, &matTransform, &actor.GetWorldMatrix());
			}
			else
			{
				D3DXMatrixMultiply(&matTransform, &effect.matTranslation, &actor.GetWorldMatrix());
			}
			outPacket.matAuraEffectSockets[idx] = matTransform;
		}

		outPacket.dwAuraEffectCount++;
	}

	return true;
}
