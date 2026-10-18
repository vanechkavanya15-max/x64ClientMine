#include "StdAfx.h"
#include "WgpuRenderBackend.h"
#include "WgpuModule.h"
#include "RenderQueueManager.h"
#include "RenderOwnership.h"

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
	m_framePacket = TRenderFramePacket{};
	m_framePacket.uniforms = uniforms;
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
	m_framePacket.pUIPackets = pPackets;
	m_framePacket.dwUICount = count;
}

void CWgpuRenderBackend::RenderDungeonBlocks(const TDungeonBlockPacket* pPackets, unsigned int count)
{
	m_framePacket.pDungeonPackets = pPackets;
	m_framePacket.dwDungeonCount = count;
}

void CWgpuRenderBackend::EndFrame()
{
	if (m_bInitialized)
	{
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

bool CWgpuRenderBackend::CaptureFrame(const char* szFilename)
{
	return false;
}
