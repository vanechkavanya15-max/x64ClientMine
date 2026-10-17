#include "StdAfx.h"
#include "SnowEnvironment.h"

#include "EterLib/Camera.h"
#include "EterLib/ResourceManager.h"
#include "EffectLib/EffectParticleBatcher.h"
#include "SnowParticle.h"
#include <algorithm>

void CSnowEnvironment::Enable()
{
	if (!m_bSnowEnable)
	{
		Create();
	}

	m_bSnowEnable = TRUE;
	m_dwLastTime = CTimer::Instance().GetCurrentMillisecond();
}

void CSnowEnvironment::Disable()
{
	m_bSnowEnable = FALSE;
}

void CSnowEnvironment::Update(const D3DXVECTOR3 & c_rv3Pos)
{
	if (!m_bSnowEnable)
	{
		if (m_kVct_pkParticleSnow.empty())
			return;
	}

	m_v3Center = c_rv3Pos;
}

void CSnowEnvironment::Deform()
{
	if (!m_bSnowEnable)
	{
		if (m_kVct_pkParticleSnow.empty())
			return;
	}

	const D3DXVECTOR3 & c_rv3Pos = m_v3Center;

	const DWORD dwCurTime = CTimer::Instance().GetCurrentMillisecond();
	if (m_dwLastTime == 0)
	{
		m_dwLastTime = dwCurTime;
	}
	float fElapsedTime = static_cast<float>(dwCurTime - m_dwLastTime) / 1000.0f;
	m_dwLastTime = dwCurTime;

	if (fElapsedTime > 0.1f)
	{
		fElapsedTime = 0.1f;
	}

	CCamera * pCamera = CCameraManager::Instance().GetCurrentCamera();
	if (!pCamera)
		return;

	const D3DXVECTOR3 & c_rv3View = pCamera->GetView();
	const D3DXVECTOR3 & c_rv3Up = pCamera->GetUp();
	const D3DXVECTOR3 & c_rv3Cross = pCamera->GetCross();

	D3DXVECTOR3 v3ChangedPos = c_rv3View * 3500.0f + c_rv3Pos;
	v3ChangedPos.z = c_rv3Pos.z;

	// Aktualizacja czasteczek sniegu (zero alokacji sterty w klatce, O(1) swap-and-pop)
	for (size_t i = 0; i < m_kVct_pkParticleSnow.size(); )
	{
		CSnowParticle * pSnow = m_kVct_pkParticleSnow[i];
		pSnow->SetCameraVertex(c_rv3Up, c_rv3Cross);
		pSnow->Update(fElapsedTime, v3ChangedPos);

		if (!pSnow->IsActivate())
		{
			CSnowParticle::Delete(pSnow);
			m_kVct_pkParticleSnow[i] = m_kVct_pkParticleSnow.back();
			m_kVct_pkParticleSnow.pop_back();
		}
		else
		{
			++i;
		}
	}

	// Emitowanie nowych platkow z prealokowanej puli obiektow
	if (m_bSnowEnable)
	{
		const size_t currentCount = m_kVct_pkParticleSnow.size();
		if (currentCount < m_dwParticleMaxNum)
		{
			const size_t spawnCount = std::min<size_t>(10, m_dwParticleMaxNum - currentCount);
			for (size_t p = 0; p < spawnCount; ++p)
			{
				CSnowParticle * pSnowParticle = CSnowParticle::New();
				if (pSnowParticle)
				{
					pSnowParticle->Init(v3ChangedPos);
					m_kVct_pkParticleSnow.push_back(pSnowParticle);
				}
			}
		}
	}
}

void CSnowEnvironment::Render()
{
	if (!m_bSnowEnable)
	{
		if (m_kVct_pkParticleSnow.empty())
			return;
	}

	if (m_kVct_pkParticleSnow.empty())
		return;

	uintptr_t dwTextureID = 0;
	if (m_pImageInstance && m_pImageInstance->GetGraphicImagePointer())
	{
		LPDIRECT3DTEXTURE9 pD3DTexture = m_pImageInstance->GetGraphicImagePointer()->GetTextureReference().GetD3DTexture();
		if (pD3DTexture)
		{
			dwTextureID = reinterpret_cast<uintptr_t>(pD3DTexture);
		}
	}

	constexpr uint32_t dwSnowEffectID = 0x534E4F57; // "SNOW"
	constexpr uint32_t dwSnowBlendMode = EFFECT_BLEND_ALPHABLEND;

	CEffectParticleBatcher & batcher = CEffectParticleBatcher::Instance();

	const size_t particleCount = std::min<size_t>(m_dwParticleMaxNum, m_kVct_pkParticleSnow.size());
	for (size_t i = 0; i < particleCount; ++i)
	{
		CSnowParticle * pSnow = m_kVct_pkParticleSnow[i];
		if (!pSnow || !pSnow->IsActivate())
			continue;

		const D3DXVECTOR3 & v3Pos = pSnow->GetPosition();
		const float fHalfWidth = pSnow->GetHalfWidth();
		const float fHalfHeight = pSnow->GetHalfHeight();
		const float fScaleAvg = (fHalfWidth + fHalfHeight) * 0.5f;

		TParticleInstanceData data{};
		data.vWorldPos = D3DXVECTOR4(v3Pos.x, v3Pos.y, v3Pos.z, fScaleAvg);
		data.vColor = D3DXCOLOR(1.0f, 1.0f, 1.0f, 1.0f);
		data.vTexCoordRect = D3DXVECTOR4(0.0f, 0.0f, 1.0f, 1.0f);
		data.fRotation = pSnow->GetRotation();
		data.fLifeRatio = 1.0f;
		data.fPad[0] = fHalfWidth;
		data.fPad[1] = fHalfHeight;

		batcher.AddParticle(dwSnowEffectID, dwTextureID, dwSnowBlendMode, data);
	}

	batcher.FlushToRenderQueue();
}

bool CSnowEnvironment::Create()
{
	Destroy();

	m_kVct_pkParticleSnow.reserve(m_dwParticleMaxNum);
	CSnowParticle::InitializePool(m_dwParticleMaxNum);

	CGraphicImage * pImage = static_cast<CGraphicImage *>(CResourceManager::Instance().GetResourcePointer("d:/ymir work/special/snow.dds"));
	m_pImageInstance = CGraphicImageInstance::New();
	if (m_pImageInstance)
	{
		m_pImageInstance->SetImagePointer(pImage);
	}

	return true;
}

void CSnowEnvironment::Destroy()
{
	m_bSnowEnable = FALSE;

	for (auto * pSnow : m_kVct_pkParticleSnow)
	{
		CSnowParticle::Delete(pSnow);
	}
	m_kVct_pkParticleSnow.clear();

	CSnowParticle::DestroyPool();

	if (m_pImageInstance)
	{
		CGraphicImageInstance::Delete(m_pImageInstance);
		m_pImageInstance = nullptr;
	}

	__Initialize();
}

void CSnowEnvironment::__Initialize()
{
	m_bSnowEnable = FALSE;
	m_v3Center = D3DXVECTOR3(0.0f, 0.0f, 0.0f);
	m_pImageInstance = nullptr;
	m_kVct_pkParticleSnow.clear();
	m_kVct_pkParticleSnow.reserve(m_dwParticleMaxNum);
	m_dwLastTime = 0;
}

CSnowEnvironment::CSnowEnvironment()
	: m_v3Center(0.0f, 0.0f, 0.0f)
	, m_pImageInstance(nullptr)
	, m_dwParticleMaxNum(3000)
	, m_bSnowEnable(FALSE)
	, m_dwLastTime(0)
{
	__Initialize();
}

CSnowEnvironment::~CSnowEnvironment()
{
	Destroy();
}
