#pragma once

#include "GrpTexture.h"
#include "GrpImageTexture.h"

#include <ft2build.h>
#include FT_FREETYPE_H

#include <vector>
#include <map>
#include <cstdint>

class CGraphicFontTexture : public CGraphicTexture
{
	public:
		typedef wchar_t TCharacterKey;

		typedef struct SCharacterInfomation
		{
			short index;
			short width;
			short height;
			float left;
			float top;
			float right;
			float bottom;
			float advance;
			float bearingX;
		} TCharacterInfomation;

		typedef std::vector<TCharacterInfomation*> TPCharacterInfomationVector;

		enum class EFontAtlasFormat : uint8_t
		{
			A8R8G8B8 = 0,
			A8 = 1
		};

		static constexpr size_t MAX_FONT_TEXTURE_PAGES = 8;

	public:
		CGraphicFontTexture();
		virtual ~CGraphicFontTexture();

		void Destroy();
		bool Create(const char* c_szFontName, int fontSize, bool bItalic);

		bool CreateDeviceObjects();
		void DestroyDeviceObjects();

		bool CheckTextureIndex(DWORD dwTexture);
		void SelectTexture(DWORD dwTexture);

		bool UpdateTexture();

		TCharacterInfomation* GetCharacterInfomation(wchar_t keyValue);
		TCharacterInfomation* UpdateCharacterInfomation(TCharacterKey keyValue);

		float GetKerning(wchar_t prev, wchar_t cur);

		bool IsEmpty() const;

		void SetAtlasFormat(EFontAtlasFormat format) noexcept { m_atlasFormat = format; }
		[[nodiscard]] EFontAtlasFormat GetAtlasFormat() const noexcept { return m_atlasFormat; }
		[[nodiscard]] bool IsA8Format() const noexcept { return m_atlasFormat == EFontAtlasFormat::A8; }

	protected:
		void Initialize();

		bool AppendTexture();

	protected:
		typedef std::vector<CGraphicImageTexture*> TGraphicImageTexturePointerVector;
		typedef std::map<TCharacterKey, TCharacterInfomation> TCharacterInfomationMap;

	protected:
		FT_Face m_ftFace;

		DWORD* m_pAtlasBuffer;
		int m_atlasWidth;
		int m_atlasHeight;

		TGraphicImageTexturePointerVector m_pFontTextureVector;
		TCharacterInfomationMap m_charInfoMap;

		int m_x;
		int m_y;
		int m_step;
		bool m_isDirty;

		LONG m_fontSize;
		bool m_bItalic;

		int m_ascender;
		int m_lineHeight;
		bool m_hasKerning;

		EFontAtlasFormat m_atlasFormat{ EFontAtlasFormat::A8R8G8B8 };
		size_t m_activePageIndex{ 0 };
		uint64_t m_currentLruTick{ 0 };
		std::vector<uint64_t> m_pageLastUsed;
		RECT m_dirtyRect{ 0, 0, 0, 0 };
};
