///////////////////////////////////////////////////////////////////////  
//	CSpeedTreeForest Class

///////////////////////////////////////////////////////////////////////  
//	Include Files
#include "StdAfx.h"

#include <vector>
#include "EterBase/Filename.h"
#include "PackLib/PackManager.h"

#include "SpeedTreeForest.h"
#include "SpeedTreeConfig.h"
#include "EterLib/RenderQueueManager.h"
#include "EterLib/RenderResourceRegistry.h"
#include <cfloat>

using namespace std;

///////////////////////////////////////////////////////////////////////  
//	CSpeedTreeForest constructor

CSpeedTreeForest::CSpeedTreeForest() :
	m_fWindStrength(0.0f),
	m_afFog{ 0.0f, 0.0f, 0.0f, 0.0f },
	m_fAccumTime(0.0f),
	m_afLighting{ 0.0f, 0.0f, 0.0f, 1.0f,  // direction
					0.2f, 0.2f, 0.2f, 1.0f,  // ambient
					0.8f, 0.8f, 0.8f, 1.0f }, // diffuse
	m_fLastWindComputedTime(-1000.0f),
	m_fLastWindStrength(-1.0f),
	m_bWindMatricesCached(false),
	m_fLastUpdateTime(0.0f),
	m_bFirstUpdate(true),
	m_afMatrixTimes{ 0.0f, 0.0f, 0.0f, 0.0f },
	m_fTimeOfLastWindCall(0.0f),
	m_fOldWindStrength(0.0f)
{
	CSpeedTreeRT::SetNumWindMatrices(c_nNumWindMatrices);

	m_afForestExtents[0] = m_afForestExtents[1] = m_afForestExtents[2] = FLT_MAX;
	m_afForestExtents[3] = m_afForestExtents[4] = m_afForestExtents[5] = -FLT_MAX;
	memset(m_afCachedWindMatrices, 0, sizeof(m_afCachedWindMatrices));
}


///////////////////////////////////////////////////////////////////////  
//	CSpeedTreeForest destructor

CSpeedTreeForest::~CSpeedTreeForest()
{
	Clear();
}

void CSpeedTreeForest::Clear()
{
	for (auto tr : m_pMainTreeMap)
	{
		SpeedTreeWrapperPtr pMainTree = tr.second;

		UINT uiCount;
		const auto& ppInstances = pMainTree->GetInstances(uiCount);

		for (auto it : ppInstances)
			it->Clear();

		pMainTree->Clear();
	}

	if (m_pMainTreeMap.begin() != m_pMainTreeMap.end())
	{
		m_pMainTreeMap.clear();
	}
}

CSpeedTreeForest::SpeedTreeWrapperPtr CSpeedTreeForest::GetMainTree(DWORD dwCRC)
{
	TTreeMap::iterator itor = m_pMainTreeMap.find(dwCRC);

	if (itor == m_pMainTreeMap.end())
		return NULL;

	return itor->second;
}

BOOL CSpeedTreeForest::GetMainTree(DWORD dwCRC, SpeedTreeWrapperPtr &ppMainTree, const char * c_pszFileName)
{
	TTreeMap::iterator itor = m_pMainTreeMap.find(dwCRC);

	SpeedTreeWrapperPtr pTree;

	if (itor != m_pMainTreeMap.end())
		pTree = itor->second;
	else
	{
		TPackFile file;

		if (!CPackManager::Instance().GetFile(c_pszFileName, file))
			return FALSE;

		pTree = std::make_shared<CSpeedTreeWrapper>();

		if (!pTree->LoadTree(c_pszFileName, (const BYTE *)file.data(), file.size()))
		{
			pTree.reset();
			return FALSE;
		}

		m_pMainTreeMap.insert(std::map<DWORD, SpeedTreeWrapperPtr>::value_type(dwCRC, pTree));
	}

	ppMainTree = pTree;
	return TRUE;
}

CSpeedTreeForest::SpeedTreeWrapperPtr CSpeedTreeForest::CreateInstance(float x, float y, float z, DWORD dwTreeCRC, const char* c_szTreeName)
{
	SpeedTreeWrapperPtr pMainTree;
	if (!GetMainTree(dwTreeCRC, pMainTree, c_szTreeName))
	{
		return NULL;
	}

	SpeedTreeWrapperPtr pTreeInst = pMainTree->MakeInstance();
	pTreeInst->SetPosition(x, y, z);
	pTreeInst->RegisterBoundingSphere();
	return pTreeInst;
}

void CSpeedTreeForest::DeleteInstance(SpeedTreeWrapperPtr pInstance)
{
	if (!pInstance)
		return;

	SpeedTreeWrapperPtr pParentTree = pInstance->InstanceOf();

	if (!pParentTree)
		return;

	pParentTree->DeleteInstance(pInstance);
}

void CSpeedTreeForest::UpdateSystem(float fCurrentTime)
{
	if (m_bFirstUpdate)
	{
		m_fLastUpdateTime = fCurrentTime;
		m_bFirstUpdate = false;
		CSpeedTreeRT::SetTime(0.0f);
		SetupWindMatrices(m_fAccumTime);
		return;
	}

	float fElapsedTime = fCurrentTime - m_fLastUpdateTime;
	if (fabsf(fElapsedTime) < 0.00001f)
	{
		// Unikaj zbednych duplikatow w tej samej klatce
		return;
	}

	if (fElapsedTime < 0.0f)
	{
		// Ochrona przed cofaniem sie czasu (np. zapetlenie lub reset timera)
		fElapsedTime = 0.0f;
	}

	m_fLastUpdateTime = fCurrentTime;
	CSpeedTreeRT::SetTime(fElapsedTime);

	m_fAccumTime += fElapsedTime; 
	SetupWindMatrices(m_fAccumTime);
}

///////////////////////////////////////////////////////////////////////  
//	CSpeedTreeForest::AdjustExtents
void CSpeedTreeForest::AdjustExtents(float x, float y, float z)
{
    // min
    m_afForestExtents[0] = __min(m_afForestExtents[0], x); 
    m_afForestExtents[1] = __min(m_afForestExtents[1], y); 
    m_afForestExtents[2] = __min(m_afForestExtents[2], z); 

    // max
    m_afForestExtents[3] = __max(m_afForestExtents[3], x); 
    m_afForestExtents[4] = __max(m_afForestExtents[4], y); 
    m_afForestExtents[5] = __max(m_afForestExtents[5], z);
}

///////////////////////////////////////////////////////////////////////  
//	CSpeedTreeForest::SetWindStrength
void CSpeedTreeForest::SetWindStrength(float fStrength)
{
	if (m_fWindStrength == fStrength)
		return;

	m_fWindStrength = fStrength;

	TTreeMap::iterator itor = m_pMainTreeMap.begin();
	UINT uiCount;

	while (itor != m_pMainTreeMap.end())
	{
		auto pMainTree = (itor++)->second;
		const auto& ppInstances = pMainTree->GetInstances(uiCount);

		for (auto it : ppInstances)
			it->GetSpeedTree()->SetWindStrength(m_fWindStrength);
	}
}

///////////////////////////////////////////////////////////////////////  
//	CSpeedTreeForest::SetupWindMatrices
void CSpeedTreeForest::SetupWindMatrices(float fTimeInSecs)
{
	static const float afFrequencies[c_nNumWindMatrices][2] = 
	{
		{ 0.15f, 0.17f },
		{ 0.25f, 0.15f },
		{ 0.19f, 0.05f },
		{ 0.15f, 0.22f }
	};

	// Unikaj ponownego przeliczania macierzy wiatru w tej samej klatce
	if (m_bWindMatricesCached && 
		fabsf(fTimeInSecs - m_fLastWindComputedTime) < 0.0001f && 
		fabsf(m_fWindStrength - m_fLastWindStrength) < 0.0001f)
	{
		return;
	}

	float fTimeSinceLastCall = fTimeInSecs - m_fTimeOfLastWindCall;
	m_fTimeOfLastWindCall = fTimeInSecs;

	// Brak wiatru - macierze jednostkowe
	if (m_fWindStrength <= 0.0001f)
	{
		for (int j = 0; j < c_nNumWindMatrices; ++j)
		{
			memset(m_afCachedWindMatrices[j], 0, sizeof(float) * 16);
			m_afCachedWindMatrices[j][0] = 1.0f;
			m_afCachedWindMatrices[j][5] = 1.0f;
			m_afCachedWindMatrices[j][10] = 1.0f;
			m_afCachedWindMatrices[j][15] = 1.0f;

			#ifdef WRAPPER_USE_CPU_WIND
				CSpeedTreeRT::SetWindMatrix(j, m_afCachedWindMatrices[j]);
			#endif

			#ifdef WRAPPER_USE_GPU_WIND
				UploadWindMatrix(c_nVertexShader_WindMatrices + j * 4, m_afCachedWindMatrices[j]);
			#endif
		}

		m_fOldWindStrength = m_fWindStrength;
		m_fLastWindComputedTime = fTimeInSecs;
		m_fLastWindStrength = m_fWindStrength;
		m_bWindMatricesCached = true;
		return;
	}

	if (fTimeSinceLastCall > 0.0f && fTimeSinceLastCall < 1.0f)
	{
		for (int i = 0; i < c_nNumWindMatrices; ++i)
			m_afMatrixTimes[i] += fTimeSinceLastCall;
	}

	const float fBaseAngle = m_fWindStrength * 35.0f;
	const float c_fDeg2Rad = 57.2957795f;
	const float fBaseFreq = m_fWindStrength * 20.0f;

	for (int j = 0; j < c_nNumWindMatrices; ++j)
	{
		if (m_fWindStrength != 0.0f && m_fOldWindStrength != 0.0f)
			m_afMatrixTimes[j] = (m_afMatrixTimes[j] * m_fOldWindStrength) / m_fWindStrength;

		const float fXPercent = sinf(fBaseFreq * afFrequencies[j % c_nNumWindMatrices][0] * m_afMatrixTimes[j]);
		const float fYPercent = cosf(fBaseFreq * afFrequencies[j % c_nNumWindMatrices][1] * m_afMatrixTimes[j]);

		const float fSinX = sinf(fBaseAngle * fXPercent / c_fDeg2Rad);
		const float fSinY = sinf(fBaseAngle * fYPercent / c_fDeg2Rad);
		const float fCosX = cosf(fBaseAngle * fXPercent / c_fDeg2Rad);
		const float fCosY = cosf(fBaseAngle * fYPercent / c_fDeg2Rad);

		float* afMatrix = m_afCachedWindMatrices[j];
		memset(afMatrix, 0, sizeof(float) * 16);
		afMatrix[0] = fCosY;
		afMatrix[2] = -fSinY;
		afMatrix[4] = fSinX * fSinY;
		afMatrix[5] = fCosX;
		afMatrix[6] = fSinX * fCosY;
		afMatrix[8] = fSinY * fCosX;
		afMatrix[9] = -fSinX;
		afMatrix[10] = fCosX * fCosY;
		afMatrix[15] = 1.0f;

		#ifdef WRAPPER_USE_CPU_WIND
			CSpeedTreeRT::SetWindMatrix(j, afMatrix);
		#endif

		#ifdef WRAPPER_USE_GPU_WIND
			UploadWindMatrix(c_nVertexShader_WindMatrices + j * 4, afMatrix);
		#endif
	}

	m_fOldWindStrength = m_fWindStrength;
	m_fLastWindComputedTime = fTimeInSecs;
	m_fLastWindStrength = m_fWindStrength;
	m_bWindMatricesCached = true;
}

const float* CSpeedTreeForest::GetCachedWindMatrix(int nIndex) const
{
	if (nIndex < 0 || nIndex >= c_nNumWindMatrices)
		return nullptr;
	return m_afCachedWindMatrices[nIndex];
}

void CSpeedTreeForest::SubmitTreePackets(unsigned long ulRenderBitVector)
{
	CRenderQueueManager& rqm = CRenderQueueManager::Instance();

	const bool bWantBranchesOrFronds = (ulRenderBitVector & Forest_RenderBranches) || (ulRenderBitVector & Forest_RenderFronds);
	const bool bWantLeavesOrBillboards = (ulRenderBitVector & Forest_RenderLeaves) || (ulRenderBitVector & Forest_RenderBillboards);
	if (!bWantBranchesOrFronds && !bWantLeavesOrBillboards)
		return;

	for (const auto& tr : m_pMainTreeMap)
	{
		const DWORD dwCRC = tr.first;
		const SpeedTreeWrapperPtr& pMainTree = tr.second;
		if (!pMainTree)
			continue;

		UINT uiCount = 0;
		const auto& ppInstances = pMainTree->GetInstances(uiCount);
		if (ppInstances.empty())
			continue;

		size_t visibleCount = 0;
		for (const auto& pInst : ppInstances)
		{
			if (pInst && pInst->isShow())
				++visibleCount;
		}

		if (visibleCount == 0)
			continue;

		TSpeedTreeInstanceData* pInstanceSlice = rqm.AllocateSpeedTreeInstanceData(visibleCount);
		if (!pInstanceSlice)
			continue;

		size_t writeIdx = 0;
		for (const auto& pInst : ppInstances)
		{
			if (pInst && pInst->isShow())
			{
				TSpeedTreeInstanceData& instData = pInstanceSlice[writeIdx++];
				instData.pTreeInstance = pInst.get();
				instData.dwTreeCRC = dwCRC;
				instData.fWindStrength = m_fWindStrength;
				const float* pos = pInst->GetPosition();
				if (pos)
				{
					instData.v3Position = D3DXVECTOR3(pos[0], pos[1], pos[2]);
					D3DXMatrixTranslation(&instData.matWorld, pos[0], pos[1], pos[2]);
				}
				else
				{
					instData.v3Position = D3DXVECTOR3(0.0f, 0.0f, 0.0f);
					D3DXMatrixIdentity(&instData.matWorld);
				}

				instData.fAlphaRefBranch = pInst->GetBranchAlphaTestValue();
				instData.fAlphaRefFrond = pInst->GetFrondAlphaTestValue();
				instData.fAlphaRefLeaf = pInst->GetLeafAlphaTestValue(0);
				instData.ucAlphaRefBranch = static_cast<uint8_t>(pInst->GetBranchAlphaTestValue());
				instData.ucAlphaRefFrond = static_cast<uint8_t>(pInst->GetFrondAlphaTestValue());
				instData.ucAlphaRefLeaf0 = static_cast<uint8_t>(pInst->GetLeafAlphaTestValue(0));
				instData.ucAlphaRefLeaf1 = static_cast<uint8_t>(pInst->GetLeafAlphaTestValue(1));
				instData.nBranchLod = static_cast<int8_t>(pInst->GetBranchLod());
				instData.nFrondLod = static_cast<int8_t>(pInst->GetFrondLod());
				instData.nLeafLod0 = static_cast<int8_t>(pInst->GetLeafLod(0));
				instData.nLeafLod1 = static_cast<int8_t>(pInst->GetLeafLod(1));
				instData.ucAlphaRefBillboard0 = static_cast<uint8_t>(pInst->GetBillboardAlphaTestValue(0));
				instData.ucAlphaRefBillboard1 = static_cast<uint8_t>(pInst->GetBillboardAlphaTestValue(1));
				instData.wInstanceFlags = 0;
			}
		}

		uint32_t dwTrunkTexID = 0;
		uint32_t dwLeafTexID = 0;
		if (CRenderResourceRegistry::InstancePtr())
		{
			auto* pTrunkTex = pMainTree->GetBranchTexture();
			if (pTrunkTex)
				dwTrunkTexID = CRenderResourceRegistry::Instance().FindTextureHandle(pTrunkTex);

			auto* pLeafTex = pMainTree->GetCompositeTexture();
			if (pLeafTex)
				dwLeafTexID = CRenderResourceRegistry::Instance().FindTextureHandle(pLeafTex);
		}

		// 1. Partia pni (siatki 3D: Branches + Fronds)
		if (bWantBranchesOrFronds)
		{
			TSpeedTreePacket branchPacket{};
			branchPacket.pMainTree = pMainTree.get();
			branchPacket.dwTreeCRC = dwCRC;
			branchPacket.dwTreeTypeID = dwCRC;
			branchPacket.dwTrunkTextureID = dwTrunkTexID;
			branchPacket.dwLeafTextureID = dwLeafTexID;
			branchPacket.bRenderBranches = (ulRenderBitVector & Forest_RenderBranches) != 0;
			branchPacket.bRenderFronds = (ulRenderBitVector & Forest_RenderFronds) != 0;
			branchPacket.dwInstanceCount = static_cast<uint32_t>(visibleCount);
			branchPacket.pInstances = pInstanceSlice;
			branchPacket.fAlphaRef = pMainTree->GetBranchAlphaTestValue();
			branchPacket.ucAlphaRefBranch = static_cast<uint8_t>(pMainTree->GetBranchAlphaTestValue());
			branchPacket.ucAlphaRefFrond = static_cast<uint8_t>(pMainTree->GetFrondAlphaTestValue());
			branchPacket.nDiscreteLod = static_cast<int16_t>(pMainTree->GetBranchLod());
			rqm.SubmitSpeedTree(branchPacket);
		}

		// 2. Partia lisci (billboardy z testem alfa: Leaves + Billboards)
		if (bWantLeavesOrBillboards)
		{
			TSpeedTreePacket leafPacket{};
			leafPacket.pMainTree = pMainTree.get();
			leafPacket.dwTreeCRC = dwCRC;
			leafPacket.dwTreeTypeID = dwCRC;
			leafPacket.dwTrunkTextureID = dwTrunkTexID;
			leafPacket.dwLeafTextureID = dwLeafTexID;
			leafPacket.bRenderLeaves = (ulRenderBitVector & Forest_RenderLeaves) != 0;
			leafPacket.bRenderBillboards = (ulRenderBitVector & Forest_RenderBillboards) != 0;
			leafPacket.dwInstanceCount = static_cast<uint32_t>(visibleCount);
			leafPacket.pInstances = pInstanceSlice;
			leafPacket.fAlphaRef = pMainTree->GetLeafAlphaTestValue(0);
			leafPacket.ucAlphaRefLeaf = static_cast<uint8_t>(pMainTree->GetLeafAlphaTestValue(0));
			leafPacket.ucAlphaRefBillboard = static_cast<uint8_t>(pMainTree->GetBillboardAlphaTestValue(0));
			leafPacket.nDiscreteLod = static_cast<int16_t>(pMainTree->GetLeafLod(0));
			rqm.SubmitSpeedTree(leafPacket);
		}
	}
}

void CSpeedTreeForest::SetLight(const float * afDirection, const float * afAmbient, const float * afDiffuse)
{
	m_afLighting[0] = afDirection[0]; m_afLighting[1] = afDirection[1]; m_afLighting[2] = afDirection[2]; m_afLighting[3] = 1.0f;
	m_afLighting[4] = afAmbient[0]; m_afLighting[5] = afAmbient[1]; m_afLighting[6] = afAmbient[2]; m_afLighting[7] = afAmbient[3];
	m_afLighting[8] = afDiffuse[0]; m_afLighting[9] = afDiffuse[1]; m_afLighting[10] = afDiffuse[2]; m_afLighting[11] = afDiffuse[3];
}

void CSpeedTreeForest::SetFog(float fFogNear, float fFogFar)
{
	const float c_fFogLinearScale = (1.0f / (fFogFar - fFogNear));
	m_afFog[0] = fFogNear; m_afFog[1] = fFogFar; m_afFog[2] = c_fFogLinearScale; m_afFog[3] = 0.0f;
}
