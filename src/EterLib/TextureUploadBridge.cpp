#include "StdAfx.h"
#include "TextureUploadBridge.h"
#include "WgpuRenderBackend.h"
#include "RenderQueueManager.h"
#include <mutex>

static std::mutex s_bridgeMutex;

CTextureUploadBridge::CTextureUploadBridge() = default;

CTextureUploadBridge::~CTextureUploadBridge()
{
	Shutdown();
}

bool CTextureUploadBridge::Initialize()
{
	std::lock_guard<std::mutex> lock(s_bridgeMutex);
	return EnsureWgpuLoaded();
}

void CTextureUploadBridge::Shutdown()
{
	std::lock_guard<std::mutex> lock(s_bridgeMutex);
	m_pfnRegisterTexture = nullptr;
	m_pfnUnregisterTexture = nullptr;

	if (m_bOwnsModule && m_hWgpuModule)
	{
		::FreeLibrary(m_hWgpuModule);
	}
	m_hWgpuModule = nullptr;
	m_bOwnsModule = false;
	m_registeredCount = 0;
}

bool CTextureUploadBridge::EnsureWgpuLoaded()
{
	if (m_pfnRegisterTexture)
		return true;

	// 1. Sprawdz czy modul m2_render_wgpu.dll jest juz zaladowany w procesie
	HMODULE hModule = ::GetModuleHandleA("m2_render_wgpu.dll");

	// 2. Jesli nie, ale konfiguracja wskazuje na wgpu, sprobuj zaladowac biblioteke
	if (!hModule && CWgpuRenderBackend::IsConfiguredInConfigJson())
	{
		hModule = ::LoadLibraryA("m2_render_wgpu.dll");
		if (hModule)
		{
			m_bOwnsModule = true;
		}
	}

	if (!hModule)
		return false;

	m_hWgpuModule = hModule;
	m_pfnRegisterTexture = reinterpret_cast<PFN_m2_wgpu_register_texture>(
		::GetProcAddress(hModule, "m2_wgpu_register_texture")
	);
	m_pfnUnregisterTexture = reinterpret_cast<PFN_m2_wgpu_unregister_texture>(
		::GetProcAddress(hModule, "m2_wgpu_unregister_texture")
	);

	return (m_pfnRegisterTexture != nullptr);
}

bool CTextureUploadBridge::IsWgpuActive() const
{
	// Sprawdz czy aktywny backend w CRenderQueueManager to WGPU
	IRenderBackend* pActive = CRenderQueueManager::Instance().GetActiveBackend();
	if (pActive && pActive == &CWgpuRenderBackend::Instance())
		return true;

	// Sprawdz czy instancja CWgpuRenderBackend ma zaladowana biblioteke
	if (CWgpuRenderBackend::Instance().IsDllLoaded())
		return true;

	// Sprawdz czy biblioteka m2_render_wgpu.dll jest obecna w pamieci procesu
	if (::GetModuleHandleA("m2_render_wgpu.dll") != nullptr)
		return true;

	// Sprawdz czy config.json lub zmienna M2_GRAPHICS_ENGINE wskazuje wgpu
	if (CWgpuRenderBackend::IsConfiguredInConfigJson())
		return true;

	return false;
}

bool CTextureUploadBridge::RegisterTexture(
	uintptr_t dwTextureID,
	uint32_t width,
	uint32_t height,
	uint32_t format,
	const uint8_t* pData,
	size_t dataSize)
{
	if (!dwTextureID || !pData || dataSize == 0)
		return false;

	if (!IsWgpuActive())
		return false;

	{
		std::lock_guard<std::mutex> lock(s_bridgeMutex);
		if (!EnsureWgpuLoaded())
			return false;
	}

	if (m_pfnRegisterTexture)
	{
		m_pfnRegisterTexture(dwTextureID, width, height, format, pData, dataSize);
		++m_registeredCount;
		return true;
	}

	return false;
}

bool CTextureUploadBridge::RegisterTexture(
	uintptr_t dwTextureID,
	uint32_t width,
	uint32_t height,
	uint32_t format,
	const void* pData,
	size_t dataSize)
{
	return RegisterTexture(
		dwTextureID,
		width,
		height,
		format,
		reinterpret_cast<const uint8_t*>(pData),
		dataSize
	);
}

void CTextureUploadBridge::UnregisterTexture(uintptr_t dwTextureID)
{
	if (!dwTextureID)
		return;

	if (m_pfnUnregisterTexture)
	{
		m_pfnUnregisterTexture(dwTextureID);
	}
}
