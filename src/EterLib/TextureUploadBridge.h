#pragma once

#include <cstdint>
#include <cstddef>
#include <windows.h>

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

	// Zdarzenie ladowania tekstury 2D - rejestracja w backendzie WGPU
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

	// Rejestracja tablicy tekstur Texture2DArray (layers > 1) dla CTextureSet podloza terenu
	bool RegisterTextureArray(
		uintptr_t dwTextureID,
		uint32_t width,
		uint32_t height,
		uint32_t layers,
		uint32_t format,
		const uint8_t* pData,
		size_t dataSize
	);

	bool RegisterTextureArray(
		uintptr_t dwTextureID,
		uint32_t width,
		uint32_t height,
		uint32_t layers,
		uint32_t format,
		const void* pData,
		size_t dataSize
	);

	// Aktualizacja pojedynczej warstwy w istniejacej tablicy tekstur
	bool UpdateTextureArrayLayer(
		uintptr_t dwTextureID,
		uint32_t layerIndex,
		const uint8_t* pData,
		size_t dataSize
	);

	bool UpdateTextureArrayLayer(
		uintptr_t dwTextureID,
		uint32_t layerIndex,
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

	uint64_t m_registeredCount{ 0 };
};