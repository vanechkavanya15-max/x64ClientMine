#include "StdAfx.h"
#include "WgpuRenderBackend.h"
#include "D3D9RenderBackend.h"

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

	if (!m_hModule)
	{
		m_hModule = ::LoadLibraryA("m2_render_wgpu.dll");
		if (m_hModule)
		{
			m_pfnInitialize = reinterpret_cast<PFN_m2_wgpu_initialize>(::GetProcAddress(m_hModule, "m2_wgpu_initialize"));
			m_pfnRenderFrame = reinterpret_cast<PFN_m2_wgpu_render_frame>(::GetProcAddress(m_hModule, "m2_wgpu_render_frame"));
			m_pfnResize = reinterpret_cast<PFN_m2_wgpu_resize>(::GetProcAddress(m_hModule, "m2_wgpu_resize"));
			m_pfnShutdown = reinterpret_cast<PFN_m2_wgpu_shutdown>(::GetProcAddress(m_hModule, "m2_wgpu_shutdown"));
			m_pfnRegisterTexture = reinterpret_cast<PFN_m2_wgpu_register_texture>(::GetProcAddress(m_hModule, "m2_wgpu_register_texture"));
		}
	}

	if (m_pfnInitialize && m_pfnRenderFrame)
	{
		m_bInitialized = m_pfnInitialize(hWnd, static_cast<uint32_t>(width), static_cast<uint32_t>(height));
		return m_bInitialized;
	}

	return false;
}

void CWgpuRenderBackend::Shutdown()
{
	if (m_bInitialized && m_pfnShutdown)
	{
		m_pfnShutdown();
	}
	m_bInitialized = false;

	if (m_hModule)
	{
		::FreeLibrary(m_hModule);
		m_hModule = nullptr;
	}
	m_pfnInitialize = nullptr;
	m_pfnRenderFrame = nullptr;
	m_pfnResize = nullptr;
	m_pfnShutdown = nullptr;
	m_pfnRegisterTexture = nullptr;
}

void CWgpuRenderBackend::RegisterTexture(uintptr_t dwTextureID, uint32_t width, uint32_t height, uint32_t format, const uint8_t* pData, size_t dataSize)
{
	if (m_bInitialized && m_pfnRegisterTexture)
	{
		m_pfnRegisterTexture(dwTextureID, width, height, format, pData, dataSize);
	}
}

void CWgpuRenderBackend::Resize(unsigned int width, unsigned int height)
{
	m_width = width;
	m_height = height;
	if (m_bInitialized && m_pfnResize)
	{
		m_pfnResize(static_cast<uint32_t>(width), static_cast<uint32_t>(height));
	}
}

void CWgpuRenderBackend::BeginFrame(const TRenderFrameUniforms& uniforms)
{
	m_framePacket = TRenderFramePacket{};
	m_framePacket.uniforms = uniforms;
	CD3D9RenderBackend::Instance().BeginFrame(uniforms);
}

void CWgpuRenderBackend::RenderSkyBox(const TSkyBoxPacket* pPackets, unsigned int count)
{
	m_framePacket.pSkyBoxPackets = pPackets;
	m_framePacket.dwSkyBoxCount = count;
	CD3D9RenderBackend::Instance().RenderSkyBox(pPackets, count);
}

void CWgpuRenderBackend::RenderTerrain(const TTerrainSectorPacket* pPackets, unsigned int count)
{
	m_framePacket.pTerrainPackets = pPackets;
	m_framePacket.dwTerrainCount = count;
	CD3D9RenderBackend::Instance().RenderTerrain(pPackets, count);
}

void CWgpuRenderBackend::RenderActors(const TActorRenderPacket* pPackets, unsigned int count)
{
	m_framePacket.pActorPackets = pPackets;
	m_framePacket.dwActorCount = count;
	CD3D9RenderBackend::Instance().RenderActors(pPackets, count);
}

void CWgpuRenderBackend::RenderWater(const TWaterPacket* pPackets, unsigned int count)
{
	m_framePacket.pWaterPackets = pPackets;
	m_framePacket.dwWaterCount = count;
	CD3D9RenderBackend::Instance().RenderWater(pPackets, count);
}

void CWgpuRenderBackend::RenderSpeedTree(const TSpeedTreePacket* pPackets, unsigned int count)
{
	m_framePacket.pSpeedTreePackets = pPackets;
	m_framePacket.dwSpeedTreeCount = count;
	CD3D9RenderBackend::Instance().RenderSpeedTree(pPackets, count);
}

void CWgpuRenderBackend::RenderEffects(const TEffectInstancePacket* pPackets, unsigned int count)
{
	m_framePacket.pEffectPackets = pPackets;
	m_framePacket.dwEffectCount = count;
	CD3D9RenderBackend::Instance().RenderEffects(pPackets, count);
}

void CWgpuRenderBackend::RenderUI(const TUIBatchPacket* pPackets, unsigned int count)
{
	m_framePacket.pUIPackets = pPackets;
	m_framePacket.dwUICount = count;
	CD3D9RenderBackend::Instance().RenderUI(pPackets, count);
}

void CWgpuRenderBackend::RenderDungeonBlocks(const TDungeonBlockPacket* pPackets, unsigned int count)
{
	CD3D9RenderBackend::Instance().RenderDungeonBlocks(pPackets, count);
}

void CWgpuRenderBackend::EndFrame()
{
	if (m_bInitialized && m_pfnRenderFrame)
	{
		m_pfnRenderFrame(&m_framePacket);
	}
	CD3D9RenderBackend::Instance().EndFrame();
}

const char* CWgpuRenderBackend::GetBackendName() const
{
	return IsDllLoaded() ? "Rust WGPU Drop-In (m2_render_wgpu.dll)" : "Rust WGPU Bridge (D3D9 Fallback)";
}

#include <fstream>
#include <string>
#include <cstdlib>

bool CWgpuRenderBackend::IsDeviceLost() const
{
	return CD3D9RenderBackend::Instance().IsDeviceLost();
}

bool CWgpuRenderBackend::IsConfiguredInConfigJson()
{
	if (const char* envVal = std::getenv("M2_GRAPHICS_ENGINE"))
	{
		if (std::string(envVal) == "wgpu")
			return true;
	}

	std::ifstream file("config.json");
	if (!file.is_open())
		return false;

	std::string content((std::istreambuf_iterator<char>(file)), std::istreambuf_iterator<char>());
	const auto keyPos = content.find("\"graphics_engine\"");
	if (keyPos == std::string::npos)
		return false;

	const auto wgpuPos = content.find("\"wgpu\"", keyPos);
	return wgpuPos != std::string::npos && (wgpuPos - keyPos) < 64;
}
