cat << 'PATCHEOF' > src/EterLib/GrpText.cpp
#include "StdAfx.h"
#include "EterBase/Utils.h"
#include "GrpText.h"
#include <string>

CGraphicText::CGraphicText(const char* c_szFileName) : CResource(c_szFileName)
{
}

CGraphicText::~CGraphicText()
{
}

bool CGraphicText::CreateDeviceObjects()
{
	return m_fontTexture.CreateDeviceObjects();
}

void CGraphicText::DestroyDeviceObjects()
{
	m_fontTexture.DestroyDeviceObjects();
}

CGraphicFontTexture* CGraphicText::GetFontTexturePointer()
{
	return &m_fontTexture;
}

CGraphicText::TType CGraphicText::Type()
{
	static TType s_type = StringToType("CGraphicText");
	return s_type;
}

bool CGraphicText::OnLoad(int /*iSize*/, const void* /*c_pvBuf*/)
{
	std::string fileName = GetFileName() ? GetFileName() : "";
	std::string fontName;
	int size = 12;
	bool bItalic = false;

	// format
	// 굴림.fnt		"굴림" 폰트 기본 사이즈 12 로 로딩
	// 굴림:18.fnt  "굴림" 폰트 사이즈 18 로 로딩
	// 굴림:14i.fnt "굴림" 폰트 사이즈 14 & 이탤릭으로 로딩
	
	size_t colonPos = fileName.find_last_of(':');
	if (colonPos != std::string::npos)
	{
		fontName = fileName.substr(0, colonPos);
		
		std::string modifiers = fileName.substr(colonPos + 1);
		
		size_t dotPos = modifiers.find_last_of('.');
		if (dotPos != std::string::npos)
			modifiers = modifiers.substr(0, dotPos);
			
		std::string numStr = "";
		for (char c : modifiers)
		{
			if (isdigit(c))
				numStr += c;
			else if (c == 'i')
				bItalic = true;
		}
		
		if (!numStr.empty())
		{
			try {
				size = std::stoi(numStr);
			} catch (...) {
				size = 12;
			}
		}
	}
	else
	{
		size_t dotPos = fileName.find_last_of('.');
		if (dotPos == std::string::npos)
		{
			assert(!"CGraphicText::OnLoadFromFile there is no extension (ie: .fnt)");
			fontName = fileName;
		}
		else
		{
			fontName = fileName.substr(0, dotPos);
		}
		
		size = 12;
	}

	if (!m_fontTexture.Create(fontName.c_str(), size, bItalic))
		return false;

	return true;
}

void CGraphicText::OnClear()
{
	m_fontTexture.Destroy();
}

bool CGraphicText::OnIsEmpty() const
{
	return m_fontTexture.IsEmpty();
}

bool CGraphicText::OnIsType(TType type)
{
	if (CGraphicText::Type() == type)
		return true;
	
	return CResource::OnIsType(type);
}
PATCHEOF
