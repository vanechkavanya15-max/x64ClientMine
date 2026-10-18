#include "StdAfx.h"
#include "BlockTexture.h"
#include "GrpBase.h"
#include "GrpDib.h"
#include "Eterbase/Stl.h"
#include "Eterlib/StateManager.h"
#include "RenderConventions.h"
#include "Render/DynamicQuadBatcher.h"
#include "RenderOwnership.h"

void CBlockTexture::SetClipRect(const RECT & c_rRect)
{
	m_bClipEnable = TRUE;
	m_clipRect = c_rRect;
}

void CBlockTexture::Render(int ix, int iy)
{
	if (!m_lpd3dTexture)
		return;

	int isx = ix + m_rect.left;
	int isy = iy + m_rect.top;
	int iex = ix + m_rect.left + m_dwWidth;
	int iey = iy + m_rect.top + m_dwHeight;

	float su = 0.0f;
	float sv = 0.0f;
	float eu = 1.0f;
	float ev = 1.0f;

	if (m_bClipEnable)
	{
		if (isx > m_clipRect.right)
			return;
		if (iex < m_clipRect.left)
			return;

		if (isy > m_clipRect.bottom)
			return;
		if (iey < m_clipRect.top)
			return;

		if (m_clipRect.left > isx)
		{
			int idx = m_clipRect.left - isx;
			isx += idx;
			su += float(idx) / float(m_dwWidth);
		}
		if (iex > m_clipRect.right)
		{
			int idx = iex - m_clipRect.right;
			iex -= idx;
			eu -= float(idx) / float(m_dwWidth);
		}

		if (m_clipRect.top > isy)
		{
			int idy = m_clipRect.top - isy;
			isy += idy;
			sv += float(idy) / float(m_dwHeight);
		}
		if (iey > m_clipRect.bottom)
		{
			int idy = iey - m_clipRect.bottom;
			iey -= idy;
			ev -= float(idy) / float(m_dwHeight);
		}
	}

	TPDTVertex vertices[4];	
	vertices[0].position.x	= isx - kUiHalfPixelOffset;
	vertices[0].position.y	= isy - kUiHalfPixelOffset;
	vertices[0].position.z	= 0.0f;
	vertices[0].texCoord	= TTextureCoordinate(su, sv);
	vertices[0].diffuse		= 0xffffffff;

	vertices[1].position.x	= iex - kUiHalfPixelOffset;
	vertices[1].position.y	= isy - kUiHalfPixelOffset;
	vertices[1].position.z	= 0.0f;
	vertices[1].texCoord	= TTextureCoordinate(eu, sv);
	vertices[1].diffuse		= 0xffffffff;

	vertices[2].position.x	= isx - kUiHalfPixelOffset;
	vertices[2].position.y	= iey - kUiHalfPixelOffset;
	vertices[2].position.z	= 0.0f;
	vertices[2].texCoord	= TTextureCoordinate(su, ev);
	vertices[2].diffuse		= 0xffffffff;

	vertices[3].position.x	= iex - kUiHalfPixelOffset;
	vertices[3].position.y	= iey - kUiHalfPixelOffset;
	vertices[3].position.z	= 0.0f;
	vertices[3].texCoord	= TTextureCoordinate(eu, ev);	
	vertices[3].diffuse		= 0xffffffff;

	uintptr_t dwTexId = reinterpret_cast<uintptr_t>(m_lpd3dTexture);
	CDynamicQuadBatcher::Instance().AddQuad(
		static_cast<float>(isx), static_cast<float>(isy),
		static_cast<float>(iex - isx), static_cast<float>(iey - isy),
		0xFFFFFFFF, dwTexId, su, sv, eu, ev, 0);

	if (RenderOwnership::LegacyDraws(RenderOwnership::ERenderCategory::UI))
	{
		if (CGraphicBase::SetPDTStream(vertices, 4))
		{
			CGraphicBase::SetDefaultIndexBuffer(CGraphicBase::DEFAULT_IB_FILL_RECT);

			STATEMANAGER.SetTexture(0, m_lpd3dTexture);
			STATEMANAGER.SetTexture(1, NULL);
			STATEMANAGER.SetFVF(D3DFVF_XYZ|D3DFVF_TEX1|D3DFVF_DIFFUSE);

			STATEMANAGER.SaveRenderState(D3DRS_ALPHABLENDENABLE, TRUE);
			STATEMANAGER.SaveRenderState(D3DRS_SRCBLEND, D3DBLEND_SRCALPHA);
			STATEMANAGER.SaveRenderState(D3DRS_DESTBLEND, D3DBLEND_INVSRCALPHA);

			STATEMANAGER.DrawIndexedPrimitive(D3DPT_TRIANGLELIST, 0, 4, 0, 2);

			STATEMANAGER.RestoreRenderState(D3DRS_DESTBLEND);
			STATEMANAGER.RestoreRenderState(D3DRS_SRCBLEND);
			STATEMANAGER.RestoreRenderState(D3DRS_ALPHABLENDENABLE);
		}
	}
}

void CBlockTexture::InvalidateRect(const RECT & c_rsrcRect)
{
	if (!m_pDIB || !m_lpd3dTexture)
		return;

	void* pDibRaw = m_pDIB->GetPointer();
	if (!pDibRaw)
		return;

	RECT dstRect = m_rect;
	if (c_rsrcRect.right < dstRect.left ||
		c_rsrcRect.left > dstRect.right ||
		c_rsrcRect.bottom < dstRect.top ||
		c_rsrcRect.top > dstRect.bottom)
	{
		Tracef("InvalidateRect() - Strange rect");
		return;
	}


	// DIBBAR_LONGSIZE_BUGFIX
	const RECT clipRect = { 				
		std::max(c_rsrcRect.left - dstRect.left, 0l),
		std::max(c_rsrcRect.top - dstRect.top, 0l),
		std::min(c_rsrcRect.right - dstRect.left, dstRect.right - dstRect.left),
		std::min(c_rsrcRect.bottom - dstRect.top, dstRect.bottom - dstRect.top),
	};
	// END_OF_DIBBAR_LONGSIZE_BUGFIX


	DWORD * pdwSrc;
	pdwSrc = (DWORD *)pDibRaw;
	pdwSrc += (dstRect.left + clipRect.left) + (dstRect.top + clipRect.top)*m_pDIB->GetWidth();

	D3DLOCKED_RECT lockedRect;
	if (FAILED(m_lpd3dTexture->LockRect(0, &lockedRect, &clipRect, 0)))
	{
		Tracef("InvalidateRect() - Failed to LockRect");
		return;
	}

	int iclipWidth = clipRect.right - clipRect.left;
	int iclipHeight = clipRect.bottom - clipRect.top;
	DWORD * pdwDst = (DWORD *)lockedRect.pBits;
	DWORD dwDstWidth = lockedRect.Pitch>>2;
	DWORD dwSrcWidth = m_pDIB->GetWidth();
	for (int y = 0; y < iclipHeight; ++y)
	{
		for (int x = 0; x < iclipWidth; ++x)
		{
			pdwDst[x] = pdwSrc[x];
		}
		pdwDst += dwDstWidth;
		pdwSrc += dwSrcWidth;
	}

	m_lpd3dTexture->UnlockRect(0);
}

bool CBlockTexture::Create(CGraphicDib * pDIB, const RECT & c_rRect, DWORD dwWidth, DWORD dwHeight)
{	
	if (!ms_lpd3dDevice)
		return false;

	if (FAILED(ms_lpd3dDevice->CreateTexture(dwWidth, dwHeight, 0, D3DUSAGE_DYNAMIC, D3DFMT_A8R8G8B8, D3DPOOL_DEFAULT, &m_lpd3dTexture, nullptr)))
	{
		Tracef("Failed to create block texture %u, %u\n", dwWidth, dwHeight);
		return false;
	}

	m_pDIB = pDIB;
	m_rect = c_rRect;
	m_dwWidth = dwWidth;
	m_dwHeight = dwHeight;
	m_bClipEnable = FALSE;

	return true;
}

void CBlockTexture::Reset()
{
	safe_release(m_lpd3dTexture);
	m_lpd3dTexture = NULL;
}

void CBlockTexture::DestroyDeviceObjects()
{
	Reset();
}

bool CBlockTexture::CreateDeviceObjects()
{
	if (!m_pDIB || m_dwWidth == 0 || m_dwHeight == 0)
		return true;
	if (m_lpd3dTexture)
		return true;
	if (!ms_lpd3dDevice)
		return false;

	if (FAILED(ms_lpd3dDevice->CreateTexture(m_dwWidth, m_dwHeight, 0, D3DUSAGE_DYNAMIC, D3DFMT_A8R8G8B8, D3DPOOL_DEFAULT, &m_lpd3dTexture, nullptr)))
	{
		Tracef("Failed to recreate block texture %u, %u\n", m_dwWidth, m_dwHeight);
		return false;
	}

	InvalidateRect(m_rect);
	return true;
}

CBlockTexture::CBlockTexture()
{
	m_pDIB = NULL;
	m_lpd3dTexture = NULL;
}

CBlockTexture::~CBlockTexture()
{
	Reset();
}
