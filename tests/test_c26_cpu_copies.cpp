#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include "EterLib/RenderResourceRegistry.h"
#include "EterLib/GrpVertexBuffer.h"
#include "EterLib/GrpIndexBuffer.h"
#include "EterBase/CRC32.h"
#include <vector>
#include <cstring>
#include <numeric>

// =========================================================================
// MOCKI D3D9 DLA TESTOW JEDNOSTKOWYCH
// =========================================================================

class MockD3DTexture : public IDirect3DTexture9
{
public:
	ULONG refCount{ 1 };
	D3DSURFACE_DESC desc;

	MockD3DTexture(UINT w = 64, UINT h = 64, D3DFORMAT fmt = D3DFMT_A8R8G8B8)
	{
		std::memset(&desc, 0, sizeof(desc));
		desc.Width = w;
		desc.Height = h;
		desc.Format = fmt;
		desc.Type = D3DRTYPE_TEXTURE;
	}

	HRESULT STDMETHODCALLTYPE QueryInterface(REFIID, void** ppvObj) override
	{
		if (!ppvObj) return E_POINTER;
		*ppvObj = nullptr;
		return E_NOINTERFACE;
	}
	ULONG STDMETHODCALLTYPE AddRef() override { return ++refCount; }
	ULONG STDMETHODCALLTYPE Release() override { return (refCount > 0) ? --refCount : 0; }
	HRESULT STDMETHODCALLTYPE GetDevice(IDirect3DDevice9**) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE SetPrivateData(REFGUID, const void*, DWORD, DWORD) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE GetPrivateData(REFGUID, void*, DWORD*) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE FreePrivateData(REFGUID) override { return D3D_OK; }
	DWORD STDMETHODCALLTYPE SetPriority(DWORD) override { return 0; }
	DWORD STDMETHODCALLTYPE GetPriority() override { return 0; }
	void STDMETHODCALLTYPE PreLoad() override {}
	D3DRESOURCETYPE STDMETHODCALLTYPE GetType() override { return D3DRTYPE_TEXTURE; }
	DWORD STDMETHODCALLTYPE SetLOD(DWORD) override { return 0; }
	DWORD STDMETHODCALLTYPE GetLOD() override { return 0; }
	DWORD STDMETHODCALLTYPE GetLevelCount() override { return 1; }
	HRESULT STDMETHODCALLTYPE SetAutoGenFilterType(D3DTEXTUREFILTERTYPE) override { return D3D_OK; }
	D3DTEXTUREFILTERTYPE STDMETHODCALLTYPE GetAutoGenFilterType() override { return D3DTEXF_NONE; }
	void STDMETHODCALLTYPE GenerateMipSubLevels() override {}
	HRESULT STDMETHODCALLTYPE GetLevelDesc(UINT, D3DSURFACE_DESC* pDesc) override
	{
		if (pDesc) { *pDesc = desc; return D3D_OK; }
		return E_POINTER;
	}
	HRESULT STDMETHODCALLTYPE GetSurfaceLevel(UINT, IDirect3DSurface9**) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE LockRect(UINT, D3DLOCKED_RECT*, const RECT*, DWORD) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE UnlockRect(UINT) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE AddDirtyRect(const RECT*) override { return D3D_OK; }
};

class MockD3DVertexBuffer : public IDirect3DVertexBuffer9
{
public:
	ULONG refCount{ 1 };
	D3DVERTEXBUFFER_DESC desc;
	std::vector<uint8_t> buffer;

	MockD3DVertexBuffer(UINT size = 1024, DWORD fvf = 0, DWORD usage = D3DUSAGE_WRITEONLY)
	{
		std::memset(&desc, 0, sizeof(desc));
		desc.Size = size;
		desc.FVF = fvf;
		desc.Usage = usage;
		desc.Type = D3DRTYPE_VERTEXBUFFER;
		buffer.resize(size, 0);
	}

	HRESULT STDMETHODCALLTYPE QueryInterface(REFIID, void** ppvObj) override
	{
		if (!ppvObj) return E_POINTER;
		*ppvObj = nullptr;
		return E_NOINTERFACE;
	}
	ULONG STDMETHODCALLTYPE AddRef() override { return ++refCount; }
	ULONG STDMETHODCALLTYPE Release() override { return (refCount > 0) ? --refCount : 0; }
	HRESULT STDMETHODCALLTYPE GetDevice(IDirect3DDevice9**) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE SetPrivateData(REFGUID, const void*, DWORD, DWORD) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE GetPrivateData(REFGUID, void*, DWORD*) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE FreePrivateData(REFGUID) override { return D3D_OK; }
	DWORD STDMETHODCALLTYPE SetPriority(DWORD) override { return 0; }
	DWORD STDMETHODCALLTYPE GetPriority() override { return 0; }
	void STDMETHODCALLTYPE PreLoad() override {}
	D3DRESOURCETYPE STDMETHODCALLTYPE GetType() override { return D3DRTYPE_VERTEXBUFFER; }
	HRESULT STDMETHODCALLTYPE Lock(UINT OffsetToLock, UINT SizeToLock, void** ppbData, DWORD) override
	{
		if (!ppbData) return E_POINTER;
		UINT lockSize = (SizeToLock == 0) ? static_cast<UINT>(buffer.size()) : SizeToLock;
		if (OffsetToLock + lockSize > buffer.size()) return E_FAIL;
		*ppbData = buffer.data() + OffsetToLock;
		return D3D_OK;
	}
	HRESULT STDMETHODCALLTYPE Unlock() override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE GetDesc(D3DVERTEXBUFFER_DESC* pDesc) override
	{
		if (pDesc) { *pDesc = desc; return D3D_OK; }
		return E_POINTER;
	}
};

class MockD3DIndexBuffer : public IDirect3DIndexBuffer9
{
public:
	ULONG refCount{ 1 };
	D3DINDEXBUFFER_DESC desc;
	std::vector<uint8_t> buffer;

	MockD3DIndexBuffer(UINT size = 512, D3DFORMAT fmt = D3DFMT_INDEX16, DWORD usage = D3DUSAGE_WRITEONLY)
	{
		std::memset(&desc, 0, sizeof(desc));
		desc.Size = size;
		desc.Format = fmt;
		desc.Usage = usage;
		desc.Type = D3DRTYPE_INDEXBUFFER;
		buffer.resize(size, 0);
	}

	HRESULT STDMETHODCALLTYPE QueryInterface(REFIID, void** ppvObj) override
	{
		if (!ppvObj) return E_POINTER;
		*ppvObj = nullptr;
		return E_NOINTERFACE;
	}
	ULONG STDMETHODCALLTYPE AddRef() override { return ++refCount; }
	ULONG STDMETHODCALLTYPE Release() override { return (refCount > 0) ? --refCount : 0; }
	HRESULT STDMETHODCALLTYPE GetDevice(IDirect3DDevice9**) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE SetPrivateData(REFGUID, const void*, DWORD, DWORD) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE GetPrivateData(REFGUID, void*, DWORD*) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE FreePrivateData(REFGUID) override { return D3D_OK; }
	DWORD STDMETHODCALLTYPE SetPriority(DWORD) override { return 0; }
	DWORD STDMETHODCALLTYPE GetPriority() override { return 0; }
	void STDMETHODCALLTYPE PreLoad() override {}
	D3DRESOURCETYPE STDMETHODCALLTYPE GetType() override { return D3DRTYPE_INDEXBUFFER; }
	HRESULT STDMETHODCALLTYPE Lock(UINT OffsetToLock, UINT SizeToLock, void** ppbData, DWORD) override
	{
		if (!ppbData) return E_POINTER;
		UINT lockSize = (SizeToLock == 0) ? static_cast<UINT>(buffer.size()) : SizeToLock;
		if (OffsetToLock + lockSize > buffer.size()) return E_FAIL;
		*ppbData = buffer.data() + OffsetToLock;
		return D3D_OK;
	}
	HRESULT STDMETHODCALLTYPE Unlock() override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE GetDesc(D3DINDEXBUFFER_DESC* pDesc) override
	{
		if (pDesc) { *pDesc = desc; return D3D_OK; }
		return E_POINTER;
	}
};

// =========================================================================
// TESTY JEDNOSTKOWE
// =========================================================================

TEST_CASE("CpuCopies - Texture Pixel Registration and Multi-Mip in CRenderResourceRegistry")
{
	CRenderResourceRegistry registry;
	MockD3DTexture mockTex(64, 64, D3DFMT_A8R8G8B8);

	// Przygotuj piksele poziomu 0 (64x64 RGBA = 16384 bajtow)
	std::vector<uint8_t> mip0(64 * 64 * 4);
	for (size_t i = 0; i < mip0.size(); ++i)
		mip0[i] = static_cast<uint8_t>((i * 7 + 13) & 0xFF);

	TRenderTextureHandle hTex = registry.RegisterTextureWithPixels(
		&mockTex,
		64, 64,
		D3DFMT_A8R8G8B8,
		mip0.data(),
		mip0.size(),
		64 * 4,
		"TestTexture64"
	);

	CHECK(hTex != kInvalidTextureHandle);
	CHECK(registry.IsValidHandle(hTex));
	CHECK(registry.HasTextureCpuCopy(hTex) == true);
	CHECK(registry.GetTextureMipCount(hTex) == 1);

	// Pobranie kopii pikseli mip 0
	std::vector<uint8_t> extractedMip0;
	uint32_t extractedCrc = 0;
	CHECK(registry.GetTextureMip0Pixels(hTex, extractedMip0, &extractedCrc) == true);
	CHECK(extractedMip0.size() == mip0.size());
	CHECK(extractedMip0 == mip0);

	// Dolaczenie kolejnych poziomow mipmap (mip 1: 32x32, mip 2: 16x16)
	std::vector<uint8_t> mip1(32 * 32 * 4, 0xAA);
	std::vector<uint8_t> mip2(16 * 16 * 4, 0x55);

	CHECK(registry.AttachTexturePixelCopy(hTex, 1, 32, 32, 32 * 4, mip1.data(), mip1.size()) == true);
	CHECK(registry.AttachTexturePixelCopy(hTex, 2, 16, 16, 16 * 4, mip2.data(), mip2.size()) == true);
	CHECK(registry.GetTextureMipCount(hTex) == 3);

	// Weryfikacja odczytu mip 1 i mip 2
	STextureMipCpuCopy copyMip1;
	CHECK(registry.GetTextureMipPixelCopy(hTex, 1, &copyMip1) == true);
	CHECK(copyMip1.width == 32);
	CHECK(copyMip1.height == 32);
	CHECK(copyMip1.pixels == mip1);

	STextureMipCpuCopy copyMip2;
	CHECK(registry.GetTextureMipPixelCopy(hTex, 2, &copyMip2) == true);
	CHECK(copyMip2.width == 16);
	CHECK(copyMip2.height == 16);
	CHECK(copyMip2.pixels == mip2);

	// Proba pobrania nieistniejacego poziomu (mip 3)
	STextureMipCpuCopy copyMip3;
	CHECK(registry.GetTextureMipPixelCopy(hTex, 3, &copyMip3) == false);

	// Wyrejestrowanie tekstury zwalnia kopie CPU
	registry.UnregisterTexture(hTex);
	CHECK(registry.IsValidHandle(hTex) == false);
	CHECK(registry.GetTextureMip0Pixels(hTex, extractedMip0) == false);
}

TEST_CASE("CpuCopies - Texture Mip 0 CRC Verification")
{
	CRenderResourceRegistry registry;
	MockD3DTexture mockTex(32, 32, D3DFMT_A8R8G8B8);

	// Generowanie powtarzalnych danych pikseli
	std::vector<uint8_t> pixels(32 * 32 * 4);
	for (size_t i = 0; i < pixels.size(); ++i)
		pixels[i] = static_cast<uint8_t>((i * 131) ^ 0x5A);

	const uint32_t expectedCrc = GetCRC32(reinterpret_cast<const char*>(pixels.data()), pixels.size());
	CHECK(expectedCrc != 0);

	TRenderTextureHandle hTex = registry.RegisterTexture(&mockTex, "CrcTestTex");
	CHECK(registry.AttachTexturePixelCopy(hTex, 0, 32, 32, 32 * 4, pixels.data(), pixels.size()) == true);

	// Weryfikacja sumy kontrolnej
	CHECK(registry.GetTextureMip0Crc32(hTex) == expectedCrc);
	CHECK(registry.VerifyTextureMip0Crc(hTex, expectedCrc) == true);
	CHECK(registry.VerifyTextureMip0Crc(hTex, expectedCrc ^ 0x12345678) == false);

	// Zmiana pojedynczego bajtu w pikselach powinna zmienic CRC
	std::vector<uint8_t> corruptedPixels = pixels;
	corruptedPixels[100] ^= 0xFF;
	const uint32_t corruptedCrc = GetCRC32(reinterpret_cast<const char*>(corruptedPixels.data()), corruptedPixels.size());
	CHECK(corruptedCrc != expectedCrc);

	// Zaktualizuj piksele mip 0
	CHECK(registry.AttachTexturePixelCopy(hTex, 0, 32, 32, 32 * 4, corruptedPixels.data(), corruptedPixels.size()) == true);
	CHECK(registry.GetTextureMip0Crc32(hTex) == corruptedCrc);
	CHECK(registry.VerifyTextureMip0Crc(hTex, corruptedCrc) == true);
	CHECK(registry.VerifyTextureMip0Crc(hTex, expectedCrc) == false);

	// Bezpieczne metody statyczne
	CHECK(CRenderResourceRegistry::SafeVerifyTextureMip0Crc(hTex, corruptedCrc) == true);
	CHECK(CRenderResourceRegistry::SafeVerifyTextureMip0Crc(kInvalidTextureHandle, corruptedCrc) == false);
}

TEST_CASE("CpuCopies - Staging Stride Pitch Unpacking and Pixel Integrity")
{
	CRenderResourceRegistry registry;
	MockD3DTexture mockTex(16, 16, D3DFMT_A8R8G8B8);

	// Symulacja bufora stagingu z paddingiem pitcha (np. rowBytes = 64, pitch = 80 bajtow)
	const size_t width = 16;
	const size_t height = 16;
	const size_t rowBytes = width * 4; // 64
	const size_t pitch = 80; // 16 bajtow paddingu per wiersz

	std::vector<uint8_t> stagingBuffer(height * pitch, 0xEE);
	std::vector<uint8_t> purePixelData(height * rowBytes);

	for (size_t y = 0; y < height; ++y)
	{
		for (size_t x = 0; x < rowBytes; ++x)
		{
			uint8_t val = static_cast<uint8_t>((y * 31 + x * 7) & 0xFF);
			stagingBuffer[y * pitch + x] = val;
			purePixelData[y * rowBytes + x] = val;
		}
	}

	// Rozpakowanie wiersz po wierszu, tak jak w DDSTextureLoader9 i GrpTexture
	std::vector<uint8_t> unpackedPixels(height * rowBytes);
	const uint8_t* pStagedRow = stagingBuffer.data();
	for (size_t y = 0; y < height; ++y)
	{
		memcpy(unpackedPixels.data() + y * rowBytes, pStagedRow, rowBytes);
		pStagedRow += pitch;
	}

	CHECK(unpackedPixels == purePixelData);

	TRenderTextureHandle hTex = registry.RegisterTextureWithPixels(
		&mockTex,
		static_cast<uint32_t>(width),
		static_cast<uint32_t>(height),
		D3DFMT_A8R8G8B8,
		unpackedPixels.data(),
		unpackedPixels.size(),
		static_cast<uint32_t>(rowBytes),
		"StagingUnpacked"
	);

	std::vector<uint8_t> fromRegistry;
	CHECK(registry.GetTextureMip0Pixels(hTex, fromRegistry) == true);
	CHECK(fromRegistry == purePixelData);

	const uint32_t expectedCrc = GetCRC32(reinterpret_cast<const char*>(purePixelData.data()), purePixelData.size());
	CHECK(registry.GetTextureMip0Crc32(hTex) == expectedCrc);
	CHECK(registry.VerifyTextureMip0Crc(hTex, expectedCrc) == true);
}

TEST_CASE("CpuCopies - Safe CPU Retrieval for Vertex Buffers (Geometry)")
{
	CRenderResourceRegistry registry;
	MockD3DVertexBuffer mockVB(2048, 0x142 /* D3DFVF_XYZ | D3DFVF_NORMAL | D3DFVF_TEX1 */, D3DUSAGE_WRITEONLY);

	// Przygotuj wierzcholki (64 wierzcholki po 32 bajty = 2048 bajtow)
	struct SVertex
	{
		float x, y, z;
		float nx, ny, nz;
		float u, v;
	};
	static_assert(sizeof(SVertex) == 32, "Rozmiar wierzcholka musi wynosic 32 bajty");

	std::vector<SVertex> vertices(64);
	for (size_t i = 0; i < vertices.size(); ++i)
	{
		vertices[i].x = static_cast<float>(i);
		vertices[i].y = static_cast<float>(i * 2);
		vertices[i].z = static_cast<float>(i * 3);
		vertices[i].nx = 0.0f; vertices[i].ny = 1.0f; vertices[i].nz = 0.0f;
		vertices[i].u = static_cast<float>(i) / 64.0f;
		vertices[i].v = 1.0f;
	}

	const size_t vbSizeBytes = vertices.size() * sizeof(SVertex);
	const uint32_t expectedCrc = GetCRC32(reinterpret_cast<const char*>(vertices.data()), vbSizeBytes);

	TRenderVertexBufferHandle hVB = registry.RegisterVertexBuffer(
		&mockVB,
		static_cast<uint32_t>(vbSizeBytes),
		0x142,
		vertices.data(),
		"MeshStaticVB"
	);

	CHECK(hVB != kInvalidVertexBufferHandle);
	CHECK(registry.GetActiveVertexBufferCount() == 1);
	CHECK(registry.GetVertexBufferCrc32(hVB) == expectedCrc);
	CHECK(registry.VerifyVertexBufferCrc(hVB, expectedCrc) == true);

	// Bezpieczne pobranie kopii CPU
	std::vector<uint8_t> extractedBytes;
	CHECK(registry.GetVertexBufferCpuCopy(hVB, extractedBytes) == true);
	CHECK(extractedBytes.size() == vbSizeBytes);
	CHECK(std::memcmp(extractedBytes.data(), vertices.data(), vbSizeBytes) == 0);

	// Pobranie po wskazniku D3D9
	std::vector<uint8_t> extractedByPtr;
	CHECK(registry.GetVertexBufferCpuCopy(&mockVB, extractedByPtr) == true);
	CHECK(extractedByPtr == extractedBytes);

	// Metadane bufora
	SVertexBufferResourceInfo vbInfo;
	CHECK(registry.GetVertexBufferInfo(hVB, &vbInfo) == true);
	CHECK(vbInfo.size == vbSizeBytes);
	CHECK(vbInfo.fvf == 0x142);
	CHECK(vbInfo.crc32 == expectedCrc);
	CHECK(vbInfo.isStatic == true);

	// Bezpieczenstwo: bledny uchwyt nie powoduje awarii
	std::vector<uint8_t> dummy;
	CHECK(registry.GetVertexBufferCpuCopy(kInvalidVertexBufferHandle, dummy) == false);
	CHECK(registry.GetVertexBufferCpuCopy(RenderResourceMakeHandle(999, 1), dummy) == false);

	// Wyrejestrowanie
	registry.UnregisterVertexBuffer(hVB);
	CHECK(registry.GetActiveVertexBufferCount() == 0);
	CHECK(registry.GetVertexBufferCpuCopy(hVB, dummy) == false);
}

TEST_CASE("CpuCopies - Safe CPU Retrieval for Index Buffers (Geometry)")
{
	CRenderResourceRegistry registry;
	MockD3DIndexBuffer mockIB(240, D3DFMT_INDEX16, D3DUSAGE_WRITEONLY);

	// 120 indeksow 16-bitowych = 40 trojkatow = 240 bajtow
	std::vector<uint16_t> indices(120);
	for (size_t i = 0; i < indices.size(); ++i)
	{
		indices[i] = static_cast<uint16_t>(i % 64);
	}

	const size_t ibSizeBytes = indices.size() * sizeof(uint16_t);
	const uint32_t expectedCrc = GetCRC32(reinterpret_cast<const char*>(indices.data()), ibSizeBytes);

	TRenderIndexBufferHandle hIB = registry.RegisterIndexBuffer(
		&mockIB,
		static_cast<uint32_t>(ibSizeBytes),
		D3DFMT_INDEX16,
		indices.data(),
		"MeshStaticIB"
	);

	CHECK(hIB != kInvalidIndexBufferHandle);
	CHECK(registry.GetActiveIndexBufferCount() == 1);
	CHECK(registry.GetIndexBufferCrc32(hIB) == expectedCrc);
	CHECK(registry.VerifyIndexBufferCrc(hIB, expectedCrc) == true);

	// Pobranie kopii CPU
	std::vector<uint8_t> extractedBytes;
	CHECK(registry.GetIndexBufferCpuCopy(hIB, extractedBytes) == true);
	CHECK(extractedBytes.size() == ibSizeBytes);
	CHECK(std::memcmp(extractedBytes.data(), indices.data(), ibSizeBytes) == 0);

	// Pobranie po wskazniku D3D9
	std::vector<uint8_t> extractedByPtr;
	CHECK(registry.GetIndexBufferCpuCopy(&mockIB, extractedByPtr) == true);
	CHECK(extractedByPtr == extractedBytes);

	// Aktualizacja kopii CPU
	std::vector<uint16_t> updatedIndices = indices;
	for (auto& idx : updatedIndices) idx = static_cast<uint16_t>(idx + 1);
	const uint32_t updatedCrc = GetCRC32(reinterpret_cast<const char*>(updatedIndices.data()), ibSizeBytes);

	CHECK(registry.SetIndexBufferCpuCopy(hIB, updatedIndices.data(), ibSizeBytes) == true);
	CHECK(registry.GetIndexBufferCrc32(hIB) == updatedCrc);
	CHECK(registry.VerifyIndexBufferCrc(hIB, updatedCrc) == true);

	// Wyrejestrowanie
	registry.UnregisterIndexBuffer(hIB);
	CHECK(registry.GetActiveIndexBufferCount() == 0);
	std::vector<uint8_t> dummy;
	CHECK(registry.GetIndexBufferCpuCopy(hIB, dummy) == false);
}

TEST_CASE("CpuCopies - Static VB/IB CGraphicVertexBuffer and CGraphicIndexBuffer CPU Shadow")
{
	CRenderResourceRegistry registry;

	// Weryfikacja metod pomocniczych CGraphicVertexBuffer
	CGraphicVertexBuffer vb;
	CHECK(vb.HasCpuCopy() == false);

	std::vector<uint8_t> srcVertices(256, 0x77);
	// W testach jednostkowych (bez zainicjalizowanego ms_lpd3dDevice) bezposrednie wywolanie Copy
	// zawiedzie na braku m_lpd3dVB, ale rejestr zachowa kopie CPU przypisana do bufora
	std::vector<uint8_t> retrieved;
	CHECK(vb.GetCpuCopy(retrieved) == false);
	CHECK(vb.GetCpuCopyCrc32() == 0);

	// Weryfikacja metod pomocniczych CGraphicIndexBuffer
	CGraphicIndexBuffer ib;
	CHECK(ib.HasCpuCopy() == false);
	CHECK(ib.GetCpuCopy(retrieved) == false);
	CHECK(ib.GetCpuCopyCrc32() == 0);
}

TEST_CASE("CpuCopies - Generation Increment and Slot Reuse Safety")
{
	CRenderResourceRegistry registry;
	MockD3DTexture tex1(32, 32);
	MockD3DTexture tex2(32, 32);

	std::vector<uint8_t> data1(32 * 32 * 4, 0x11);
	std::vector<uint8_t> data2(32 * 32 * 4, 0x22);

	TRenderTextureHandle h1 = registry.RegisterTextureWithPixels(&tex1, 32, 32, D3DFMT_A8R8G8B8, data1.data(), data1.size());
	uint32_t slot1 = RenderResourceGetSlotIndex(h1);
	uint32_t gen1 = RenderResourceGetGeneration(h1);

	// Wyrejestrowanie tekstury 1
	registry.UnregisterTexture(h1);
	CHECK(registry.IsValidHandle(h1) == false);

	// Rejestracja tekstury 2 powinna ponownie uzyc slot1, lecz z nowa generacja
	TRenderTextureHandle h2 = registry.RegisterTextureWithPixels(&tex2, 32, 32, D3DFMT_A8R8G8B8, data2.data(), data2.size());
	uint32_t slot2 = RenderResourceGetSlotIndex(h2);
	uint32_t gen2 = RenderResourceGetGeneration(h2);

	CHECK(slot1 == slot2);
	CHECK(gen2 > gen1);

	// Stary uchwyt h1 NIE MOZE miec dostepu do nowych pikseli tekstury 2
	std::vector<uint8_t> staleData;
	CHECK(registry.GetTextureMip0Pixels(h1, staleData) == false);
	CHECK(registry.VerifyTextureMip0Crc(h1, GetCRC32(reinterpret_cast<const char*>(data1.data()), data1.size())) == false);

	// Nowy uchwyt h2 ma prawidlowy dostep
	std::vector<uint8_t> validData;
	CHECK(registry.GetTextureMip0Pixels(h2, validData) == true);
	CHECK(validData == data2);
}
