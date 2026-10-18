#include "StdAfx.h"
#include "EffectInstance.h"
#include "ParticleSystemInstance.h"
#include "SimpleLightInstance.h"
#include "EffectParticleBatcher.h"
#include "EterLib/RenderQueueManager.h"
#include "EterLib/RenderOwnership.h"

#include "EterBase/Stl.h"
#include "EterLib/StateManager.h"
#include "AudioLib/SoundEngine.h"

CDynamicPool<CEffectInstance>	CEffectInstance::ms_kPool;
int CEffectInstance::ms_iRenderingEffectCount = 0;

bool CEffectInstance::LessRenderOrder(const CEffectInstance* pkEftInst) const
{
	if (!pkEftInst)
		return false;
	if (this == pkEftInst)
		return false;

	DWORD dwMyCRC = m_pkEftData ? m_pkEftData->GetCRC32() : 0;
	DWORD dwOtherCRC = pkEftInst->m_pkEftData ? pkEftInst->m_pkEftData->GetCRC32() : 0;

	if (dwMyCRC != dwOtherCRC)
		return dwMyCRC < dwOtherCRC;

	if (m_fLifeTime != pkEftInst->m_fLifeTime)
		return m_fLifeTime < pkEftInst->m_fLifeTime;

	return false;
}

DWORD CEffectInstance::GetEffectDataCRC() const
{
	if (!m_pkEftData)
		return 0;
	return m_pkEftData->GetCRC32();
}

void CEffectInstance::ResetRenderingEffectCount()
{
	ms_iRenderingEffectCount = 0;
}

int CEffectInstance::GetRenderingEffectCount()
{
	return ms_iRenderingEffectCount;
}

CEffectInstance* CEffectInstance::New()
{
	CEffectInstance* pkEftInst=ms_kPool.Alloc();
	return pkEftInst;
}

void CEffectInstance::Delete(CEffectInstance* pkEftInst)
{
	pkEftInst->Clear();
	ms_kPool.Free(pkEftInst);
}

void CEffectInstance::DestroySystem()
{
	ms_kPool.Destroy();

	CParticleSystemInstance::DestroySystem();
	CEffectMeshInstance::DestroySystem();
	CLightInstance::DestroySystem();
}

void CEffectInstance::UpdateSound()
{
	if (m_pSoundInstanceVector)
	{
		SoundEngine::Instance().UpdateSoundInstance(m_matGlobal._41,
													m_matGlobal._42,
													m_matGlobal._43,
													m_dwFrame,
													m_pSoundInstanceVector,
													false);
		// NOTE : 매트릭스에서 위치를 직접 얻어온다 - [levites]
	}
	++m_dwFrame;
}

struct FEffectUpdator
{
	BOOL isAlive;
	float fElapsedTime;
	FEffectUpdator(float fElapsedTime)
		: isAlive(FALSE), fElapsedTime(fElapsedTime)
	{
	}
	void operator () (CEffectElementBaseInstance * pInstance)
	{
		if (pInstance->Update(fElapsedTime)) [[likely]]
			isAlive = TRUE;
	}
};

void CEffectInstance::OnUpdate()
{
	Transform();

	float fCurrentSecond = CTimer::InstancePtr() ? CTimer::Instance().GetCurrentSecond() : (m_fLastTime + 0.016f);
	float fElapsedTime = fCurrentSecond - m_fLastTime;
	m_fLifeTime += fElapsedTime;

	FEffectUpdator f(fElapsedTime);
	f = std::for_each(m_ParticleInstanceVector.begin(), m_ParticleInstanceVector.end(),f);
	f = std::for_each(m_MeshInstanceVector.begin(), m_MeshInstanceVector.end(),f);
	f = std::for_each(m_LightInstanceVector.begin(), m_LightInstanceVector.end(),f);
	m_isAlive = f.isAlive;

	m_fLastTime = fCurrentSecond;
}

void CEffectInstance::OnRender()
{
	// 1. Zbieraj instancje czasteczek do ciaglego bufora wsadowego
	CEffectParticleBatcher::Instance().BeginFrame();

	std::for_each(m_ParticleInstanceVector.begin(), m_ParticleInstanceVector.end(), std::mem_fn(&CEffectElementBaseInstance::Render));

	// 2. Agreguj wedlug blend mode / texture ID i emituj pakiety do CRenderQueueManager
	CEffectParticleBatcher::Instance().FlushToRenderQueue();

	// 3. Renderuj geometrie siatek (EffectMesh), jesli wystepuja
	if (!m_MeshInstanceVector.empty())
	{
		if (RenderOwnership::LegacyDraws(ERenderCategory::EffectMesh))
		{
			STATEMANAGER.SetFVF(D3DFVF_XYZ | D3DFVF_TEX1);

			STATEMANAGER.SaveSamplerState(0, D3DSAMP_MINFILTER, D3DTEXF_NONE);
			STATEMANAGER.SaveSamplerState(0, D3DSAMP_MAGFILTER, D3DTEXF_NONE);

			STATEMANAGER.SaveRenderState(D3DRS_ALPHABLENDENABLE, TRUE);
			STATEMANAGER.SaveRenderState(D3DRS_SRCBLEND, D3DBLEND_SRCALPHA);
			STATEMANAGER.SaveRenderState(D3DRS_DESTBLEND, D3DBLEND_INVSRCALPHA);
			STATEMANAGER.SaveRenderState(D3DRS_ALPHATESTENABLE, FALSE);
			STATEMANAGER.SaveRenderState(D3DRS_CULLMODE, D3DCULL_NONE);
			STATEMANAGER.SaveRenderState(D3DRS_ZWRITEENABLE, FALSE);

			STATEMANAGER.SaveTextureStageState(0, D3DTSS_COLORARG1, D3DTA_TFACTOR);
			STATEMANAGER.SaveTextureStageState(0, D3DTSS_COLORARG2, D3DTA_TEXTURE);
			STATEMANAGER.SaveTextureStageState(0, D3DTSS_COLOROP,   D3DTOP_MODULATE);
			STATEMANAGER.SaveTextureStageState(0, D3DTSS_ALPHAARG1, D3DTA_TFACTOR);
			STATEMANAGER.SaveTextureStageState(0, D3DTSS_ALPHAARG2, D3DTA_TEXTURE);
			STATEMANAGER.SaveTextureStageState(0, D3DTSS_ALPHAOP,   D3DTOP_MODULATE);

			std::for_each(m_MeshInstanceVector.begin(), m_MeshInstanceVector.end(), std::mem_fn(&CEffectElementBaseInstance::Render));

			STATEMANAGER.RestoreTextureStageState(0, D3DTSS_ALPHAOP);
			STATEMANAGER.RestoreTextureStageState(0, D3DTSS_ALPHAARG2);
			STATEMANAGER.RestoreTextureStageState(0, D3DTSS_ALPHAARG1);
			STATEMANAGER.RestoreTextureStageState(0, D3DTSS_COLOROP);
			STATEMANAGER.RestoreTextureStageState(0, D3DTSS_COLORARG2);
			STATEMANAGER.RestoreTextureStageState(0, D3DTSS_COLORARG1);

			STATEMANAGER.RestoreRenderState(D3DRS_ZWRITEENABLE);
			STATEMANAGER.RestoreRenderState(D3DRS_CULLMODE);
			STATEMANAGER.RestoreRenderState(D3DRS_ALPHATESTENABLE);
			STATEMANAGER.RestoreRenderState(D3DRS_DESTBLEND);
			STATEMANAGER.RestoreRenderState(D3DRS_SRCBLEND);
			STATEMANAGER.RestoreRenderState(D3DRS_ALPHABLENDENABLE);

			STATEMANAGER.RestoreSamplerState(0, D3DSAMP_MAGFILTER);
			STATEMANAGER.RestoreSamplerState(0, D3DSAMP_MINFILTER);
		}
		else
		{
			std::for_each(m_MeshInstanceVector.begin(), m_MeshInstanceVector.end(), std::mem_fn(&CEffectElementBaseInstance::Render));
		}
	}

	++ms_iRenderingEffectCount;
}

void CEffectInstance::SetGlobalMatrix(const D3DXMATRIX & c_rmatGlobal)
{
	m_matGlobal = c_rmatGlobal;
}

BOOL CEffectInstance::isAlive()
{
	return m_isAlive;
}

void CEffectInstance::SetActive()
{
	std::for_each(
		m_ParticleInstanceVector.begin(),
		m_ParticleInstanceVector.end(),
		std::mem_fn(&CEffectElementBaseInstance::SetActive));
	std::for_each(
		m_MeshInstanceVector.begin(),
		m_MeshInstanceVector.end(),
		std::mem_fn(&CEffectElementBaseInstance::SetActive));
	std::for_each(
		m_LightInstanceVector.begin(),
		m_LightInstanceVector.end(),
		std::mem_fn(&CEffectElementBaseInstance::SetActive));
}

void CEffectInstance::SetDeactive()
{
	std::for_each(
		m_ParticleInstanceVector.begin(),
		m_ParticleInstanceVector.end(),
		std::mem_fn(&CEffectElementBaseInstance::SetDeactive));
	std::for_each(
		m_MeshInstanceVector.begin(),
		m_MeshInstanceVector.end(),
		std::mem_fn(&CEffectElementBaseInstance::SetDeactive));
	std::for_each(
		m_LightInstanceVector.begin(),
		m_LightInstanceVector.end(),
		std::mem_fn(&CEffectElementBaseInstance::SetDeactive));
}

void CEffectInstance::__SetParticleData(CParticleSystemData * pData)
{
	CParticleSystemInstance * pInstance = CParticleSystemInstance::New();
	pInstance->SetDataPointer(pData);
	pInstance->SetLocalMatrixPointer(&m_matGlobal);

	m_ParticleInstanceVector.push_back(pInstance);
}
void CEffectInstance::__SetMeshData(CEffectMeshScript * pMesh)
{
	CEffectMeshInstance * pMeshInstance = CEffectMeshInstance::New();
	pMeshInstance->SetDataPointer(pMesh);
	pMeshInstance->SetLocalMatrixPointer(&m_matGlobal);

	m_MeshInstanceVector.push_back(pMeshInstance);
}

void CEffectInstance::__SetLightData(CLightData* pData)
{
	CLightInstance * pInstance = CLightInstance::New();
	pInstance->SetDataPointer(pData);
	pInstance->SetLocalMatrixPointer(&m_matGlobal);

	m_LightInstanceVector.push_back(pInstance);
}

void CEffectInstance::SetEffectDataPointer(CEffectData * pEffectData)
{
	Clear();

	if (!pEffectData)
		return;

	m_isAlive=true;

	m_pkEftData=pEffectData;

	m_fLifeTime = 0.0f;
	m_fLastTime = CTimer::InstancePtr() ? CTimer::Instance().GetCurrentSecond() : 0.0f;
	m_fBoundingSphereRadius = pEffectData->GetBoundingSphereRadius();
	m_v3BoundingSpherePosition = pEffectData->GetBoundingSpherePosition();

	if (m_fBoundingSphereRadius > 0.0f)
		CGraphicObjectInstance::RegisterBoundingSphere();

	DWORD i;

	for (i = 0; i < pEffectData->GetParticleCount(); ++i)
	{
		CParticleSystemData * pParticle = pEffectData->GetParticlePointer(i);

		__SetParticleData(pParticle);
	}

	for (i = 0; i < pEffectData->GetMeshCount(); ++i)
	{
		CEffectMeshScript * pMesh = pEffectData->GetMeshPointer(i);

		__SetMeshData(pMesh);
	}

	for (i = 0; i < pEffectData->GetLightCount(); ++i)
	{
		CLightData * pLight = pEffectData->GetLightPointer(i);

		__SetLightData(pLight);
	}

	m_pSoundInstanceVector = pEffectData->GetSoundInstanceVector();
}

bool CEffectInstance::GetBoundingSphere(D3DXVECTOR3 & v3Center, float & fRadius)
{
	v3Center.x = m_matGlobal._41 + m_v3BoundingSpherePosition.x;
	v3Center.y = m_matGlobal._42 + m_v3BoundingSpherePosition.y;
	v3Center.z = m_matGlobal._43 + m_v3BoundingSpherePosition.z;
	fRadius = m_fBoundingSphereRadius;
	return true;
}

void CEffectInstance::Clear()
{
	CGraphicObjectInstance::Clear();

	if (!m_ParticleInstanceVector.empty())
	{
		std::for_each(m_ParticleInstanceVector.begin(), m_ParticleInstanceVector.end(), CParticleSystemInstance::Delete);
		m_ParticleInstanceVector.clear();
	}

	if (!m_MeshInstanceVector.empty())
	{
		std::for_each(m_MeshInstanceVector.begin(), m_MeshInstanceVector.end(), CEffectMeshInstance::Delete);
		m_MeshInstanceVector.clear();
	}

	if (!m_LightInstanceVector.empty())
	{
		std::for_each(m_LightInstanceVector.begin(), m_LightInstanceVector.end(), CLightInstance::Delete);
		m_LightInstanceVector.clear();
	}

	__Initialize();
}

void CEffectInstance::__Initialize()
{
	ReleaseAlwaysHidden();
	
	m_isAlive = FALSE;
	m_dwFrame = 0;
	m_pSoundInstanceVector = NULL;
	m_fBoundingSphereRadius = 0.0f;
	m_v3BoundingSpherePosition.x = m_v3BoundingSpherePosition.y = m_v3BoundingSpherePosition.z = 0.0f;
	m_fLastTime = 0.0f;
	m_fLifeTime = 0.0f;

	m_pkEftData=NULL;

	D3DXMatrixIdentity(&m_matGlobal);
}

CEffectInstance::CEffectInstance() 
{
	__Initialize();
}
CEffectInstance::~CEffectInstance()
{
	assert(m_ParticleInstanceVector.empty());
	assert(m_MeshInstanceVector.empty());
	assert(m_LightInstanceVector.empty());
}

bool CEffectInstance::ExtractRenderPacket(TEffectRenderPacket& outPacket) const
{
	if (!m_isAlive)
		return false;

	outPacket.dwEffectID = GetEffectDataCRC();
	outPacket.dwTextureID = 0;
	outPacket.dwBlendMode = 0;
	outPacket.dwParticleCount = 0;
	outPacket.pParticles = nullptr;
	outPacket.stateKey = STATE_KEY_ALPHA_BLEND;

	return true;
}
