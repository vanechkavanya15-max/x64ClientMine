#pragma once

#include <cstdint>
#include <cstddef>
#include <windows.h>

// Typy wskaznikow na eksportowane funkcje FFI z m2_render_wgpu.dll
using PFN_m2_wgpu_register_texture = void(__cdecl*)(uintptr_t, uint32_t, uint32_t, uint32_t, const uint8_t*, size_t);
using PFN_m2_wgpu_unregister_texture = void(__cdecl*)(uintptr_t);

// Singleton mostu przesylania tekstur C++ -> Rust WGPU
class CTextureUploadBridge
{
public:
	static CTextureUploadBridge& Instance()
	{
		static CTextureUploadBridge s_instance;
		return s_instance;
	}

	bool Initialize();
	void Shutdown();

	// Zdarzenie ladowania tekstury - rejestracja w backendzie WGPU
	bool RegisterTexture(
		uintptr_t dwTextureID,
		uint32_t width,
		uint32_t height,
		uint32_t format,
		const uint8_t* pData,
		size_t dataSize
	);

	// Przeciazenie pomocnicze dla bufora const void*
	bool RegisterTexture(
		uintptr_t dwTextureID,
		uint32_t width,
		uint32_t height,
		uint32_t format,
		const void* pData,
		size_t dataSize
	);

	// Wyrejestrowanie tekstury (np. przy zniszczeniu obiektu D3D9)
	void UnregisterTexture(uintptr_t dwTextureID);

	// Sprawdzenie czy backend WGPU jest aktywny
	bool IsWgpuActive() const;

	// Licznik przetworzonych rejestracji tekstur
	uint64_t GetRegisteredCount() const noexcept { return m_registeredCount; }

private:
	CTextureUploadBridge();
	~CTextureUploadBridge();

	CTextureUploadBridge(const CTextureUploadBridge&) = delete;
	CTextureUploadBridge& operator=(const CTextureUploadBridge&) = delete;

	bool EnsureWgpuLoaded();

	HMODULE m_hWgpuModule{ nullptr };
	PFN_m2_wgpu_register_texture m_pfnRegisterTexture{ nullptr };
	PFN_m2_wgpu_unregister_texture m_pfnUnregisterTexture{ nullptr };
	bool m_bOwnsModule{ false };
	uint64_t m_registeredCount{ 0 };
};
