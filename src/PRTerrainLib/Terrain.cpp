#include "Stdafx.h"
#include "PackLib/PackManager.h"

#include "terrain.h"
#include <math.h>

//////////////////////////////////////////////////////////////////////////
// Texture Set
//////////////////////////////////////////////////////////////////////////
CTextureSet * CTerrainImpl::ms_pTextureSet = NULL;

void CTerrainImpl::SetTextureSet(CTextureSet * pTextureSet)
{
	static CTextureSet s_EmptyTextureSet;

	if (!pTextureSet)
		ms_pTextureSet = &s_EmptyTextureSet;
	else
		ms_pTextureSet = pTextureSet;
}

CTextureSet * CTerrainImpl::GetTextureSet()
{
	if (!ms_pTextureSet)
		SetTextureSet(NULL);

	return ms_pTextureSet;
}

CTerrainImpl::CTerrainImpl()
{
	Initialize();
}

CTerrainImpl::~CTerrainImpl()
{
	Clear();
}

void CTerrainImpl::Initialize()
{
	m_byNumWater = 0;
	memset(m_lWaterHeight, -1, sizeof(m_lWaterHeight));
	memset(m_awShadowMap, 0xFF, sizeof(m_awShadowMap));
	memset(m_awRawHeightMap, 0, sizeof(m_awRawHeightMap));
	memset(m_abyTileMap, 0, sizeof(m_abyTileMap));
	memset(m_abyAttrMap, 0, sizeof(m_abyAttrMap));
	memset(m_abyWaterMap, 0xFF, sizeof(m_abyWaterMap));
	memset(m_acNormalMap, 0, sizeof(m_acNormalMap));
	memset(m_lpAlphaTexture, 0, sizeof(m_lpAlphaTexture));
	memset(&m_TerrainSplatPatch, 0, sizeof(m_TerrainSplatPatch));
	
	m_lViewRadius = 0;
	m_wTileMapVersion = 8976;
	m_fHeightScale = 0.0f;
	m_lpShadowTexture = NULL;
	m_lSplatTilesX = 0;
	m_lSplatTilesY = 0;	
}

void CTerrainImpl::Clear()
{
	Initialize();
}

bool CTerrainImpl::LoadHeightMap(const char * c_szFileName)
{
	TPackFile kMappedFile;
	if (!CPackManager::Instance().GetFile(c_szFileName, kMappedFile))
	{
		TraceError("CTerrainImpl::LoadHeightMap - %s OPEN ERROR", c_szFileName);
		return false;
	}
	
	const size_t expectedSize = sizeof(m_awRawHeightMap);
	if (kMappedFile.size() < expectedSize)
	{
		TraceError("CTerrainImpl::LoadHeightMap - %s FILE SIZE ERROR (got %zu < need %zu)", c_szFileName, kMappedFile.size(), expectedSize);
		return false;
	}

	memcpy(m_awRawHeightMap, kMappedFile.data(), expectedSize);
	return true;
}

bool CTerrainImpl::LoadAttrMap(const char *c_szFileName)
{
	DWORD dwStart = ELTimer_GetMSec();
	Tracef("LoadAttrMapFile %s ", c_szFileName);

	TPackFile	kMappedFile;

	if (!CPackManager::Instance().GetFile(c_szFileName, kMappedFile))
	{
		TraceError("CTerrainImpl::LoadAttrMap - %s OPEN ERROR", c_szFileName);
		return false;
	}

	DWORD dwFileSize = kMappedFile.size();
	BYTE * abFileData = kMappedFile.data();

	// LoadAttrMap
	{
#pragma pack(push)
#pragma pack(1)
		struct SAttrMapHeader
		{
			WORD m_wMagic;
			WORD m_wWidth;
			WORD m_wHeight;
		};
#pragma pack(pop)
		
		if (dwFileSize < sizeof(SAttrMapHeader))
		{
			TraceError(" CTerrainImpl::LoadAttrMap - %s FILE SIZE ERROR", c_szFileName);
			return false;
		}

		SAttrMapHeader kAttrMapHeader;
		memcpy(&kAttrMapHeader, abFileData, sizeof(kAttrMapHeader));

		const WORD c_wAttrMapMagic = 2634;
		if (c_wAttrMapMagic != kAttrMapHeader.m_wMagic)
		{
			TraceError("CTerrainImpl::LoadAttrMap - %s MAGIC NUMBER(%d!=MAGIC[%d]) ERROR", c_szFileName, kAttrMapHeader.m_wMagic, c_wAttrMapMagic);
			return false;
		}

		if (ATTRMAP_XSIZE != kAttrMapHeader.m_wWidth)
		{
			TraceError("CTerrainImpl::LoadAttrMap - kAttrMapHeader(%s).m_width(%d)!=ATTRMAP_XSIZE(%d)", c_szFileName, kAttrMapHeader.m_wWidth, ATTRMAP_XSIZE);
			return false;
		}

		if (ATTRMAP_YSIZE != kAttrMapHeader.m_wHeight)
		{
			TraceError("CTerrainImpl::LoadAttrMap - kAttrMapHeader(%s).m_height(%d)!=ATTRMAP_YSIZE(%d)", c_szFileName, kAttrMapHeader.m_wHeight, ATTRMAP_YSIZE);
			return false;
		}

		DWORD dwFileRestSize = dwFileSize - sizeof(kAttrMapHeader);
		DWORD dwFileNeedSize = sizeof(m_abyAttrMap);
		if (dwFileRestSize != dwFileNeedSize)
		{
			TraceError("CTerrainImpl::LoadAttrMap - %s FILE DATA SIZE(rest %d != need %d) ERROR", c_szFileName, dwFileRestSize, dwFileNeedSize);
			return false;
		}

		BYTE * abSrcAttrData = abFileData + sizeof(kAttrMapHeader);
		memcpy(m_abyAttrMap, abSrcAttrData, sizeof(m_abyAttrMap));		
	}

	Tracef("%d\n", ELTimer_GetMSec() - dwStart);
	return true;
}

bool CTerrainImpl::RAW_LoadTileMap(const char * c_szFileName)
{
	TPackFile kMappedFile;
	if (!CPackManager::Instance().GetFile(c_szFileName, kMappedFile))
	{
		TraceError("CTerrainImpl::RAW_LoadTileMap - %s OPEN ERROR", c_szFileName);
		return false;
	}
	
	const size_t expectedSize = sizeof(m_abyTileMap);
	if (kMappedFile.size() < expectedSize)
	{
		TraceError("CTerrainImpl::RAW_LoadTileMap - %s FILE SIZE ERROR (got %zu < need %zu)", c_szFileName, kMappedFile.size(), expectedSize);
		return false;
	}

	memcpy(m_abyTileMap, kMappedFile.data(), expectedSize);
	return true;
}

bool CTerrainImpl::LoadWaterMap(const char * c_szFileName)
{	
	DWORD dwStart = ELTimer_GetMSec();

	if (!LoadWaterMapFile(c_szFileName))
	{
		memset(m_abyWaterMap, 0xFF, sizeof(m_abyWaterMap));

		m_byNumWater = 0;
		memset(m_lWaterHeight, -1, sizeof(m_lWaterHeight));
		
		TraceError("CMapOutdoor::LoadWaterMap LoadWaterMapFile(%s) Failed", c_szFileName);
		return false;
	}

	Tracef("LoadWaterMapFile %s %d\n", c_szFileName, ELTimer_GetMSec() - dwStart);
	return true;
}

bool CTerrainImpl::LoadWaterMapFile(const char * c_szFileName)
{	
	TPackFile kMappedFile;
	if (!CPackManager::Instance().GetFile(c_szFileName, kMappedFile))
	{
		TraceError("CTerrainImpl::LoadWaterMap - %s OPEN ERROR", c_szFileName);
		return false;
	}	

	const size_t dwFileSize = kMappedFile.size();
	const BYTE * abFileData = kMappedFile.data();

#pragma pack(push, 1)
	struct SWaterMapHeader
	{
		WORD m_wMagic;
		WORD m_wWidth;
		WORD m_wHeight;
		BYTE m_byLayerCount;
	};
#pragma pack(pop)
	
	if (dwFileSize < sizeof(SWaterMapHeader))
	{
		TraceError("CTerrainImpl::LoadWaterMap - %s FILE SIZE ERROR", c_szFileName);
		return false;
	}

	SWaterMapHeader kWaterMapHeader;
	memcpy(&kWaterMapHeader, abFileData, sizeof(kWaterMapHeader));		

	constexpr WORD c_wWaterMapMagic = 5426;
	if (c_wWaterMapMagic != kWaterMapHeader.m_wMagic)
	{
		TraceError("CTerrainImpl::LoadWaterMap - %s MAGIC NUMBER(%d!=MAGIC[%d]) ERROR", c_szFileName, kWaterMapHeader.m_wMagic, c_wWaterMapMagic);
		return false;
	}	
	
	if (WATERMAP_XSIZE != kWaterMapHeader.m_wWidth)
	{
		TraceError("CTerrainImpl::LoadWaterMap - kWaterMapHeader(%s).m_width(%d)!=WATERMAP_XSIZE(%d)", c_szFileName, kWaterMapHeader.m_wWidth, WATERMAP_XSIZE);
		return false;
	}

	if (WATERMAP_YSIZE != kWaterMapHeader.m_wHeight)
	{
		TraceError("CTerrainImpl::LoadWaterMap - kWaterMapHeader(%s).m_height(%d)!=WATERMAP_YSIZE(%d)", c_szFileName, kWaterMapHeader.m_wHeight, WATERMAP_YSIZE);
		return false;
	}

	const size_t dwFileRestSize = dwFileSize - sizeof(kWaterMapHeader);
	const size_t count = kWaterMapHeader.m_byLayerCount;
	const size_t mapSize = sizeof(m_abyWaterMap);
	const BYTE * src = abFileData + sizeof(kWaterMapHeader);

	long heights[MAX_WATER_NUM + 1] = {};

	if (dwFileRestSize == mapSize + sizeof(long) * count)
	{
		const size_t copyCount = std::min<size_t>(count, MAX_WATER_NUM);
		for (size_t i = 0; i < copyCount; ++i)
		{
			long v = -1;
			memcpy(&v, src + mapSize + i * sizeof(long), sizeof(long));
			heights[i] = v;
		}
	}
	else if (dwFileRestSize == mapSize + sizeof(WORD) * count)
	{
		const size_t copyCount = std::min<size_t>(count, MAX_WATER_NUM);
		for (size_t i = 0; i < copyCount; ++i)
		{
			WORD v = 0;
			memcpy(&v, src + mapSize + i * sizeof(WORD), sizeof(WORD));
			heights[i] = static_cast<long>(v);
		}
	}
	else
	{
		TraceError("CTerrainImpl::LoadWaterMap - %s FILE DATA SIZE(rest %zu) ERROR", c_szFileName, dwFileRestSize);
		return false;
	}

	// Dopiero po poprawnej walidacji aktualizujemy stan obiektu
	memcpy(m_abyWaterMap, src, mapSize);
	memcpy(m_lWaterHeight, heights, sizeof(m_lWaterHeight));
	m_byNumWater = static_cast<BYTE>(std::min<size_t>(count, MAX_WATER_NUM));

	return true;
}

DWORD CTerrainImpl::GetShadowMapColor(float fx, float fy) const
{
	constexpr float fMapSize = static_cast<float>(TERRAIN_XSIZE);
	if (!(fx >= 0.0f && fy >= 0.0f && fx < fMapSize && fy < fMapSize))
		return 0xFFFFFFFFu;

	constexpr float fooMapSize = 1.0f / fMapSize;
	const int ix = static_cast<int>(fx * fooMapSize * static_cast<float>(SHADOWMAP_XSIZE - 1));
	const int iy = static_cast<int>(fy * fooMapSize * static_cast<float>(SHADOWMAP_YSIZE - 1));

	if (ix < 0 || ix >= SHADOWMAP_XSIZE || iy < 0 || iy >= SHADOWMAP_YSIZE)
		return 0xFFFFFFFFu;

	const WORD w = m_awShadowMap[iy * SHADOWMAP_XSIZE + ix];

	auto expand5to8 = [](unsigned int v) noexcept -> unsigned int {
		return (v << 3) | (v >> 2); // 0..31 -> 0..255
	};

	const unsigned int b = expand5to8(w & 0x1F);
	const unsigned int g = expand5to8((w >> 5) & 0x1F);
	const unsigned int r = expand5to8((w >> 10) & 0x1F);

	return 0xFF000000u | (r << 16) | (g << 8) | b;
}
