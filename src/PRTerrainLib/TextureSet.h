#pragma once

#include "EterLib/GrpImageInstance.h"

typedef struct STerrainTexture
{
	STerrainTexture() :	pd3dTexture(NULL),
		UScale(4.0f),
		VScale(4.0f),
		UOffset(0.0f),
		VOffset(0.0f),
		bSplat(true),
		Begin(0),
		End(0)
	{
		D3DXMatrixIdentity(&m_matTransform);
	}

	std::string					stFilename;
	LPDIRECT3DTEXTURE9			pd3dTexture;
	CGraphicImageInstance 		ImageInstance;
	float						UScale;
	float						VScale;
	float						UOffset;
	float						VOffset;
	bool						bSplat;
	unsigned short				Begin, End;	// 0 ~ 65535 의 16bit heightfield 높이값.
	D3DXMATRIX					m_matTransform;
} TTerrainTexture;

class CTextureSet
{
	public:
		typedef std::vector<TTerrainTexture> TTextureVector;

		CTextureSet();
		virtual ~CTextureSet();

		void			Initialize();
		void			Clear();

		void			Create();

		bool			Load(const char * c_pszFileName, float fTerrainTexCoordBase);
		bool			Save(const char * c_pszFileName);
		
		unsigned long	GetTextureCount() const;
		
		TTerrainTexture	& GetTexture(unsigned long ulIndex);
		const TTerrainTexture & GetTexture(unsigned long ulIndex) const;
		bool			RemoveTexture(unsigned long ulIndex);

		bool			SetTexture(unsigned long ulIndex,
								   const char * c_szFileName,
			 					   float fuScale,
							       float fvScale,
								   float fuOffset,
								   float fvOffset,
								   bool bSplat,
								   unsigned short usBegin,
								   unsigned short usEnd,
								   float fTerrainTexCoordBase);

		void			Reload(float fTerrainTexCoordBase);

		bool			AddTexture(const char * c_szFileName,
			 					   float fuScale,
							       float fvScale,
								   float fuOffset,
								   float fvOffset,
								   bool bSplat,
								   unsigned short usBegin,
								   unsigned short usEnd,
								   float fTerrainTexCoordBase);

		const char *	GetFileName() const	{ return m_stFileName.c_str(); }

	protected:
		void			AddEmptyTexture();

	protected:
		TTextureVector			m_Textures;
		TTerrainTexture			m_ErrorTexture;
		std::string				m_stFileName;
};

