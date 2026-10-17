#pragma once

#include "GrpBase.h"

// Unified RGBA -> BGRA swizzle function with row pitch support
void SwizzleRgbaToBgra(void* dst, size_t dstPitch, const void* src, size_t width, size_t height, size_t srcPitch = 0);

// Helper for D3D9Ex staging texture creation (D3DPOOL_DEFAULT + D3DPOOL_SYSTEMMEM staging + UpdateTexture)
bool CreateTextureFromRGBA(
	LPDIRECT3DDEVICE9 pDevice,
	UINT width,
	UINT height,
	D3DFORMAT format,
	const void* pSrcRGBA,
	size_t srcPitch,
	LPDIRECT3DTEXTURE9* ppDestTexture);

class CGraphicTexture : public CGraphicBase
{
	public:
		static void SwizzleRgbaToBgra(void* dst, size_t dstPitch, const void* src, size_t width, size_t height, size_t srcPitch = 0)
		{
			::SwizzleRgbaToBgra(dst, dstPitch, src, width, height, srcPitch);
		}

		static bool CreateTextureFromRGBA(
			LPDIRECT3DDEVICE9 pDevice,
			UINT width,
			UINT height,
			D3DFORMAT format,
			const void* pSrcRGBA,
			size_t srcPitch,
			LPDIRECT3DTEXTURE9* ppDestTexture)
		{
			return ::CreateTextureFromRGBA(pDevice, width, height, format, pSrcRGBA, srcPitch, ppDestTexture);
		}

		virtual bool IsEmpty() const;

		int GetWidth() const;
		int GetHeight() const;

		void SetTextureStage(int stage) const;
		LPDIRECT3DTEXTURE9 GetD3DTexture() const;
		uint32_t GetTextureHandle() const;

		void DestroyDeviceObjects();

		void SetOwnsTexture(bool bOwns) { m_bOwnsTexture = bOwns; }
		bool DoesOwnTexture() const { return m_bOwnsTexture; }
		
	protected:
		CGraphicTexture();
		virtual	~CGraphicTexture();

		void Destroy();
		void Initialize();

	protected:
		bool m_bEmpty;
		bool m_bOwnsTexture;

		int m_width;
		int m_height;

		LPDIRECT3DTEXTURE9 m_lpd3dTexture;
};
