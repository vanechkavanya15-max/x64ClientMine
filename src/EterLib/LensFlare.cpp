#include "StdAfx.h"
#include "LensFlare.h"
#include "Camera.h"
#include "StateManager.h"
#include "ResourceManager.h"
#include "Render/DynamicQuadBatcher.h"
#include "RenderQueueManager.h"

#include <math.h>
#include <algorithm>
#include <string>

using namespace std;

static string g_strFiles[] = 
{
	"flare2.dds",
	"flare1.dds",
	"flare2.dds",
	"flare1.dds",
	"flare6.dds",
	"flare4.dds",
	"flare2.dds",
	"flare3.dds",
	""
};

static float g_fPosition[] =
{
	-0.55f,
	-0.5f,
	-0.45f,
	0.2f,
	0.3f,
	0.95f,
	0.9f,
	1.0f
};

static float g_fWidth[] =
{
	20.0f,
	32.0f,
	20.0f,
	32.0f,
	100.0f,
	32.0f,
	20.0f,
	250.0f
};

static float g_afColors[][4] =
{
	{ 1.0f, 1.0f, 0.0f, 1.0f },
	{ 1.0f, 1.0f, 1.0f, 1.0f },
	{ 0.0f, 1.0f, 0.0f, 0.8f },
	{ 0.3f, 0.5f, 1.0f, 0.9f },
	{ 0.3f, 0.5f, 1.0f, 0.6f },
	{ 1.0f, 0.6f, 0.9f, 0.4f },
	{ 1.0f, 0.0f, 0.0f, 0.5f },
	{ 1.0f, 0.6f, 0.3f, 0.4f }
};

CLensFlare::CLensFlare() :
	m_fSunSize(0),
	m_fBeforeBright(0.0f),
	m_fAfterBright(0.0f),
	m_bFlareVisible(false),
	m_bDrawFlare(true),
	m_bDrawBrightScreen(true),
	m_bEnabled(true),
	m_bShowMainFlare(true),
	m_fMaxBrightness(1.0f)
{
	m_afFlarePos[0] = m_afFlarePos[1] = 0.0f;
	m_afFlareWinPos[0] = m_afFlareWinPos[1] = 0.0f;
	m_pControlPixels = new float[c_nDepthTestDimension * c_nDepthTestDimension]();
	m_pTestPixels = new float[c_nDepthTestDimension * c_nDepthTestDimension]();
	m_afColor[0] = m_afColor[1] = m_afColor[2] = 1.0f;
	m_afColor[3] = 1.0f;
}

CLensFlare::~CLensFlare()
{
	delete[] m_pControlPixels;
	delete[] m_pTestPixels;
}

float CLensFlare::Interpolate(float fStart, float fEnd, float fPercent)
{
	return fStart + (fEnd - fStart) * fPercent;
}

void CLensFlare::Compute(const D3DXVECTOR3 & c_rv3LightDirection)
{
	float afSunPos[3];

	D3DXVECTOR3 v3Target = CCameraManager::Instance().GetCurrentCamera()->GetTarget();
	
	afSunPos[0] = v3Target.x - c_rv3LightDirection.x * kLensFlareMaxDistance;
	afSunPos[1] = v3Target.y - c_rv3LightDirection.y * kLensFlareMaxDistance;
	afSunPos[2] = v3Target.z - c_rv3LightDirection.z * kLensFlareMaxDistance;
	
	float fX, fY;
	ProjectPosition(afSunPos[0], afSunPos[1], afSunPos[2], &fX, &fY);
	
	// Ustawienie pozycji flary na ekranie
	SetFlareLocation(fX, fY);
	
	// Wyznaczenie widocznosci na podstawie iloczynu skalarnego
	float fSunVectorMagnitude = sqrtf(afSunPos[0] * afSunPos[0] +
		afSunPos[1] * afSunPos[1] +
		afSunPos[2] * afSunPos[2]);
	float afSunVector[3];
	afSunVector[0] = -afSunPos[0] / fSunVectorMagnitude;
	afSunVector[1] = -afSunPos[1] / fSunVectorMagnitude;
	afSunVector[2] = -afSunPos[2] / fSunVectorMagnitude;
	
	float afCameraDirection[3];
	afCameraDirection[0] = ms_matView._13;
	afCameraDirection[1] = ms_matView._23;
	afCameraDirection[2] = ms_matView._33;

	float fDotProduct = 
		(afSunVector[0] * afCameraDirection[0]) +
		(afSunVector[1] * afCameraDirection[1]) +
		(afSunVector[2] * afCameraDirection[2]);
	
	if (acosf(fDotProduct) < 0.5f * D3DX_PI)
		SetVisible(true);
	else
		SetVisible(false);
	
	// Obliczenie jasnosci flary wzgledem srodka ekranu
	fX /= ms_Viewport.Width;
	fY /= ms_Viewport.Height;
	
	float fDistance = sqrtf(((0.5f - fX) * (0.5f - fX)) + ((0.5f - fY) * (0.5f - fY)));
	float fBeforeBright = Interpolate(0.0f, c_fHalfMaxBright, 1.0f - (fDistance * c_fDistanceScale));
	float fAfterBright = Interpolate(0.0f, 1.0f, 1.0f - (fDistance * c_fDistanceScale));
	
	SetBrightnesses(fBeforeBright, fAfterBright);
}

void CLensFlare::DrawBeforeFlare()
{
	if (!m_bFlareVisible || !m_bEnabled || !m_bShowMainFlare)
		return;

	if (m_SunFlareImageInstance.IsEmpty())
		return;

	CGraphicImage* pImage = m_SunFlareImageInstance.GetGraphicImagePointer();
	if (!pImage)
		return;

	CGraphicTexture* pTexture = pImage->GetTexturePointer();
	if (!pTexture)
		return;

	uintptr_t dwTextureID = reinterpret_cast<uintptr_t>(pTexture->GetD3DTexture());
	if (dwTextureID == 0)
		return;

	float fHalfSize = m_fSunSize * static_cast<float>(ms_Viewport.Width);
	float cx = m_afFlareWinPos[0];
	float cy = m_afFlareWinPos[1];

	const RECT& rRect = pImage->GetRectReference();
	float texRevW = 1.0f / static_cast<float>(pTexture->GetWidth());
	float texRevH = 1.0f / static_cast<float>(pTexture->GetHeight());
	float u0 = rRect.left * texRevW;
	float v0 = rRect.top * texRevH;
	float u1 = (rRect.left + (rRect.right - rRect.left)) * texRevW;
	float v1 = (rRect.top + (rRect.bottom - rRect.top)) * texRevH;

	uint32_t color = 0xFFFFFFFF;

	auto& batcher = EterLib::Render::DynamicQuadBatcher::Instance();
	batcher.AddQuad(cx - fHalfSize, cy - fHalfSize, 2.0f * fHalfSize, 2.0f * fHalfSize,
	                color, dwTextureID, u0, v0, u1, v1, UI_BLEND_NORMAL);

	// Natychmiastowe wypchniecie flary slonca przed renderowaniem chmur
	batcher.FlushToQueue(CRenderQueueManager::Instance());
	CRenderQueueManager::Instance().FlushUI();
}

void CLensFlare::DrawAfterFlare()
{
	if (m_bEnabled && m_fAfterBright != 0.0f && m_bDrawBrightScreen)
	{
		uint8_t a = static_cast<uint8_t>(std::clamp(m_fAfterBright, 0.0f, 1.0f) * 255.0f);
		uint8_t r = static_cast<uint8_t>(std::clamp(m_afColor[0], 0.0f, 1.0f) * 255.0f);
		uint8_t g = static_cast<uint8_t>(std::clamp(m_afColor[1], 0.0f, 1.0f) * 255.0f);
		uint8_t b = static_cast<uint8_t>(std::clamp(m_afColor[2], 0.0f, 1.0f) * 255.0f);
		uint32_t color = (a << 24) | (r << 16) | (g << 8) | b;

		float screenWidth = static_cast<float>(ms_Viewport.Width);
		float screenHeight = static_cast<float>(ms_Viewport.Height);
		EterLib::Render::DynamicQuadBatcher::Instance().AddBar2d(0.0f, 0.0f, screenWidth, screenHeight, color);
	}
}

void CLensFlare::SetMainFlare(string strSunFile, float fSunSize)
{
	if (m_bEnabled && m_bShowMainFlare)
	{
		m_fSunSize = fSunSize;
		CResource * pResource = CResourceManager::Instance().GetResourcePointer(strSunFile.c_str()); 

		if (!pResource || !pResource->IsType(CGraphicImage::Type()))
			return;
		
		m_SunFlareImageInstance.SetImagePointer(static_cast<CGraphicImage *>(pResource));
	}
}

void CLensFlare::DrawFlare()
{
	if (m_bEnabled && m_bFlareVisible && m_bDrawFlare && m_fAfterBright != 0.0f)
	{
		DrawAfterFlare();

		m_cFlare.Draw(m_fAfterBright,
					  ms_Viewport.Width,
					  ms_Viewport.Height,
					  static_cast<int>(m_afFlareWinPos[0]),
					  static_cast<int>(m_afFlareWinPos[1]));

		// Wypchniecie zgrupowanych quadow flary bezposrednio do kolejki renderowania
		auto& batcher = EterLib::Render::DynamicQuadBatcher::Instance();
		batcher.FlushToQueue(CRenderQueueManager::Instance());
		CRenderQueueManager::Instance().FlushUI();
	}
}

void CLensFlare::Render()
{
	DrawFlare();
}

void CLensFlare::CharacterizeFlare(bool bEnabled, bool bShowMainFlare, float fMaxBrightness, const D3DXCOLOR & c_rColor)
{
	m_bEnabled = bEnabled;
	m_bShowMainFlare = bShowMainFlare;
	m_fMaxBrightness = fMaxBrightness;

	m_afColor[0] = c_rColor.r;
	m_afColor[1] = c_rColor.g;
	m_afColor[2] = c_rColor.b;
}

void CLensFlare::Initialize(std::string strPath)
{
	if (m_bEnabled)
		m_cFlare.Init(strPath);
}

void CLensFlare::SetFlareLocation(double dX, double dY)
{
	if (m_bEnabled)
	{
		m_afFlareWinPos[0] = float(dX);
		m_afFlareWinPos[1] = float(dY);

		m_afFlarePos[0] = float(dX) / ms_Viewport.Width;
		m_afFlarePos[1] = float(dY) / ms_Viewport.Height;
	}
}

void CLensFlare::SetBrightnesses(float fBeforeBright, float fAfterBright)
{
	if (m_bEnabled)
	{
		m_fBeforeBright = fBeforeBright;
		m_fAfterBright = fAfterBright;

		ClampBrightness();
	}
}

void CLensFlare::ReadControlPixels()
{
	if (m_bEnabled)
		ReadDepthPixels(m_pControlPixels);
}

void CLensFlare::AdjustBrightness()
{
	if (m_bEnabled)
	{
		ReadDepthPixels(m_pTestPixels);

		int nDifferent = 0;

		for (int i = 0; i < c_nDepthTestDimension * c_nDepthTestDimension; ++i)
		{
			if (m_pTestPixels[i] != m_pControlPixels[i])
				++nDifferent;
		}

		float fAdjust = (static_cast<float>(nDifferent) / (c_nDepthTestDimension * c_nDepthTestDimension));
		fAdjust = sqrtf(fAdjust) * kFlareIntensityScale;
		m_fAfterBright *= 1.0f - fAdjust;
	}
}

void CLensFlare::ReadDepthPixels(float * /*pPixels*/)
{
}

void CLensFlare::ClampBrightness()
{
	if (m_fBeforeBright < 0.0f)
		m_fBeforeBright = 0.0f;
	else if (m_fBeforeBright > 1.0f)
		m_fBeforeBright = 1.0f;

	m_fBeforeBright *= m_fMaxBrightness;

	if (m_fAfterBright < 0.0f)
		m_fAfterBright = 0.0f;
	else if (m_fAfterBright > 1.0f)
		m_fAfterBright = 1.0f;
	
	m_fAfterBright *= m_fMaxBrightness;
}

// CFlare implementation

CFlare::CFlare()
{
}

CFlare::~CFlare()
{
	for (auto* pPiece : m_vFlares)
	{
		delete pPiece;
	}
	m_vFlares.clear();
}

void CFlare::Init(std::string strPath)
{
	for (auto* pPiece : m_vFlares)
	{
		delete pPiece;
	}
	m_vFlares.clear();

	int i = 0;
	while (!g_strFiles[i].empty())
	{
		CResource * pResource = CResourceManager::Instance().GetResourcePointer((strPath + "/" + string(g_strFiles[i])).c_str());
		
		if (pResource && pResource->IsType(CGraphicImage::Type()))
		{
			SFlarePiece * pPiece = new SFlarePiece;
			pPiece->m_imageInstance.SetImagePointer(static_cast<CGraphicImage *>(pResource));
			pPiece->m_fPosition = g_fPosition[i];
			pPiece->m_fWidth = g_fWidth[i];
			pPiece->m_pColor = g_afColors[i];

			m_vFlares.push_back(pPiece);
		}
		i++;
	}
}

void CFlare::Draw(float fBrightScale, int nWidth, int nHeight, int nX, int nY)
{
	float fDX = float(nX) - float(nWidth) / 2.0f;
	float fDY = float(nY) - float(nHeight) / 2.0f;

	auto& batcher = EterLib::Render::DynamicQuadBatcher::Instance();

	for (size_t i = 0; i < m_vFlares.size(); i++)
	{
		if (!m_vFlares[i])
			continue;

		CGraphicImageInstance& imageInstance = m_vFlares[i]->m_imageInstance;
		if (imageInstance.IsEmpty())
			continue;

		CGraphicImage* pImage = imageInstance.GetGraphicImagePointer();
		if (!pImage)
			continue;

		CGraphicTexture* pTexture = pImage->GetTexturePointer();
		if (!pTexture)
			continue;

		uintptr_t dwTextureID = reinterpret_cast<uintptr_t>(pTexture->GetD3DTexture());
		if (dwTextureID == 0)
			continue;

		// 100% zachowane obliczenia pozycji flary wzgledem wektora slonca i promieni
		float fCenterX = float(nX) - (m_vFlares[i]->m_fPosition + 1.0f) * fDX;
		float fCenterY = float(nY) - (m_vFlares[i]->m_fPosition + 1.0f) * fDY;
		float fW = m_vFlares[i]->m_fWidth;

		float r = m_vFlares[i]->m_pColor[0] * fBrightScale;
		float g = m_vFlares[i]->m_pColor[1] * fBrightScale;
		float b = m_vFlares[i]->m_pColor[2] * fBrightScale;
		float a = m_vFlares[i]->m_pColor[3] * fBrightScale;

		uint8_t dwA = static_cast<uint8_t>(std::clamp(a, 0.0f, 1.0f) * 255.0f);
		uint8_t dwR = static_cast<uint8_t>(std::clamp(r, 0.0f, 1.0f) * 255.0f);
		uint8_t dwG = static_cast<uint8_t>(std::clamp(g, 0.0f, 1.0f) * 255.0f);
		uint8_t dwB = static_cast<uint8_t>(std::clamp(b, 0.0f, 1.0f) * 255.0f);
		uint32_t color = (dwA << 24) | (dwR << 16) | (dwG << 8) | dwB;

		float qx = fCenterX - fW;
		float qy = fCenterY - fW;
		float qw = 2.0f * fW;
		float qh = 2.0f * fW;

		const RECT& rRect = pImage->GetRectReference();
		float texRevW = 1.0f / static_cast<float>(pTexture->GetWidth());
		float texRevH = 1.0f / static_cast<float>(pTexture->GetHeight());
		float u0 = rRect.left * texRevW;
		float v0 = rRect.top * texRevH;
		float u1 = (rRect.left + (rRect.right - rRect.left)) * texRevW;
		float v1 = (rRect.top + (rRect.bottom - rRect.top)) * texRevH;

		// Dodanie quada do DynamicQuadBatcher z trybem ADDITIVE bez przelaczania stanow D3D per quad
		batcher.AddQuad(qx, qy, qw, qh, color, dwTextureID, u0, v0, u1, v1, UI_BLEND_ADDITIVE);
	}
}
