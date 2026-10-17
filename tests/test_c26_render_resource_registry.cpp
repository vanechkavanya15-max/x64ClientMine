#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include "EterLib/RenderResourceRegistry.h"
#include <vector>
#include <thread>
#include <atomic>

// Mock Direct3D 9 texture dla testow jednostkowych
class MockDirect3DTexture9 : public IDirect3DTexture9
{
public:
	ULONG refCount{ 1 };
	D3DSURFACE_DESC desc;

	MockDirect3DTexture9(UINT w = 64, UINT h = 64, D3DFORMAT fmt = D3DFMT_A8R8G8B8)
	{
		std::memset(&desc, 0, sizeof(desc));
		desc.Width = w;
		desc.Height = h;
		desc.Format = fmt;
		desc.Type = D3DRTYPE_TEXTURE;
	}

	HRESULT STDMETHODCALLTYPE QueryInterface(REFIID riid, void** ppvObj) override
	{
		if (!ppvObj)
			return E_POINTER;
		*ppvObj = nullptr;
		return E_NOINTERFACE;
	}

	ULONG STDMETHODCALLTYPE AddRef() override { return ++refCount; }
	ULONG STDMETHODCALLTYPE Release() override
	{
		if (refCount > 0)
		{
			--refCount;
			if (refCount == 0)
				return 0;
		}
		return refCount;
	}

	HRESULT STDMETHODCALLTYPE GetDevice(IDirect3DDevice9** ppDevice) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE SetPrivateData(REFGUID refguid, const void* pData, DWORD SizeOfData, DWORD Flags) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE GetPrivateData(REFGUID refguid, void* pData, DWORD* pSizeOfData) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE FreePrivateData(REFGUID refguid) override { return D3D_OK; }
	DWORD STDMETHODCALLTYPE SetPriority(DWORD PriorityNew) override { return 0; }
	DWORD STDMETHODCALLTYPE GetPriority() override { return 0; }
	void STDMETHODCALLTYPE PreLoad() override {}
	D3DRESOURCETYPE STDMETHODCALLTYPE GetType() override { return D3DRTYPE_TEXTURE; }

	DWORD STDMETHODCALLTYPE SetLOD(DWORD LODNew) override { return 0; }
	DWORD STDMETHODCALLTYPE GetLOD() override { return 0; }
	DWORD STDMETHODCALLTYPE GetLevelCount() override { return 1; }
	HRESULT STDMETHODCALLTYPE SetAutoGenFilterType(D3DTEXTUREFILTERTYPE FilterType) override { return D3D_OK; }
	D3DTEXTUREFILTERTYPE STDMETHODCALLTYPE GetAutoGenFilterType() override { return D3DTEXF_NONE; }
	void STDMETHODCALLTYPE GenerateMipSubLevels() override {}

	HRESULT STDMETHODCALLTYPE GetLevelDesc(UINT Level, D3DSURFACE_DESC* pDesc) override
	{
		if (pDesc)
		{
			*pDesc = desc;
			return D3D_OK;
		}
		return E_POINTER;
	}

	HRESULT STDMETHODCALLTYPE GetSurfaceLevel(UINT Level, IDirect3DSurface9** ppSurfaceLevel) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE LockRect(UINT Level, D3DLOCKED_RECT* pLockedRect, const RECT* pRect, DWORD Flags) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE UnlockRect(UINT Level) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE AddDirtyRect(const RECT* pDirtyRect) override { return D3D_OK; }
};

TEST_CASE("RenderResourceRegistry - Handle bit layout and format constants")
{
	CHECK(kInvalidTextureHandle == 0);
	CHECK(kTextureIndexBits == 20);
	CHECK(kTextureIndexMask == 0x000FFFFF);
	CHECK(kTextureGenerationBits == 12);
	CHECK(kTextureGenerationMask == 0x00000FFF);
	CHECK(kTextureGenerationShift == 20);

	// Test kodowania i dekodowania
	uint32_t slot = 12345;
	uint32_t gen = 42;
	TRenderTextureHandle h = RenderResourceMakeHandle(slot, gen);
	CHECK(RenderResourceGetSlotIndex(h) == slot);
	CHECK(RenderResourceGetGeneration(h) == gen);
	CHECK(RenderResourceIsValidHandleFormat(h) == true);

	// Handle 0 jest nieprawidlowy
	CHECK(RenderResourceIsValidHandleFormat(kInvalidTextureHandle) == false);

	// Slot 0 jest nieprawidlowy nawet z niezerowa generacja
	TRenderTextureHandle hSlot0 = RenderResourceMakeHandle(0, 5);
	CHECK(RenderResourceIsValidHandleFormat(hSlot0) == false);
}

TEST_CASE("RenderResourceRegistry - Basic Registration and Resolve")
{
	CRenderResourceRegistry registry;
	registry.Reset();

	MockDirect3DTexture9 tex1(128, 64, D3DFMT_A8R8G8B8);
	MockDirect3DTexture9 tex2(256, 256, D3DFMT_X8R8G8B8);

	// Rejestracja pierwszej tekstury
	TRenderTextureHandle h1 = registry.RegisterTexture(&tex1, "test_tex_1");
	CHECK(h1 != kInvalidTextureHandle);
	CHECK(RenderResourceGetSlotIndex(h1) == 1); // Pierwszy przydzielony slot to index 1 (slot 0 zarezerwowany)
	CHECK(RenderResourceGetGeneration(h1) == 1);
	CHECK(registry.GetActiveTextureCount() == 1);

	// Rozwiazywanie uchwytu
	IDirect3DTexture9* resolved1 = registry.ResolveTexture(h1);
	CHECK(resolved1 == &tex1);

	// Idempotentnosc rejestracji tego samego wskaznika
	TRenderTextureHandle h1_repeat = registry.RegisterTexture(&tex1);
	CHECK(h1_repeat == h1);
	CHECK(registry.GetActiveTextureCount() == 1);

	// Rejestracja drugiej tekstury
	TRenderTextureHandle h2 = registry.RegisterTexture(&tex2, "test_tex_2");
	CHECK(h2 != kInvalidTextureHandle);
	CHECK(h2 != h1);
	CHECK(RenderResourceGetSlotIndex(h2) == 2);
	CHECK(RenderResourceGetGeneration(h2) == 1);
	CHECK(registry.GetActiveTextureCount() == 2);

	CHECK(registry.ResolveTexture(h2) == &tex2);
	CHECK(registry.ResolveTexture(h1) == &tex1);
}

TEST_CASE("RenderResourceRegistry - Slot Reuse and Generation Increment")
{
	CRenderResourceRegistry registry;
	registry.Reset();

	MockDirect3DTexture9 texA(64, 64);
	MockDirect3DTexture9 texB(128, 128);

	// Rejestracja texA -> slot 1, generacja 1
	TRenderTextureHandle hA = registry.RegisterTexture(&texA, "TextureA");
	CHECK(hA != kInvalidTextureHandle);
	const uint32_t slotA = RenderResourceGetSlotIndex(hA);
	const uint32_t genA = RenderResourceGetGeneration(hA);
	CHECK(slotA == 1);
	CHECK(genA == 1);
	CHECK(registry.ResolveTexture(hA) == &texA);

	// Zwolnienie uchwytu hA
	registry.UnregisterTexture(hA);
	CHECK(registry.GetActiveTextureCount() == 0);
	CHECK(registry.ResolveTexture(hA) == nullptr);

	// Rejestracja nowej tekstury texB - powinna wykorzystac ponownie zwolniony slot 1
	TRenderTextureHandle hB = registry.RegisterTexture(&texB, "TextureB");
	CHECK(hB != kInvalidTextureHandle);
	const uint32_t slotB = RenderResourceGetSlotIndex(hB);
	const uint32_t genB = RenderResourceGetGeneration(hB);

	// Ponowne uzycie slotu z inkrementacja generacji
	CHECK(slotB == slotA);
	CHECK(genB == genA + 1);
	CHECK(hB != hA);

	// STARY UCHWYT MUSI BYC ODRZUCONY
	CHECK(registry.ResolveTexture(hA) == nullptr);
	CHECK(registry.IsValidHandle(hA) == false);

	// Nowy uchwyt rozwiazuje sie prawidlowo
	CHECK(registry.ResolveTexture(hB) == &texB);
	CHECK(registry.IsValidHandle(hB) == true);
}

TEST_CASE("RenderResourceRegistry - Rejection of Invalid Handles (Slot 0, Stale Generation, Out of Bounds)")
{
	CRenderResourceRegistry registry;
	registry.Reset();

	MockDirect3DTexture9 tex(64, 64);
	TRenderTextureHandle validHandle = registry.RegisterTexture(&tex, "ValidTex");
	CHECK(validHandle != kInvalidTextureHandle);

	// 1. Odrzucenie kInvalidTextureHandle (0)
	CHECK(registry.ResolveTexture(kInvalidTextureHandle) == nullptr);
	CHECK(registry.IsValidHandle(kInvalidTextureHandle) == false);

	// 2. Odrzucenie uchwytu ze slotem 0, niezaleznie od numeru generacji
	TRenderTextureHandle handleSlot0Gen1 = RenderResourceMakeHandle(0, 1);
	TRenderTextureHandle handleSlot0Gen42 = RenderResourceMakeHandle(0, 42);
	CHECK(registry.ResolveTexture(handleSlot0Gen1) == nullptr);
	CHECK(registry.ResolveTexture(handleSlot0Gen42) == nullptr);
	CHECK(registry.IsValidHandle(handleSlot0Gen1) == false);

	// 3. Odrzucenie uchwytu poza zakresem zaalokowanych slotow
	TRenderTextureHandle handleOutOfBounds = RenderResourceMakeHandle(9999, 1);
	CHECK(registry.ResolveTexture(handleOutOfBounds) == nullptr);
	CHECK(registry.IsValidHandle(handleOutOfBounds) == false);

	// 4. Odrzucenie uchwytu z niepoprawna/stara generacja dla istniejacego slotu
	const uint32_t slotIdx = RenderResourceGetSlotIndex(validHandle);
	const uint32_t curGen = RenderResourceGetGeneration(validHandle);
	TRenderTextureHandle handleWrongGen = RenderResourceMakeHandle(slotIdx, curGen + 5);
	CHECK(registry.ResolveTexture(handleWrongGen) == nullptr);
	CHECK(registry.IsValidHandle(handleWrongGen) == false);

	// 5. Odrzucenie nullptr przy rejestracji
	CHECK(registry.RegisterTexture(nullptr) == kInvalidTextureHandle);
}

TEST_CASE("RenderResourceRegistry - Resource Info and Metadata")
{
	CRenderResourceRegistry registry;
	registry.Reset();

	MockDirect3DTexture9 tex(512, 256, D3DFMT_DXT1);
	TRenderTextureHandle handle = registry.RegisterTexture(&tex, "terrain_normal_map");

	STextureResourceInfo info;
	CHECK(registry.GetTextureInfo(handle, info) == true);
	CHECK(info.width == 512);
	CHECK(info.height == 256);
	CHECK(info.format == D3DFMT_DXT1);
	CHECK(info.debugName == "terrain_normal_map");
	CHECK(info.pTexture == &tex);

	// Pojedyncze metody dostepowe
	CHECK(registry.GetWidth(handle) == 512);
	CHECK(registry.GetHeight(handle) == 256);
	CHECK(registry.GetFormat(handle) == D3DFMT_DXT1);
	CHECK(registry.GetDebugName(handle) == "terrain_normal_map");

	// Rejestracja z jawnymi metadanymi (RegisterTextureWithInfo)
	MockDirect3DTexture9 texCustom(0, 0);
	TRenderTextureHandle hCustom = registry.RegisterTextureWithInfo(&texCustom, 1024, 768, D3DFMT_A8R8G8B8, "custom_target");
	CHECK(registry.GetWidth(hCustom) == 1024);
	CHECK(registry.GetHeight(hCustom) == 768);
	CHECK(registry.GetFormat(hCustom) == D3DFMT_A8R8G8B8);
	CHECK(registry.GetDebugName(hCustom) == "custom_target");

	// Informacje dla nieprawidlowego uchwytu
	STextureResourceInfo invalidInfo;
	CHECK(registry.GetTextureInfo(kInvalidTextureHandle, invalidInfo) == false);
	CHECK(registry.GetWidth(kInvalidTextureHandle) == 0);
	CHECK(registry.GetHeight(kInvalidTextureHandle) == 0);
	CHECK(registry.GetFormat(kInvalidTextureHandle) == D3DFMT_UNKNOWN);
	CHECK(registry.GetDebugName(kInvalidTextureHandle).empty());
}

TEST_CASE("RenderResourceRegistry - Unregister by Texture Pointer and FindHandle")
{
	CRenderResourceRegistry registry;
	registry.Reset();

	MockDirect3DTexture9 tex(64, 64);
	TRenderTextureHandle handle = registry.RegisterTexture(&tex, "find_me");

	CHECK(registry.FindTextureHandle(&tex) == handle);
	CHECK(registry.FindTextureHandle(nullptr) == kInvalidTextureHandle);

	MockDirect3DTexture9 unregTex(32, 32);
	CHECK(registry.FindTextureHandle(&unregTex) == kInvalidTextureHandle);

	// Wyrejestrowanie przez wskaznik
	registry.UnregisterTexture(&tex);
	CHECK(registry.ResolveTexture(handle) == nullptr);
	CHECK(registry.FindTextureHandle(&tex) == kInvalidTextureHandle);
	CHECK(registry.GetActiveTextureCount() == 0);
}

TEST_CASE("RenderResourceRegistry - Generation Wrap-Around Boundary (12 bits)")
{
	CRenderResourceRegistry registry;
	registry.Reset();

	MockDirect3DTexture9 tex(32, 32);

	// Cykliczne rejestrowanie i zwalnianie tego samego slotu
	TRenderTextureHandle hFirst = registry.RegisterTexture(&tex);
	CHECK(RenderResourceGetSlotIndex(hFirst) == 1);
	CHECK(RenderResourceGetGeneration(hFirst) == 1);
	registry.UnregisterTexture(hFirst);

	// Wykonaj serie cykli zwalniania aby sprawdzic zachowanie generacji
	TRenderTextureHandle lastHandle = kInvalidTextureHandle;
	for (int i = 0; i < 20; ++i)
	{
		lastHandle = registry.RegisterTexture(&tex);
		CHECK(RenderResourceGetSlotIndex(lastHandle) == 1);
		CHECK(RenderResourceGetGeneration(lastHandle) == static_cast<uint32_t>(i + 2));
		registry.UnregisterTexture(lastHandle);
	}

	CHECK(registry.ResolveTexture(lastHandle) == nullptr);
}

TEST_CASE("RenderResourceRegistry - Thread-Safety Concurrent Registration and Resolve")
{
	CRenderResourceRegistry registry;
	registry.Reset();

	constexpr int kThreadCount = 4;
	constexpr int kIterations = 100;
	std::vector<std::thread> threads;
	std::atomic<bool> startFlag{ false };

	std::vector<MockDirect3DTexture9> textures;
	textures.reserve(kThreadCount * kIterations);
	for (int i = 0; i < kThreadCount * kIterations; ++i)
	{
		textures.emplace_back(32 + (i % 64), 32 + (i % 64));
	}

	for (int t = 0; t < kThreadCount; ++t)
	{
		threads.emplace_back([&, t]() {
			while (!startFlag.load(std::memory_order_acquire))
			{
				std::this_thread::yield();
			}

			for (int i = 0; i < kIterations; ++i)
			{
				const int idx = t * kIterations + i;
				TRenderTextureHandle handle = registry.RegisterTexture(&textures[idx]);
				CHECK(handle != kInvalidTextureHandle);

				IDirect3DTexture9* pResolved = registry.ResolveTexture(handle);
				CHECK(pResolved == &textures[idx]);

				registry.UnregisterTexture(handle);
				CHECK(registry.ResolveTexture(handle) == nullptr);
			}
		});
	}

	startFlag.store(true, std::memory_order_release);
	for (auto& th : threads)
	{
		th.join();
	}

	CHECK(registry.GetActiveTextureCount() == 0);
}
