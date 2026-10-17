#include "StdAfx.h"
#include "EterBase/Stl.h"
#include "GrpTexture.h"
#include "StateManager.h"
#include "TextureUploadBridge.h"

#if defined(_M_IX86) || defined(_M_X64)
#include <emmintrin.h> // SSE2
#include <tmmintrin.h> // SSSE3 (for _mm_shuffle_epi8)
#endif

void SwizzleRgbaToBgra(void* dst, size_t dstPitch, const void* src, size_t width, size_t height, size_t srcPitch)
{
	if (!dst || !src || width == 0 || height == 0)
		return;

	if (srcPitch == 0)
		srcPitch = width * 4;

	if (dstPitch < width * 4 || srcPitch < width * 4)
		return;

	const uint8_t* srcBytes = static_cast<const uint8_t*>(src);
	uint8_t* dstBytes = static_cast<uint8_t*>(dst);

#if defined(_M_IX86) || defined(_M_X64)
	const __m128i shuffle_mask = _mm_setr_epi8(2, 1, 0, 3, 6, 5, 4, 7, 10, 9, 8, 11, 14, 13, 12, 15);
	const size_t simdPixels = width & ~static_cast<size_t>(3);
#endif

	for (size_t y = 0; y < height; ++y)
	{
		const uint8_t* rowSrc = srcBytes + y * srcPitch;
		uint8_t* rowDst = dstBytes + y * dstPitch;
		size_t x = 0;

#if defined(_M_IX86) || defined(_M_X64)
		for (; x < simdPixels; x += 4)
		{
			__m128i px = _mm_loadu_si128(reinterpret_cast<const __m128i*>(rowSrc + x * 4));
			px = _mm_shuffle_epi8(px, shuffle_mask);
			_mm_storeu_si128(reinterpret_cast<__m128i*>(rowDst + x * 4), px);
		}
#endif

		for (; x < width; ++x)
		{
			const size_t idx = x * 4;
			rowDst[idx + 0] = rowSrc[idx + 2];
			rowDst[idx + 1] = rowSrc[idx + 1];
			rowDst[idx + 2] = rowSrc[idx + 0];
			rowDst[idx + 3] = rowSrc[idx + 3];
		}
	}
}

bool CreateTextureFromRGBA(
	LPDIRECT3DDEVICE9 pDevice,
	UINT width,
	UINT height,
	D3DFORMAT format,
	const void* pSrcRGBA,
	size_t srcPitch,
	LPDIRECT3DTEXTURE9* ppDestTexture)
{
	if (!pDevice || !ppDestTexture || !pSrcRGBA || width == 0 || height == 0)
		return false;

	*ppDestTexture = nullptr;

	LPDIRECT3DTEXTURE9 pDestTexture = nullptr;
	HRESULT hr = pDevice->CreateTexture(width, height, 1, 0, format, D3DPOOL_DEFAULT, &pDestTexture, nullptr);
	if (FAILED(hr))
	{
		TraceError("CreateTexture (DEFAULT) failed: 0x%08x (%ux%u)", hr, width, height);
		return false;
	}

	LPDIRECT3DTEXTURE9 pStagingTexture = nullptr;
	hr = pDevice->CreateTexture(width, height, 1, 0, format, D3DPOOL_SYSTEMMEM, &pStagingTexture, nullptr);
	if (FAILED(hr))
	{
		pDestTexture->Release();
		TraceError("CreateTexture (SYSTEMMEM staging) failed: 0x%08x (%ux%u)", hr, width, height);
		return false;
	}

	D3DLOCKED_RECT rect;
	hr = pStagingTexture->LockRect(0, &rect, nullptr, 0);
	if (FAILED(hr))
	{
		pStagingTexture->Release();
		pDestTexture->Release();
		TraceError("LockRect on staging texture failed: 0x%08x", hr);
		return false;
	}

	SwizzleRgbaToBgra(rect.pBits, static_cast<size_t>(rect.Pitch), pSrcRGBA, width, height, srcPitch);

	pStagingTexture->UnlockRect(0);

	hr = pDevice->UpdateTexture(pStagingTexture, pDestTexture);
	pStagingTexture->Release();
	if (FAILED(hr))
	{
		pDestTexture->Release();
		TraceError("UpdateTexture failed: 0x%08x", hr);
		return false;
	}

	*ppDestTexture = pDestTexture;
	return true;
}

void CGraphicTexture::DestroyDeviceObjects()
{
	if (m_lpd3dTexture)
	{
		if (m_bOwnsTexture)
		{
			CTextureUploadBridge::Instance().UnregisterTexture(reinterpret_cast<uintptr_t>(m_lpd3dTexture));
			safe_release(m_lpd3dTexture);
		}
		else
		{
			m_lpd3dTexture = NULL;
		}
	}
}

void CGraphicTexture::Destroy()
{
	DestroyDeviceObjects();

	Initialize();
}

void CGraphicTexture::Initialize()
{
	m_lpd3dTexture = NULL;
	m_width = 0;
	m_height = 0;
	m_bEmpty = true;
	m_bOwnsTexture = true;
}

bool CGraphicTexture::IsEmpty() const
{
	return m_bEmpty;
}

void CGraphicTexture::SetTextureStage(int stage) const
{
	assert(ms_lpd3dDevice != NULL);
	STATEMANAGER.SetTexture(stage, m_lpd3dTexture);	
}

LPDIRECT3DTEXTURE9 CGraphicTexture::GetD3DTexture() const
{
	return m_lpd3dTexture;
}

int CGraphicTexture::GetWidth() const
{
	return m_width;
}

int CGraphicTexture::GetHeight() const
{
	return m_height;
}

CGraphicTexture::CGraphicTexture()
{
	Initialize();
}

CGraphicTexture::~CGraphicTexture()	
{
	DestroyDeviceObjects();
}
