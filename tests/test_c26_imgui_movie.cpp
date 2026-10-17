#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"

#include <d3d9.h>
#include <vector>
#include <cstring>
#include "EterLib/RenderOwnership.h"
#include "EterLib/D3D9StateBlockScope.h"

namespace
{
	// Mock implementacji IDirect3DStateBlock9 dla weryfikacji sekwencji Save/Restore
	class MockD3D9StateBlock : public IDirect3DStateBlock9
	{
	public:
		ULONG refCount{ 1 };
		int captureCalls{ 0 };
		int applyCalls{ 0 };
		bool failCapture{ false };
		bool failApply{ false };

		HRESULT STDMETHODCALLTYPE QueryInterface(REFIID riid, void** ppvObj) override
		{
			if (!ppvObj)
				return E_POINTER;
			*ppvObj = nullptr;
			return E_NOINTERFACE;
		}

		ULONG STDMETHODCALLTYPE AddRef() override
		{
			return ++refCount;
		}

		ULONG STDMETHODCALLTYPE Release() override
		{
			if (refCount > 0)
			{
				--refCount;
				if (refCount == 0)
				{
					// Obiekt zarzadzany
					return 0;
				}
			}
			return refCount;
		}

		HRESULT STDMETHODCALLTYPE GetDevice(IDirect3DDevice9** ppDevice) override
		{
			if (!ppDevice)
				return E_POINTER;
			*ppDevice = nullptr;
			return D3D_OK;
		}

		HRESULT STDMETHODCALLTYPE Capture() override
		{
			++captureCalls;
			return failCapture ? E_FAIL : D3D_OK;
		}

		HRESULT STDMETHODCALLTYPE Apply() override
		{
			++applyCalls;
			return failApply ? E_FAIL : D3D_OK;
		}
	};

	// Mock urzadzenia IDirect3DDevice9 dla testow zarzadzania stanem
	class MockD3D9Device : public IDirect3DDevice9
	{
	public:
		ULONG refCount{ 1 };
		int createStateBlockCalls{ 0 };
		bool failCreateStateBlock{ false };
		MockD3D9StateBlock* lastCreatedStateBlock{ nullptr };
		MockD3D9StateBlock instanceStateBlock{};

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

		HRESULT STDMETHODCALLTYPE CreateStateBlock(D3DSTATEBLOCKTYPE Type, IDirect3DStateBlock9** ppSB) override
		{
			++createStateBlockCalls;
			if (!ppSB || failCreateStateBlock)
				return E_FAIL;

			instanceStateBlock.refCount = 1;
			*ppSB = &instanceStateBlock;
			lastCreatedStateBlock = &instanceStateBlock;
			return D3D_OK;
		}

		// Pozostale metody interfejsu IDirect3DDevice9 jako bezpieczne no-op dla celow testowych
		HRESULT STDMETHODCALLTYPE TestCooperativeLevel() override { return D3D_OK; }
		UINT STDMETHODCALLTYPE GetAvailableTextureMem() override { return 1024 * 1024 * 128; }
		HRESULT STDMETHODCALLTYPE EvictManagedResources() override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE GetDirect3D(IDirect3D9** ppD3D9) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE GetDeviceCaps(D3DCAPS9* pCaps) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE GetDisplayMode(UINT iSwapChain, D3DDISPLAYMODE* pMode) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE GetCreationParameters(D3DDEVICE_CREATION_PARAMETERS* pParameters) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE SetCursorProperties(UINT XHotSpot, UINT YHotSpot, IDirect3DSurface9* pCursorBitmap) override { return D3D_OK; }
		void STDMETHODCALLTYPE SetCursorPosition(int X, int Y, DWORD Flags) override {}
		BOOL STDMETHODCALLTYPE ShowCursor(BOOL bShow) override { return bShow; }
		HRESULT STDMETHODCALLTYPE CreateAdditionalSwapChain(D3DPRESENT_PARAMETERS* pPresentationParameters, IDirect3DSwapChain9** pSwapChain) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE GetSwapChain(UINT iSwapChain, IDirect3DSwapChain9** pSwapChain) override { return D3D_OK; }
		UINT STDMETHODCALLTYPE GetNumberOfSwapChains() override { return 1; }
		HRESULT STDMETHODCALLTYPE Reset(D3DPRESENT_PARAMETERS* pPresentationParameters) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE Present(CONST RECT* pSourceRect, CONST RECT* pDestRect, HWND hDestWindowOverride, CONST RGNDATA* pDirtyRegion) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE GetBackBuffer(UINT iSwapChain, UINT iBackBuffer, D3DBACKBUFFER_TYPE Type, IDirect3DSurface9** ppBackBuffer) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE GetRasterStatus(UINT iSwapChain, D3DRASTER_STATUS* pRasterStatus) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE SetDialogBoxMode(BOOL bEnableDialogs) override { return D3D_OK; }
		void STDMETHODCALLTYPE SetGammaRamp(UINT iSwapChain, DWORD Flags, CONST D3DGAMMARAMP* pRamp) override {}
		void STDMETHODCALLTYPE GetGammaRamp(UINT iSwapChain, D3DGAMMARAMP* pRamp) override {}
		HRESULT STDMETHODCALLTYPE CreateRenderTarget(UINT Width, UINT Height, D3DFORMAT Format, D3DMULTISAMPLE_TYPE MultiSample, DWORD MultisampleQuality, BOOL Lockable, IDirect3DSurface9** ppSurface, HANDLE* pSharedHandle) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE CreateDepthStencilSurface(UINT Width, UINT Height, D3DFORMAT Format, D3DMULTISAMPLE_TYPE MultiSample, DWORD MultisampleQuality, BOOL Discard, IDirect3DSurface9** ppSurface, HANDLE* pSharedHandle) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE UpdateSurface(IDirect3DSurface9* pSourceSurface, CONST RECT* pSourceRect, IDirect3DSurface9* pDestinationSurface, CONST POINT* pDestPoint) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE UpdateTexture(IDirect3DBaseTexture9* pSourceTexture, IDirect3DBaseTexture9* pDestinationTexture) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE GetRenderTargetData(IDirect3DSurface9* pRenderTarget, IDirect3DSurface9* pDestSurface) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE GetFrontBufferData(UINT iSwapChain, IDirect3DSurface9* pDestSurface) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE StretchRect(IDirect3DSurface9* pSourceSurface, CONST RECT* pSourceRect, IDirect3DSurface9* pDestSurface, CONST RECT* pDestRect, D3DTEXTUREFILTERTYPE Filter) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE ColorFill(IDirect3DSurface9* pSurface, CONST RECT* pRect, D3DCOLOR color) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE CreateOffscreenPlainSurface(UINT Width, UINT Height, D3DFORMAT Format, D3DPOOL Pool, IDirect3DSurface9** ppSurface, HANDLE* pSharedHandle) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE SetRenderTarget(DWORD RenderTargetIndex, IDirect3DSurface9* pRenderTarget) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE GetRenderTarget(DWORD RenderTargetIndex, IDirect3DSurface9** ppRenderTarget) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE SetDepthStencilSurface(IDirect3DSurface9* pNewZStencil) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE GetDepthStencilSurface(IDirect3DSurface9** ppZStencilSurface) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE BeginScene() override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE EndScene() override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE Clear(DWORD Count, CONST D3DRECT* pRects, DWORD Flags, D3DCOLOR Color, float Z, DWORD Stencil) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE SetTransform(D3DTRANSFORMSTATETYPE State, CONST D3DMATRIX* pMatrix) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE GetTransform(D3DTRANSFORMSTATETYPE State, D3DMATRIX* pMatrix) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE MultiplyTransform(D3DTRANSFORMSTATETYPE State, CONST D3DMATRIX* pMatrix) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE SetViewport(CONST D3DVIEWPORT9* pViewport) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE GetViewport(D3DVIEWPORT9* pViewport) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE SetMaterial(CONST D3DMATERIAL9* pMaterial) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE GetMaterial(D3DMATERIAL9* pMaterial) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE SetLight(DWORD Index, CONST D3DLIGHT9* pLight) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE GetLight(DWORD Index, D3DLIGHT9* pLight) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE LightEnable(DWORD Index, BOOL Enable) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE GetLightEnable(DWORD Index, BOOL* pEnable) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE SetClipPlane(DWORD Index, CONST float* pPlane) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE GetClipPlane(DWORD Index, float* pPlane) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE SetRenderState(D3DRENDERSTATETYPE State, DWORD Value) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE GetRenderState(D3DRENDERSTATETYPE State, DWORD* pValue) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE BeginStateBlock() override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE EndStateBlock(IDirect3DStateBlock9** ppSB) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE SetClipStatus(CONST D3DCLIPSTATUS9* pClipStatus) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE GetClipStatus(D3DCLIPSTATUS9* pClipStatus) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE GetTexture(DWORD Stage, IDirect3DBaseTexture9** ppTexture) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE SetTexture(DWORD Stage, IDirect3DBaseTexture9* pTexture) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE GetTextureStageState(DWORD Stage, D3DTEXTURESTAGESTATETYPE Type, DWORD* pValue) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE SetTextureStageState(DWORD Stage, D3DTEXTURESTAGESTATETYPE Type, DWORD Value) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE GetSamplerState(DWORD Sampler, D3DSAMPLERSTATETYPE Type, DWORD* pValue) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE SetSamplerState(DWORD Sampler, D3DSAMPLERSTATETYPE Type, DWORD Value) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE ValidateDevice(DWORD* pNumPasses) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE SetPaletteEntries(UINT PaletteNumber, CONST PALETTEENTRY* pEntries) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE GetPaletteEntries(UINT PaletteNumber, PALETTEENTRY* pEntries) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE SetCurrentTexturePalette(UINT PaletteNumber) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE GetCurrentTexturePalette(UINT* PaletteNumber) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE SetScissorRect(CONST RECT* pRect) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE GetScissorRect(RECT* pRect) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE SetSoftwareVertexProcessing(BOOL bSoftware) override { return D3D_OK; }
		BOOL STDMETHODCALLTYPE GetSoftwareVertexProcessing() override { return FALSE; }
		HRESULT STDMETHODCALLTYPE SetNPatchMode(float nSegments) override { return D3D_OK; }
		float STDMETHODCALLTYPE GetNPatchMode() override { return 0.0f; }
		HRESULT STDMETHODCALLTYPE DrawPrimitive(D3DPRIMITIVETYPE PrimitiveType, UINT StartVertex, UINT PrimitiveCount) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE DrawIndexedPrimitive(D3DPRIMITIVETYPE PrimitiveType, INT BaseVertexIndex, UINT MinVertexIndex, UINT NumVertices, UINT startIndex, UINT primCount) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE DrawPrimitiveUP(D3DPRIMITIVETYPE PrimitiveType, UINT PrimitiveCount, CONST void* pVertexStreamZeroData, UINT VertexStreamZeroStride) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE DrawIndexedPrimitiveUP(D3DPRIMITIVETYPE PrimitiveType, UINT MinVertexIndex, UINT NumVertices, UINT PrimitiveCount, CONST void* pIndexData, D3DFORMAT IndexDataFormat, CONST void* pVertexStreamZeroData, UINT VertexStreamZeroStride) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE ProcessVertices(UINT SrcStartIndex, UINT DestIndex, UINT VertexCount, IDirect3DVertexBuffer9* pDestBuffer, IDirect3DVertexDeclaration9* pVertexDeclaration, DWORD Flags) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE CreateVertexDeclaration(CONST D3DVERTEXELEMENT9* pVertexElements, IDirect3DVertexDeclaration9** ppDecl) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE SetVertexDeclaration(IDirect3DVertexDeclaration9* pDecl) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE GetVertexDeclaration(IDirect3DVertexDeclaration9** ppDecl) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE SetFVF(DWORD FVF) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE GetFVF(DWORD* pFVF) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE CreateVertexShader(CONST DWORD* pFunction, IDirect3DVertexShader9** ppShader) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE SetVertexShader(IDirect3DVertexShader9* pShader) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE GetVertexShader(IDirect3DVertexShader9** ppShader) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE SetVertexShaderConstantF(UINT StartRegister, CONST float* pConstantData, UINT Vector4fCount) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE GetVertexShaderConstantF(UINT StartRegister, float* pConstantData, UINT Vector4fCount) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE SetVertexShaderConstantI(UINT StartRegister, CONST int* pConstantData, UINT Vector4iCount) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE GetVertexShaderConstantI(UINT StartRegister, int* pConstantData, UINT Vector4iCount) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE SetVertexShaderConstantB(UINT StartRegister, CONST BOOL* pConstantData, UINT BoolCount) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE GetVertexShaderConstantB(UINT StartRegister, BOOL* pConstantData, UINT BoolCount) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE SetStreamSource(UINT StreamNumber, IDirect3DVertexBuffer9* pStreamData, UINT OffsetInBytes, UINT Stride) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE GetStreamSource(UINT StreamNumber, IDirect3DVertexBuffer9** ppStreamData, UINT* pOffsetInBytes, UINT* pStride) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE SetStreamSourceFreq(UINT StreamNumber, UINT Setting) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE GetStreamSourceFreq(UINT StreamNumber, UINT* pSetting) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE SetIndices(IDirect3DIndexBuffer9* pIndexData) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE GetIndices(IDirect3DIndexBuffer9** ppIndexData) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE CreatePixelShader(CONST DWORD* pFunction, IDirect3DPixelShader9** ppShader) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE SetPixelShader(IDirect3DPixelShader9* pShader) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE GetPixelShader(IDirect3DPixelShader9** ppShader) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE SetPixelShaderConstantF(UINT StartRegister, CONST float* pConstantData, UINT Vector4fCount) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE GetPixelShaderConstantF(UINT StartRegister, float* pConstantData, UINT Vector4fCount) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE SetPixelShaderConstantI(UINT StartRegister, CONST int* pConstantData, UINT Vector4iCount) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE GetPixelShaderConstantI(UINT StartRegister, int* pConstantData, UINT Vector4iCount) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE SetPixelShaderConstantB(UINT StartRegister, CONST BOOL* pConstantData, UINT BoolCount) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE GetPixelShaderConstantB(UINT StartRegister, BOOL* pConstantData, UINT BoolCount) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE DrawRectPatch(UINT Handle, CONST float* pNumSegs, CONST D3DRECTPATCH_INFO* pRectPatchInfo) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE DrawTriPatch(UINT Handle, CONST float* pNumSegs, CONST D3DTRIPATCH_INFO* pTriPatchInfo) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE DeletePatch(UINT Handle) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE CreateQuery(D3DQUERYTYPE Type, IDirect3DQuery9** ppQuery) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE CreateVertexBuffer(UINT Length, DWORD Usage, DWORD FVF, D3DPOOL Pool, IDirect3DVertexBuffer9** ppVertexBuffer, HANDLE* pSharedHandle) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE CreateIndexBuffer(UINT Length, DWORD Usage, D3DFORMAT Format, D3DPOOL Pool, IDirect3DIndexBuffer9** ppIndexBuffer, HANDLE* pSharedHandle) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE CreateTexture(UINT Width, UINT Height, UINT Levels, DWORD Usage, D3DFORMAT Format, D3DPOOL Pool, IDirect3DTexture9** ppTexture, HANDLE* pSharedHandle) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE CreateVolumeTexture(UINT Width, UINT Height, UINT Depth, UINT Levels, DWORD Usage, D3DFORMAT Format, D3DPOOL Pool, IDirect3DVolumeTexture9** ppVolumeTexture, HANDLE* pSharedHandle) override { return D3D_OK; }
		HRESULT STDMETHODCALLTYPE CreateCubeTexture(UINT EdgeLength, UINT Levels, DWORD Usage, D3DFORMAT Format, D3DPOOL Pool, IDirect3DCubeTexture9** ppCubeTexture, HANDLE* pSharedHandle) override { return D3D_OK; }
	};

	// Logika bramkowania bezposredniego renderowania ImGui
	bool CheckCanDrawImGuiDirectly() noexcept
	{
		return RenderOwnership::LegacyDraws(ERenderCategory::UI) ||
		       RenderOwnership::LegacyDraws(ERenderCategory::ImGui);
	}

	// Logika bramkowania odtwarzania wideo (DEC-10)
	bool CheckCanPlayMovieDirectly() noexcept
	{
		if (RenderOwnership::IsWgpuConfigured() || RenderOwnership::Owner(ERenderCategory::Movie) == ERenderOwner::Wgpu)
			return false;
		return RenderOwnership::LegacyDraws(ERenderCategory::Movie);
	}
}

TEST_CASE("RenderOwnership - Domyslne wartosci wlasnosci dla ImGui i Movie")
{
	RenderOwnership::ResetDefaults();

	// Kategoria ImGui domyslnie w trybie Legacy
	CHECK(RenderOwnership::Owner(ERenderCategory::ImGui) == ERenderOwner::Legacy);
	CHECK(RenderOwnership::LegacyDraws(ERenderCategory::ImGui) == true);

	// Kategoria Movie domyslnie w trybie Legacy
	CHECK(RenderOwnership::Owner(ERenderCategory::Movie) == ERenderOwner::Legacy);
	CHECK(RenderOwnership::LegacyDraws(ERenderCategory::Movie) == true);

	// Kategoria UI domyslnie pod zarzadem D3D9Backend
	CHECK(RenderOwnership::Owner(ERenderCategory::UI) == ERenderOwner::D3D9Backend);
	CHECK(RenderOwnership::LegacyDraws(ERenderCategory::UI) == false);
}

TEST_CASE("RenderOwnership - Przelaczanie wlasnosci dla kategorii ImGui")
{
	RenderOwnership::ResetDefaults();

	// 1. Zmiana wlasnosci ImGui na Wgpu
	RenderOwnership::SetOwner(ERenderCategory::ImGui, ERenderOwner::Wgpu);
	CHECK(RenderOwnership::Owner(ERenderCategory::ImGui) == ERenderOwner::Wgpu);
	CHECK_FALSE(RenderOwnership::LegacyDraws(ERenderCategory::ImGui));

	// 2. Zmiana wlasnosci ImGui na D3D9Backend
	RenderOwnership::SetOwner(ERenderCategory::ImGui, ERenderOwner::D3D9Backend);
	CHECK(RenderOwnership::Owner(ERenderCategory::ImGui) == ERenderOwner::D3D9Backend);
	CHECK_FALSE(RenderOwnership::LegacyDraws(ERenderCategory::ImGui));

	// 3. Przywrocenie wlasnosci Legacy
	RenderOwnership::SetOwner(ERenderCategory::ImGui, ERenderOwner::Legacy);
	CHECK(RenderOwnership::Owner(ERenderCategory::ImGui) == ERenderOwner::Legacy);
	CHECK(RenderOwnership::LegacyDraws(ERenderCategory::ImGui) == true);

	// 4. Reset do domyslnych
	RenderOwnership::ResetDefaults();
	CHECK(RenderOwnership::Owner(ERenderCategory::ImGui) == ERenderOwner::Legacy);
	CHECK(RenderOwnership::LegacyDraws(ERenderCategory::ImGui) == true);
}

TEST_CASE("RenderOwnership - Przelaczanie wlasnosci dla kategorii Movie")
{
	RenderOwnership::ResetDefaults();

	// 1. Zmiana wlasnosci Movie na Wgpu
	RenderOwnership::SetOwner(ERenderCategory::Movie, ERenderOwner::Wgpu);
	CHECK(RenderOwnership::Owner(ERenderCategory::Movie) == ERenderOwner::Wgpu);
	CHECK_FALSE(RenderOwnership::LegacyDraws(ERenderCategory::Movie));

	// 2. Zmiana wlasnosci Movie na D3D9Backend
	RenderOwnership::SetOwner(ERenderCategory::Movie, ERenderOwner::D3D9Backend);
	CHECK(RenderOwnership::Owner(ERenderCategory::Movie) == ERenderOwner::D3D9Backend);
	CHECK_FALSE(RenderOwnership::LegacyDraws(ERenderCategory::Movie));

	// 3. Przywrocenie wlasnosci Legacy
	RenderOwnership::SetOwner(ERenderCategory::Movie, ERenderOwner::Legacy);
	CHECK(RenderOwnership::Owner(ERenderCategory::Movie) == ERenderOwner::Legacy);
	CHECK(RenderOwnership::LegacyDraws(ERenderCategory::Movie) == true);

	// 4. Reset do domyslnych
	RenderOwnership::ResetDefaults();
	CHECK(RenderOwnership::Owner(ERenderCategory::Movie) == ERenderOwner::Legacy);
	CHECK(RenderOwnership::LegacyDraws(ERenderCategory::Movie) == true);
}

TEST_CASE("Bramkowanie ImGui - Kontrola renderowania bezposredniego (E24)")
{
	RenderOwnership::ResetDefaults();

	// Domyslnie: ImGui = Legacy, UI = D3D9Backend -> dozwolone renderowanie bezposrednie w D3D9
	CHECK(CheckCanDrawImGuiDirectly() == true);

	// Przelaczenie UI na Legacy przy ImGui = Legacy
	RenderOwnership::SetOwner(ERenderCategory::UI, ERenderOwner::Legacy);
	CHECK(CheckCanDrawImGuiDirectly() == true);

	// Przelaczenie ImGui i UI na Wgpu (tryb pelnego WGPU backendu)
	RenderOwnership::SetOwner(ERenderCategory::ImGui, ERenderOwner::Wgpu);
	RenderOwnership::SetOwner(ERenderCategory::UI, ERenderOwner::Wgpu);
	// Bezposredni render ImGui w D3D9 musi byc zablokowany, rozwiazujac problem E24
	CHECK(CheckCanDrawImGuiDirectly() == false);

	// Przywrocenie domyslnych
	RenderOwnership::ResetDefaults();
	CHECK(CheckCanDrawImGuiDirectly() == true);
}

TEST_CASE("Bramkowanie Movie - Zasada DEC-10 i blokada w trybie WGPU")
{
	RenderOwnership::ResetDefaults();

	// Domyslnie pod D3D9 odtwarzanie filmu jest dopuszczalne
	CHECK(CheckCanPlayMovieDirectly() == true);

	// W trybie Wgpu kategoria Movie jest blokowana natychmiast (DEC-10)
	RenderOwnership::SetOwner(ERenderCategory::Movie, ERenderOwner::Wgpu);
	CHECK(CheckCanPlayMovieDirectly() == false);

	// W trybie D3D9Backend odtwarzanie legacy DirectDraw jest wylaczone
	RenderOwnership::SetOwner(ERenderCategory::Movie, ERenderOwner::D3D9Backend);
	CHECK(CheckCanPlayMovieDirectly() == false);

	// Przywrocenie domyslnych
	RenderOwnership::ResetDefaults();
	CHECK(CheckCanPlayMovieDirectly() == true);
}

TEST_CASE("Direct3D State Management - D3D9StateBlockScope Save i Restore")
{
	MockD3D9Device mockDevice;

	{
		// Sekwencja Save: konstruktor scope przechwytuje stan przez CreateStateBlock + Capture
		EterLib::Render::D3D9StateBlockScope stateScope(&mockDevice);

		CHECK(stateScope.HasCaptured() == true);
		CHECK(mockDevice.createStateBlockCalls == 1);
		REQUIRE(mockDevice.lastCreatedStateBlock != nullptr);
		CHECK(mockDevice.lastCreatedStateBlock->captureCalls == 1);
		CHECK(mockDevice.lastCreatedStateBlock->applyCalls == 0);

		// Jawne wywolanie Restore przed wyjsciem z zakresu
		stateScope.Restore();
		CHECK(mockDevice.lastCreatedStateBlock->applyCalls == 1);
		CHECK(mockDevice.lastCreatedStateBlock->refCount == 0);
		CHECK_FALSE(stateScope.HasCaptured());

		// Ponowne Restore nie powinno wykonac podwojnego Apply
		stateScope.Restore();
		CHECK(mockDevice.lastCreatedStateBlock->applyCalls == 1);
	}

	// Weryfikacja automatycznego Restore przez destruktor (RAII)
	{
		mockDevice.createStateBlockCalls = 0;
		mockDevice.instanceStateBlock.captureCalls = 0;
		mockDevice.instanceStateBlock.applyCalls = 0;

		{
			EterLib::Render::D3D9StateBlockScope autoScope(&mockDevice);
			CHECK(autoScope.HasCaptured() == true);
			CHECK(mockDevice.instanceStateBlock.captureCalls == 1);
			CHECK(mockDevice.instanceStateBlock.applyCalls == 0);
		} // autoScope niszczony tutaj

		CHECK(mockDevice.instanceStateBlock.applyCalls == 1);
		CHECK(mockDevice.instanceStateBlock.refCount == 0);
	}
}

TEST_CASE("Direct3D State Management - Odpornosc na bledy urzadzenia D3D9")
{
	// 1. Obsluga nullptr device
	{
		EterLib::Render::D3D9StateBlockScope nullScope(nullptr);
		CHECK_FALSE(nullScope.HasCaptured());
		nullScope.Restore(); // Brak crasha
	}

	// 2. Blad CreateStateBlock
	{
		MockD3D9Device failingDevice;
		failingDevice.failCreateStateBlock = true;

		EterLib::Render::D3D9StateBlockScope failScope(&failingDevice);
		CHECK_FALSE(failScope.HasCaptured());
		failScope.Restore(); // Brak crasha
	}

	// 3. Blad Capture
	{
		MockD3D9Device failingCaptureDevice;
		failingCaptureDevice.instanceStateBlock.failCapture = true;

		EterLib::Render::D3D9StateBlockScope failCaptureScope(&failingCaptureDevice);
		CHECK_FALSE(failCaptureScope.HasCaptured());
		// Przy bledzie Capture obiekt powinien zostac zwolniony natychmiast
		CHECK(failingCaptureDevice.instanceStateBlock.refCount == 0);
	}
}
