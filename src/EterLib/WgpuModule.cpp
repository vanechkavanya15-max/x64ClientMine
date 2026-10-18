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

	if (pszDllPath && *pszDllPath)
	{
		m_hModule = ::LoadLibraryA(pszDllPath);
		if (!m_hModule && std::strcmp(pszDllPath, "m2_render_wgpu.dll") == 0)
		{
			const char* candidatePaths[] = {
				"Release/m2_render_wgpu.dll",
				"build/tests/Release/m2_render_wgpu.dll",
				"../build/bin/Release/m2_render_wgpu.dll",
				"../../build/bin/Release/m2_render_wgpu.dll",
				"E:/metin2ToRust/target/release/m2_render_wgpu.dll",
				"E:/m2dev client backlup/m2dev-client-main/m2_render_wgpu.dll",
				"E:/m2dev client backlup/build/bin/Release/m2_render_wgpu.dll"
			};

			for (const char* candidate : candidatePaths)
			{
				m_hModule = ::LoadLibraryA(candidate);
				if (m_hModule)
				{
					break;
				}
			}
		}
	}
	else
	{
		const char* candidatePaths[] = {
			"m2_render_wgpu.dll",
			"Release/m2_render_wgpu.dll",
			"build/bin/Release/m2_render_wgpu.dll",
			"build/tests/Release/m2_render_wgpu.dll",
			"../build/bin/Release/m2_render_wgpu.dll",
			"../../build/bin/Release/m2_render_wgpu.dll",
			"E:/metin2ToRust/target/release/m2_render_wgpu.dll",
			"E:/m2dev client backlup/m2dev-client-main/m2_render_wgpu.dll",
			"E:/m2dev client backlup/build/bin/Release/m2_render_wgpu.dll"
		};

		for (const char* candidate : candidatePaths)
		{
			m_hModule = ::LoadLibraryA(candidate);
			if (m_hModule)
			{
				break;
			}
		}
	}

	if (!m_hModule)
	{
		return false;
	}

	m_functions.initialize = reinterpret_cast<PFN_m2_wgpu_initialize>(::GetProcAddress(m_hModule, "m2_render_wgpu_initialize"));
	if (!m_functions.initialize)
	{
		m_functions.initialize = reinterpret_cast<PFN_m2_wgpu_initialize>(::GetProcAddress(m_hModule, "m2_wgpu_initialize"));
	}

	m_functions.render_frame = reinterpret_cast<PFN_m2_wgpu_render_frame>(::GetProcAddress(m_hModule, "m2_render_wgpu_render_frame"));
	if (!m_functions.render_frame)
	{
		m_functions.render_frame = reinterpret_cast<PFN_m2_wgpu_render_frame>(::GetProcAddress(m_hModule, "m2_wgpu_render_frame"));
	}

	m_functions.register_texture = reinterpret_cast<PFN_m2_wgpu_register_texture>(::GetProcAddress(m_hModule, "m2_render_wgpu_register_texture"));
	if (!m_functions.register_texture)
	{
		m_functions.register_texture = reinterpret_cast<PFN_m2_wgpu_register_texture>(::GetProcAddress(m_hModule, "m2_wgpu_register_texture"));
	}

	m_functions.unregister_texture = reinterpret_cast<PFN_m2_wgpu_unregister_texture>(::GetProcAddress(m_hModule, "m2_render_wgpu_unregister_texture"));
	if (!m_functions.unregister_texture)
	{
		m_functions.unregister_texture = reinterpret_cast<PFN_m2_wgpu_unregister_texture>(::GetProcAddress(m_hModule, "m2_wgpu_unregister_texture"));
	}

	m_functions.resize = reinterpret_cast<PFN_m2_wgpu_resize>(::GetProcAddress(m_hModule, "m2_render_wgpu_resize"));
	if (!m_functions.resize)
	{
		m_functions.resize = reinterpret_cast<PFN_m2_wgpu_resize>(::GetProcAddress(m_hModule, "m2_wgpu_resize"));
	}

	m_functions.destroy = reinterpret_cast<PFN_m2_wgpu_destroy>(::GetProcAddress(m_hModule, "m2_render_wgpu_destroy"));
	if (!m_functions.destroy)
	{
		m_functions.destroy = reinterpret_cast<PFN_m2_wgpu_destroy>(::GetProcAddress(m_hModule, "m2_wgpu_destroy"));
	}
	if (!m_functions.destroy)
	{
		m_functions.destroy = reinterpret_cast<PFN_m2_wgpu_destroy>(::GetProcAddress(m_hModule, "m2_wgpu_shutdown"));
	}

	m_functions.read_rgba_pixels = reinterpret_cast<PFN_m2_wgpu_read_rgba_pixels>(::GetProcAddress(m_hModule, "m2_render_wgpu_read_rgba_pixels"));
	if (!m_functions.read_rgba_pixels)
	{
		m_functions.read_rgba_pixels = reinterpret_cast<PFN_m2_wgpu_read_rgba_pixels>(::GetProcAddress(m_hModule, "m2_wgpu_read_rgba_pixels"));
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
	std::lock_guard<std::mutex> lock(m_mutex);
	if (m_functions.destroy)
	{
		m_functions.destroy();
	}
	m_functions = SWgpuFunctions{};

	if (m_hModule)
	{
		::FreeLibrary(m_hModule);
		m_hModule = nullptr;
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
	std::lock_guard<std::mutex> lock(m_mutex);
	if (!m_hModule || !m_functions.initialize)
		return false;

	++m_transactionCount;
	return m_functions.initialize(hWnd, width, height);
}

void CWgpuModule::RenderFrame(const TRenderFramePacket* pPacket)
{
	if (!pPacket)
		return;

	std::lock_guard<std::mutex> lock(m_mutex);
	if (!m_hModule || !m_functions.render_frame)
		return;

	++m_transactionCount;
	m_functions.render_frame(pPacket);
}

bool CWgpuModule::RegisterTexture(uintptr_t dwTextureID, uint32_t width, uint32_t height, uint32_t format, const uint8_t* pData, size_t dataSize)
{
	if (!dwTextureID || !pData || dataSize == 0)
		return false;

	std::lock_guard<std::mutex> lock(m_mutex);
	if (!m_hModule || !m_functions.register_texture)
		return false;

	++m_transactionCount;
	m_functions.register_texture(dwTextureID, width, height, format, pData, dataSize);
	return true;
}

void CWgpuModule::UnregisterTexture(uintptr_t dwTextureID)
{
	if (!dwTextureID)
		return;

	std::lock_guard<std::mutex> lock(m_mutex);
	if (!m_hModule || !m_functions.unregister_texture)
		return;

	++m_transactionCount;
	m_functions.unregister_texture(dwTextureID);
}

void CWgpuModule::Resize(uint32_t width, uint32_t height)
{
	std::lock_guard<std::mutex> lock(m_mutex);
	if (!m_hModule || !m_functions.resize)
		return;

	++m_transactionCount;
	m_functions.resize(width, height);
}

void CWgpuModule::Destroy()
{
	std::lock_guard<std::mutex> lock(m_mutex);
	if (!m_hModule || !m_functions.destroy)
		return;

	++m_transactionCount;
	m_functions.destroy();
}

size_t CWgpuModule::ReadRgbaPixels(uint8_t* pDstBuffer, size_t maxLen)
{
	if (!pDstBuffer || maxLen == 0)
		return 0;

	std::lock_guard<std::mutex> lock(m_mutex);
	if (!m_hModule || !m_functions.read_rgba_pixels)
		return 0;

	++m_transactionCount;
	return m_functions.read_rgba_pixels(pDstBuffer, maxLen);
}

bool CWgpuModule::ReadbackFramebuffer(uint8_t* pDstBuffer, size_t bufferSize, uint32_t* pOutWidth, uint32_t* pOutHeight)
{
	size_t copied = ReadRgbaPixels(pDstBuffer, bufferSize);
	return (copied > 0);
}

