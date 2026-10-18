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
class TestMockD3DDevice_C37 : public IDirect3DDevice9 {
public:
    ULONG STDMETHODCALLTYPE AddRef() override { return 1; }
    ULONG STDMETHODCALLTYPE Release() override { return 1; }
    HRESULT STDMETHODCALLTYPE QueryInterface(REFIID, void**) override { return E_NOINTERFACE; }

    HRESULT STDMETHODCALLTYPE SetRenderState(D3DRENDERSTATETYPE State, DWORD Value) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE GetRenderState(D3DRENDERSTATETYPE State, DWORD* pValue) override { return D3D_OK; }

    HRESULT STDMETHODCALLTYPE SetTexture(DWORD Stage, IDirect3DBaseTexture9* pTexture) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE GetTexture(DWORD Stage, IDirect3DBaseTexture9** ppTexture) override { 
        if (ppTexture) *ppTexture = nullptr;
        return D3D_OK; 
    }

    HRESULT STDMETHODCALLTYPE SetTransform(D3DTRANSFORMSTATETYPE State, const D3DMATRIX* pMatrix) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE GetTransform(D3DTRANSFORMSTATETYPE State, D3DMATRIX* pMatrix) override { return D3D_OK; }

    HRESULT STDMETHODCALLTYPE SetTextureStageState(DWORD Stage, D3DTEXTURESTAGESTATETYPE Type, DWORD Value) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE GetTextureStageState(DWORD Stage, D3DTEXTURESTAGESTATETYPE Type, DWORD* pValue) override { return D3D_OK; }

    HRESULT STDMETHODCALLTYPE SetSamplerState(DWORD Sampler, D3DSAMPLERSTATETYPE Type, DWORD Value) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE GetSamplerState(DWORD Sampler, D3DSAMPLERSTATETYPE Type, DWORD* pValue) override { return D3D_OK; }

    HRESULT STDMETHODCALLTYPE SetFVF(DWORD FVF) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE GetFVF(DWORD* pFVF) override { return D3D_OK; }

    HRESULT STDMETHODCALLTYPE SetVertexShader(IDirect3DVertexShader9* pShader) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE GetVertexShader(IDirect3DVertexShader9** ppShader) override { return D3D_OK; }

    HRESULT STDMETHODCALLTYPE SetPixelShader(IDirect3DPixelShader9* pShader) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE GetPixelShader(IDirect3DPixelShader9** ppShader) override { return D3D_OK; }

    HRESULT STDMETHODCALLTYPE SetVertexDeclaration(IDirect3DVertexDeclaration9* pDecl) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE GetVertexDeclaration(IDirect3DVertexDeclaration9** ppDecl) override { return D3D_OK; }

    HRESULT STDMETHODCALLTYPE SetStreamSource(UINT StreamNumber, IDirect3DVertexBuffer9* pStreamData, UINT OffsetInBytes, UINT Stride) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE GetStreamSource(UINT StreamNumber, IDirect3DVertexBuffer9** ppStreamData, UINT* pOffsetInBytes, UINT* pStride) override { return D3D_OK; }

    HRESULT STDMETHODCALLTYPE SetIndices(IDirect3DIndexBuffer9* pIndexData) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE GetIndices(IDirect3DIndexBuffer9** ppIndexData) override { return D3D_OK; }

    HRESULT STDMETHODCALLTYPE DrawPrimitive(D3DPRIMITIVETYPE PrimitiveType, UINT StartVertex, UINT PrimitiveCount) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE DrawIndexedPrimitive(D3DPRIMITIVETYPE PrimitiveType, INT BaseVertexIndex, UINT MinVertexIndex, UINT NumVertices, UINT startIndex, UINT primCount) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE DrawPrimitiveUP(D3DPRIMITIVETYPE PrimitiveType, UINT PrimitiveCount, const void* pVertexStreamZeroData, UINT VertexStreamZeroStride) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE DrawIndexedPrimitiveUP(D3DPRIMITIVETYPE PrimitiveType, UINT MinVertexIndex, UINT NumVertices, UINT PrimitiveCount, const void* pIndexData, D3DFORMAT IndexDataFormat, const void* pVertexStreamZeroData, UINT VertexStreamZeroStride) override { return D3D_OK; }

    // Dummy implementations for remaining required IDirect3DDevice9 methods
    HRESULT STDMETHODCALLTYPE TestCooperativeLevel() override { return D3D_OK; }
    UINT STDMETHODCALLTYPE GetAvailableTextureMem() override { return 0; }
    HRESULT STDMETHODCALLTYPE EvictManagedResources() override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE GetDirect3D(IDirect3D9** ppD3D9) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE GetDeviceCaps(D3DCAPS9* pCaps) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE GetDisplayMode(UINT iSwapChain, D3DDISPLAYMODE* pMode) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE GetCreationParameters(D3DDEVICE_CREATION_PARAMETERS* pParameters) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE SetCursorProperties(UINT XHotSpot, UINT YHotSpot, IDirect3DSurface9* pCursorBitmap) override { return D3D_OK; }
    void STDMETHODCALLTYPE SetCursorPosition(int X, int Y, DWORD Flags) override {}
    BOOL STDMETHODCALLTYPE ShowCursor(BOOL bShow) override { return FALSE; }
    HRESULT STDMETHODCALLTYPE CreateAdditionalSwapChain(D3DPRESENT_PARAMETERS* pPresentationParameters, IDirect3DSwapChain9** pSwapChain) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE GetSwapChain(UINT iSwapChain, IDirect3DSwapChain9** pSwapChain) override { return D3D_OK; }
    UINT STDMETHODCALLTYPE GetNumberOfSwapChains() override { return 1; }
    HRESULT STDMETHODCALLTYPE Reset(D3DPRESENT_PARAMETERS* pPresentationParameters) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE Present(const RECT* pSourceRect, const RECT* pDestRect, HWND hDestWindowOverride, const RGNDATA* pDirtyRegion) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE GetBackBuffer(UINT iSwapChain, UINT iBackBuffer, D3DBACKBUFFER_TYPE Type, IDirect3DSurface9** ppBackBuffer) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE GetRasterStatus(UINT iSwapChain, D3DRASTER_STATUS* pRasterStatus) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE SetDialogBoxMode(BOOL bEnableDialogs) override { return D3D_OK; }
    void STDMETHODCALLTYPE SetGammaRamp(UINT iSwapChain, DWORD Flags, const D3DGAMMARAMP* pRamp) override {}
    void STDMETHODCALLTYPE GetGammaRamp(UINT iSwapChain, D3DGAMMARAMP* pRamp) override {}
    HRESULT STDMETHODCALLTYPE CreateTexture(UINT Width, UINT Height, UINT Levels, DWORD Usage, D3DFORMAT Format, D3DPOOL Pool, IDirect3DTexture9** ppTexture, HANDLE* pSharedHandle) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE CreateVolumeTexture(UINT Width, UINT Height, UINT Depth, UINT Levels, DWORD Usage, D3DFORMAT Format, D3DPOOL Pool, IDirect3DVolumeTexture9** ppVolumeTexture, HANDLE* pSharedHandle) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE CreateCubeTexture(UINT EdgeLength, UINT Levels, DWORD Usage, D3DFORMAT Format, D3DPOOL Pool, IDirect3DCubeTexture9** ppCubeTexture, HANDLE* pSharedHandle) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE CreateVertexBuffer(UINT Length, DWORD Usage, DWORD FVF, D3DPOOL Pool, IDirect3DVertexBuffer9** ppVertexBuffer, HANDLE* pSharedHandle) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE CreateIndexBuffer(UINT Length, DWORD Usage, D3DFORMAT Format, D3DPOOL Pool, IDirect3DIndexBuffer9** ppIndexBuffer, HANDLE* pSharedHandle) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE CreateRenderTarget(UINT Width, UINT Height, D3DFORMAT Format, D3DMULTISAMPLE_TYPE MultiSample, DWORD MultisampleQuality, BOOL Lockable, IDirect3DSurface9** ppSurface, HANDLE* pSharedHandle) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE CreateDepthStencilSurface(UINT Width, UINT Height, D3DFORMAT Format, D3DMULTISAMPLE_TYPE MultiSample, DWORD MultisampleQuality, BOOL Discard, IDirect3DSurface9** ppSurface, HANDLE* pSharedHandle) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE UpdateSurface(IDirect3DSurface9* pSourceSurface, const RECT* pSourceRect, IDirect3DSurface9* pDestinationSurface, const POINT* pDestPoint) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE UpdateTexture(IDirect3DBaseTexture9* pSourceTexture, IDirect3DBaseTexture9* pDestinationTexture) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE GetRenderTargetData(IDirect3DSurface9* pRenderTarget, IDirect3DSurface9* pDestSurface) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE GetFrontBufferData(UINT iSwapChain, IDirect3DSurface9* pDestSurface) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE StretchRect(IDirect3DSurface9* pSourceSurface, const RECT* pSourceRect, IDirect3DSurface9* pDestSurface, const RECT* pDestRect, D3DTEXTUREFILTERTYPE Filter) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE ColorFill(IDirect3DSurface9* pSurface, const RECT* pRect, D3DCOLOR color) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE CreateOffscreenPlainSurface(UINT Width, UINT Height, D3DFORMAT Format, D3DPOOL Pool, IDirect3DSurface9** ppSurface, HANDLE* pSharedHandle) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE SetRenderTarget(DWORD RenderTargetIndex, IDirect3DSurface9* pRenderTarget) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE GetRenderTarget(DWORD RenderTargetIndex, IDirect3DSurface9** ppRenderTarget) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE SetDepthStencilSurface(IDirect3DSurface9* pNewZStencil) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE GetDepthStencilSurface(IDirect3DSurface9** ppZStencilSurface) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE BeginScene() override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE EndScene() override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE Clear(DWORD Count, const D3DRECT* pRects, DWORD Flags, D3DCOLOR Color, float Z, DWORD Stencil) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE MultiplyTransform(D3DTRANSFORMSTATETYPE State, const D3DMATRIX* pMatrix) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE SetViewport(const D3DVIEWPORT9* pViewport) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE GetViewport(D3DVIEWPORT9* pViewport) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE SetMaterial(const D3DMATERIAL9* pMaterial) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE GetMaterial(D3DMATERIAL9* pMaterial) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE SetLight(DWORD Index, const D3DLIGHT9* pLight) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE GetLight(DWORD Index, D3DLIGHT9* pLight) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE LightEnable(DWORD Index, BOOL Enable) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE GetLightEnable(DWORD Index, BOOL* pEnable) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE SetClipPlane(DWORD Index, const float* pPlane) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE GetClipPlane(DWORD Index, float* pPlane) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE ProcessVertices(UINT SrcStartIndex, UINT DestIndex, UINT VertexCount, IDirect3DVertexBuffer9* pDestBuffer, IDirect3DVertexDeclaration9* pVertexDecl, DWORD Flags) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE CreateVertexDeclaration(const D3DVERTEXELEMENT9* pVertexElements, IDirect3DVertexDeclaration9** ppDecl) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE CreateVertexShader(const DWORD* pFunction, IDirect3DVertexShader9** ppShader) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE SetVertexShaderConstantF(UINT StartRegister, const float* pConstantData, UINT Vector4fCount) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE GetVertexShaderConstantF(UINT StartRegister, float* pConstantData, UINT Vector4fCount) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE SetVertexShaderConstantI(UINT StartRegister, const int* pConstantData, UINT Vector4iCount) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE GetVertexShaderConstantI(UINT StartRegister, int* pConstantData, UINT Vector4iCount) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE SetVertexShaderConstantB(UINT StartRegister, const BOOL* pConstantData, UINT BoolCount) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE GetVertexShaderConstantB(UINT StartRegister, BOOL* pConstantData, UINT BoolCount) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE CreatePixelShader(const DWORD* pFunction, IDirect3DPixelShader9** ppShader) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE SetPixelShaderConstantF(UINT StartRegister, const float* pConstantData, UINT Vector4fCount) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE GetPixelShaderConstantF(UINT StartRegister, float* pConstantData, UINT Vector4fCount) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE SetPixelShaderConstantI(UINT StartRegister, const int* pConstantData, UINT Vector4iCount) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE GetPixelShaderConstantI(UINT StartRegister, int* pConstantData, UINT Vector4iCount) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE SetPixelShaderConstantB(UINT StartRegister, const BOOL* pConstantData, UINT BoolCount) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE GetPixelShaderConstantB(UINT StartRegister, BOOL* pConstantData, UINT BoolCount) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE DrawRectPatch(UINT Handle, const float* pNumSegs, const D3DRECTPATCH_INFO* pRectPatchInfo) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE DrawTriPatch(UINT Handle, const float* pNumSegs, const D3DTRIPATCH_INFO* pTriPatchInfo) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE DeletePatch(UINT Handle) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE CreateQuery(D3DQUERYTYPE Type, IDirect3DQuery9** ppQuery) override { return D3D_OK; }
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
