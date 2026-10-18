#pragma once

#include <windows.h>
#include <cstdint>
#include <cstddef>
#include <atomic>
#include <mutex>

struct TRenderFramePacket;

// Typy wskaznikow funkcji eksportowanych przez DLL m2_render_wgpu.dll
using PFN_m2_wgpu_initialize = bool(__cdecl*)(HWND, uint32_t, uint32_t);
using PFN_m2_wgpu_render_frame = void(__cdecl*)(const TRenderFramePacket*);
using PFN_m2_wgpu_register_texture = void(__cdecl*)(uintptr_t, uint32_t, uint32_t, uint32_t, const uint8_t*, size_t);
using PFN_m2_wgpu_unregister_texture = void(__cdecl*)(uintptr_t);
using PFN_m2_wgpu_resize = void(__cdecl*)(uint32_t, uint32_t);
using PFN_m2_wgpu_destroy = void(__cdecl*)();
using PFN_m2_wgpu_shutdown = PFN_m2_wgpu_destroy;
using PFN_m2_wgpu_read_rgba_pixels = size_t(__cdecl*)(uint8_t*, size_t);
using PFN_m2_render_wgpu_read_rgba_pixels = PFN_m2_wgpu_read_rgba_pixels;

struct SWgpuFunctions
{
	PFN_m2_wgpu_initialize initialize{ nullptr };
	PFN_m2_wgpu_render_frame render_frame{ nullptr };
	PFN_m2_wgpu_register_texture register_texture{ nullptr };
	PFN_m2_wgpu_unregister_texture unregister_texture{ nullptr };
	PFN_m2_wgpu_resize resize{ nullptr };
	PFN_m2_wgpu_destroy destroy{ nullptr };
	PFN_m2_wgpu_read_rgba_pixels read_rgba_pixels{ nullptr };
};

class CWgpuModule
{
public:
	static CWgpuModule& Instance()
	{
		static CWgpuModule s_instance;
		return s_instance;
	}

	bool Load(const char* pszDllPath = "m2_render_wgpu.dll");
	void FreeLibrary();

	bool IsLoaded() const;
	HMODULE GetHandle() const;
	const SWgpuFunctions& GetFunctions() const;

	bool Initialize(HWND hWnd, uint32_t width, uint32_t height);
	void RenderFrame(const TRenderFramePacket* pPacket);
	bool RegisterTexture(uintptr_t dwTextureID, uint32_t width, uint32_t height, uint32_t format, const uint8_t* pData, size_t dataSize);
	void UnregisterTexture(uintptr_t dwTextureID);
	void Resize(uint32_t width, uint32_t height);
	void Destroy();
	size_t ReadRgbaPixels(uint8_t* pDstBuffer, size_t maxLen);
	bool ReadbackFramebuffer(uint8_t* pDstBuffer, size_t bufferSize, uint32_t* pOutWidth = nullptr, uint32_t* pOutHeight = nullptr);

	// Licznik transakcji FFI / operacji
	uint64_t GetTransactionCount() const noexcept { return m_transactionCount.load(); }
	// Licznik referencji
	uint64_t GetRefCount() const noexcept { return m_refCount.load(); }
	void AddRef() noexcept { ++m_refCount; }
	void ReleaseRef() noexcept { if (m_refCount > 0) --m_refCount; }
	void ResetCounters() noexcept { m_transactionCount.store(0); m_refCount.store(0); }

private:
	CWgpuModule();
	~CWgpuModule();
	CWgpuModule(const CWgpuModule&) = delete;
	CWgpuModule& operator=(const CWgpuModule&) = delete;

	mutable std::mutex m_mutex;
	HMODULE m_hModule{ nullptr };
	SWgpuFunctions m_functions{};
	std::atomic<uint64_t> m_transactionCount{ 0 };
	std::atomic<uint64_t> m_refCount{ 0 };
};
