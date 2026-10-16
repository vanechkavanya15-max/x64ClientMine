#pragma once

#include "EterLib/SkyBox.h"
#include "EterLib/RenderPacketTypes.h"
#include "EterLib/RenderQueueManager.h"
#include "MapType.h"

/**
 * @brief Manages sky atmosphere, sunlight parameters, ambient lighting,
 * and distance fog centralized into TRenderFrameUniforms (SRP compliance).
 */
class CMapOutdoorSky
{
public:
	CMapOutdoorSky();
	~CMapOutdoorSky();

	void Initialize(CSkyBox* pSkyBox);
	void Reset();

	// Centralized atmosphere and uniform synchronization
	void UpdateAtmosphere(const TEnvironmentData* pEnvData, int iFogLevel = 1);
	void SetSunLight(const D3DXVECTOR3& v3Direction, const D3DCOLORVALUE& cDiffuse);
	void SetAmbientColor(const D3DCOLORVALUE& cAmbient);
	void SetFogRange(float fNear, float fFar, DWORD dwFogColor, bool bEnable);

	// Direct uniform access
	const TRenderFrameUniforms& GetFrameUniforms() const { return m_uniforms; }
	TRenderFrameUniforms& GetFrameUniforms() { return m_uniforms; }

	// Sky rendering delegates
	void Update();
	void RenderSkyBox();
	void RenderCloud();

	// Configuration from environment data
	void ConfigureSkyBox(const TEnvironmentData* pEnvData);

private:
	void SyncUniformsToRenderQueue();

private:
	CSkyBox* m_pSkyBox;
	TRenderFrameUniforms m_uniforms{};
	bool m_bInitialized;
};
