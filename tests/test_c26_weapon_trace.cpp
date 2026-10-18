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
// Mock interfejsu IDirect3DTexture9 dla testow jednostkowych rejestru zasobow
class MockTraceD3DTexture : public IDirect3DTexture9
{
public:
	ULONG refCount{ 1 };
	D3DSURFACE_DESC desc;

	MockTraceD3DTexture(UINT w = 128, UINT h = 128, D3DFORMAT fmt = D3DFMT_A8R8G8B8)
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

// Mock backendu przechwytujacy wywolania RenderWeaponTrace
class MockTraceBackend : public IRenderBackend
{
public:
	bool Initialize(HWND hWnd, unsigned int width, unsigned int height) override { return true; }
	void Shutdown() override {}
	void Resize(unsigned int width, unsigned int height) override {}
	void BeginFrame(const TRenderFrameUniforms& uniforms) override { m_bBeginFrameCalled = true; }
	void EndFrame() override { m_bEndFrameCalled = true; }
	const char* GetBackendName() const override { return "MockTraceBackend"; }
	bool IsDeviceLost() const override { return false; }
	bool CaptureFrame(const char* szFilename) override { return true; }

	void RenderTerrain(const TTerrainSectorPacket* pPackets, unsigned int count) override {}
	void RenderActors(const TActorRenderPacket* pPackets, unsigned int count) override {}
	void RenderWater(const TWaterPacket* pPackets, unsigned int count) override {}
	void RenderSpeedTree(const TSpeedTreePacket* pPackets, unsigned int count) override {}
	void RenderEffects(const TEffectInstancePacket* pPackets, unsigned int count) override {}
	void RenderUI(const TUIBatchPacket* pPackets, unsigned int count) override {}

	void RenderWeaponTrace(const TWeaponTracePacket* pPackets, unsigned int count) override
	{
		m_receivedPackets.assign(pPackets, pPackets + count);
	}

	bool m_bBeginFrameCalled{ false };
	bool m_bEndFrameCalled{ false };
	std::vector<TWeaponTracePacket> m_receivedPackets;
};
}

TEST_CASE("WeaponTraceABI - Rozmiar i uklad binarny TWeaponTracePacket")
{
	CHECK(sizeof(TWeaponTracePacket) == 24u);
	CHECK(sizeof(TPipelineStateKey) == 8u);

	CHECK(offsetof(TWeaponTracePacket, textureHandle) == 0u);
	CHECK(offsetof(TWeaponTracePacket, stateKey) == 4u);
	CHECK(offsetof(TWeaponTracePacket, vertexOffset) == 12u);
	CHECK(offsetof(TWeaponTracePacket, vertexCount) == 16u);
	CHECK(offsetof(TWeaponTracePacket, dwPad) == 20u);

	// Sprawdzenie pol klucza potoku TPipelineStateKey
	CHECK(offsetof(TPipelineStateKey, srcBlend) == 0u);
	CHECK(offsetof(TPipelineStateKey, destBlend) == 1u);
	CHECK(offsetof(TPipelineStateKey, cullMode) == 2u);
	CHECK(offsetof(TPipelineStateKey, zTest) == 3u);
	CHECK(offsetof(TPipelineStateKey, zWrite) == 4u);
	CHECK(offsetof(TPipelineStateKey, alphaRef) == 5u);
	CHECK(offsetof(TPipelineStateKey, fogEnable) == 6u);
	CHECK(offsetof(TPipelineStateKey, colorWrite) == 7u);
}

TEST_CASE("WeaponTrace - Stan potoku z TPipelineStateKey (F-NEW-030)")
{
	TPipelineStateKey key{};
	key.srcBlend = D3DBLEND_SRCALPHA;
	key.destBlend = D3DBLEND_INVSRCALPHA;
	key.cullMode = D3DCULL_NONE;
	key.zTest = 1;
	key.zWrite = 0;
	key.alphaRef = 0x11;
	key.fogEnable = 0;
	key.colorWrite = 0x0F;

	CHECK(key.srcBlend == static_cast<uint8_t>(D3DBLEND_SRCALPHA));
	CHECK(key.destBlend == static_cast<uint8_t>(D3DBLEND_INVSRCALPHA));
	CHECK(key.cullMode == static_cast<uint8_t>(D3DCULL_NONE));
	CHECK(key.zTest == 1u);
	CHECK(key.zWrite == 0u);
	CHECK(key.alphaRef == 0x11u);
	CHECK(key.fogEnable == 0u);
	CHECK(key.colorWrite == 0x0Fu);
}

TEST_CASE("WeaponTrace - Rejestracja tekstury w CRenderResourceRegistry wg DEC-5 (F-P1-26)")
{
	CRenderResourceRegistry registry;
	MockTraceD3DTexture mockTexture(256, 256, D3DFMT_A8R8G8B8);

	// Rejestracja tekstury lot_ade10-2.tga
	const TRenderTextureHandle handle = registry.RegisterTexture(&mockTexture, "lot_ade10-2.tga");
	CHECK(handle != kInvalidTextureHandle);
	CHECK(registry.IsValidHandle(handle));
	CHECK(registry.ResolveTexture(handle) == &mockTexture);
	CHECK(registry.GetDebugName(handle) == "lot_ade10-2.tga");
	CHECK(registry.GetWidth(handle) == 256u);
	CHECK(registry.GetHeight(handle) == 256u);

	// Odrejestrowanie
	registry.UnregisterTexture(handle);
	CHECK_FALSE(registry.IsValidHandle(handle));
	CHECK(registry.ResolveTexture(handle) == nullptr);
}

TEST_CASE("WeaponTrace - Alokacja i zarzadzanie wierzcholkami w CRenderQueueManager")
{
	CRenderQueueManager& rqm = CRenderQueueManager::Instance();
	rqm.BeginFrame();

	CHECK(rqm.GetWeaponTraceVertexOffset() == 0u);
	CHECK(rqm.GetWeaponTracePoolCapacity() >= 16384u);

	const size_t vertexCount = 10;
	TPDTVertex* pVertices = rqm.AllocateWeaponTraceVertexData(vertexCount);
	REQUIRE(pVertices != nullptr);
	CHECK(rqm.GetWeaponTraceVertexOffset() == 10u);

	for (size_t i = 0; i < vertexCount; ++i)
	{
		pVertices[i].position = D3DXVECTOR3(static_cast<float>(i), 10.0f, 20.0f);
		pVertices[i].diffuse = 0xFFFFFFFF;
		pVertices[i].texCoord = D3DXVECTOR2(0.5f, 0.5f);
	}

	const TPDTVertex* pBase = rqm.GetWeaponTraceVertexData();
	REQUIRE(pBase != nullptr);
	CHECK(pBase[0].position.x == 0.0f);
	CHECK(pBase[9].position.x == 9.0f);

	// Druga alokacja - ciaglosc w buforze
	TPDTVertex* pSecond = rqm.AllocateWeaponTraceVertexData(6);
	REQUIRE(pSecond != nullptr);
	CHECK(rqm.GetWeaponTraceVertexOffset() == 16u);
	CHECK(pSecond == pBase + 10);

	// Nowa klatka resetuje offset
	rqm.BeginFrame();
	CHECK(rqm.GetWeaponTraceVertexOffset() == 0u);
}

TEST_CASE("WeaponTrace - Przeksztalcenie wstegi w pakiet TWeaponTracePacket i kolejkowanie")
{
	CRenderQueueManager& rqm = CRenderQueueManager::Instance();
	rqm.BeginFrame();

	// Tworzymy przykladowa wstege trojkatow (4 wierzcholki = 2 trojkaty)
	const size_t vertexCount = 4;
	const uint32_t currentOffset = static_cast<uint32_t>(rqm.GetWeaponTraceVertexOffset());
	TPDTVertex* pVtx = rqm.AllocateWeaponTraceVertexData(vertexCount);
	REQUIRE(pVtx != nullptr);

	pVtx[0].position = D3DXVECTOR3(0.0f, 0.0f, 0.0f);
	pVtx[0].diffuse = 0xFF00FF00;
	pVtx[0].texCoord = D3DXVECTOR2(0.0f, 0.0f);

	pVtx[1].position = D3DXVECTOR3(0.0f, 10.0f, 0.0f);
	pVtx[1].diffuse = 0xFF00FF00;
	pVtx[1].texCoord = D3DXVECTOR2(0.0f, 1.0f);

	pVtx[2].position = D3DXVECTOR3(10.0f, 0.0f, 0.0f);
	pVtx[2].diffuse = 0xFF00FF00;
	pVtx[2].texCoord = D3DXVECTOR2(1.0f, 0.0f);

	pVtx[3].position = D3DXVECTOR3(10.0f, 10.0f, 0.0f);
	pVtx[3].diffuse = 0xFF00FF00;
	pVtx[3].texCoord = D3DXVECTOR2(1.0f, 1.0f);

	TPipelineStateKey stateKey{};
	stateKey.srcBlend = D3DBLEND_SRCALPHA;
	stateKey.destBlend = D3DBLEND_INVSRCALPHA;
	stateKey.cullMode = D3DCULL_NONE;
	stateKey.zTest = 1;
	stateKey.zWrite = 0;
	stateKey.alphaRef = 0x11;
	stateKey.fogEnable = 0;
	stateKey.colorWrite = 0x0F;

	TWeaponTracePacket packet{};
	packet.textureHandle = 101u;
	packet.stateKey = stateKey;
	packet.vertexOffset = currentOffset;
	packet.vertexCount = static_cast<uint32_t>(vertexCount);
	packet.dwPad = 0u;

	rqm.SubmitWeaponTrace(packet);

	const auto& packets = rqm.GetWeaponTracePackets();
	REQUIRE(packets.size() == 1u);
	CHECK(packets[0].textureHandle == 101u);
	CHECK(packets[0].vertexOffset == 0u);
	CHECK(packets[0].vertexCount == 4u);
	CHECK(packets[0].stateKey.srcBlend == static_cast<uint8_t>(D3DBLEND_SRCALPHA));
	CHECK(packets[0].stateKey.destBlend == static_cast<uint8_t>(D3DBLEND_INVSRCALPHA));
	CHECK(packets[0].stateKey.cullMode == static_cast<uint8_t>(D3DCULL_NONE));
}

TEST_CASE("WeaponTrace - Dispatch pakietu do backendu przez Flush")
{
	CRenderQueueManager& rqm = CRenderQueueManager::Instance();
	rqm.BeginFrame();

	TPDTVertex* pVtx = rqm.AllocateWeaponTraceVertexData(4);
	REQUIRE(pVtx != nullptr);

	TWeaponTracePacket pkt{};
	pkt.textureHandle = 555u;
	pkt.stateKey.srcBlend = D3DBLEND_SRCALPHA;
	pkt.stateKey.destBlend = D3DBLEND_INVSRCALPHA;
	pkt.vertexOffset = 0u;
	pkt.vertexCount = 4u;
	rqm.SubmitWeaponTrace(pkt);

	MockTraceBackend mockBackend;
	rqm.Flush(&mockBackend);

	CHECK(mockBackend.m_bBeginFrameCalled);
	CHECK(mockBackend.m_bEndFrameCalled);
	REQUIRE(mockBackend.m_receivedPackets.size() == 1u);
	CHECK(mockBackend.m_receivedPackets[0].textureHandle == 555u);
	CHECK(mockBackend.m_receivedPackets[0].vertexCount == 4u);
}

TEST_CASE("WeaponTrace - Bramka wlasnosci RenderOwnership (ERenderCategory::WeaponTrace)")
{
	RenderOwnership::ResetDefaults();

	// W Wave W5 (T228) domyslnie WeaponTrace jest w trybie Wgpu
	CHECK(RenderOwnership::Owner(ERenderCategory::WeaponTrace) == ERenderOwner::Wgpu);
	CHECK_FALSE(RenderOwnership::LegacyDraws(ERenderCategory::WeaponTrace));

	// Przelaczenie wlasnosci na Legacy
	RenderOwnership::SetOwner(ERenderCategory::WeaponTrace, ERenderOwner::Legacy);
	CHECK(RenderOwnership::Owner(ERenderCategory::WeaponTrace) == ERenderOwner::Legacy);
	CHECK(RenderOwnership::LegacyDraws(ERenderCategory::WeaponTrace));

	// Przelaczenie wlasnosci na D3D9Backend
	RenderOwnership::SetOwner(ERenderCategory::WeaponTrace, ERenderOwner::D3D9Backend);
	CHECK(RenderOwnership::Owner(ERenderCategory::WeaponTrace) == ERenderOwner::D3D9Backend);
	CHECK_FALSE(RenderOwnership::LegacyDraws(ERenderCategory::WeaponTrace));

	// Przelaczenie wlasnosci na Wgpu
	RenderOwnership::SetOwner(ERenderCategory::WeaponTrace, ERenderOwner::Wgpu);
	CHECK(RenderOwnership::Owner(ERenderCategory::WeaponTrace) == ERenderOwner::Wgpu);
	CHECK_FALSE(RenderOwnership::LegacyDraws(ERenderCategory::WeaponTrace));

	// Przywrocenie domyslnych
	RenderOwnership::ResetDefaults();
	CHECK(RenderOwnership::Owner(ERenderCategory::WeaponTrace) == ERenderOwner::Wgpu);
	CHECK_FALSE(RenderOwnership::LegacyDraws(ERenderCategory::WeaponTrace));
}
