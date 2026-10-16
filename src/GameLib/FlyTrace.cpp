#include "stdafx.h"
#include "EterLib/StateManager.h"
#include "EterLib/Camera.h"

#include "FlyingData.h"
#include "FlyTrace.h"
#include "EterLib/RenderQueueManager.h"
#include <algorithm>

CDynamicPool<CFlyTrace>		CFlyTrace::ms_kPool;		

void CFlyTrace::DestroySystem()
{
	ms_kPool.Destroy();
}

CFlyTrace* CFlyTrace::New()
{
	return ms_kPool.Alloc();
}

void CFlyTrace::Delete(CFlyTrace* pkInst)
{
	pkInst->Destroy();
	ms_kPool.Free(pkInst);
}

CFlyTrace::CFlyTrace()
{
	__Initialize();

	/*
	// Code for texture
	CGraphicImage * pImage = (CGraphicImage *)CResourceManager::Instance().GetResourcePointer("d:/ray.jpg");
	m_ImageInstance.SetImagePointer(pImage);
	
	CGraphicTexture * pTexture = m_ImageInstance.GetTexturePointer();
	m_lpTexture = pTexture->GetD3DTexture();
	*/
}

CFlyTrace::~CFlyTrace()
{
	Destroy();
}

				
void CFlyTrace::__Initialize()
{
	m_bRectShape=false;
	m_dwColor=0;
	m_fSize=0.0f;
	m_fTailLength=0.0f;	
}

void CFlyTrace::Destroy()
{
	m_TimePositionDeque.clear();

	__Initialize();
}

void CFlyTrace::UpdateNewPosition(const D3DXVECTOR3 & v3Position)
{
	m_TimePositionDeque.push_front(TTimePosition(CTimer::Instance().GetCurrentSecond(),v3Position));
	//Tracenf("%f %f",m_TimePositionDeque.back().first, CTimer::Instance().GetCurrentSecond());
	while(!m_TimePositionDeque.empty() && m_TimePositionDeque.back().first+m_fTailLength<CTimer::Instance().GetCurrentSecond())
	{
		m_TimePositionDeque.pop_back();
	}
}

void CFlyTrace::Create(const CFlyingData::TFlyingAttachData & rFlyingAttachData)
{
	//assert(rFlyingAttachData.bHasTail);
	m_dwColor = rFlyingAttachData.dwTailColor;
	m_fTailLength = rFlyingAttachData.fTailLength;
	m_fSize = rFlyingAttachData.fTailSize;
	m_bRectShape = rFlyingAttachData.bRectShape;
}


void CFlyTrace::Update()
{ 
	
}

//1. 알파를 쓰려면 색깔만 줄수있다.
//2. 텍스쳐를 쓰려면 알파 없다-_-


struct TFlyVertex
{
	D3DXVECTOR3 p;
	DWORD c;
	D3DXVECTOR2 t;
	TFlyVertex(){};
	TFlyVertex(const D3DXVECTOR3& p, DWORD c, const D3DXVECTOR2 & t):p(p),c(c),t(t){}
};

struct TFlyVertexSet
{
	TFlyVertex v[6];
	TFlyVertexSet(TFlyVertex * pv)
	{
		memcpy(v,pv,sizeof(v));
	}
	bool operator < (const TFlyVertexSet& ) const
	{
		return false;
	}
	TFlyVertexSet & operator = ( const TFlyVertexSet& rhs )
	{
		memcpy(v,rhs.v,sizeof(v));
		return *this;
	}
};

typedef std::vector<std::pair<float, TFlyVertexSet> > TFlyVertexSetVector;

void CFlyTrace::Render()
{
	if (m_TimePositionDeque.size() <= 1)
		return;

	const float fCurrentTime = CTimer::Instance().GetCurrentSecond();
	const size_t pointCount = m_TimePositionDeque.size();

	CRenderQueueManager& rqm = CRenderQueueManager::Instance();
	TParticleInstanceData* pParticles = rqm.AllocateParticleData(pointCount);
	if (!pParticles)
		return;

	for (size_t i = 0; i < pointCount; ++i)
	{
		const auto& tp = m_TimePositionDeque[i];
		const float fAge = fCurrentTime - tp.first;
		const float fLifeRatio = (m_fTailLength > 0.0f) ? std::clamp(fAge / m_fTailLength, 0.0f, 1.0f) : 0.0f;
		const float fAlphaFactor = 1.0f - fLifeRatio;

		pParticles[i].vWorldPos = D3DXVECTOR4(tp.second.x, tp.second.y, tp.second.z, m_fSize * (1.0f - 0.5f * fLifeRatio));

		D3DXCOLOR baseColor(m_dwColor);
		baseColor.a *= fAlphaFactor;
		pParticles[i].vColor = baseColor;
		pParticles[i].vTexCoordRect = D3DXVECTOR4(0.0f, 0.0f, 1.0f, 1.0f);
		pParticles[i].fRotation = 0.0f;
		pParticles[i].fLifeRatio = fLifeRatio;
		pParticles[i].fPad[0] = m_fSize;
		pParticles[i].fPad[1] = 0.0f;
	}

	TEffectInstancePacket packet{};
	packet.dwEffectID = 0x0000F7A1;
	packet.dwTextureID = 0;
	packet.dwBlendMode = EFFECT_BLEND_ADDITIVE;
	packet.dwParticleCount = static_cast<uint32_t>(pointCount);
	packet.pParticles = pParticles;

	rqm.SubmitEffect(packet);
}
