#pragma once

#include "EterBase/Singleton.h"
#include <d3d9.h>
#include <cstdint>
#include <string>
#include <vector>
#include <unordered_map>
#include <mutex>

#ifndef _RENDER_TEXTURE_HANDLE_DEFINED_
#define _RENDER_TEXTURE_HANDLE_DEFINED_
using TRenderTextureHandle = uint32_t;
constexpr TRenderTextureHandle kInvalidTextureHandle = 0;
#endif

#ifndef _RENDER_GEOMETRY_HANDLES_DEFINED_
#define _RENDER_GEOMETRY_HANDLES_DEFINED_
using TRenderVertexBufferHandle = uint32_t;
using TRenderIndexBufferHandle = uint32_t;
constexpr TRenderVertexBufferHandle kInvalidVertexBufferHandle = 0;
constexpr TRenderIndexBufferHandle kInvalidIndexBufferHandle = 0;
#endif

// Podzial 32-bitowego uchwytu: 20 bitow indeksu slotu, 12 bitow licznika generacji
constexpr uint32_t kTextureIndexBits = 20;
constexpr uint32_t kTextureIndexMask = (1u << kTextureIndexBits) - 1u; // 0x000FFFFF (do 1 048 575 slotow)
constexpr uint32_t kTextureGenerationBits = 12;
constexpr uint32_t kTextureGenerationMask = (1u << kTextureGenerationBits) - 1u; // 0x00000FFF (4095 generacji)
constexpr uint32_t kTextureGenerationShift = kTextureIndexBits; // 20

// Funkcje pomocnicze kodowania i dekodowania uchwytu
inline constexpr uint32_t RenderResourceGetSlotIndex(uint32_t handle) noexcept
{
	return handle & kTextureIndexMask;
}

inline constexpr uint32_t RenderResourceGetGeneration(uint32_t handle) noexcept
{
	return (handle >> kTextureGenerationShift) & kTextureGenerationMask;
}

inline constexpr uint32_t RenderResourceMakeHandle(uint32_t slotIndex, uint32_t generation) noexcept
{
	return ((generation & kTextureGenerationMask) << kTextureGenerationShift) | (slotIndex & kTextureIndexMask);
}

inline constexpr bool RenderResourceIsValidHandleFormat(uint32_t handle) noexcept
{
	return handle != 0 && RenderResourceGetSlotIndex(handle) != 0;
}

// Kopia CPU pojedynczego poziomu mipmapy tekstury
struct STextureMipCpuCopy
{
	uint32_t level{ 0 };
	uint32_t width{ 0 };
	uint32_t height{ 0 };
	uint32_t pitch{ 0 };
	uint32_t dataSize{ 0 };
	uint32_t crc32{ 0 };
	std::vector<uint8_t> pixels;
};

// Struktura opisujaca zarejestrowany zasob tekstury
struct STextureResourceInfo
{
	uint32_t width{ 0 };
	uint32_t height{ 0 };
	D3DFORMAT format{ D3DFMT_UNKNOWN };
	std::string debugName;
	IDirect3DTexture9* pTexture{ nullptr };
	std::vector<STextureMipCpuCopy> mipCopies;
	uint32_t mip0Crc32{ 0 };
	bool hasCpuCopy{ false };
};

// Struktura opisujaca zarejestrowany zasob bufora wierzcholkow
struct SVertexBufferResourceInfo
{
	IDirect3DVertexBuffer9* pVB{ nullptr };
	uint32_t size{ 0 };
	uint32_t fvf{ 0 };
	uint32_t stride{ 0 };
	uint32_t vertexCount{ 0 };
	DWORD usage{ 0 };
	D3DPOOL pool{ D3DPOOL_DEFAULT };
	std::string debugName;
	std::vector<uint8_t> cpuCopy;
	uint32_t crc32{ 0 };
	bool isStatic{ true };
};

// Struktura opisujaca zarejestrowany zasob bufora indeksow
struct SIndexBufferResourceInfo
{
	IDirect3DIndexBuffer9* pIB{ nullptr };
	uint32_t size{ 0 };
	D3DFORMAT format{ D3DFMT_INDEX16 };
	uint32_t indexCount{ 0 };
	DWORD usage{ 0 };
	D3DPOOL pool{ D3DPOOL_DEFAULT };
	std::string debugName;
	std::vector<uint8_t> cpuCopy;
	uint32_t crc32{ 0 };
	bool isStatic{ true };
};

// Rejestr zasobow renderera zarzadzajacy uchwytami tekstur oraz kopiami CPU geometrii i pikseli
class CRenderResourceRegistry : public CSingleton<CRenderResourceRegistry>
{
public:
	CRenderResourceRegistry();
	virtual ~CRenderResourceRegistry();

	// --- Tekstury ---
	// Rejestracja tekstury Direct3D 9 z opcjonalna nazwa debugowa
	TRenderTextureHandle RegisterTexture(IDirect3DTexture9* pTexture, const char* debugName = nullptr);

	// Rejestracja tekstury z jawnym podaniem metadanych (przydatne przy braku dostepu do GetLevelDesc lub w testach)
	TRenderTextureHandle RegisterTextureWithInfo(
		IDirect3DTexture9* pTexture,
		uint32_t width,
		uint32_t height,
		D3DFORMAT format,
		const char* debugName = nullptr
	);

	// Rejestracja tekstury z jednoczesnym dolaczeniem kopii CPU pikseli mip 0
	TRenderTextureHandle RegisterTextureWithPixels(
		IDirect3DTexture9* pTexture,
		uint32_t width,
		uint32_t height,
		D3DFORMAT format,
		const void* pMip0Pixels,
		size_t mip0Size,
		uint32_t pitch = 0,
		const char* debugName = nullptr
	);

	// Zwolnienie uchwytu tekstury - inkrementacja generacji slotu
	void UnregisterTexture(TRenderTextureHandle handle);
	void UnregisterTexture(IDirect3DTexture9* pTexture);

	// Pobranie natywnego wskaznika Direct3D 9 na podstawie uchwytu
	IDirect3DTexture9* ResolveTexture(TRenderTextureHandle handle) const;

	// Pobranie pelnych informacji o zasobie tekstury
	bool GetTextureInfo(TRenderTextureHandle handle, STextureResourceInfo* pOutInfo) const;
	bool GetTextureInfo(TRenderTextureHandle handle, STextureResourceInfo& outInfo) const;

	// Pobieranie pojedynczych metadanych tekstury
	uint32_t GetWidth(TRenderTextureHandle handle) const;
	uint32_t GetHeight(TRenderTextureHandle handle) const;
	D3DFORMAT GetFormat(TRenderTextureHandle handle) const;
	std::string GetDebugName(TRenderTextureHandle handle) const;
	TRenderTextureHandle FindTextureHandle(IDirect3DTexture9* pTexture) const;
	bool IsValidHandle(TRenderTextureHandle handle) const;
	size_t GetActiveTextureCount() const;
	size_t GetTotalSlotCapacity() const;

	// Kopia CPU pikseli tekstury (per-mip level ze stagingu)
	bool AttachTexturePixelCopy(TRenderTextureHandle handle, uint32_t mipLevel, uint32_t width, uint32_t height, uint32_t pitch, const void* pPixels, size_t size);
	bool AttachTexturePixelCopy(IDirect3DTexture9* pTexture, uint32_t mipLevel, uint32_t width, uint32_t height, uint32_t pitch, const void* pPixels, size_t size);
	bool GetTextureMipPixelCopy(TRenderTextureHandle handle, uint32_t mipLevel, STextureMipCpuCopy* pOutMip) const;
	bool GetTextureMip0Pixels(TRenderTextureHandle handle, std::vector<uint8_t>& outPixels, uint32_t* pOutCrc32 = nullptr) const;
	uint32_t GetTextureMip0Crc32(TRenderTextureHandle handle) const;
	bool VerifyTextureMip0Crc(TRenderTextureHandle handle, uint32_t expectedCrc) const;
	size_t GetTextureMipCount(TRenderTextureHandle handle) const;
	bool HasTextureCpuCopy(TRenderTextureHandle handle) const;

	// --- Bufory wierzcholkow (VB) ---
	TRenderVertexBufferHandle RegisterVertexBuffer(IDirect3DVertexBuffer9* pVB, uint32_t size, uint32_t fvf, const void* pCpuData = nullptr, const char* debugName = nullptr);
	TRenderVertexBufferHandle RegisterVertexBufferWithInfo(IDirect3DVertexBuffer9* pVB, uint32_t size, uint32_t fvf, uint32_t stride, uint32_t vertexCount, DWORD usage, D3DPOOL pool, const void* pCpuData = nullptr, const char* debugName = nullptr);
	void UnregisterVertexBuffer(TRenderVertexBufferHandle handle);
	void UnregisterVertexBuffer(IDirect3DVertexBuffer9* pVB);
	IDirect3DVertexBuffer9* ResolveVertexBuffer(TRenderVertexBufferHandle handle) const;
	bool SetVertexBufferCpuCopy(TRenderVertexBufferHandle handle, const void* pData, size_t size);
	bool SetVertexBufferCpuCopy(IDirect3DVertexBuffer9* pVB, const void* pData, size_t size);
	bool GetVertexBufferCpuCopy(TRenderVertexBufferHandle handle, std::vector<uint8_t>& outData) const;
	bool GetVertexBufferCpuCopy(IDirect3DVertexBuffer9* pVB, std::vector<uint8_t>& outData) const;
	bool GetVertexBufferInfo(TRenderVertexBufferHandle handle, SVertexBufferResourceInfo* pOutInfo) const;
	TRenderVertexBufferHandle FindVertexBufferHandle(IDirect3DVertexBuffer9* pVB) const;
	uint32_t GetVertexBufferCrc32(TRenderVertexBufferHandle handle) const;
	bool VerifyVertexBufferCrc(TRenderVertexBufferHandle handle, uint32_t expectedCrc) const;
	size_t GetActiveVertexBufferCount() const;

	// --- Bufory indeksow (IB) ---
	TRenderIndexBufferHandle RegisterIndexBuffer(IDirect3DIndexBuffer9* pIB, uint32_t size, D3DFORMAT format, const void* pCpuData = nullptr, const char* debugName = nullptr);
	TRenderIndexBufferHandle RegisterIndexBufferWithInfo(IDirect3DIndexBuffer9* pIB, uint32_t size, D3DFORMAT format, uint32_t indexCount, DWORD usage, D3DPOOL pool, const void* pCpuData = nullptr, const char* debugName = nullptr);
	void UnregisterIndexBuffer(TRenderIndexBufferHandle handle);
	void UnregisterIndexBuffer(IDirect3DIndexBuffer9* pIB);
	IDirect3DIndexBuffer9* ResolveIndexBuffer(TRenderIndexBufferHandle handle) const;
	bool SetIndexBufferCpuCopy(TRenderIndexBufferHandle handle, const void* pData, size_t size);
	bool SetIndexBufferCpuCopy(IDirect3DIndexBuffer9* pIB, const void* pData, size_t size);
	bool GetIndexBufferCpuCopy(TRenderIndexBufferHandle handle, std::vector<uint8_t>& outData) const;
	bool GetIndexBufferCpuCopy(IDirect3DIndexBuffer9* pIB, std::vector<uint8_t>& outData) const;
	bool GetIndexBufferInfo(TRenderIndexBufferHandle handle, SIndexBufferResourceInfo* pOutInfo) const;
	TRenderIndexBufferHandle FindIndexBufferHandle(IDirect3DIndexBuffer9* pIB) const;
	uint32_t GetIndexBufferCrc32(TRenderIndexBufferHandle handle) const;
	bool VerifyIndexBufferCrc(TRenderIndexBufferHandle handle, uint32_t expectedCrc) const;
	size_t GetActiveIndexBufferCount() const;

	// Reset calego stanu rejestru
	void Reset();

	// Statyczne metody pomocnicze bezpieczne w przypadku braku aktywnej instancji singletona
	static TRenderTextureHandle SafeRegisterTexture(IDirect3DTexture9* pTexture, const char* debugName = nullptr);
	static void SafeUnregisterTexture(TRenderTextureHandle handle);
	static void SafeUnregisterTexture(IDirect3DTexture9* pTexture);
	static IDirect3DTexture9* SafeResolveTexture(TRenderTextureHandle handle);
	static bool SafeAttachTexturePixelCopy(IDirect3DTexture9* pTexture, uint32_t mipLevel, uint32_t width, uint32_t height, uint32_t pitch, const void* pPixels, size_t size);
	static bool SafeAttachTexturePixelCopy(TRenderTextureHandle handle, uint32_t mipLevel, uint32_t width, uint32_t height, uint32_t pitch, const void* pPixels, size_t size);
	static bool SafeGetTextureMip0Pixels(TRenderTextureHandle handle, std::vector<uint8_t>& outPixels, uint32_t* pOutCrc32 = nullptr);
	static uint32_t SafeGetTextureMip0Crc32(TRenderTextureHandle handle);
	static bool SafeVerifyTextureMip0Crc(TRenderTextureHandle handle, uint32_t expectedCrc);

	static TRenderVertexBufferHandle SafeRegisterVertexBuffer(IDirect3DVertexBuffer9* pVB, uint32_t size, uint32_t fvf, const void* pCpuData = nullptr, const char* debugName = nullptr);
	static void SafeUnregisterVertexBuffer(TRenderVertexBufferHandle handle);
	static void SafeUnregisterVertexBuffer(IDirect3DVertexBuffer9* pVB);
	static bool SafeSetVertexBufferCpuCopy(IDirect3DVertexBuffer9* pVB, const void* pData, size_t size);
	static bool SafeGetVertexBufferCpuCopy(IDirect3DVertexBuffer9* pVB, std::vector<uint8_t>& outData);

	static TRenderIndexBufferHandle SafeRegisterIndexBuffer(IDirect3DIndexBuffer9* pIB, uint32_t size, D3DFORMAT format, const void* pCpuData = nullptr, const char* debugName = nullptr);
	static void SafeUnregisterIndexBuffer(TRenderIndexBufferHandle handle);
	static void SafeUnregisterIndexBuffer(IDirect3DIndexBuffer9* pIB);
	static bool SafeSetIndexBufferCpuCopy(IDirect3DIndexBuffer9* pIB, const void* pData, size_t size);
	static bool SafeGetIndexBufferCpuCopy(IDirect3DIndexBuffer9* pIB, std::vector<uint8_t>& outData);

private:
	struct SSlot
	{
		uint32_t generation{ 1 };
		bool occupied{ false };
		IDirect3DTexture9* pTexture{ nullptr };
		uint32_t width{ 0 };
		uint32_t height{ 0 };
		D3DFORMAT format{ D3DFMT_UNKNOWN };
		std::string debugName;
		std::vector<STextureMipCpuCopy> mipCopies;
		uint32_t mip0Crc32{ 0 };
		bool hasCpuCopy{ false };
	};

	struct SVBSlot
	{
		uint32_t generation{ 1 };
		bool occupied{ false };
		IDirect3DVertexBuffer9* pVB{ nullptr };
		uint32_t size{ 0 };
		uint32_t fvf{ 0 };
		uint32_t stride{ 0 };
		uint32_t vertexCount{ 0 };
		DWORD usage{ 0 };
		D3DPOOL pool{ D3DPOOL_DEFAULT };
		std::string debugName;
		std::vector<uint8_t> cpuCopy;
		uint32_t crc32{ 0 };
		bool isStatic{ true };
	};

	struct SIBSlot
	{
		uint32_t generation{ 1 };
		bool occupied{ false };
		IDirect3DIndexBuffer9* pIB{ nullptr };
		uint32_t size{ 0 };
		D3DFORMAT format{ D3DFMT_INDEX16 };
		uint32_t indexCount{ 0 };
		DWORD usage{ 0 };
		D3DPOOL pool{ D3DPOOL_DEFAULT };
		std::string debugName;
		std::vector<uint8_t> cpuCopy;
		uint32_t crc32{ 0 };
		bool isStatic{ true };
	};

	void UnregisterTextureLocked(TRenderTextureHandle handle);
	void UnregisterVertexBufferLocked(TRenderVertexBufferHandle handle);
	void UnregisterIndexBufferLocked(TRenderIndexBufferHandle handle);

	mutable std::mutex m_mutex;

	// Sloty tekstur
	std::vector<SSlot> m_slots;
	std::vector<uint32_t> m_freeSlots;
	std::unordered_map<IDirect3DTexture9*, TRenderTextureHandle> m_textureToHandle;

	// Sloty buforow wierzcholkow
	std::vector<SVBSlot> m_vbSlots;
	std::vector<uint32_t> m_vbFreeSlots;
	std::unordered_map<IDirect3DVertexBuffer9*, TRenderVertexBufferHandle> m_vbToHandle;

	// Sloty buforow indeksow
	std::vector<SIBSlot> m_ibSlots;
	std::vector<uint32_t> m_ibFreeSlots;
	std::unordered_map<IDirect3DIndexBuffer9*, TRenderIndexBufferHandle> m_ibToHandle;
};
