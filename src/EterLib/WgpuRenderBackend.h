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
	bool CaptureFrame(const char* szFilename) override;

	bool IsDllLoaded() const noexcept { return m_hModule != nullptr && m_pfnRenderFrame != nullptr; }
	static bool IsConfiguredInConfigJson();
	void RegisterTexture(uintptr_t dwTextureID, uint32_t width, uint32_t height, uint32_t format, const uint8_t* pData, size_t dataSize);
	HMODULE GetModuleHandle() const noexcept { return m_hModule; }

private:
	using PFN_m2_wgpu_initialize = bool(__cdecl*)(HWND, uint32_t, uint32_t);
	using PFN_m2_wgpu_render_frame = void(__cdecl*)(const TRenderFramePacket*);
	using PFN_m2_wgpu_resize = void(__cdecl*)(uint32_t, uint32_t);
	using PFN_m2_wgpu_shutdown = void(__cdecl*)();
	using PFN_m2_wgpu_register_texture = void(__cdecl*)(uintptr_t, uint32_t, uint32_t, uint32_t, const uint8_t*, size_t);

	HMODULE m_hModule{ nullptr };
	PFN_m2_wgpu_initialize m_pfnInitialize{ nullptr };
	PFN_m2_wgpu_render_frame m_pfnRenderFrame{ nullptr };
	PFN_m2_wgpu_resize m_pfnResize{ nullptr };
	PFN_m2_wgpu_shutdown m_pfnShutdown{ nullptr };
	PFN_m2_wgpu_register_texture m_pfnRegisterTexture{ nullptr };

	HWND m_hWnd{ nullptr };
	unsigned int m_width{ 0 };
	unsigned int m_height{ 0 };
	bool m_bInitialized{ false };
	TRenderFramePacket m_framePacket{};
};
