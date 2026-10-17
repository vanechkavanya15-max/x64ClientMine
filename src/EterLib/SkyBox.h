// SkyBox.h: interface for the CSkyBox class.
//
//////////////////////////////////////////////////////////////////////

#if !defined(AFX_SKYBOX_H__AB5049E1_8F1C_4C35_9406_45EC7EF4AD1B__INCLUDED_)
#define AFX_SKYBOX_H__AB5049E1_8F1C_4C35_9406_45EC7EF4AD1B__INCLUDED_

#if _MSC_VER > 1000
#pragma once
#endif // _MSC_VER > 1000

#include "GrpBase.h"
#include "GrpScreen.h"
#include "GrpImageInstance.h"
#include "ColorTransitionHelper.h"
#include "RenderPacketTypes.h"
#include "RenderResourceRegistry.h"

#include <map>
#include <string>

typedef struct SColor
{
	SColor(float _r = 0.0f, float _g = 0.0f, float _b = 0.0f, float _a = 0.0f) : r(_r), g(_g), b(_b), a(_a){}
	float r, g, b, a;
} TColor;

typedef struct 
{
	TColor	m_FirstColor;
	TColor	m_SecondColor;
}TGradientColor;

typedef std::vector<TGradientColor> TVectorGradientColor;
typedef TVectorGradientColor::iterator TVectorGradientIterator;

class CSkyObjectQuad
{
public:
	CSkyObjectQuad();
	virtual ~CSkyObjectQuad();

	void Clear(const unsigned char & c_rucNumVertex,
		const float & c_rfRed,
		const float & c_rfGreen,
		const float & c_rfBlue,
		const float & c_rfAlpha);
	void SetSrcColor(const unsigned char & c_rucNumVertex,
		const float & c_rfRed,
		const float & c_rfGreen,
		const float & c_rfBlue,
		const float & c_rfAlpha);
	void SetTransition(const unsigned char & c_rucNumVertex,
		const float & c_rfRed,
		const float & c_rfGreen,
		const float & c_rfBlue,
		const float & c_rfAlpha,
		DWORD dwDuration);
	
	void SetVertex(const unsigned char & c_rucNumVertex, const TPDTVertex & c_rPDTVertex);
	
	void StartTransition();
	bool Update();
	void Render();

	const TPDTVertex * GetVertices() const { return m_Vertex; }
	const TIndex * GetIndices() const { return m_Indices; }
	const CColorTransitionHelper * GetHelpers() const { return m_Helper; }
	
private:
	TPDTVertex m_Vertex[4];
	TIndex m_Indices[4];					// 인덱스 버퍼...
	CColorTransitionHelper m_Helper[4];
};

class CSkyObject : public CScreen
{
public:
	enum
	{
		SKY_RENDER_MODE_DEFAULT,	// = SKY_RENDER_MODE_TEXTURE
		SKY_RENDER_MODE_DIFFUSE,
		SKY_RENDER_MODE_TEXTURE,
		SKY_RENDER_MODE_MODULATE,
		SKY_RENDER_MODE_MODULATE2X,
		SKY_RENDER_MODE_MODULATE4X,
	};

	CSkyObject();
	virtual ~CSkyObject();

	virtual void Destroy() = 0;

	virtual void Render() = 0;
	virtual void Update() = 0;
	virtual void StartTransition();

	void SetRenderMode(unsigned char ucRenderMode) { m_ucRenderMode = ucRenderMode;	}

	const bool & isTransitionStarted() { return m_bTransitionStarted; }

protected:
	CGraphicImageInstance * GenerateTexture(const char * szfilename);
	void DeleteTexture(CGraphicImageInstance * pGraphicImageInstance);

protected:
	//////////////////////////////////////////////////////////////////////////
	// 타입 정의

	typedef std::vector<CSkyObjectQuad> TSkyObjectQuadVector;
	typedef TSkyObjectQuadVector::iterator TSkyObjectQuadIterator;
	
	typedef struct SSkyObjectFace
	{
		void StartTransition();
		bool Update();
		void Render();
		std::string m_strfacename;
		std::string m_strFaceTextureFileName;
		TSkyObjectQuadVector m_SkyObjectQuadVector;
	} TSkyObjectFace;

	typedef std::map <std::string, CGraphicImageInstance*> TGraphicImageInstanceMap;

	//////////////////////////////////////////////////////////////////////////
	
	// 구름...
	TSkyObjectFace m_FaceCloud;		// 구름 일단 한장...
	D3DXMATRIX m_matWorldCloud, m_matTranslationCloud, m_matTextureCloud;
	D3DXVECTOR3 m_v3PositionCloud;
	float m_fCloudScaleX, m_fCloudScaleY, m_fCloudHeight;
	float m_fCloudTextureScaleX, m_fCloudTextureScaleY;
	float m_fCloudScrollSpeedU, m_fCloudScrollSpeedV;
	float m_fCloudPositionU, m_fCloudPositionV;

	DWORD m_dwlastTime;

	// 스카이 박스 이미지...
	TGraphicImageInstanceMap m_GraphicImageInstanceMap;
	
	// Transform...
	D3DXMATRIX m_matWorld, m_matTranslation;
	D3DXVECTOR3 m_v3Position;
	float m_fScaleX, m_fScaleY, m_fScaleZ;

	// 랜더링 관련... 임시 변수..
	unsigned char m_ucRenderMode;

	std::string m_strCurTime;
	bool m_bTransitionStarted;
	bool m_bSkyMatrixUpdated;
	
	CGraphicImageInstance m_CloudAlphaImageInstance;
};

struct SSkyFaceDesc
{
	std::string strFaceName;
	std::string strFaceTextureFileName;
	TRenderTextureHandle textureHandle{ kInvalidTextureHandle };
	uint32_t textureId{ 0 };
	std::vector<CSkyObjectQuad> quadVector;
	std::vector<TPDTVertex> vertices;
};

struct SSkyDescription
{
	unsigned char ucRenderMode{ 0 };
	TRenderTextureHandle faceTextures[6]{ 0, 0, 0, 0, 0, 0 };
	uint32_t faceTextureIds[6]{ 0, 0, 0, 0, 0, 0 };
	SSkyFaceDesc faces[6];
	uint32_t totalQuadCount{ 0 };
	std::vector<TPDTVertex> gradientVertices;

	// Macierze nieba i chmur
	D3DXMATRIX matSkyWorld;
	D3DXMATRIX matCloudWorld;
	D3DXMATRIX matCloudTexture;

	// Offset UV chmur i parametry
	float fCloudPositionU{ 0.0f };
	float fCloudPositionV{ 0.0f };
	float fCloudScaleX{ 0.0f };
	float fCloudScaleY{ 0.0f };
	float fCloudHeight{ 0.0f };
	float fCloudScrollSpeedU{ 0.0f };
	float fCloudScrollSpeedV{ 0.0f };

	TRenderTextureHandle cloudTextureHandle{ kInvalidTextureHandle };
	uint32_t cloudTextureId{ 0 };
	SSkyFaceDesc faceCloud;
};

using TSkyDescription = SSkyDescription;

class CSkyBox : public CSkyObject
{
public:
	CSkyBox();
	virtual ~CSkyBox();

	void Update();
	void Render();
	void RenderCloud();

	void Destroy();
	void Unload();

	void SetSkyBoxScale(const D3DXVECTOR3 & c_rv3Scale);
	void SetGradientLevel(BYTE byUpper, BYTE byLower);
	void SetFaceTexture( const char* c_szFileName, int iFaceIndex );
	void SetCloudTexture(const char * c_szFileName);
	void SetCloudScale(const D3DXVECTOR2 & c_rv2CloudScale);
	void SetCloudHeight(float fHeight);
	void SetCloudTextureScale(const D3DXVECTOR2 & c_rv2CloudTextureScale);
	void SetCloudScrollSpeed(const D3DXVECTOR2 & c_rv2CloudScrollSpeed);
	void SetCloudColor(const TGradientColor & c_rColor, const TGradientColor & c_rNextColor, const DWORD & dwTransitionTime);
	void Refresh();
	void SetSkyColor(const TVectorGradientColor & c_rColorVector, const TVectorGradientColor & c_rNextColorVector, long lTransitionTime);
	void StartTransition();

	// Atmosphere uniforms centralization
	void SetAtmosphereUniforms(const TRenderFrameUniforms& uniforms);
	const TRenderFrameUniforms& GetAtmosphereUniforms() const;
	void SetSunLight(const D3DXVECTOR4& v4Direction, const D3DXVECTOR4& v4Color);
	void SetAmbientColor(const D3DXVECTOR4& v4Ambient);
	void SetFogParams(float fFogNear, float fFogFar, float fLinearScale, bool bEnable);

	// CPU Description of Sky & Clouds state
	SSkyDescription BuildSkyDescription() const;

	const TSkyObjectFace & GetFace(size_t index) const { return m_Faces[index]; }
	const TSkyObjectFace & GetFaceCloud() const { return m_FaceCloud; }
	unsigned char GetRenderMode() const { return m_ucRenderMode; }

protected:
	void SetSkyObjectQuadVertical(TSkyObjectQuadVector * pSkyObjectQuadVector, const D3DXVECTOR2 * c_pv2QuadPoints);
	void SetSkyObjectQuadHorizon(TSkyObjectQuadVector * pSkyObjectQuadVector, const D3DXVECTOR3 * c_pv3QuadPoints);

	//void UpdateSkyFaceQuadTransform(D3DXVECTOR3 * c_pv3QuadPoints);

protected:
	unsigned char m_ucVirticalGradientLevelUpper;
	unsigned char m_ucVirticalGradientLevelLower;
	TSkyObjectFace m_Faces[6];
	TRenderFrameUniforms m_atmosphereUniforms{};
};

#endif // !defined(AFX_SKYBOX_H__AB5049E1_8F1C_4C35_9406_45EC7EF4AD1B__INCLUDED_)
