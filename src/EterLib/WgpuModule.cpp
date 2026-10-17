#include "StdAfx.h"
#include "WgpuModule.h"

CWgpuModule::CWgpuModule() = default;

CWgpuModule::~CWgpuModule()
{
	FreeLibrary();
}

bool CWgpuModule::Load(const char* pszDllPath)
{
	std::lock_guard<std::mutex> lock(m_mutex);
	if (m_hModule != nullptr)
	{
		return true;
	}

	const char* path = (pszDllPath && *pszDllPath) ? pszDllPath : "m2_render_wgpu.dll";
	m_hModule = ::LoadLibraryA(path);
	if (!m_hModule)
	{
		return false;
	}

	m_functions.initialize = reinterpret_cast<PFN_m2_wgpu_initialize>(::GetProcAddress(m_hModule, "m2_wgpu_initialize"));
	m_functions.render_frame = reinterpret_cast<PFN_m2_wgpu_render_frame>(::GetProcAddress(m_hModule, "m2_wgpu_render_frame"));
	m_functions.register_texture = reinterpret_cast<PFN_m2_wgpu_register_texture>(::GetProcAddress(m_hModule, "m2_wgpu_register_texture"));
	m_functions.unregister_texture = reinterpret_cast<PFN_m2_wgpu_unregister_texture>(::GetProcAddress(m_hModule, "m2_wgpu_unregister_texture"));
	m_functions.resize = reinterpret_cast<PFN_m2_wgpu_resize>(::GetProcAddress(m_hModule, "m2_wgpu_resize"));
	m_functions.destroy = reinterpret_cast<PFN_m2_wgpu_destroy>(::GetProcAddress(m_hModule, "m2_wgpu_destroy"));
	if (!m_functions.destroy)
	{
		m_functions.destroy = reinterpret_cast<PFN_m2_wgpu_destroy>(::GetProcAddress(m_hModule, "m2_wgpu_shutdown"));
	}

	// Jezeli biblioteka nie eksportuje kluczowych funkcji inicjalizacji i renderowania
	if (!m_functions.initialize || !m_functions.render_frame)
	{
		::FreeLibrary(m_hModule);
		m_hModule = nullptr;
		m_functions = SWgpuFunctions{};
		return false;
	}

	return true;
}

void CWgpuModule::FreeLibrary()
{
	HMODULE hToFree = nullptr;
	PFN_m2_wgpu_destroy pfnDestroy = nullptr;

	{
		std::lock_guard<std::mutex> lock(m_mutex);
		pfnDestroy = m_functions.destroy;
		m_functions = SWgpuFunctions{};
		hToFree = m_hModule;
		m_hModule = nullptr;
	}

	if (pfnDestroy)
	{
		pfnDestroy();
	}

	if (hToFree)
	{
		::FreeLibrary(hToFree);
	}
}

bool CWgpuModule::IsLoaded() const
{
	std::lock_guard<std::mutex> lock(m_mutex);
	return (m_hModule != nullptr && m_functions.initialize != nullptr);
}

HMODULE CWgpuModule::GetHandle() const
{
	std::lock_guard<std::mutex> lock(m_mutex);
	return m_hModule;
}

const SWgpuFunctions& CWgpuModule::GetFunctions() const
{
	std::lock_guard<std::mutex> lock(m_mutex);
	return m_functions;
}

bool CWgpuModule::Initialize(HWND hWnd, uint32_t width, uint32_t height)
{
	PFN_m2_wgpu_initialize pfn = nullptr;
	{
		std::lock_guard<std::mutex> lock(m_mutex);
		if (!m_hModule)
			return false;
		pfn = m_functions.initialize;
	}

	if (pfn)
	{
		++m_transactionCount;
		return pfn(hWnd, width, height);
	}
	return false;
}

void CWgpuModule::RenderFrame(const TRenderFramePacket* pPacket)
{
	if (!pPacket)
		return;

	PFN_m2_wgpu_render_frame pfn = nullptr;
	{
		std::lock_guard<std::mutex> lock(m_mutex);
		if (!m_hModule)
			return;
		pfn = m_functions.render_frame;
	}

	if (pfn)
	{
		++m_transactionCount;
		pfn(pPacket);
	}
}

bool CWgpuModule::RegisterTexture(uintptr_t dwTextureID, uint32_t width, uint32_t height, uint32_t format, const uint8_t* pData, size_t dataSize)
{
	if (!dwTextureID || !pData || dataSize == 0)
		return false;

	PFN_m2_wgpu_register_texture pfn = nullptr;
	{
		std::lock_guard<std::mutex> lock(m_mutex);
		if (!m_hModule)
			return false;
		pfn = m_functions.register_texture;
	}

	if (pfn)
	{
		++m_transactionCount;
		pfn(dwTextureID, width, height, format, pData, dataSize);
		return true;
	}
	return false;
}

void CWgpuModule::UnregisterTexture(uintptr_t dwTextureID)
{
	if (!dwTextureID)
		return;

	PFN_m2_wgpu_unregister_texture pfn = nullptr;
	{
		std::lock_guard<std::mutex> lock(m_mutex);
		if (!m_hModule)
			return; // Bezpieczny no-op, gdy modul jest niezaladowany lub zamkniety
		pfn = m_functions.unregister_texture;
	}

	if (pfn)
	{
		++m_transactionCount;
		pfn(dwTextureID);
	}
}

void CWgpuModule::Resize(uint32_t width, uint32_t height)
{
	PFN_m2_wgpu_resize pfn = nullptr;
	{
		std::lock_guard<std::mutex> lock(m_mutex);
		if (!m_hModule)
			return;
		pfn = m_functions.resize;
	}

	if (pfn)
	{
		++m_transactionCount;
		pfn(width, height);
	}
}

void CWgpuModule::Destroy()
{
	PFN_m2_wgpu_destroy pfn = nullptr;
	{
		std::lock_guard<std::mutex> lock(m_mutex);
		if (!m_hModule)
			return;
		pfn = m_functions.destroy;
	}

	if (pfn)
	{
		++m_transactionCount;
		pfn();
	}
}
