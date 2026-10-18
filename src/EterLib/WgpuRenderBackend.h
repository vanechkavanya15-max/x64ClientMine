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

	// Getters diagnostyczne dla weryfikacji i testow jednostkowych
	const TRenderFramePacket& GetFramePacket() const noexcept { return m_framePacket; }
	const std::vector<TSkyBoxPacket>& GetCachedSkyBoxPackets() const noexcept { return m_cachedSkyBoxPackets; }
	const std::vector<TTerrainSectorPacket>& GetCachedTerrainPackets() const noexcept { return m_cachedTerrainPackets; }
	const std::vector<TActorRenderPacket>& GetCachedActorPackets() const noexcept { return m_cachedActorPackets; }
	const std::vector<TWaterPacket>& GetCachedWaterPackets() const noexcept { return m_cachedWaterPackets; }
	const std::vector<TSpeedTreePacket>& GetCachedSpeedTreePackets() const noexcept { return m_cachedSpeedTreePackets; }
	const std::vector<TEffectInstancePacket>& GetCachedEffectPackets() const noexcept { return m_cachedEffectPackets; }
	const std::vector<TUIBatchPacket>& GetCachedUIPackets() const noexcept { return m_cachedUIPackets; }
	const std::vector<TDungeonBlockPacket>& GetCachedDungeonPackets() const noexcept { return m_cachedDungeonPackets; }

private:
	HWND m_hWnd{ nullptr };
	unsigned int m_width{ 0 };
	unsigned int m_height{ 0 };
	bool m_bInitialized{ false };
	bool m_bFrameRendered{ false };
	TRenderFramePacket m_framePacket{};

	// Bufory glebokiego buforowania zapobiegajace Use-After-Free (T282)
	std::vector<TSkyBoxPacket> m_cachedSkyBoxPackets{};
	std::vector<TTerrainSectorPacket> m_cachedTerrainPackets{};
	std::vector<TActorRenderPacket> m_cachedActorPackets{};
	std::vector<TWaterPacket> m_cachedWaterPackets{};
	std::vector<TSpeedTreePacket> m_cachedSpeedTreePackets{};
	std::vector<TEffectInstancePacket> m_cachedEffectPackets{};
	std::vector<TUIBatchPacket> m_cachedUIPackets{};
	std::vector<TDungeonBlockPacket> m_cachedDungeonPackets{};
};
