#include "StdAfx.h"
#include "SnowParticle.h"
#include <cmath>

const float c_fSnowDistance = 70000.0f;

std::vector<CSnowParticle*> CSnowParticle::ms_kVct_SnowParticlePool;

void CSnowParticle::SetCameraVertex(const D3DXVECTOR3 & rv3Up, const D3DXVECTOR3 & rv3Cross)
{
	m_v3Up = rv3Up * m_fHalfWidth;
	m_v3Cross = rv3Cross * m_fHalfHeight;
}

bool CSnowParticle::IsActivate() const
{
	return m_bActivate;
}

void CSnowParticle::Update(float fElapsedTime, const D3DXVECTOR3 & c_rv3Pos)
{
	m_v3Position += m_v3Velocity * fElapsedTime;

	const float fCrossNormX = (m_fHalfHeight > 0.0f) ? (m_v3Cross.x / m_fHalfHeight) : 1.0f;
	const float fCrossNormY = (m_fHalfHeight > 0.0f) ? (m_v3Cross.y / m_fHalfHeight) : 0.0f;
	m_v3Position.x += fCrossNormX * std::sin(m_fcurRadian) * m_fAmplitude * 6.0f * fElapsedTime;
	m_v3Position.y += fCrossNormY * std::sin(m_fcurRadian) * m_fAmplitude * 6.0f * fElapsedTime;
	m_fcurRadian += m_fPeriod * fElapsedTime;

	constexpr float c_fTwoPi = 6.28318530717958647692f;
	if (m_fcurRadian > c_fTwoPi)
	{
		m_fcurRadian = std::fmod(m_fcurRadian, c_fTwoPi);
	}
	else if (m_fcurRadian < -c_fTwoPi)
	{
		m_fcurRadian = std::fmod(m_fcurRadian, c_fTwoPi);
	}

	constexpr float c_fMaxSnowRadius = c_fSnowDistance / 10.0f + 500.0f;
	if (m_v3Position.z < c_rv3Pos.z - 500.0f)
		m_bActivate = false;
	else if (std::abs(m_v3Position.x - c_rv3Pos.x) > c_fMaxSnowRadius)
		m_bActivate = false;
	else if (std::abs(m_v3Position.y - c_rv3Pos.y) > c_fMaxSnowRadius)
		m_bActivate = false;
}

void CSnowParticle::GetVerticies(SParticleVertex & rv3Vertex1, SParticleVertex & rv3Vertex2,
								 SParticleVertex & rv3Vertex3, SParticleVertex & rv3Vertex4)
{
	rv3Vertex1.v3Pos = m_v3Position - m_v3Cross - m_v3Up;
	rv3Vertex1.u = 0.0f;
	rv3Vertex1.v = 0.0f;

	rv3Vertex2.v3Pos = m_v3Position + m_v3Cross - m_v3Up;
	rv3Vertex2.u = 1.0f;
	rv3Vertex2.v = 0.0f;

	rv3Vertex3.v3Pos = m_v3Position - m_v3Cross + m_v3Up;
	rv3Vertex3.u = 0.0f;
	rv3Vertex3.v = 1.0f;

	rv3Vertex4.v3Pos = m_v3Position + m_v3Cross + m_v3Up;
	rv3Vertex4.u = 1.0f;
	rv3Vertex4.v = 1.0f;
}

void CSnowParticle::Init(const D3DXVECTOR3 & c_rv3Pos)
{
	float fRot = frandom(0.0f, 36000.0f) / 100.0f;
	float fDistance = frandom(0.0f, c_fSnowDistance) / 10.0f;

	m_v3Position.x = c_rv3Pos.x + fDistance * std::sin(static_cast<double>(D3DXToRadian(fRot)));
	m_v3Position.y = c_rv3Pos.y + fDistance * std::cos(static_cast<double>(D3DXToRadian(fRot)));
	m_v3Position.z = c_rv3Pos.z + frandom(1500.0f, 2000.0f);
	m_v3Velocity.x = 0.0f;
	m_v3Velocity.y = 0.0f;
	m_v3Velocity.z = frandom(-50.0f, -200.0f);
	m_fHalfWidth = frandom(2.0f, 7.0f);
	m_fHalfHeight = m_fHalfWidth;
	m_bActivate = true;
	m_bChangedSize = false;

	m_fPeriod = frandom(1.5f, 5.0f);
	m_fcurRadian = frandom(-1.6f, 1.6f);
	m_fAmplitude = frandom(1.0f, 3.0f);
}

void CSnowParticle::InitializePool(size_t poolSize)
{
	DestroyPool();
	ms_kVct_SnowParticlePool.reserve(poolSize);
	for (size_t i = 0; i < poolSize; ++i)
	{
		ms_kVct_SnowParticlePool.push_back(new CSnowParticle());
	}
}

CSnowParticle * CSnowParticle::New()
{
	if (ms_kVct_SnowParticlePool.empty())
	{
		return new CSnowParticle();
	}

	CSnowParticle * pParticle = ms_kVct_SnowParticlePool.back();
	ms_kVct_SnowParticlePool.pop_back();
	return pParticle;
}

void CSnowParticle::Delete(CSnowParticle * pSnowParticle)
{
	if (pSnowParticle)
	{
		ms_kVct_SnowParticlePool.push_back(pSnowParticle);
	}
}

void CSnowParticle::DestroyPool()
{
	stl_wipe(ms_kVct_SnowParticlePool);
}

CSnowParticle::CSnowParticle()
	: m_bActivate(false)
	, m_bChangedSize(false)
	, m_fHalfWidth(0.0f)
	, m_fHalfHeight(0.0f)
	, m_v3Velocity(0.0f, 0.0f, 0.0f)
	, m_v3Position(0.0f, 0.0f, 0.0f)
	, m_v3Up(0.0f, 0.0f, 0.0f)
	, m_v3Cross(0.0f, 0.0f, 0.0f)
	, m_fPeriod(0.0f)
	, m_fcurRadian(0.0f)
	, m_fAmplitude(0.0f)
{
}

CSnowParticle::~CSnowParticle()
{
}
