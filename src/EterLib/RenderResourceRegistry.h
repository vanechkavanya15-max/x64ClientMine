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

// Podzial 32-bitowego uchwytu: 20 bitow indeksu slotu, 12 bitow licznika generacji
constexpr uint32_t kTextureIndexBits = 20;
constexpr uint32_t kTextureIndexMask = (1u << kTextureIndexBits) - 1u; // 0x000FFFFF (do 1 048 575 slotow)
constexpr uint32_t kTextureGenerationBits = 12;
constexpr uint32_t kTextureGenerationMask = (1u << kTextureGenerationBits) - 1u; // 0x00000FFF (4095 generacji)
constexpr uint32_t kTextureGenerationShift = kTextureIndexBits; // 20

// Funkcje pomocnicze kodowania i dekodowania uchwytu
inline constexpr uint32_t RenderResourceGetSlotIndex(TRenderTextureHandle handle) noexcept
{
	return handle & kTextureIndexMask;
}

inline constexpr uint32_t RenderResourceGetGeneration(TRenderTextureHandle handle) noexcept
{
	return (handle >> kTextureGenerationShift) & kTextureGenerationMask;
}

inline constexpr TRenderTextureHandle RenderResourceMakeHandle(uint32_t slotIndex, uint32_t generation) noexcept
{
	return ((generation & kTextureGenerationMask) << kTextureGenerationShift) | (slotIndex & kTextureIndexMask);
}

inline constexpr bool RenderResourceIsValidHandleFormat(TRenderTextureHandle handle) noexcept
{
	return handle != kInvalidTextureHandle && RenderResourceGetSlotIndex(handle) != 0;
}

// Struktura opisujaca zarejestrowany zasob tekstury
struct STextureResourceInfo
{
	uint32_t width{ 0 };
	uint32_t height{ 0 };
	D3DFORMAT format{ D3DFMT_UNKNOWN };
	std::string debugName;
	IDirect3DTexture9* pTexture{ nullptr };
};

// Rejestr zasobow renderera zarzadzajacy uchwytami tekstur (slot index + generation counter)
class CRenderResourceRegistry : public CSingleton<CRenderResourceRegistry>
{
public:
	CRenderResourceRegistry();
	virtual ~CRenderResourceRegistry();

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

	// Zwolnienie uchwytu tekstury - powoduje inkrementacje generacji slotu i mozliwosc jego ponownego uzycia
	void UnregisterTexture(TRenderTextureHandle handle);

	// Zwolnienie rejestracji wedlug wskaznika Direct3D 9
	void UnregisterTexture(IDirect3DTexture9* pTexture);

	// Pobranie natywnego wskaznika Direct3D 9 na podstawie uchwytu (z weryfikacja poprawnosci generacji i slotu)
	IDirect3DTexture9* ResolveTexture(TRenderTextureHandle handle) const;

	// Pobranie pelnych informacji o zasobie tekstury
	bool GetTextureInfo(TRenderTextureHandle handle, STextureResourceInfo* pOutInfo) const;
	bool GetTextureInfo(TRenderTextureHandle handle, STextureResourceInfo& outInfo) const;

	// Pobieranie pojedynczych metadanych
	uint32_t GetWidth(TRenderTextureHandle handle) const;
	uint32_t GetHeight(TRenderTextureHandle handle) const;
	D3DFORMAT GetFormat(TRenderTextureHandle handle) const;
	std::string GetDebugName(TRenderTextureHandle handle) const;

	// Wyszukanie uchwytu dla juz zarejestrowanego wskaznika Direct3D 9
	TRenderTextureHandle FindTextureHandle(IDirect3DTexture9* pTexture) const;

	// Sprawdzenie poprawnosci uchwytu
	bool IsValidHandle(TRenderTextureHandle handle) const;

	// Liczba aktywnych zarejestrowanych tekstur
	size_t GetActiveTextureCount() const;

	// Calkowity rozmiar tablicy slotow (w tym slot 0 i wolne sloty)
	size_t GetTotalSlotCapacity() const;

	// Reset calego stanu rejestru (np. przy resecie urzadzenia lub miedzy testami)
	void Reset();

	// Statyczne metody pomocnicze bezpieczne w przypadku braku aktywnej instancji singletona
	static TRenderTextureHandle SafeRegisterTexture(IDirect3DTexture9* pTexture, const char* debugName = nullptr);
	static void SafeUnregisterTexture(TRenderTextureHandle handle);
	static void SafeUnregisterTexture(IDirect3DTexture9* pTexture);
	static IDirect3DTexture9* SafeResolveTexture(TRenderTextureHandle handle);

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
	};

	void UnregisterTextureLocked(TRenderTextureHandle handle);

	mutable std::mutex m_mutex;
	std::vector<SSlot> m_slots;
	std::vector<uint32_t> m_freeSlots;
	std::unordered_map<IDirect3DTexture9*, TRenderTextureHandle> m_textureToHandle;
};
