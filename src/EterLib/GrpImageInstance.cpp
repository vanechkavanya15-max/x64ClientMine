#include "StdAfx.h"
#include "GrpImageInstance.h"
#include "StateManager.h"
#include "Render/DynamicQuadBatcher.h"
#include "RenderConventions.h"

#include "EterBase/CRC32.h"
//STATEMANAGER.SaveRenderState(D3DRS_SRCBLEND, D3DBLEND_INVDESTCOLOR);
//STATEMANAGER.SaveRenderState(D3DRS_DESTBLEND, D3DBLEND_ONE);
//STATEMANAGER.RestoreRenderState(D3DRS_SRCBLEND);
//STATEMANAGER.RestoreRenderState(D3DRS_DESTBLEND);

CDynamicPool<CGraphicImageInstance>		CGraphicImageInstance::ms_kPool;

void CGraphicImageInstance::CreateSystem(UINT uCapacity)
{
	ms_kPool.Create(uCapacity);
}

void CGraphicImageInstance::DestroySystem()
{
	ms_kPool.Destroy();
}

CGraphicImageInstance* CGraphicImageInstance::New()
{
	return ms_kPool.Alloc();
}

void CGraphicImageInstance::Delete(CGraphicImageInstance* pkImgInst)
{
	pkImgInst->Destroy();
	ms_kPool.Free(pkImgInst);
}

void CGraphicImageInstance::Render()
{
	if (IsEmpty())
		return;

	assert(!IsEmpty());

	OnRender();
}

void CGraphicImageInstance::OnRender()
{
	CGraphicImage * pImage = m_roImage.GetPointer();
	if (!pImage)
		return;

	CGraphicTexture * pTexture = pImage->GetTexturePointer();
	if (!pTexture)
		return;

	int iTexWidth = pTexture->GetWidth();
	int iTexHeight = pTexture->GetHeight();
	if (iTexWidth <= 0 || iTexHeight <= 0)
		return;

	float fimgWidth = pImage->GetWidth();
	float fimgHeight = pImage->GetHeight();

	const RECT& c_rRect = pImage->GetRectReference();
	float texReverseWidth = 1.0f / float(iTexWidth);
	float texReverseHeight = 1.0f / float(iTexHeight);
	float su = c_rRect.left * texReverseWidth;
	float sv = c_rRect.top * texReverseHeight;
	float eu = (c_rRect.left + (c_rRect.right-c_rRect.left)) * texReverseWidth;
	float ev = (c_rRect.top + (c_rRect.bottom-c_rRect.top)) * texReverseHeight;
	
	
	const float snappedX = std::floor(m_v2Position.x + 0.5f) - kUiHalfPixelOffset;
	const float snappedY = std::floor(m_v2Position.y + 0.5f) - kUiHalfPixelOffset;
	const uint32_t packedColor = static_cast<uint32_t>(static_cast<DWORD>(m_DiffuseColor));

	TUIVertex uiVertices[4] = {
		{ D3DXVECTOR3(snappedX,             snappedY,              0.0f), packedColor, TTextureCoordinate(su, sv) },
		{ D3DXVECTOR3(snappedX + fimgWidth, snappedY,              0.0f), packedColor, TTextureCoordinate(eu, sv) },
		{ D3DXVECTOR3(snappedX,             snappedY + fimgHeight, 0.0f), packedColor, TTextureCoordinate(su, ev) },
		{ D3DXVECTOR3(snappedX + fimgWidth, snappedY + fimgHeight, 0.0f), packedColor, TTextureCoordinate(eu, ev) }
	};

	EterLib::Render::DynamicQuadBatcher::Instance().AddQuadVertices(
		uiVertices,
		reinterpret_cast<uintptr_t>(pTexture->GetD3DTexture()),
		0
	);
}

const CGraphicTexture & CGraphicImageInstance::GetTextureReference() const
{
	return m_roImage->GetTextureReference();
}

CGraphicTexture * CGraphicImageInstance::GetTexturePointer() const
{
	CGraphicImage* pkImage = m_roImage.GetPointer();
	return pkImage ? pkImage->GetTexturePointer() : NULL;
}

CGraphicImage * CGraphicImageInstance::GetGraphicImagePointer()
{
	return m_roImage.GetPointer();
}

int CGraphicImageInstance::GetWidth()
{
	if (IsEmpty())
		return 0;
	
	return m_roImage->GetWidth();
}

int CGraphicImageInstance::GetHeight()
{
	if (IsEmpty())
		return 0;
	
	return m_roImage->GetHeight();
}

void CGraphicImageInstance::SetDiffuseColor(float fr, float fg, float fb, float fa)
{
	m_DiffuseColor.r = fr;
	m_DiffuseColor.g = fg;
	m_DiffuseColor.b = fb;
	m_DiffuseColor.a = fa;
}
void CGraphicImageInstance::SetPosition(float fx, float fy)
{
	m_v2Position.x = fx;
	m_v2Position.y = fy;
}

void CGraphicImageInstance::SetImagePointer(CGraphicImage * pImage)
{
	m_roImage.SetPointer(pImage);

	OnSetImagePointer();
}

void CGraphicImageInstance::ReloadImagePointer(CGraphicImage * pImage)
{
	if (m_roImage.IsNull())
	{
		SetImagePointer(pImage);
		return;
	}

	CGraphicImage * pkImage = m_roImage.GetPointer();

	if (pkImage)
		pkImage->Reload();
}

bool CGraphicImageInstance::IsEmpty() const
{
	if (!m_roImage.IsNull() && !m_roImage->IsEmpty())
		return false;

	return true;
}

bool CGraphicImageInstance::operator == (const CGraphicImageInstance & rhs) const
{
	return (m_roImage.GetPointer() == rhs.m_roImage.GetPointer());
}

DWORD CGraphicImageInstance::Type()
{
	static DWORD s_dwType = GetCRC32("CGraphicImageInstance", strlen("CGraphicImageInstance"));
	return (s_dwType);
}

BOOL CGraphicImageInstance::IsType(DWORD dwType)
{
	return OnIsType(dwType);
}

BOOL CGraphicImageInstance::OnIsType(DWORD dwType)
{
	if (CGraphicImageInstance::Type() == dwType)
		return TRUE;

	return FALSE;
}

void CGraphicImageInstance::OnSetImagePointer()
{
}

void CGraphicImageInstance::Initialize()
{
	m_DiffuseColor.r = m_DiffuseColor.g = m_DiffuseColor.b = m_DiffuseColor.a = 1.0f;
	m_v2Position.x = m_v2Position.y = 0.0f;
}

void CGraphicImageInstance::Destroy()
{
	m_roImage.SetPointer(NULL); // CRef 에서 레퍼런스 카운트가 떨어져야 함.
	Initialize();
}

CGraphicImageInstance::CGraphicImageInstance()
{
	Initialize();
}

CGraphicImageInstance::~CGraphicImageInstance()
{
	Destroy();
}
