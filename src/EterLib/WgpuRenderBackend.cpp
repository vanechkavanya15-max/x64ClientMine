#include "StdAfx.h"
#include "WgpuRenderBackend.h"
#include "WgpuModule.h"
#include "RenderQueueManager.h"
#include "RenderOwnership.h"
#include <stb_image_write.h>
#include <algorithm>

CWgpuRenderBackend::CWgpuRenderBackend() = default;

CWgpuRenderBackend::~CWgpuRenderBackend()
{
	Shutdown();
}

bool CWgpuRenderBackend::Initialize(HWND hWnd, unsigned int width, unsigned int height)
{
	m_hWnd = hWnd;
	m_width = width;
	m_height = height;

	CWgpuModule& module = CWgpuModule::Instance();
	if (!module.IsLoaded())
	{
		if (!module.Load())
		{
			m_bInitialized = false;
			return false;
		}
	}

	if (module.Initialize(hWnd, static_cast<uint32_t>(width), static_cast<uint32_t>(height)))
	{
		m_bInitialized = true;
		CRenderQueueManager::Instance().SetActiveBackend(this);
		return true;
	}

	m_bInitialized = false;
	return false;
}

void CWgpuRenderBackend::Shutdown()
{
	if (m_bInitialized)
	{
		CWgpuModule::Instance().Destroy();
	}
	m_bInitialized = false;
}

void CWgpuRenderBackend::Resize(unsigned int width, unsigned int height)
{
	m_width = width;
	m_height = height;
	if (m_bInitialized)
	{
		CWgpuModule::Instance().Resize(static_cast<uint32_t>(width), static_cast<uint32_t>(height));
	}
}

void CWgpuRenderBackend::BeginFrame(const TRenderFrameUniforms& uniforms)
{
	m_cachedUIPackets.clear();
	m_framePacket = TRenderFramePacket{};
	m_framePacket.uniforms = uniforms;
	m_bFrameRendered = false;
}

void CWgpuRenderBackend::RenderSkyBox(const TSkyBoxPacket* pPackets, unsigned int count)
{
	m_framePacket.pSkyBoxPackets = pPackets;
	m_framePacket.dwSkyBoxCount = count;
}

void CWgpuRenderBackend::RenderTerrain(const TTerrainSectorPacket* pPackets, unsigned int count)
{
	m_framePacket.pTerrainPackets = pPackets;
	m_framePacket.dwTerrainCount = count;
}

void CWgpuRenderBackend::RenderActors(const TActorRenderPacket* pPackets, unsigned int count)
{
	m_framePacket.pActorPackets = pPackets;
	m_framePacket.dwActorCount = count;
}

void CWgpuRenderBackend::RenderWater(const TWaterPacket* pPackets, unsigned int count)
{
	m_framePacket.pWaterPackets = pPackets;
	m_framePacket.dwWaterCount = count;
}

void CWgpuRenderBackend::RenderSpeedTree(const TSpeedTreePacket* pPackets, unsigned int count)
{
	m_framePacket.pSpeedTreePackets = pPackets;
	m_framePacket.dwSpeedTreeCount = count;
}

void CWgpuRenderBackend::RenderEffects(const TEffectInstancePacket* pPackets, unsigned int count)
{
	m_framePacket.pEffectPackets = pPackets;
	m_framePacket.dwEffectCount = count;
}

void CWgpuRenderBackend::RenderUI(const TUIBatchPacket* pPackets, unsigned int count)
{
	if (pPackets && count > 0)
	{
		m_cachedUIPackets.insert(m_cachedUIPackets.end(), pPackets, pPackets + count);
		m_framePacket.pUIPackets = m_cachedUIPackets.data();
		m_framePacket.dwUICount = static_cast<uint32_t>(m_cachedUIPackets.size());
	}
}

void CWgpuRenderBackend::RenderDungeonBlocks(const TDungeonBlockPacket* pPackets, unsigned int count)
{
	m_framePacket.pDungeonPackets = pPackets;
	m_framePacket.dwDungeonCount = count;
}

void CWgpuRenderBackend::EndFrame()
{
	if (m_bInitialized && !m_bFrameRendered)
	{
		m_bFrameRendered = true;
		CWgpuModule::Instance().RenderFrame(&m_framePacket);
	}
}

const char* CWgpuRenderBackend::GetBackendName() const
{
	return IsDllLoaded() ? "Rust WGPU Drop-In (m2_render_wgpu.dll)" : "Rust WGPU Drop-In (Unloaded)";
}

bool CWgpuRenderBackend::IsDllLoaded() const noexcept
{
	return CWgpuModule::Instance().IsLoaded();
}

HMODULE CWgpuRenderBackend::GetModuleHandle() const noexcept
{
	return CWgpuModule::Instance().GetHandle();
}

bool CWgpuRenderBackend::IsDeviceLost() const
{
	return false;
}

bool CWgpuRenderBackend::IsConfiguredInConfigJson()
{
	return RenderOwnership::IsWgpuConfigured();
}

bool CWgpuRenderBackend::ReadbackFramebuffer(std::vector<uint8_t>& outPixels, uint32_t& outWidth, uint32_t& outHeight)
{
	outWidth = m_width;
	outHeight = m_height;
	if (m_width == 0 || m_height == 0)
		return false;

	size_t reqSize = static_cast<size_t>(m_width) * static_cast<size_t>(m_height) * 4;
	outPixels.resize(reqSize);

	CWgpuModule& module = CWgpuModule::Instance();
	if (module.IsLoaded())
	{
		size_t copied = module.ReadRgbaPixels(outPixels.data(), reqSize);
		if (copied > 0)
		{
			outWidth = m_width;
			outHeight = m_height;
			return true;
		}
	}

	// Fallback/dummy frame when module is offline/mock
	std::fill(outPixels.begin(), outPixels.end(), static_cast<uint8_t>(0));
	for (size_t i = 3; i < reqSize; i += 4)
	{
		outPixels[i] = 255;
	}
	return true;
}

std::vector<uint8_t> CWgpuRenderBackend::ReadbackFramebuffer(uint32_t* pOutWidth, uint32_t* pOutHeight)
{
	std::vector<uint8_t> pixels;
	uint32_t w = 0, h = 0;
	ReadbackFramebuffer(pixels, w, h);
	if (pOutWidth) *pOutWidth = w;
	if (pOutHeight) *pOutHeight = h;
	return pixels;
}

bool CWgpuRenderBackend::CaptureFrame(const char* szFilename)
{
	if (!szFilename || !szFilename[0])
		return false;

	std::vector<uint8_t> pixels;
	uint32_t w = 0, h = 0;
	if (!ReadbackFramebuffer(pixels, w, h) || pixels.empty() || w == 0 || h == 0)
		return false;

	std::string fileName = szFilename;
	if (fileName.length() < 4 || (fileName.compare(fileName.length() - 4, 4, ".png") != 0 && fileName.compare(fileName.length() - 4, 4, ".PNG") != 0))
	{
		fileName += ".png";
	}

	return stbi_write_png(fileName.c_str(), static_cast<int>(w), static_cast<int>(h), 4, pixels.data(), static_cast<int>(w * 4)) != 0;
}
