#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"

#include <d3d9.h>
#include <d3dx9.h>
#include <vector>
#include <cstring>

#include "EterLib/RenderPacketTypes.h"
#include "EterLib/RenderQueueManager.h"
#include "EterLib/RenderResourceRegistry.h"
#include "EterLib/RenderOwnership.h"
#include "EterLib/IRenderBackend.h"
#include "EterLib/GrpBase.h"

namespace
{
// Mock interfejsu IDirect3DTexture9 dla testow jednostkowych
class MockEffectMeshD3DTexture : public IDirect3DTexture9
{
public:
	ULONG refCount{ 1 };
	D3DSURFACE_DESC desc;

	MockEffectMeshD3DTexture(UINT w = 128, UINT h = 128, D3DFORMAT fmt = D3DFMT_A8R8G8B8)
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

// Mock backendu przechwytujacy wywolania RenderEffectMesh
class MockEffectMeshBackend : public IRenderBackend
{
public:
	bool Initialize(HWND hWnd, unsigned int width, unsigned int height) override { return true; }
	void Shutdown() override {}
	void Resize(unsigned int width, unsigned int height) override {}
	void BeginFrame(const TRenderFrameUniforms& uniforms) override { m_bBeginFrameCalled = true; }
	void EndFrame() override { m_bEndFrameCalled = true; }
	const char* GetBackendName() const override { return "MockEffectMeshBackend"; }
	bool IsDeviceLost() const override { return false; }
	bool CaptureFrame(const char* szFilename) override { return true; }

	void RenderTerrain(const TTerrainSectorPacket* pPackets, unsigned int count) override {}
	void RenderActors(const TActorRenderPacket* pPackets, unsigned int count) override {}
	void RenderWater(const TWaterPacket* pPackets, unsigned int count) override {}
	void RenderSpeedTree(const TSpeedTreePacket* pPackets, unsigned int count) override {}
	void RenderEffects(const TEffectInstancePacket* pPackets, unsigned int count) override {}
	void RenderUI(const TUIBatchPacket* pPackets, unsigned int count) override {}
	void RenderWeaponTrace(const TWeaponTracePacket* pPackets, unsigned int count) override {}

	void RenderEffectMesh(const TEffectMeshPacket* pPackets, unsigned int count) override
	{
		if (pPackets && count > 0)
		{
			m_receivedPackets.assign(pPackets, pPackets + count);
		}
	}

	bool m_bBeginFrameCalled{ false };
	bool m_bEndFrameCalled{ false };
	std::vector<TEffectMeshPacket> m_receivedPackets;
};
}

TEST_CASE("EffectMeshABI - Rozmiar, uklad pamieci i zero-inicjalizacja TEffectMeshPacket")
{
	CHECK(sizeof(TEffectMeshPacket) == 88u);

	CHECK(offsetof(TEffectMeshPacket, matWorld) == 0u);
	CHECK(offsetof(TEffectMeshPacket, m_matWorld) == 0u);
	CHECK(offsetof(TEffectMeshPacket, textureHandle) == 64u);
	CHECK(offsetof(TEffectMeshPacket, hTexture) == 64u);
	CHECK(offsetof(TEffectMeshPacket, textureFactorColor) == 68u);
	CHECK(offsetof(TEffectMeshPacket, dwTFactor) == 68u);
	CHECK(offsetof(TEffectMeshPacket, colorOp) == 72u);
	CHECK(offsetof(TEffectMeshPacket, dwColorOp) == 72u);
	CHECK(offsetof(TEffectMeshPacket, srcBlend) == 76u);
	CHECK(offsetof(TEffectMeshPacket, destBlend) == 77u);
	CHECK(offsetof(TEffectMeshPacket, bPad) == 78u);
	CHECK(offsetof(TEffectMeshPacket, vertexOffset) == 80u);
	CHECK(offsetof(TEffectMeshPacket, vertexCount) == 84u);

	// Zero-inicjalizacja domyslnego konstruktora
	TEffectMeshPacket packet;
	const uint8_t* pRaw = reinterpret_cast<const uint8_t*>(&packet);
	bool bAllZero = true;
	for (size_t b = 0; b < sizeof(TEffectMeshPacket); ++b)
	{
		if (pRaw[b] != 0)
		{
			bAllZero = false;
			break;
		}
	}
	CHECK(bAllZero);

	// Weryfikacja pol unii
	CHECK(packet.textureHandle == 0u);
	CHECK(packet.hTexture == 0u);
	CHECK(packet.dwTFactor == 0u);
	CHECK(packet.textureFactorColor == 0u);
	CHECK(packet.dwColorOp == 0u);
	CHECK(packet.colorOp == 0u);
	CHECK(packet.vertexOffset == 0u);
	CHECK(packet.vertexCount == 0u);
}

TEST_CASE("EffectMeshABI - Poprawnosc pol pakietu i aliasow")
{
	TEffectMeshPacket pkt{};
	pkt.hTexture = 0x12345678;
	pkt.dwTFactor = 0xAABBCCDD;
	pkt.dwColorOp = 4; // D3DTOP_MODULATE
	pkt.srcBlend = static_cast<uint8_t>(BLEND_FACTOR_SRC_ALPHA);
	pkt.destBlend = static_cast<uint8_t>(BLEND_FACTOR_INV_SRC_ALPHA);
	pkt.vertexOffset = 100;
	pkt.vertexCount = 30;

	D3DXMATRIX testMat;
	D3DXMatrixIdentity(&testMat);
	testMat._41 = 100.0f;
	testMat._42 = 200.0f;
	testMat._43 = 300.0f;
	std::memcpy(&pkt.m_matWorld, &testMat, sizeof(TRenderMatrix4x4));

	// Weryfikacja unii i wartosci
	CHECK(pkt.textureHandle == 0x12345678);
	CHECK(pkt.textureFactorColor == 0xAABBCCDD);
	CHECK(pkt.colorOp == 4);
	CHECK(pkt.srcBlend == 5);
	CHECK(pkt.destBlend == 6);
	CHECK(pkt.vertexOffset == 100);
	CHECK(pkt.vertexCount == 30);

	CHECK(pkt.m_matWorld._41 == 100.0f);
	CHECK(pkt.m_matWorld._42 == 200.0f);
	CHECK(pkt.m_matWorld._43 == 300.0f);
	CHECK(pkt.matWorld._41 == 100.0f);
}

TEST_CASE("EffectMesh - Rejestracja tekstury w CRenderResourceRegistry")
{
	CRenderResourceRegistry registry;
	MockEffectMeshD3DTexture mockTexture(256, 128, D3DFMT_A8R8G8B8);

	const TRenderTextureHandle handle = registry.RegisterTexture(&mockTexture, "EffectMeshTex");
	CHECK(handle != kInvalidTextureHandle);
	CHECK(registry.IsValidHandle(handle));
	CHECK(registry.ResolveTexture(handle) == &mockTexture);
	CHECK(registry.GetDebugName(handle) == "EffectMeshTex");
	CHECK(registry.GetWidth(handle) == 256u);
	CHECK(registry.GetHeight(handle) == 128u);

	registry.UnregisterTexture(handle);
	CHECK_FALSE(registry.IsValidHandle(handle));
	CHECK(registry.ResolveTexture(handle) == nullptr);
}

TEST_CASE("EffectMesh - Alokacja i zarzadzanie wierzcholkami w CRenderQueueManager")
{
	CRenderQueueManager& rqm = CRenderQueueManager::Instance();
	rqm.BeginFrame();

	CHECK(rqm.GetEffectMeshVertexOffset() == 0u);
	CHECK(rqm.GetEffectMeshPoolCapacity() >= 32768u);
	CHECK(rqm.GetDroppedEffectMesh() == 0u);

	const size_t vertexCount = 12;
	TPTVertex* pVertices = rqm.AllocateEffectMeshVertexData(vertexCount);
	REQUIRE(pVertices != nullptr);
	CHECK(rqm.GetEffectMeshVertexOffset() == 12u);

	for (size_t i = 0; i < vertexCount; ++i)
	{
		pVertices[i].position = D3DXVECTOR3(static_cast<float>(i * 10), 0.0f, 0.0f);
		pVertices[i].texCoord = D3DXVECTOR2(0.25f, 0.75f);
	}

	const TPTVertex* pAllVertices = rqm.GetEffectMeshVertexData();
	REQUIRE(pAllVertices != nullptr);
	CHECK(pAllVertices[0].position.x == 0.0f);
	CHECK(pAllVertices[5].position.x == 50.0f);
	CHECK(pAllVertices[11].position.x == 110.0f);

	// Test alokacji zero
	CHECK(rqm.AllocateEffectMeshVertexData(0) == nullptr);
	CHECK(rqm.GetEffectMeshVertexOffset() == 12u);

	// Test przepelnienia puli
	const size_t remaining = rqm.GetEffectMeshPoolCapacity() - rqm.GetEffectMeshVertexOffset();
	TPTVertex* pOverflow = rqm.AllocateEffectMeshVertexData(remaining + 100);
	CHECK(pOverflow == nullptr);
	CHECK(rqm.GetDroppedEffectMesh() == remaining + 100);

	rqm.ResetDroppedCounters();
	CHECK(rqm.GetDroppedEffectMesh() == 0u);
}

TEST_CASE("EffectMesh - Kolejkowanie pakietow i dispatch do backendu (Flush)")
{
	CRenderQueueManager& rqm = CRenderQueueManager::Instance();
	rqm.BeginFrame();

	CHECK(rqm.GetEffectMeshPackets().empty());

	// Przygotowanie geometrii w puli
	const size_t vtxCount = 6;
	TPTVertex* pAllocated = rqm.AllocateEffectMeshVertexData(vtxCount);
	REQUIRE(pAllocated != nullptr);
	const uint32_t offset = static_cast<uint32_t>(rqm.GetEffectMeshVertexOffset() - vtxCount);

	pAllocated[0].position = D3DXVECTOR3(1.0f, 2.0f, 3.0f);
	pAllocated[0].texCoord = D3DXVECTOR2(0.0f, 0.0f);

	// Pakiet 1: Blending wlaczony (SRCALPHA, INVSRCALPHA)
	TEffectMeshPacket pkt1{};
	D3DXMatrixIdentity((D3DXMATRIX*)&pkt1.m_matWorld);
	pkt1.hTexture = 42;
	pkt1.dwTFactor = 0xFF804020;
	pkt1.dwColorOp = 4;
	pkt1.srcBlend = static_cast<uint8_t>(BLEND_FACTOR_SRC_ALPHA);
	pkt1.destBlend = static_cast<uint8_t>(BLEND_FACTOR_INV_SRC_ALPHA);
	pkt1.vertexOffset = offset;
	pkt1.vertexCount = 3;

	// Pakiet 2: Blending wylaczony (BLEND_FACTOR_ONE, BLEND_FACTOR_ZERO)
	TEffectMeshPacket pkt2{};
	D3DXMatrixIdentity((D3DXMATRIX*)&pkt2.m_matWorld);
	pkt2.hTexture = 43;
	pkt2.dwTFactor = 0xFFFFFFFF;
	pkt2.dwColorOp = 4;
	pkt2.srcBlend = static_cast<uint8_t>(BLEND_FACTOR_ONE);
	pkt2.destBlend = static_cast<uint8_t>(BLEND_FACTOR_ZERO);
	pkt2.vertexOffset = offset + 3;
	pkt2.vertexCount = 3;

	rqm.SubmitEffectMesh(pkt1);
	rqm.SubmitEffectMesh(pkt2);

	CHECK(rqm.GetEffectMeshPackets().size() == 2u);
	CHECK(rqm.GetTotalPacketCount() >= 2u);

	// Dispatch przez Flush do mock backendu
	MockEffectMeshBackend mockBackend;
	rqm.Flush(&mockBackend);

	REQUIRE(mockBackend.m_receivedPackets.size() == 2u);

	const TEffectMeshPacket& recv1 = mockBackend.m_receivedPackets[0];
	CHECK(recv1.hTexture == 42u);
	CHECK(recv1.dwTFactor == 0xFF804020);
	CHECK(recv1.dwColorOp == 4u);
	CHECK(recv1.srcBlend == static_cast<uint8_t>(BLEND_FACTOR_SRC_ALPHA));
	CHECK(recv1.destBlend == static_cast<uint8_t>(BLEND_FACTOR_INV_SRC_ALPHA));
	CHECK(recv1.vertexOffset == offset);
	CHECK(recv1.vertexCount == 3u);

	const TEffectMeshPacket& recv2 = mockBackend.m_receivedPackets[1];
	CHECK(recv2.hTexture == 43u);
	CHECK(recv2.dwTFactor == 0xFFFFFFFF);
	CHECK(recv2.dwColorOp == 4u);
	CHECK(recv2.srcBlend == static_cast<uint8_t>(BLEND_FACTOR_ONE));
	CHECK(recv2.destBlend == static_cast<uint8_t>(BLEND_FACTOR_ZERO));
	CHECK(recv2.vertexOffset == offset + 3);
	CHECK(recv2.vertexCount == 3u);

	// BeginFrame czysci pakiety
	rqm.BeginFrame();
	CHECK(rqm.GetEffectMeshPackets().empty());
	CHECK(rqm.GetEffectMeshVertexOffset() == 0u);
}

TEST_CASE("EffectMesh - RenderQueueManager GetPacketSizes zawiera TEffectMeshPacket")
{
	const auto sizeMap = CRenderQueueManager::GetPacketSizeMap();
	CHECK(sizeMap.find("TEffectMeshPacket") != sizeMap.end());
	CHECK(sizeMap.at("TEffectMeshPacket") == sizeof(TEffectMeshPacket));
	CHECK(sizeMap.at("TEffectMeshPacket") == 88u);
}
