#include "StdAfx.h"
#include "GrpFontTexture.h"
#include "FontManager.h"
#include "StateManager.h"
#include "Util.h"
#include "EterBase/Stl.h"

#include <cmath>
#include <algorithm>

CGraphicFontTexture::CGraphicFontTexture()
{
	Initialize();
}

CGraphicFontTexture::~CGraphicFontTexture()
{
	Destroy();
}

void CGraphicFontTexture::Initialize()
{
	CGraphicTexture::Initialize();

	m_bOwnsTexture = false;
	m_ftFace = nullptr;
	m_pAtlasBuffer = nullptr;
	m_atlasWidth = 0;
	m_atlasHeight = 0;
	m_isDirty = false;
	m_bItalic = false;
	m_ascender = 0;
	m_lineHeight = 0;
	m_hasKerning = false;
	m_x = 0;
	m_y = 0;
	m_step = 0;
	m_fontSize = 0;
	m_atlasFormat = EFontAtlasFormat::A8R8G8B8;
	m_activePageIndex = 0;
	m_currentLruTick = 0;
	m_pageLastUsed.clear();
	SetRectEmpty(&m_dirtyRect);
}

void CGraphicFontTexture::Destroy()
{
	if (m_pAtlasBuffer)
	{
		delete[] reinterpret_cast<uint8_t*>(m_pAtlasBuffer);
		m_pAtlasBuffer = nullptr;
	}

	CGraphicTexture::Destroy();
	stl_wipe(m_pFontTextureVector);
	m_charInfoMap.clear();
	m_pageLastUsed.clear();

	if (m_ftFace)
	{
		FT_Done_Face(m_ftFace);
		m_ftFace = nullptr;
	}

	Initialize();
}

bool CGraphicFontTexture::CreateDeviceObjects()
{
	if (!m_ftFace)
		return true;

	std::vector<TCharacterKey> cachedKeys;
	cachedKeys.reserve(m_charInfoMap.size());
	for (const auto& pair : m_charInfoMap)
		cachedKeys.push_back(pair.first);

	stl_wipe(m_pFontTextureVector);
	m_pFontTextureVector.reserve(4);
	m_pageLastUsed.clear();
	m_charInfoMap.clear();
	m_x = 0;
	m_y = 0;
	m_step = 0;
	m_isDirty = false;
	m_activePageIndex = 0;
	m_currentLruTick = 0;
	SetRectEmpty(&m_dirtyRect);

	if (m_pAtlasBuffer)
	{
		const size_t bufferBytes = (m_atlasFormat == EFontAtlasFormat::A8)
			? (m_atlasWidth * m_atlasHeight * sizeof(uint8_t))
			: (m_atlasWidth * m_atlasHeight * sizeof(DWORD));
		memset(m_pAtlasBuffer, 0, bufferBytes);
	}

	if (!AppendTexture())
		return false;

	for (TCharacterKey key : cachedKeys)
		UpdateCharacterInfomation(key);

	UpdateTexture();
	return true;
}

void CGraphicFontTexture::DestroyDeviceObjects()
{
	CGraphicTexture::DestroyDeviceObjects();
	stl_wipe(m_pFontTextureVector);
	m_pageLastUsed.clear();
}

bool CGraphicFontTexture::Create(const char* c_szFontName, int fontSize, bool bItalic)
{
	Destroy();

	m_pFontTextureVector.reserve(4);
	m_pageLastUsed.reserve(MAX_FONT_TEXTURE_PAGES);

	m_fontSize = fontSize;
	m_bItalic = bItalic;
	m_activePageIndex = 0;
	m_currentLruTick = 0;

	m_x = 0;
	m_y = 0;
	m_step = 0;

	DWORD width = 256, height = 256;
	if (GetMaxTextureWidth() > 512)
		width = 512;
	if (GetMaxTextureHeight() > 512)
		height = 512;

	m_atlasWidth = width;
	m_atlasHeight = height;

	const size_t bufferBytes = (m_atlasFormat == EFontAtlasFormat::A8)
		? (width * height * sizeof(uint8_t))
		: (width * height * sizeof(DWORD));

	m_pAtlasBuffer = reinterpret_cast<DWORD*>(new uint8_t[bufferBytes]);
	memset(m_pAtlasBuffer, 0, bufferBytes);

	m_ftFace = CFontManager::Instance().CreateFace(c_szFontName);
	if (!m_ftFace)
	{
		TraceError("CGraphicFontTexture::Create - Failed to create face for '%s'", c_szFontName ? c_szFontName : "(null)");
		delete[] reinterpret_cast<uint8_t*>(m_pAtlasBuffer);
		m_pAtlasBuffer = nullptr;
		return false;
	}

	int pixelSize = (fontSize < 0) ? -fontSize : fontSize;
	if (pixelSize == 0)
		pixelSize = 12;

	FT_Set_Pixel_Sizes(m_ftFace, 0, pixelSize);

	m_hasKerning = FT_HAS_KERNING(m_ftFace) != 0;

	if (bItalic)
	{
		FT_Matrix matrix;
		matrix.xx = 0x10000L;
		matrix.xy = 0x5800L;
		matrix.yx = 0;
		matrix.yy = 0x10000L;
		FT_Set_Transform(m_ftFace, &matrix, NULL);
	}
	else
	{
		FT_Set_Transform(m_ftFace, NULL, NULL);
	}

	m_ascender = (int)(m_ftFace->size->metrics.ascender >> 6);
	m_lineHeight = (int)(m_ftFace->size->metrics.height >> 6);

	if (!AppendTexture())
	{
		Destroy();
		return false;
	}

	return true;
}

bool CGraphicFontTexture::AppendTexture()
{
	// Bounded atlas growth & LRU page recycling
	if (m_pFontTextureVector.size() >= MAX_FONT_TEXTURE_PAGES)
	{
		size_t lruIndex = 0;
		uint64_t minTick = UINT64_MAX;
		for (size_t p = 0; p < m_pageLastUsed.size(); ++p)
		{
			if (m_pageLastUsed[p] < minTick)
			{
				minTick = m_pageLastUsed[p];
				lruIndex = p;
			}
		}

		for (auto it = m_charInfoMap.begin(); it != m_charInfoMap.end(); )
		{
			if (it->second.index == static_cast<short>(lruIndex))
				it = m_charInfoMap.erase(it);
			else
				++it;
		}

		m_activePageIndex = lruIndex;
		m_pageLastUsed[lruIndex] = ++m_currentLruTick;

		const size_t bufferBytes = (m_atlasFormat == EFontAtlasFormat::A8)
			? (m_atlasWidth * m_atlasHeight * sizeof(uint8_t))
			: (m_atlasWidth * m_atlasHeight * sizeof(DWORD));
		memset(m_pAtlasBuffer, 0, bufferBytes);

		m_x = 0;
		m_y = 0;
		m_step = 0;
		m_isDirty = false;
		SetRectEmpty(&m_dirtyRect);
		return true;
	}

	CGraphicImageTexture* pNewTexture = new CGraphicImageTexture;
	const D3DFORMAT d3dFmt = (m_atlasFormat == EFontAtlasFormat::A8) ? D3DFMT_A8 : D3DFMT_A8R8G8B8;

	if (!pNewTexture->Create(m_atlasWidth, m_atlasHeight, d3dFmt))
	{
		delete pNewTexture;
		return false;
	}

	m_activePageIndex = m_pFontTextureVector.size();
	m_pFontTextureVector.push_back(pNewTexture);
	m_pageLastUsed.push_back(++m_currentLruTick);
	return true;
}

bool CGraphicFontTexture::UpdateTexture()
{
	if (!m_isDirty)
		return true;

	if (!ms_lpd3dDevice || m_pFontTextureVector.empty())
	{
		m_isDirty = false;
		SetRectEmpty(&m_dirtyRect);
		return true;
	}

	m_isDirty = false;

	if (m_activePageIndex >= m_pFontTextureVector.size())
		m_activePageIndex = m_pFontTextureVector.size() - 1;

	CGraphicImageTexture* pFontTexture = m_pFontTextureVector[m_activePageIndex];
	if (!pFontTexture)
		return false;

	void* pDst = nullptr;
	int pitch = 0;

	if (!pFontTexture->Lock(&pitch, &pDst))
		return false;

	if (m_atlasFormat == EFontAtlasFormat::A8)
	{
		uint8_t* pDst8 = reinterpret_cast<uint8_t*>(pDst);
		const uint8_t* pSrc8 = reinterpret_cast<const uint8_t*>(m_pAtlasBuffer);

		for (int y = 0; y < m_atlasHeight; ++y, pDst8 += pitch, pSrc8 += m_atlasWidth)
		{
			memcpy(pDst8, pSrc8, m_atlasWidth * sizeof(uint8_t));
		}
	}
	else
	{
		DWORD* pdwDst = reinterpret_cast<DWORD*>(pDst);
		const DWORD* pdwSrc = m_pAtlasBuffer;
		const int pitchDwords = pitch / sizeof(DWORD);

		for (int y = 0; y < m_atlasHeight; ++y, pdwDst += pitchDwords, pdwSrc += m_atlasWidth)
		{
			memcpy(pdwDst, pdwSrc, m_atlasWidth * sizeof(DWORD));
		}
	}

	pFontTexture->Unlock();
	SetRectEmpty(&m_dirtyRect);
	return true;
}

float CGraphicFontTexture::GetKerning(wchar_t prev, wchar_t cur)
{
	if (!m_hasKerning || !m_ftFace || prev == 0)
		return 0.0f;

	FT_UInt prevIndex = FT_Get_Char_Index(m_ftFace, prev);
	FT_UInt curIndex = FT_Get_Char_Index(m_ftFace, cur);

	if (prevIndex == 0 || curIndex == 0)
		return 0.0f;

	FT_Vector delta;
	if (FT_Get_Kerning(m_ftFace, prevIndex, curIndex, FT_KERNING_DEFAULT, &delta) != 0)
		return 0.0f;

	return (float)(delta.x) / 64.0f;
}

CGraphicFontTexture::TCharacterInfomation* CGraphicFontTexture::GetCharacterInfomation(wchar_t keyValue)
{
	TCharacterKey code = keyValue;
	TCharacterInfomationMap::iterator f = m_charInfoMap.find(code);

	if (m_charInfoMap.end() == f)
	{
		return UpdateCharacterInfomation(code);
	}

	if (f->second.index >= 0 && static_cast<size_t>(f->second.index) < m_pageLastUsed.size())
	{
		m_pageLastUsed[f->second.index] = ++m_currentLruTick;
	}

	return &f->second;
}

bool CGraphicFontTexture::CheckTextureIndex(DWORD dwTexture)
{
	return dwTexture < m_pFontTextureVector.size();
}

void CGraphicFontTexture::SelectTexture(DWORD dwTexture)
{
	if (dwTexture >= m_pFontTextureVector.size())
		return;

	m_lpd3dTexture = m_pFontTextureVector[dwTexture]->GetD3DTexture();

	if (dwTexture < m_pageLastUsed.size())
		m_pageLastUsed[dwTexture] = ++m_currentLruTick;
}

CGraphicFontTexture::TCharacterInfomation* CGraphicFontTexture::UpdateCharacterInfomation(TCharacterKey keyValue)
{
	if (!m_ftFace)
		return NULL;

	FT_UInt glyphIndex = FT_Get_Char_Index(m_ftFace, keyValue);
	if (glyphIndex == 0 && keyValue != ' ')
	{
		glyphIndex = FT_Get_Char_Index(m_ftFace, '?');
	}

	FT_Int32 loadFlags = FT_LOAD_DEFAULT;
	if (FT_Load_Glyph(m_ftFace, glyphIndex, loadFlags) != 0)
		return NULL;

	FT_Render_Mode renderMode = FT_RENDER_MODE_NORMAL;
	if (FT_Render_Glyph(m_ftFace->glyph, renderMode) != 0)
		return NULL;

	FT_GlyphSlot slot = m_ftFace->glyph;
	FT_Bitmap& bitmap = slot->bitmap;

	int glyphBitmapWidth = bitmap.width;
	int glyphBitmapHeight = bitmap.rows;

	int bearingX = slot->bitmap_left;
	int bearingY = slot->bitmap_top;
	float advance = ceilf((float)(slot->advance.x) / 64.0f);

	int yOffset = m_ascender - bearingY;
	if (yOffset < 0)
		yOffset = 0;

	int cellHeight = m_lineHeight;
	int cellWidth = glyphBitmapWidth;

	if (glyphBitmapWidth == 0 || glyphBitmapHeight == 0)
	{
		TCharacterInfomation& rNewCharInfo = m_charInfoMap[keyValue];
		rNewCharInfo.index = static_cast<short>(m_activePageIndex);
		rNewCharInfo.width = 0;
		rNewCharInfo.height = (short)cellHeight;
		rNewCharInfo.left = 0;
		rNewCharInfo.top = 0;
		rNewCharInfo.right = 0;
		rNewCharInfo.bottom = 0;
		rNewCharInfo.advance = advance;
		rNewCharInfo.bearingX = 0.0f;
		return &rNewCharInfo;
	}

	int requiredHeight = yOffset + glyphBitmapHeight;
	if (requiredHeight > cellHeight)
		cellHeight = requiredHeight;

	int width = m_atlasWidth;
	int height = m_atlasHeight;

	if (m_x + cellWidth >= (width - 1))
	{
		m_y += (m_step + 1);
		m_step = 0;
		m_x = 0;

		if (m_y + cellHeight >= (height - 1))
		{
			if (!UpdateTexture())
				return NULL;

			if (!AppendTexture())
				return NULL;

			m_y = 0;
		}
	}

	const int srcPitch = bitmap.pitch;
	const unsigned char* srcBase = (srcPitch < 0)
		? (bitmap.buffer + (glyphBitmapHeight - 1) * (-srcPitch))
		: bitmap.buffer;

	for (int row = 0; row < glyphBitmapHeight; ++row)
	{
		int atlasY = m_y + yOffset + row;
		if (atlasY < 0 || atlasY >= height)
			continue;

		const unsigned char* srcRow = (srcPitch < 0)
			? (srcBase - row * (-srcPitch))
			: (srcBase + row * srcPitch);

		if (m_atlasFormat == EFontAtlasFormat::A8)
		{
			uint8_t* dstRow = reinterpret_cast<uint8_t*>(m_pAtlasBuffer) + atlasY * m_atlasWidth + m_x;

			if (bitmap.pixel_mode == FT_PIXEL_MODE_LCD)
			{
				for (int col = 0; col < glyphBitmapWidth; ++col)
				{
					unsigned char r = srcRow[col * 3 + 0];
					unsigned char g = srcRow[col * 3 + 1];
					unsigned char b = srcRow[col * 3 + 2];
					unsigned char a = (r > g) ? r : g;
					if (b > a) a = b;
					if (a > 0)
						dstRow[col] = a;
				}
			}
			else if (bitmap.pixel_mode == FT_PIXEL_MODE_MONO)
			{
				for (int col = 0; col < glyphBitmapWidth; ++col)
				{
					if (srcRow[col >> 3] & (0x80 >> (col & 7)))
						dstRow[col] = 0xFF;
				}
			}
			else
			{
				for (int col = 0; col < glyphBitmapWidth; ++col)
				{
					unsigned char a = srcRow[col];
					if (a > 0)
						dstRow[col] = a;
				}
			}
		}
		else
		{
			DWORD* dstRow = m_pAtlasBuffer + atlasY * m_atlasWidth + m_x;

			if (bitmap.pixel_mode == FT_PIXEL_MODE_LCD)
			{
				for (int col = 0; col < glyphBitmapWidth; ++col)
				{
					unsigned char r = srcRow[col * 3 + 0];
					unsigned char g = srcRow[col * 3 + 1];
					unsigned char b = srcRow[col * 3 + 2];
					unsigned char a = (r > g) ? r : g;
					if (b > a) a = b;
					if (a > 0)
						dstRow[col] = ((DWORD)a << 24) | 0x00FFFFFF;
				}
			}
			else if (bitmap.pixel_mode == FT_PIXEL_MODE_MONO)
			{
				for (int col = 0; col < glyphBitmapWidth; ++col)
				{
					if (srcRow[col >> 3] & (0x80 >> (col & 7)))
						dstRow[col] = 0xFFFFFFFF;
				}
			}
			else
			{
				for (int col = 0; col < glyphBitmapWidth; ++col)
				{
					unsigned char a = srcRow[col];
					if (a > 0)
						dstRow[col] = ((DWORD)a << 24) | 0x00FFFFFF;
				}
			}
		}
	}

	float rhwidth = 1.0f / float(width);
	float rhheight = 1.0f / float(height);

	TCharacterInfomation& rNewCharInfo = m_charInfoMap[keyValue];
	rNewCharInfo.index = static_cast<short>(m_activePageIndex);
	rNewCharInfo.width = (short)cellWidth;
	rNewCharInfo.height = (short)cellHeight;
	rNewCharInfo.left = float(m_x) * rhwidth;
	rNewCharInfo.top = float(m_y) * rhheight;
	rNewCharInfo.right = float(m_x + cellWidth) * rhwidth;
	rNewCharInfo.bottom = float(m_y + cellHeight) * rhheight;
	rNewCharInfo.advance = advance;
	rNewCharInfo.bearingX = (float)bearingX;

	if (m_activePageIndex < m_pageLastUsed.size())
		m_pageLastUsed[m_activePageIndex] = ++m_currentLruTick;

	m_x += cellWidth + 1;

	if (m_step < cellHeight)
		m_step = cellHeight;

	m_isDirty = true;
	return &rNewCharInfo;
}

bool CGraphicFontTexture::IsEmpty() const
{
	return m_pFontTextureVector.empty();
}
