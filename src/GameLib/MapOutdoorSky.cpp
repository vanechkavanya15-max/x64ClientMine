#include "StdAfx.h"
#include "MapOutdoorSky.h"
#include "SpeedTreeLib/SpeedTreeForestDirectX.h"
#include "EterLib/CFrameSnapshot.h"

CMapOutdoorSky::CMapOutdoorSky()
	: m_pSkyBox(nullptr), m_bInitialized(false)
{
	memset(&m_uniforms, 0, sizeof(TRenderFrameUniforms));
}

CMapOutdoorSky::~CMapOutdoorSky()
{
	Reset();
}

void CMapOutdoorSky::Initialize(CSkyBox* pSkyBox)
{
	m_pSkyBox = pSkyBox;
	m_bInitialized = (pSkyBox != nullptr);
}

void CMapOutdoorSky::Reset()
{
	if (m_pSkyBox)
	{
		m_pSkyBox->Unload();
	}
	m_pSkyBox = nullptr;
	m_bInitialized = false;
	memset(&m_uniforms, 0, sizeof(TRenderFrameUniforms));
}

void CMapOutdoorSky::UpdateAtmosphere(const TEnvironmentData* pEnvData, int iFogLevel)
{
	if (!pEnvData)
		return;

	// 1. Swiatlo slonca (kierunek, kolor)
	const D3DLIGHT9& sunLight = pEnvData->DirLights[ENV_DIRLIGHT_CHARACTER];
	m_uniforms.vSunLightDir = D3DXVECTOR3(sunLight.Direction.x, sunLight.Direction.y, sunLight.Direction.z);
	m_uniforms.vSunLightColor = D3DXCOLOR(sunLight.Diffuse.r, sunLight.Diffuse.g, sunLight.Diffuse.b, sunLight.Diffuse.a);
	m_uniforms.v4SunColor = D3DXVECTOR4(sunLight.Diffuse.r, sunLight.Diffuse.g, sunLight.Diffuse.b, sunLight.Diffuse.a);

	// 2. Kolor otoczenia (ambient)
	m_uniforms.vAmbientColor = D3DXCOLOR(sunLight.Ambient.r, sunLight.Ambient.g, sunLight.Ambient.b, sunLight.Ambient.a);
	m_uniforms.v4AmbientColor = D3DXVECTOR4(sunLight.Ambient.r, sunLight.Ambient.g, sunLight.Ambient.b, sunLight.Ambient.a);

	// 3. Zasieg i kolor mgly (fFogNear, fFogFar, vFogColor)
	float fFogNear = pEnvData->GetFogNearDistance();
	float fFogFar = pEnvData->GetFogFarDistance();
	m_uniforms.vFogColor = pEnvData->FogColor;

	if (pEnvData->bFogEnable)
	{
		const float fFogScaleLevel[3] = { 0.75f, 1.0f, 1.25f };
		int fogIdx = (iFogLevel >= 0 && iFogLevel < 3) ? iFogLevel : 1;
		fFogNear *= fFogScaleLevel[fogIdx];
		fFogFar *= fFogScaleLevel[fogIdx];

		m_uniforms.fFogNear = fFogNear;
		m_uniforms.fFogFar = fFogFar;
		const float fLinearScale = (fFogFar > fFogNear) ? (1.0f / (fFogFar - fFogNear)) : 0.0f;
		m_uniforms.v4FogParams = D3DXVECTOR4(fFogNear, fFogFar, fLinearScale, 1.0f);
	}
	else
	{
		m_uniforms.fFogNear = fFogNear;
		m_uniforms.fFogFar = fFogFar;
		const float fLinearScale = (fFogFar > fFogNear) ? (1.0f / (fFogFar - fFogNear)) : 0.0f;
		m_uniforms.v4FogParams = D3DXVECTOR4(fFogNear, fFogFar, fLinearScale, 0.0f);
	}

	// 4. Synchronizacja ze struktura TRenderFrameUniforms w CRenderQueueManager oraz CSkyBox
	SyncUniformsToRenderQueue();

	// Zsynchronizuj oswietlenie swiata i postaci oraz mgle w CFrameSnapshot
	TFrameSnapshot snapshot = CFrameSnapshot::Current();
	snapshot.lights[0] = pEnvData->DirLights[ENV_DIRLIGHT_BACKGROUND];
	if (pEnvData->bDirLightsEnable[ENV_DIRLIGHT_CHARACTER])
	{
		snapshot.lights[1] = pEnvData->DirLights[ENV_DIRLIGHT_CHARACTER];
	}
	snapshot.v4FogParams = m_uniforms.v4FogParams;
	CFrameSnapshot::SetCurrent(snapshot);

	// 5. Aktualizacja SpeedTree
	CSpeedTreeForestDirectX& rkForest = CSpeedTreeForestDirectX::Instance();
	rkForest.SetLight((const float*)&sunLight.Direction, (const float*)&sunLight.Ambient, (const float*)&sunLight.Diffuse);
	rkForest.SetFog(fFogNear, fFogFar);
	rkForest.SetWindStrength(pEnvData->fWindStrength);
}

void CMapOutdoorSky::SetSunLight(const D3DXVECTOR3& v3Direction, const D3DCOLORVALUE& cDiffuse)
{
	m_uniforms.vSunLightDir = v3Direction;
	m_uniforms.vSunLightColor = D3DXCOLOR(cDiffuse.r, cDiffuse.g, cDiffuse.b, cDiffuse.a);
	m_uniforms.v4SunColor = D3DXVECTOR4(cDiffuse.r, cDiffuse.g, cDiffuse.b, cDiffuse.a);
	SyncUniformsToRenderQueue();
}

void CMapOutdoorSky::SetAmbientColor(const D3DCOLORVALUE& cAmbient)
{
	m_uniforms.vAmbientColor = D3DXCOLOR(cAmbient.r, cAmbient.g, cAmbient.b, cAmbient.a);
	m_uniforms.v4AmbientColor = D3DXVECTOR4(cAmbient.r, cAmbient.g, cAmbient.b, cAmbient.a);
	SyncUniformsToRenderQueue();
}

void CMapOutdoorSky::SetFogRange(float fNear, float fFar, DWORD dwFogColor, bool bEnable)
{
	m_uniforms.fFogNear = fNear;
	m_uniforms.fFogFar = fFar;
	m_uniforms.vFogColor = D3DXCOLOR(dwFogColor);
	const float fLinearScale = (fFar > fNear) ? (1.0f / (fFar - fNear)) : 0.0f;
	m_uniforms.v4FogParams = D3DXVECTOR4(fNear, fFar, fLinearScale, bEnable ? 1.0f : 0.0f);
	SyncUniformsToRenderQueue();
}

void CMapOutdoorSky::SyncUniformsToRenderQueue()
{
	TRenderFrameUniforms merged = CRenderQueueManager::Instance().GetFrameUniforms();
	merged.vSunLightDir = m_uniforms.vSunLightDir;
	merged.vSunLightColor = m_uniforms.vSunLightColor;
	merged.vAmbientColor = m_uniforms.vAmbientColor;
	merged.vFogColor = m_uniforms.vFogColor;
	merged.fFogNear = m_uniforms.fFogNear;
	merged.fFogFar = m_uniforms.fFogFar;
	merged.v4SunColor = m_uniforms.v4SunColor;
	merged.v4AmbientColor = m_uniforms.v4AmbientColor;
	merged.v4FogParams = m_uniforms.v4FogParams;
	CRenderQueueManager::Instance().SetFrameUniforms(merged);
	if (m_pSkyBox)
	{
		m_pSkyBox->SetAtmosphereUniforms(merged);
	}
}

void CMapOutdoorSky::ConfigureSkyBox(const TEnvironmentData* pEnvData)
{
	if (!m_pSkyBox || !pEnvData)
		return;

	m_pSkyBox->SetSkyBoxScale(pEnvData->v3SkyBoxScale);
	m_pSkyBox->SetGradientLevel(pEnvData->bySkyBoxGradientLevelUpper, pEnvData->bySkyBoxGradientLevelLower);
	m_pSkyBox->SetRenderMode((pEnvData->bSkyBoxTextureRenderMode == TRUE) ? CSkyObject::SKY_RENDER_MODE_TEXTURE : CSkyObject::SKY_RENDER_MODE_DIFFUSE);

	for (int i = 0; i < 6; ++i)
	{
		if (!pEnvData->strSkyBoxFaceFileName[i].empty())
			m_pSkyBox->SetFaceTexture(pEnvData->strSkyBoxFaceFileName[i].c_str(), i);
	}

	if (!pEnvData->strCloudTextureFileName.empty())
		m_pSkyBox->SetCloudTexture(pEnvData->strCloudTextureFileName.c_str());

	m_pSkyBox->SetCloudScale(pEnvData->v2CloudScale);
	m_pSkyBox->SetCloudHeight(pEnvData->fCloudHeight);
	m_pSkyBox->SetCloudTextureScale(pEnvData->v2CloudTextureScale);
	m_pSkyBox->SetCloudScrollSpeed(pEnvData->v2CloudSpeed);
	m_pSkyBox->Refresh();

	m_pSkyBox->SetCloudColor(pEnvData->CloudGradientColor, pEnvData->CloudGradientColor, 1);

	if (!pEnvData->SkyBoxGradientColorVector.empty())
		m_pSkyBox->SetSkyColor(pEnvData->SkyBoxGradientColorVector, pEnvData->SkyBoxGradientColorVector, 1);

	m_pSkyBox->StartTransition();
}

void CMapOutdoorSky::Update()
{
	if (m_pSkyBox)
		m_pSkyBox->Update();
}

void CMapOutdoorSky::RenderSkyBox()
{
	if (m_pSkyBox)
		m_pSkyBox->Render();
}

void CMapOutdoorSky::RenderCloud()
{
	if (m_pSkyBox)
		m_pSkyBox->RenderCloud();
}
