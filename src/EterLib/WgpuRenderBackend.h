#pragma once

#include "IRenderBackend.h"

class CWgpuRenderBackend final : public IRenderBackend
{
public:
	static CWgpuRenderBackend& Instance()
	{
		static CWgpuRenderBackend s_instance;
		return s_instance;
	}

	CWgpuRenderBackend();
	~CWgpuRenderBackend() override;

	bool Initialize(HWND hWnd, unsigned int width, unsigned int height) override;
	void Shutdown() override;
	void Resize(unsigned int width, unsigned int height) override;
	void BeginFrame(const TRenderFrameUniforms& uniforms) override;

	void RenderSkyBox(const TSkyBoxPacket* pPackets, unsigned int count) override;
	void RenderTerrain(const TTerrainSectorPacket* pPackets, unsigned int count) override;
	void RenderActors(const TActorRenderPacket* pPackets, unsigned int count) override;
	void RenderWater(const TWaterPacket* pPackets, unsigned int count) override;
	void RenderSpeedTree(const TSpeedTreePacket* pPackets, unsigned int count) override;
	void RenderEffects(const TEffectInstancePacket* pPackets, unsigned int count) override;
	void RenderUI(const TUIBatchPacket* pPackets, unsigned int count) override;
	void RenderDungeonBlocks(const TDungeonBlockPacket* pPackets, unsigned int count) override;

	void EndFrame() override;
	const char* GetBackendName() const override;
	bool IsDeviceLost() const override;

	bool IsDllLoaded() const noexcept;
	static bool IsConfiguredInConfigJson();
	HMODULE GetModuleHandle() const noexcept;

private:
	HWND m_hWnd{ nullptr };
	unsigned int m_width{ 0 };
	unsigned int m_height{ 0 };
	bool m_bInitialized{ false };
	TRenderFramePacket m_framePacket{};
};
