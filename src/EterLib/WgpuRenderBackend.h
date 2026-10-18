#pragma once

#include "IRenderBackend.h"
#include <vector>
#include <cstdint>

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

	bool ReadbackFramebuffer(std::vector<uint8_t>& outPixels, uint32_t& outWidth, uint32_t& outHeight);
	std::vector<uint8_t> ReadbackFramebuffer(uint32_t* pOutWidth = nullptr, uint32_t* pOutHeight = nullptr);

	bool IsDllLoaded() const noexcept;
	static bool IsConfiguredInConfigJson();
	HMODULE GetModuleHandle() const noexcept;
	bool IsDeferred() const override { return true; }

private:
	HWND m_hWnd{ nullptr };
	unsigned int m_width{ 0 };
	unsigned int m_height{ 0 };
	bool m_bInitialized{ false };
	bool m_bFrameRendered{ false };
	TRenderFramePacket m_framePacket{};
	std::vector<TUIBatchPacket> m_cachedUIPackets{};
};
