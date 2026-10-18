#include "StdAfx.h"
#include "TextureSet.h"

CTextureSet::CTextureSet()
{
	Initialize();
}

CTextureSet::~CTextureSet()
{
	Clear();
}

void CTextureSet::Initialize()
{
}

void CTextureSet::Create()
{
	CResourceManager * pResourceManager = CResourceManager::InstancePtr();
	if (pResourceManager)
	{
		CResource * pResource = pResourceManager->GetResourcePointer("d:/ymir work/special/error.tga");
		if (pResource && pResource->IsType(CGraphicImage::Type()))
		{
			m_ErrorTexture.ImageInstance.SetImagePointer(static_cast<CGraphicImage *>(pResource));
			if (m_ErrorTexture.ImageInstance.GetTexturePointer())
				m_ErrorTexture.pd3dTexture = m_ErrorTexture.ImageInstance.GetTexturePointer()->GetD3DTexture();
			else
				m_ErrorTexture.pd3dTexture = NULL;
		}
		else
		{
			m_ErrorTexture.pd3dTexture = NULL;
		}
	}
	else
	{
		m_ErrorTexture.pd3dTexture = NULL;
	}

	AddEmptyTexture();	// 지우개 텍스춰를 처음에 추가 해야 함
}

bool CTextureSet::Load(const char * c_szTextureSetFileName, float fTerrainTexCoordBase)
{
	Clear();

	CTokenVectorMap stTokenVectorMap;

	if (!LoadMultipleTextData(c_szTextureSetFileName, stTokenVectorMap))
	{
		TraceError("TextureSet::Load : cannot load %s", c_szTextureSetFileName);
		return false;
	}

	if (stTokenVectorMap.end() == stTokenVectorMap.find("textureset"))
	{
		TraceError("TextureSet::Load : syntax error, TextureSet (filename: %s)", c_szTextureSetFileName);
		return false;
	}
	
	auto itCount = stTokenVectorMap.find("texturecount");
	if (itCount == stTokenVectorMap.end() || itCount->second.empty())
	{
		TraceError("TextureSet::Load : syntax error, TextureCount (filename: %s)", c_szTextureSetFileName);
		return false;
	}

	Create();

	const long lCount = atol(itCount->second[0].c_str());
	if (lCount < 0 || lCount > 255)
	{
		TraceError("TextureSet::Load : invalid texture count %ld (filename: %s)", lCount, c_szTextureSetFileName);
		return false;
	}

	char szTextureName[32 + 1];
	m_Textures.resize(lCount + 1);

	std::vector<std::string> textureFiles;
	textureFiles.reserve(lCount);

	for (long i = 0; i < lCount; ++i)
	{
		_snprintf(szTextureName, sizeof(szTextureName), "texture%03ld", i + 1);

		auto itTex = stTokenVectorMap.find(szTextureName);
		if (itTex == stTokenVectorMap.end() || itTex->second.empty())
			continue;

		textureFiles.push_back(itTex->second[0]);
	}

	for (const auto& filename : textureFiles)
	{
		CResourceManager::Instance().GetResourcePointer(filename.c_str());
	}

	for (long i = 0; i < lCount; ++i)
	{
		_snprintf(szTextureName, sizeof(szTextureName), "texture%03ld", i + 1);

		auto itTex = stTokenVectorMap.find(szTextureName);
		if (itTex == stTokenVectorMap.end())
			continue;

		const CTokenVector & rVector = itTex->second;
		if (rVector.size() < 8)
		{
			TraceError("TextureSet::Load : %s expected at least 8 tokens, got %zu (filename: %s)", szTextureName, rVector.size(), c_szTextureSetFileName);
			continue;
		}

		const std::string & c_rstrFileName	= rVector[0];
		const std::string & c_rstrUScale	= rVector[1];
		const std::string & c_rstrVScale	= rVector[2];
		const std::string & c_rstrUOffset	= rVector[3];
		const std::string & c_rstrVOffset	= rVector[4];
		const std::string & c_rstrbSplat	= rVector[5];
		const std::string & c_rstrBegin		= rVector[6];
		const std::string & c_rstrEnd		= rVector[7];

		float fuScale = static_cast<float>(atof(c_rstrUScale.c_str()));
		float fvScale = static_cast<float>(atof(c_rstrVScale.c_str()));
		float fuOffset = static_cast<float>(atof(c_rstrUOffset.c_str()));
		float fvOffset = static_cast<float>(atof(c_rstrVOffset.c_str()));
		bool bSplat = 0 != atoi(c_rstrbSplat.c_str());
		unsigned short usBegin = static_cast<unsigned short>(atoi(c_rstrBegin.c_str()));
		unsigned short usEnd = static_cast<unsigned short>(atoi(c_rstrEnd.c_str()));

		if (!SetTexture(i + 1, c_rstrFileName.c_str(), fuScale, fvScale, fuOffset, fvOffset, bSplat, usBegin, usEnd, fTerrainTexCoordBase))
			TraceError("CTextureSet::Load : SetTexture failed : Filename: %s", c_rstrFileName.c_str());
	}

	m_stFileName.assign(c_szTextureSetFileName);
	return true;
}

void CTextureSet::Clear()
{
	m_ErrorTexture.ImageInstance.Destroy();
	m_ErrorTexture.pd3dTexture = NULL;
	m_Textures.clear();
	Initialize();
}

void CTextureSet::AddEmptyTexture()
{
	TTerrainTexture eraser;
	m_Textures.push_back(eraser);
}

unsigned long CTextureSet::GetTextureCount() const
{
	return static_cast<unsigned long>(m_Textures.size());
}

TTerrainTexture	& CTextureSet::GetTexture(unsigned long ulIndex)
{
	if (ulIndex >= GetTextureCount())
		return m_ErrorTexture;

	return m_Textures[ulIndex];
}

const TTerrainTexture & CTextureSet::GetTexture(unsigned long ulIndex) const
{
	if (ulIndex >= GetTextureCount())
		return m_ErrorTexture;

	return m_Textures[ulIndex];
}

bool CTextureSet::SetTexture(unsigned long ulIndex,
							 const char * c_szFileName,
							 float fuScale,
							 float fvScale,
							 float fuOffset,
							 float fvOffset,
							 bool bSplat,
							 unsigned short usBegin,
							 unsigned short usEnd,
							 float fTerrainTexCoordBase)
{
	if (ulIndex >= m_Textures.size())
	{
		TraceError("CTextureSet::SetTexture : Index Error : Index(%lu) is Larger than TextureSet Size(%zu)", ulIndex, m_Textures.size());
		return false;
	}

	CResource * pResource = CResourceManager::Instance().GetResourcePointer(c_szFileName);
	if (!pResource)
	{
		TraceError("CTextureSet::SetTexture : %s resource pointer is NULL", c_szFileName);
		return false;
	}

	if (!pResource->IsType(CGraphicImage::Type()))
	{
		TraceError("CTextureSet::SetTexture : %s is NOT Image File", pResource->GetFileName());
		return false;
	}

	TTerrainTexture & tex = m_Textures[ulIndex];

	tex.stFilename = c_szFileName;
	tex.UScale = fuScale;
	tex.VScale = fvScale;
	tex.UOffset = fuOffset;
	tex.VOffset = fvOffset;
	tex.bSplat = bSplat;
	tex.Begin = usBegin;
	tex.End = usEnd;
	tex.ImageInstance.SetImagePointer(static_cast<CGraphicImage *>(pResource));
	if (tex.ImageInstance.GetTexturePointer())
		tex.pd3dTexture = tex.ImageInstance.GetTexturePointer()->GetD3DTexture();
	else
		tex.pd3dTexture = NULL;
	
	D3DXMatrixScaling(&tex.m_matTransform, fTerrainTexCoordBase * tex.UScale, -fTerrainTexCoordBase * tex.VScale, 0.0f);
	tex.m_matTransform._41 = tex.UOffset;
	tex.m_matTransform._42 = -tex.VOffset;
	return true;
}

void CTextureSet::Reload(float fTerrainTexCoordBase)
{
	for (DWORD dwIndex = 1; dwIndex < GetTextureCount(); ++dwIndex)
	{
		TTerrainTexture & tex = m_Textures[dwIndex];

		if (tex.stFilename.empty())
			continue;

		CResource * pResource = CResourceManager::Instance().GetResourcePointer(tex.stFilename.c_str());
		if (pResource)
		{
			tex.ImageInstance.ReloadImagePointer(static_cast<CGraphicImage *>(pResource));
			if (tex.ImageInstance.GetTexturePointer())
				tex.pd3dTexture = tex.ImageInstance.GetTexturePointer()->GetD3DTexture();
			else
				tex.pd3dTexture = NULL;
		}

		D3DXMatrixScaling(&tex.m_matTransform, fTerrainTexCoordBase * tex.UScale, -fTerrainTexCoordBase * tex.VScale, 0.0f);
		tex.m_matTransform._41 = tex.UOffset;
		tex.m_matTransform._42 = -tex.VOffset;
	}
}
							 
bool CTextureSet::AddTexture(const char * c_szFileName,
							 float fuScale,
							 float fvScale,
							 float fuOffset,
							 float fvOffset,
							 bool bSplat,
							 unsigned short usBegin,
							 unsigned short usEnd,
							 float fTerrainTexCoordBase)
{
	if (GetTextureCount() >= 256)
	{
		TraceError("CTextureSet::AddTexture: Maximum texture count (256) reached");
		return false;
	}

	for (unsigned long i = 1; i < GetTextureCount(); ++i)
	{
		if (_stricmp(m_Textures[i].stFilename.c_str(), c_szFileName) == 0)
		{
			TraceError("CTextureSet::AddTexture: Duplicate texture name %s", c_szFileName);
			return false;
		}
	}

	m_Textures.emplace_back();
	const unsigned long newIndex = static_cast<unsigned long>(m_Textures.size() - 1);

	if (!SetTexture(newIndex,
				   c_szFileName,
				   fuScale,
				   fvScale,
				   fuOffset,
				   fvOffset,
				   bSplat,
				   usBegin,
				   usEnd,
				   fTerrainTexCoordBase))
	{
		m_Textures.pop_back();
		return false;
	}

	return true;
}

bool CTextureSet::RemoveTexture(unsigned long ulIndex)
{
	// Slot 0 jest zarezerwowany dla gumki (eraser) - usuniecie zabronione
	if (ulIndex == 0 || ulIndex >= GetTextureCount())
		return false;

	TTextureVector::iterator itor = m_Textures.begin() + ulIndex;
	m_Textures.erase(itor);
	return true;
}

bool CTextureSet::Save(const char * c_pszFileName)
{
	FILE * pFile = fopen(c_pszFileName, "w");
	
	if (!pFile)
		return false;
	
	fprintf(pFile, "TextureSet\n");
	fprintf(pFile, "\n");
	
	const unsigned long count = GetTextureCount();
	const unsigned long saveCount = (count > 0) ? (count - 1) : 0;
	fprintf(pFile, "TextureCount %lu\n", saveCount);	// -1 poniewaz slot 0 to gumka
	fprintf(pFile, "\n");

	for (DWORD i = 1; i < count; ++i)
	{
		TTerrainTexture & rTex = m_Textures[i];
		
		fprintf(pFile, "Start Texture%03lu\n", i);
		fprintf(pFile, "    \"%s\"\n", rTex.stFilename.c_str());
		fprintf(pFile, "    %f\n", rTex.UScale);
		fprintf(pFile, "    %f\n", rTex.VScale);
		fprintf(pFile, "    %f\n", rTex.UOffset);
		fprintf(pFile, "    %f\n", rTex.VOffset);
		fprintf(pFile, "    %d\n", rTex.bSplat ? 1 : 0);
		fprintf(pFile, "    %hu\n", rTex.Begin);
		fprintf(pFile, "    %hu\n", rTex.End);
		fprintf(pFile, "End Texture%03lu\n", i);
	}
	
	fclose(pFile);
	return true;
}
