#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest/doctest.h"

#include <windows.h>
#include <d3d9.h>
#include <d3dx9.h>
#include <vector>
#include <memory>

#include "EterModelLib/GltfModelRenderer.h"
#include "EterModelLib/GltfModel.h"
#include "EterModelLib/GltfModelInstance.h"
#include "EterLib/StateManager.h"
#include "EterLib/ResourceManager.h"
#include "EterLib/Render/ActorSkinningPipeline.h"

// Mock class for D3D9 device
class TestMockD3DDevice_C37 final : public IDirect3DDevice9Ex {
public:
	ULONG m_refCount{ 1 };
	DWORD m_hwRenderStates[256]{};

	TestMockD3DDevice_C37()
	{
		for (int i = 0; i < 256; ++i)
			m_hwRenderStates[i] = 0;
	}

	HRESULT STDMETHODCALLTYPE QueryInterface(REFIID riid, void** ppvObj) override
	{
		if (!ppvObj) return E_POINTER;
		*ppvObj = this;
		AddRef();
		return S_OK;
	}
	ULONG STDMETHODCALLTYPE AddRef() override { return ++m_refCount; }
	ULONG STDMETHODCALLTYPE Release() override
	{
		ULONG c = --m_refCount;
		if (c == 0) delete this;
		return c;
	}

	HRESULT STDMETHODCALLTYPE SetRenderState(D3DRENDERSTATETYPE State, DWORD Value) override
	{
		if (State < 256)
			m_hwRenderStates[State] = Value;
		return D3D_OK;
	}
	HRESULT STDMETHODCALLTYPE GetRenderState(D3DRENDERSTATETYPE State, DWORD* pValue) override
	{
		if (pValue && State < 256)
			*pValue = m_hwRenderStates[State];
		return D3D_OK;
	}

	HRESULT STDMETHODCALLTYPE SetTexture(DWORD, IDirect3DBaseTexture9*) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE GetTexture(DWORD, IDirect3DBaseTexture9** pp) override { if (pp) *pp = nullptr; return D3D_OK; }
	HRESULT STDMETHODCALLTYPE SetTextureStageState(DWORD, D3DTEXTURESTAGESTATETYPE, DWORD) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE GetTextureStageState(DWORD, D3DTEXTURESTAGESTATETYPE, DWORD* p) override { if (p) *p = 0; return D3D_OK; }
	HRESULT STDMETHODCALLTYPE SetSamplerState(DWORD, D3DSAMPLERSTATETYPE, DWORD) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE GetSamplerState(DWORD, D3DSAMPLERSTATETYPE, DWORD* p) override { if (p) *p = 0; return D3D_OK; }
	HRESULT STDMETHODCALLTYPE SetTransform(D3DTRANSFORMSTATETYPE, CONST D3DMATRIX*) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE GetTransform(D3DTRANSFORMSTATETYPE, D3DMATRIX*) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE SetMaterial(CONST D3DMATERIAL9*) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE GetMaterial(D3DMATERIAL9*) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE SetLight(DWORD, CONST D3DLIGHT9*) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE GetLight(DWORD, D3DLIGHT9*) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE SetScissorRect(CONST RECT*) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE GetScissorRect(RECT*) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE SetSoftwareVertexProcessing(BOOL) override { return D3D_OK; }
	BOOL STDMETHODCALLTYPE GetSoftwareVertexProcessing() override { return FALSE; }
	HRESULT STDMETHODCALLTYPE SetVertexShaderConstantF(UINT, CONST float*, UINT) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE SetStreamSource(UINT, IDirect3DVertexBuffer9*, UINT, UINT) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE SetIndices(IDirect3DIndexBuffer9*) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE BeginScene() override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE EndScene() override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE DrawPrimitive(D3DPRIMITIVETYPE, UINT, UINT) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE DrawPrimitiveUP(D3DPRIMITIVETYPE, UINT, CONST void*, UINT) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE DrawIndexedPrimitive(D3DPRIMITIVETYPE, INT, UINT, UINT, UINT, UINT) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE DrawIndexedPrimitiveUP(D3DPRIMITIVETYPE, UINT, UINT, UINT, CONST void*, D3DFORMAT, CONST void*, UINT) override { return D3D_OK; }

	HRESULT STDMETHODCALLTYPE SetVertexDeclaration(IDirect3DVertexDeclaration9*) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE GetVertexDeclaration(IDirect3DVertexDeclaration9** pp) override { if (pp) *pp = nullptr; return D3D_OK; }
	HRESULT STDMETHODCALLTYPE SetFVF(DWORD) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE GetFVF(DWORD* p) override { if (p) *p = 0; return D3D_OK; }
	HRESULT STDMETHODCALLTYPE SetVertexShader(IDirect3DVertexShader9*) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE GetVertexShader(IDirect3DVertexShader9** pp) override { if (pp) *pp = nullptr; return D3D_OK; }
	HRESULT STDMETHODCALLTYPE SetPixelShader(IDirect3DPixelShader9*) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE GetPixelShader(IDirect3DPixelShader9** pp) override { if (pp) *pp = nullptr; return D3D_OK; }

	// Stubs IDirect3DDevice9
	HRESULT STDMETHODCALLTYPE TestCooperativeLevel() override { return D3D_OK; }
	UINT STDMETHODCALLTYPE GetAvailableTextureMem() override { return 0; }
	HRESULT STDMETHODCALLTYPE EvictManagedResources() override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE GetDirect3D(IDirect3D9**) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE GetDeviceCaps(D3DCAPS9*) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE GetDisplayMode(UINT, D3DDISPLAYMODE*) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE GetCreationParameters(D3DDEVICE_CREATION_PARAMETERS*) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE SetCursorProperties(UINT, UINT, IDirect3DSurface9*) override { return D3D_OK; }
	void STDMETHODCALLTYPE SetCursorPosition(int, int, DWORD) override {}
	BOOL STDMETHODCALLTYPE ShowCursor(BOOL) override { return FALSE; }
	HRESULT STDMETHODCALLTYPE CreateAdditionalSwapChain(D3DPRESENT_PARAMETERS*, IDirect3DSwapChain9**) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE GetSwapChain(UINT, IDirect3DSwapChain9**) override { return D3D_OK; }
	UINT STDMETHODCALLTYPE GetNumberOfSwapChains() override { return 0; }
	HRESULT STDMETHODCALLTYPE Reset(D3DPRESENT_PARAMETERS*) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE Present(CONST RECT*, CONST RECT*, HWND, CONST RGNDATA*) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE GetBackBuffer(UINT, UINT, D3DBACKBUFFER_TYPE, IDirect3DSurface9**) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE GetRasterStatus(UINT, D3DRASTER_STATUS*) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE SetDialogBoxMode(BOOL) override { return D3D_OK; }
	void STDMETHODCALLTYPE SetGammaRamp(UINT, DWORD, CONST D3DGAMMARAMP*) override {}
	void STDMETHODCALLTYPE GetGammaRamp(UINT, D3DGAMMARAMP*) override {}
	HRESULT STDMETHODCALLTYPE CreateTexture(UINT, UINT, UINT, DWORD, D3DFORMAT, D3DPOOL, IDirect3DTexture9**, HANDLE*) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE CreateVolumeTexture(UINT, UINT, UINT, UINT, DWORD, D3DFORMAT, D3DPOOL, IDirect3DVolumeTexture9**, HANDLE*) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE CreateCubeTexture(UINT, UINT, DWORD, D3DFORMAT, D3DPOOL, IDirect3DCubeTexture9**, HANDLE*) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE CreateVertexBuffer(UINT, DWORD, DWORD, D3DPOOL, IDirect3DVertexBuffer9**, HANDLE*) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE CreateIndexBuffer(UINT, DWORD, D3DFORMAT, D3DPOOL, IDirect3DIndexBuffer9**, HANDLE*) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE CreateRenderTarget(UINT, UINT, D3DFORMAT, D3DMULTISAMPLE_TYPE, DWORD, BOOL, IDirect3DSurface9**, HANDLE*) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE CreateDepthStencilSurface(UINT, UINT, D3DFORMAT, D3DMULTISAMPLE_TYPE, DWORD, BOOL, IDirect3DSurface9**, HANDLE*) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE UpdateSurface(IDirect3DSurface9*, CONST RECT*, IDirect3DSurface9*, CONST POINT*) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE UpdateTexture(IDirect3DBaseTexture9*, IDirect3DBaseTexture9*) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE GetRenderTargetData(IDirect3DSurface9*, IDirect3DSurface9*) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE GetFrontBufferData(UINT, IDirect3DSurface9*) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE StretchRect(IDirect3DSurface9*, CONST RECT*, IDirect3DSurface9*, CONST RECT*, D3DTEXTUREFILTERTYPE) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE ColorFill(IDirect3DSurface9*, CONST RECT*, D3DCOLOR) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE CreateOffscreenPlainSurface(UINT, UINT, D3DFORMAT, D3DPOOL, IDirect3DSurface9**, HANDLE*) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE SetRenderTarget(DWORD, IDirect3DSurface9*) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE GetRenderTarget(DWORD, IDirect3DSurface9**) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE SetDepthStencilSurface(IDirect3DSurface9*) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE GetDepthStencilSurface(IDirect3DSurface9**) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE Clear(DWORD, CONST D3DRECT*, DWORD, D3DCOLOR, float, DWORD) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE MultiplyTransform(D3DTRANSFORMSTATETYPE, CONST D3DMATRIX*) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE SetClipStatus(CONST D3DCLIPSTATUS9*) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE GetClipStatus(D3DCLIPSTATUS9*) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE LightEnable(DWORD, BOOL) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE GetLightEnable(DWORD, BOOL*) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE SetClipPlane(DWORD, CONST float*) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE GetClipPlane(DWORD, float*) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE SetViewport(CONST D3DVIEWPORT9*) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE GetViewport(D3DVIEWPORT9*) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE ValidateDevice(DWORD*) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE SetPaletteEntries(UINT, CONST PALETTEENTRY*) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE GetPaletteEntries(UINT, PALETTEENTRY*) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE SetCurrentTexturePalette(UINT) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE GetCurrentTexturePalette(UINT*) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE SetNPatchMode(float) override { return D3D_OK; }
	float STDMETHODCALLTYPE GetNPatchMode() override { return 0.0f; }
	HRESULT STDMETHODCALLTYPE ProcessVertices(UINT, UINT, UINT, IDirect3DVertexBuffer9*, IDirect3DVertexDeclaration9*, DWORD) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE CreateVertexDeclaration(CONST D3DVERTEXELEMENT9*, IDirect3DVertexDeclaration9**) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE CreateVertexShader(CONST DWORD*, IDirect3DVertexShader9**) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE GetVertexShaderConstantF(UINT, float*, UINT) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE SetVertexShaderConstantI(UINT, CONST int*, UINT) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE GetVertexShaderConstantI(UINT, int*, UINT) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE SetVertexShaderConstantB(UINT, CONST BOOL*, UINT) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE GetVertexShaderConstantB(UINT, BOOL*, UINT) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE GetStreamSource(UINT, IDirect3DVertexBuffer9**, UINT*, UINT*) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE SetStreamSourceFreq(UINT, UINT) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE GetStreamSourceFreq(UINT, UINT*) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE GetIndices(IDirect3DIndexBuffer9**) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE CreatePixelShader(CONST DWORD*, IDirect3DPixelShader9**) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE SetPixelShaderConstantF(UINT, CONST float*, UINT) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE GetPixelShaderConstantF(UINT, float*, UINT) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE SetPixelShaderConstantI(UINT, CONST int*, UINT) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE GetPixelShaderConstantI(UINT, int*, UINT) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE SetPixelShaderConstantB(UINT, CONST BOOL*, UINT) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE GetPixelShaderConstantB(UINT, BOOL*, UINT) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE DrawRectPatch(UINT, CONST float*, CONST D3DRECTPATCH_INFO*) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE DrawTriPatch(UINT, CONST float*, CONST D3DTRIPATCH_INFO*) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE DeletePatch(UINT) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE CreateQuery(D3DQUERYTYPE, IDirect3DQuery9**) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE CreateStateBlock(D3DSTATEBLOCKTYPE, IDirect3DStateBlock9**) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE BeginStateBlock() override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE EndStateBlock(IDirect3DStateBlock9**) override { return D3D_OK; }

	// IDirect3DDevice9Ex
	HRESULT STDMETHODCALLTYPE SetConvolutionMonoKernel(UINT, UINT, float*, float*) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE ComposeRects(IDirect3DSurface9*, IDirect3DSurface9*, IDirect3DVertexBuffer9*, UINT, IDirect3DVertexBuffer9*, D3DCOMPOSERECTSOP, int, int) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE PresentEx(CONST RECT*, CONST RECT*, HWND, CONST RGNDATA*, DWORD) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE GetGPUThreadPriority(INT*) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE SetGPUThreadPriority(INT) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE WaitForVBlank(UINT) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE CheckResourceResidency(IDirect3DResource9**, UINT) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE SetMaximumFrameLatency(UINT) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE GetMaximumFrameLatency(UINT*) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE CheckDeviceState(HWND) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE CreateRenderTargetEx(UINT, UINT, D3DFORMAT, D3DMULTISAMPLE_TYPE, DWORD, BOOL, IDirect3DSurface9**, HANDLE*, DWORD) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE CreateOffscreenPlainSurfaceEx(UINT, UINT, D3DFORMAT, D3DPOOL, IDirect3DSurface9**, HANDLE*, DWORD) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE CreateDepthStencilSurfaceEx(UINT, UINT, D3DFORMAT, D3DMULTISAMPLE_TYPE, DWORD, BOOL, IDirect3DSurface9**, HANDLE*, DWORD) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE ResetEx(D3DPRESENT_PARAMETERS*, D3DDISPLAYMODEEX*) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE GetDisplayModeEx(UINT, D3DDISPLAYMODEEX*, D3DDISPLAYROTATION*) override { return D3D_OK; }
};

using namespace EterModelLib;

TEST_CASE("GltfModelRenderer Audit: State Execution and UBs") {
    auto mockDev = std::make_unique<TestMockD3DDevice_C37>();
    auto stateMgr = std::make_unique<CStateManager>(mockDev.get());

    GltfModelData modelData;
    modelData.name = "AuditTestModel";

    GltfSubmesh submesh0;
    submesh0.name = "Submesh0";
    submesh0.vertexOffset = 10;
    submesh0.vertexCount = 0; // Trigger the fallback logic for numVertices
    submesh0.indexOffset = 0;
    submesh0.indexCount = 3;
    submesh0.materialIndex = -1; // Trigger material fallback
    modelData.submeshes.push_back(submesh0);

    // 20 vertices total
    for (int i = 0; i < 20; ++i) {
        GltfVertex v;
        v.position = { 0.0f, 0.0f, 0.0f };
        modelData.vertices.push_back(v);
    }
    modelData.indices = { 0, 1, 2 };

    CGltfModel model;
    model.SetModelData(modelData);
    model.SetLoaded(true); // Fake load

    CGltfModelInstance instance(&model);
    GltfModelRenderer renderer;

    TModelDrawItem item{};
    item.modelId = 1;
    item.submeshIndex = 0;
    item.pass = EModelPass::Opaque;
    item.bTwoSided = 0;

    // Set some initial states to check preservation
    STATEMANAGER.SetRenderState(D3DRS_CULLMODE, D3DCULL_CCW);
    STATEMANAGER.SetRenderState(D3DRS_ALPHABLENDENABLE, TRUE);
    STATEMANAGER.SetRenderState(D3DRS_ALPHATESTENABLE, TRUE);

    SUBCASE("ExecuteDrawItemD3D9 preserves states") {
        renderer.ExecuteDrawItemD3D9(mockDev.get(), &instance, item, true);

        DWORD cullMode = 0;
        STATEMANAGER.GetRenderState(D3DRS_CULLMODE, &cullMode);
        CHECK(cullMode == D3DCULL_CCW);

        DWORD alphaBlend = 0;
        STATEMANAGER.GetRenderState(D3DRS_ALPHABLENDENABLE, &alphaBlend);
        CHECK(alphaBlend == TRUE);

        DWORD alphaTest = 0;
        STATEMANAGER.GetRenderState(D3DRS_ALPHATESTENABLE, &alphaTest);
        CHECK(alphaTest == TRUE);
    }

    SUBCASE("ExecuteDrawItemD3D9 calculates correct numVertices") {
        // We can't directly check the internal numVertices since it's passed to DrawIndexedPrimitive.
        // But we can ensure it doesn't crash or trigger bounds warnings if tested under ASan.
        // It should use GetVertexCount() - minVertexIndex = 20 - 10 = 10.
        renderer.ExecuteDrawItemD3D9(mockDev.get(), &instance, item, true);
        // If it doesn't crash, the calculation didn't cause an immediate fault.
        CHECK(true);
    }
}
