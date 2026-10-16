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
					0.8f, 0.8f, 0.8f, 1.0f } // diffuse
{
	CSpeedTreeRT::SetNumWindMatrices(c_nNumWindMatrices);

	m_afForestExtents[0] = m_afForestExtents[1] = m_afForestExtents[2] = FLT_MAX;
	m_afForestExtents[3] = m_afForestExtents[4] = m_afForestExtents[5] = -FLT_MAX;
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
		auto ppInstances = pMainTree->GetInstances(uiCount);

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
	static float fLastTime = fCurrentTime;
	float fElapsedTime = fCurrentTime - fLastTime;
	fLastTime = fCurrentTime;
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
		auto ppInstances = pMainTree->GetInstances(uiCount);

		for (auto it : ppInstances)
			it->GetSpeedTree()->SetWindStrength(m_fWindStrength);
	}
}

///////////////////////////////////////////////////////////////////////  
//	CSpeedTreeForest::SetupWindMatrices
void CSpeedTreeForest::SetupWindMatrices(float fTimeInSecs)
{
	// matrix computational data
	static float afMatrixTimes[c_nNumWindMatrices] = { 0.0f };
	static const float afFrequencies[c_nNumWindMatrices][2] = 
	{
		{ 0.15f, 0.17f },
		{ 0.25f, 0.15f },
		{ 0.19f, 0.05f },
		{ 0.15f, 0.22f }
	};

	// Buforowanie macierzy wiatru per klatka, aby zapobiec ich wielokrotnemu przeliczaniu
	static float s_afCachedMatrices[c_nNumWindMatrices][16] = { { 0.0f } };
	static float s_fLastComputedTime = -1000.0f;
	static float s_fLastStrength = -1.0f;
	static bool s_bMatricesCached = false;

	// Jesli macierze zostaly juz policzone dla tej klatki (czasu) i sily wiatru, pomin kosztowne funkcje trygonometryczne
	if (s_bMatricesCached && fabsf(fTimeInSecs - s_fLastComputedTime) < 0.0001f && fabsf(m_fWindStrength - s_fLastStrength) < 0.0001f)
	{
		return;
	}

	// compute time since last call
	static float fTimeOfLastCall = 0.0f;
	float fTimeSinceLastCall = fTimeInSecs - fTimeOfLastCall;
	fTimeOfLastCall = fTimeInSecs;

	// wind strength
	static float fOldStrength = m_fWindStrength;

	// Jesli brak wiatru, ustaw macierze jednostkowe bez zbednych obliczen sin/cos
	if (m_fWindStrength <= 0.0001f)
	{
		for (int j = 0; j < c_nNumWindMatrices; ++j)
		{
			memset(s_afCachedMatrices[j], 0, sizeof(float) * 16);
			s_afCachedMatrices[j][0] = 1.0f;
			s_afCachedMatrices[j][5] = 1.0f;
			s_afCachedMatrices[j][10] = 1.0f;
			s_afCachedMatrices[j][15] = 1.0f;

			#ifdef WRAPPER_USE_CPU_WIND
				CSpeedTreeRT::SetWindMatrix(j, s_afCachedMatrices[j]);
			#endif

			#ifdef WRAPPER_USE_GPU_WIND
				UploadWindMatrix(c_nVertexShader_WindMatrices + j * 4, s_afCachedMatrices[j]);
			#endif
		}

		fOldStrength = m_fWindStrength;
		s_fLastComputedTime = fTimeInSecs;
		s_fLastStrength = m_fWindStrength;
		s_bMatricesCached = true;
		return;
	}

	// increment matrix times
	if (fTimeSinceLastCall > 0.0f && fTimeSinceLastCall < 1.0f)
	{
		for (int i = 0; i < c_nNumWindMatrices; ++i)
			afMatrixTimes[i] += fTimeSinceLastCall;
	}

	// compute maximum branch throw
	const float fBaseAngle = m_fWindStrength * 35.0f;
	const float c_fDeg2Rad = 57.2957795f;
	const float fBaseFreq = m_fWindStrength * 20.0f;

	// build rotation matrices
	for (int j = 0; j < c_nNumWindMatrices; ++j)
	{
		// adjust time to prevent "jumping"
		if (m_fWindStrength != 0.0f && fOldStrength != 0.0f)
			afMatrixTimes[j] = (afMatrixTimes[j] * fOldStrength) / m_fWindStrength;

		// compute percentages for each axis
		const float fXPercent = sinf(fBaseFreq * afFrequencies[j % c_nNumWindMatrices][0] * afMatrixTimes[j]);
		const float fYPercent = cosf(fBaseFreq * afFrequencies[j % c_nNumWindMatrices][1] * afMatrixTimes[j]);

		// build compound rotation matrix (rotate on 'x' then on 'y')
		const float fSinX = sinf(fBaseAngle * fXPercent / c_fDeg2Rad);
		const float fSinY = sinf(fBaseAngle * fYPercent / c_fDeg2Rad);
		const float fCosX = cosf(fBaseAngle * fXPercent / c_fDeg2Rad);
		const float fCosY = cosf(fBaseAngle * fYPercent / c_fDeg2Rad);

		float* afMatrix = s_afCachedMatrices[j];
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
			// graphics API specific
			UploadWindMatrix(c_nVertexShader_WindMatrices + j * 4, afMatrix);
		#endif
	}

	// track wind strength and frame cache
	fOldStrength = m_fWindStrength;
	s_fLastComputedTime = fTimeInSecs;
	s_fLastStrength = m_fWindStrength;
	s_bMatricesCached = true;
}


///////////////////////////////////////////////////////////////////////  
//	CSpeedTreeForest::SetLodLimits
/*
void CSpeedTreeForest::SetLodLimits(void)
{
	// find tallest tree
	float fTallest = -1.0f;

	TTreeMap::iterator itor = m_pMainTreeMap.begin();
	UINT uiCount;

	while (itor != m_pMainTreeMap.end())
	{
		CSpeedTreeWrapper * pMainTree = (itor++)->second;
		CSpeedTreeWrapper ** ppInstances = pMainTree->GetInstances(uiCount);

		float fHeight;
		fHeight = pMainTree->GetBoundingBox()[5] - pMainTree->GetBoundingBox()[0];
		fTallest = __max(fHeight, fTallest);

		for (UINT i = 0; i < uiCount; ++i)
		{
			fHeight = ppInstances[i]->GetBoundingBox()[5] - ppInstances[i]->GetBoundingBox()[0];
			fTallest = __max(fHeight, fTallest);
		}
	}

	itor = m_pMainTreeMap.begin();

	while (itor != m_pMainTreeMap.end())
	{
		CSpeedTreeWrapper * pMainTree = (itor++)->second;
		CSpeedTreeWrapper ** ppInstances = pMainTree->GetInstances(uiCount);

		pMainTree->GetSpeedTree()->SetLodLimits(fTallest * c_fNearLodFactor, fTallest * c_fFarLodFactor);

		for (UINT i = 0; i < uiCount; ++i)
			ppInstances[i]->GetSpeedTree()->SetLodLimits(fTallest * c_fNearLodFactor, fTallest * c_fFarLodFactor);
	}
}
*/
void CSpeedTreeForest::SetLight(const float * afDirection, const float * afAmbient, const float * afDiffuse)
{
	m_afLighting[0] = afDirection[0];
	m_afLighting[1] = afDirection[1];
	m_afLighting[2] = afDirection[2];
	m_afLighting[3] = 1.0f;

	m_afLighting[4] = afAmbient[0];
	m_afLighting[5] = afAmbient[1];
	m_afLighting[6] = afAmbient[2];
	m_afLighting[7] = afAmbient[3];

	m_afLighting[8] = afDiffuse[0];
	m_afLighting[9] = afDiffuse[1];
	m_afLighting[10] = afDiffuse[2];
	m_afLighting[11] = afDiffuse[3];
}

void CSpeedTreeForest::SetFog(float fFogNear, float fFogFar)
{
	const float c_fFogLinearScale = (1.0f / (fFogFar - fFogNear));

	m_afFog[0] = fFogNear;
	m_afFog[1] = fFogFar;
	m_afFog[2] = c_fFogLinearScale;
	m_afFog[3] = 0.0f;
}
