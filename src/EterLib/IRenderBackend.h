#pragma once

#include <windows.h>
#include "RenderPacketTypes.h"

class IRenderBackend
{
public:
	virtual ~IRenderBackend() = default;

	virtual bool Initialize(HWND hWnd, unsigned int width, unsigned int height) = 0;
	virtual void Shutdown() = 0;
	virtual void Resize(unsigned int width, unsigned int height) = 0;
	virtual void BeginFrame(const TRenderFrameUniforms& uniforms) = 0;

	virtual void RenderSkyBox(const TSkyBoxPacket* pPackets, unsigned int count) {}
	virtual void RenderTerrain(const TTerrainSectorPacket* pPackets, unsigned int count) = 0;
	virtual void RenderActors(const TActorRenderPacket* pPackets, unsigned int count) = 0;
	virtual void RenderWater(const TWaterPacket* pPackets, unsigned int count) = 0;
	virtual void RenderSpeedTree(const TSpeedTreePacket* pPackets, unsigned int count) = 0;
	virtual void RenderEffects(const TEffectInstancePacket* pPackets, unsigned int count) = 0;
	virtual void RenderUI(const TUIBatchPacket* pPackets, unsigned int count) = 0;
	virtual void RenderDungeonBlocks(const TDungeonBlockPacket* pPackets, unsigned int count) {}
	virtual void RenderWeaponTrace(const TWeaponTracePacket* pPackets, unsigned int count) {}
	virtual void RenderDebugLines(const TDebugLinePacket* pPackets, unsigned int count) {}

	virtual void EndFrame() = 0;
	virtual const char* GetBackendName() const = 0;
	virtual bool IsDeviceLost() const = 0;
	virtual bool CaptureFrame(const char* szFilename) = 0;
};
