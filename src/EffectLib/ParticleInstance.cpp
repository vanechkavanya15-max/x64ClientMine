#include "StdAfx.h"
#include "ParticleInstance.h"
#include "ParticleProperty.h"

#include "EterBase/Random.h"
#include "EterBase/MathSIMD.h"
#include "EterLib/Camera.h"
#include "EterLib/StateManager.h"
#include "EterLib/ParticleQuad.h"
#include "EterBase/Timer.h"

CDynamicPool<CParticleInstance> CParticleInstance::ms_kPool;

void CParticleInstance::DestroySystem()
{
	ms_kPool.Destroy();
}

CParticleInstance* CParticleInstance::New()
{
	return ms_kPool.Alloc();
}

void CParticleInstance::DeleteThis()
{
	Destroy();
	
	ms_kPool.Free(this);	
}



float CParticleInstance::GetRadiusApproximation()
{
	return m_v2HalfSize.y*m_v2Scale.y + m_v2HalfSize.x*m_v2Scale.x;
}


BOOL CParticleInstance::Update(float fElapsedTime, float fAngle)
{
	m_fLastLifeTime -= fElapsedTime;
	if (m_fLastLifeTime < 0.0f)
		return FALSE;

	float fLifePercentage = (m_fLifeTime - m_fLastLifeTime) / m_fLifeTime;

	UpdateRotation(fLifePercentage, fElapsedTime);
	UpdateTextureAnimation(fLifePercentage, fElapsedTime);
	UpdateScale(fLifePercentage, fElapsedTime);
	UpdateColor(fLifePercentage, fElapsedTime);
	UpdateGravity(fLifePercentage, fElapsedTime);
	UpdateAirResistance(fLifePercentage, fElapsedTime);

	// Wektorowa aktualizacja pozycji czasteczek za pomoca MathSIMD (rejestry CPU / FMA)
	const DirectX::XMVECTOR vPos = MathSIMD::Load3(&m_v3Position.x);
	const DirectX::XMVECTOR vVel = MathSIMD::Load3(&m_v3Velocity.x);
	const DirectX::XMVECTOR vDt = DirectX::XMVectorReplicate(fElapsedTime);

	MathSIMD::Store3(&m_v3LastPosition.x, vPos);
	const DirectX::XMVECTOR vNewPos = DirectX::XMVectorMultiplyAdd(vVel, vDt, vPos);
	MathSIMD::Store3(&m_v3Position.x, vNewPos);

	if (fAngle)
	{
		if (m_pParticleProperty->m_bAttachFlag)
		{
			float fCos, fSin;
			fAngle = D3DXToRadian(fAngle);
			fCos = cos(fAngle);
			fSin = sin(fAngle);

			float rx = m_v3Position.x - m_v3StartPosition.x;
			float ry = m_v3Position.y - m_v3StartPosition.y;

			m_v3Position.x =   fCos * rx + fSin * ry + m_v3StartPosition.x;
			m_v3Position.y = - fSin * rx + fCos * ry + m_v3StartPosition.y;
		}
		else
		{
			D3DXQUATERNION q,qc;
			D3DXQuaternionRotationAxis(&q,&m_pParticleProperty->m_v3ZAxis,D3DXToRadian(fAngle));
			D3DXQuaternionConjugate(&qc,&q);

			D3DXQUATERNION qr(
				m_v3Position.x-m_v3StartPosition.x,
				m_v3Position.y-m_v3StartPosition.y,
				m_v3Position.z-m_v3StartPosition.z,
				0.0f);
			D3DXQuaternionMultiply(&qr,&q,&qr);
			D3DXQuaternionMultiply(&qr,&qr,&qc);

			m_v3Position.x = qr.x;
			m_v3Position.y = qr.y;
			m_v3Position.z = qr.z;

			m_v3Position += m_v3StartPosition;
		}
	}

	return TRUE;
}

void CParticleInstance::UpdateRotation(float time, float elapsedTime)
{
	if (m_rotationType == CParticleProperty::ROTATION_TYPE_NONE)
		return;

	if (m_rotationType == CParticleProperty::ROTATION_TYPE_TIME_EVENT)
		m_fRotationSpeed = GetTimeEventBlendValue(time, m_pParticleProperty->m_TimeEventRotation);

	m_fRotation += m_fRotationSpeed * elapsedTime;
}

void CParticleInstance::UpdateTextureAnimation(float time, float elapsedTime)
{
	if (m_byTextureAnimationType == CParticleProperty::TEXTURE_ANIMATION_TYPE_NONE)
		return;

	const float frameDelay = m_pParticleProperty->GetTextureAnimationFrameDelay();
	const DWORD frameCount = m_pParticleProperty->GetTextureAnimationFrameCount();

	m_fFrameTime += elapsedTime;

	const uint64_t elapsedFrames = static_cast<uint64_t>(m_fFrameTime / frameDelay);

	if (0 == elapsedFrames)
		return;

	m_fFrameTime -= elapsedFrames * frameDelay;

	switch (m_byTextureAnimationType)
	{
	case CParticleProperty::TEXTURE_ANIMATION_TYPE_CW:
		m_byFrameIndex += elapsedFrames;
		if (m_byFrameIndex >= frameCount)
			m_byFrameIndex = 0;
		break;

	case CParticleProperty::TEXTURE_ANIMATION_TYPE_CCW:
		m_byFrameIndex = std::min<uint8_t>(m_byFrameIndex - elapsedFrames, frameCount - 1);
		break;

	case CParticleProperty::TEXTURE_ANIMATION_TYPE_RANDOM_FRAME:
		if (frameCount != 0)
			m_byFrameIndex = random_range(0, frameCount - 1);
		break;

	default:
		break;
	}
}

void CParticleInstance::UpdateScale(float time, float elapsedTime)
{
	if (!m_pParticleProperty->m_TimeEventScaleX.empty())
		m_v2Scale.x = GetTimeEventBlendValue(time, m_pParticleProperty->m_TimeEventScaleX);

	if (!m_pParticleProperty->m_TimeEventScaleY.empty())
		m_v2Scale.y = GetTimeEventBlendValue(time, m_pParticleProperty->m_TimeEventScaleY);
}

void CParticleInstance::UpdateColor(float time, float elapsedTime)
{
	if (m_pParticleProperty->m_TimeEventColor.empty())
		return;

	m_Color = GetTimeEventBlendValue(time, m_pParticleProperty->m_TimeEventColor);
}

void CParticleInstance::UpdateGravity(float time, float elapsedTime)
{
	if (m_pParticleProperty->m_TimeEventGravity.empty())
		return;

	float fGravity;
	fGravity = GetTimeEventBlendValue(time, m_pParticleProperty->m_TimeEventGravity);

	m_v3Velocity.z -= fGravity * elapsedTime;
}

void CParticleInstance::UpdateAirResistance(float time, float elapsedTime)
{
	if (m_pParticleProperty->m_TimeEventAirResistance.empty())
		return;

	const float fFactor = 1.0f - GetTimeEventBlendValue(time, m_pParticleProperty->m_TimeEventAirResistance);
	const DirectX::XMVECTOR vVel = MathSIMD::Load3(&m_v3Velocity.x);
	const DirectX::XMVECTOR vScale = DirectX::XMVectorReplicate(fFactor);
	MathSIMD::Store3(&m_v3Velocity.x, DirectX::XMVectorMultiply(vVel, vScale));
}

void CParticleInstance::Transform(const D3DXMATRIX * c_matLocal)
{
	STATEMANAGER.SetRenderState(D3DRS_TEXTUREFACTOR, m_Color);

	static D3DXVECTOR3 s_v3Up(0.0f, 0.0f, 1.0f);
	static D3DXVECTOR3 s_v3Cross(1.0f, 0.0f, 0.0f);
	static D3DXVECTOR3 s_v3View(0.0f, 1.0f, 0.0f);
	static DWORD s_dwLastCacheTime = 0xFFFFFFFF;

	DWORD dwCurrentTime = ELTimer_GetMSec();
	if (dwCurrentTime != s_dwLastCacheTime)
	{
		s_dwLastCacheTime = dwCurrentTime;
		CCamera * pCurrentCamera = CCameraManager::InstancePtr() ? CCameraManager::Instance().GetCurrentCamera() : nullptr;
		if (pCurrentCamera)
		{
			s_v3Up = pCurrentCamera->GetUp();
			s_v3Cross = pCurrentCamera->GetCross();
			s_v3View = pCurrentCamera->GetView();
		}
	}

	const D3DXVECTOR3 v3Velocity = m_v3Position - m_v3LastPosition;
	D3DXVECTOR3 positions[4];

	BuildParticleQuad(
		m_pParticleProperty->m_byBillboardType,
		m_pParticleProperty->m_bStretchFlag != FALSE,
		m_v3Position,
		m_v2HalfSize.x * m_v2Scale.x,
		m_v2HalfSize.y * m_v2Scale.y,
		m_fRotation,
		s_v3Up,
		s_v3Cross,
		s_v3View,
		v3Velocity,
		positions,
		c_matLocal,
		m_pParticleProperty->m_bAttachFlag != FALSE,
		0.0f
	);

	for (int i = 0; i < 4; ++i)
	{
		m_ParticleMesh[i].position = positions[i];
	}
}


void CParticleInstance::Transform(const D3DXMATRIX * c_matLocal, const float c_fZRotation)
{
	STATEMANAGER.SetRenderState(D3DRS_TEXTUREFACTOR, m_Color);

	static D3DXVECTOR3 s_v3Up(0.0f, 0.0f, 1.0f);
	static D3DXVECTOR3 s_v3Cross(1.0f, 0.0f, 0.0f);
	static D3DXVECTOR3 s_v3View(0.0f, 1.0f, 0.0f);
	static DWORD s_dwLastCacheTime = 0xFFFFFFFF;

	DWORD dwCurrentTime = ELTimer_GetMSec();
	if (dwCurrentTime != s_dwLastCacheTime)
	{
		s_dwLastCacheTime = dwCurrentTime;
		CCamera * pCurrentCamera = CCameraManager::InstancePtr() ? CCameraManager::Instance().GetCurrentCamera() : nullptr;
		if (pCurrentCamera)
		{
			s_v3Up = pCurrentCamera->GetUp();
			s_v3Cross = pCurrentCamera->GetCross();
			s_v3View = pCurrentCamera->GetView();
		}
	}

	const D3DXVECTOR3 v3Velocity = m_v3Position - m_v3LastPosition;
	D3DXVECTOR3 positions[4];

	BuildParticleQuad(
		m_pParticleProperty->m_byBillboardType,
		m_pParticleProperty->m_bStretchFlag != FALSE,
		m_v3Position,
		m_v2HalfSize.x * m_v2Scale.x,
		m_v2HalfSize.y * m_v2Scale.y,
		m_fRotation,
		s_v3Up,
		s_v3Cross,
		s_v3View,
		v3Velocity,
		positions,
		c_matLocal,
		m_pParticleProperty->m_bAttachFlag != FALSE,
		c_fZRotation
	);

	for (int i = 0; i < 4; ++i)
	{
		m_ParticleMesh[i].position = positions[i];
	}
}

void CParticleInstance::Destroy()
{
	__Initialize();
}

void CParticleInstance::__Initialize()
{
	m_v3Position = D3DXVECTOR3(0.0f, 0.0f, 0.0f);
	m_v3LastPosition = m_v3Position;
	m_v3Velocity = D3DXVECTOR3(0.0f, 0.0f, 0.0f);

	m_v2Scale = D3DXVECTOR2(1.0f, 1.0f);
	m_Color = D3DXCOLOR(1.0f, 1.0f, 1.0f, 1.0f);

	m_byFrameIndex = 0;
	m_rotationType = CParticleProperty::ROTATION_TYPE_NONE;
	m_fFrameTime = 0;

	m_ParticleMesh[0].texCoord = D3DXVECTOR2(0.0f, 1.0f);
	m_ParticleMesh[1].texCoord = D3DXVECTOR2(0.0f, 0.0f);
	m_ParticleMesh[2].texCoord = D3DXVECTOR2(1.0f, 1.0f);
	m_ParticleMesh[3].texCoord = D3DXVECTOR2(1.0f, 0.0f);
}

CParticleInstance::CParticleInstance()
{
	__Initialize();
}

CParticleInstance::~CParticleInstance()
{
	Destroy();
}

TPTVertex * CParticleInstance::GetParticleMeshPointer()
{
	return m_ParticleMesh;
}
