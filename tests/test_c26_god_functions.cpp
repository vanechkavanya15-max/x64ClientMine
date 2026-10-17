#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest/doctest.h>
#include <d3d9.h>
#include <d3dx9.h>
#include <memory>

#include "EterLib/StateManager.h"

// Lekki mock urzadzenia D3D9 do bezpiecznego testowania CStateManager
class MockD3D9DeviceForGodFunctions final : public IDirect3DDevice9Ex
{
public:
    ULONG m_refCount = 1;

    HRESULT STDMETHODCALLTYPE QueryInterface(REFIID, void** ppvObj) override
    {
        if (!ppvObj) return E_FAIL;
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
    HRESULT STDMETHODCALLTYPE BeginScene() override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE EndScene() override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE Clear(DWORD, CONST D3DRECT*, DWORD, D3DCOLOR, float, DWORD) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE SetTransform(D3DTRANSFORMSTATETYPE, CONST D3DMATRIX*) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE GetTransform(D3DTRANSFORMSTATETYPE, D3DMATRIX*) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE MultiplyTransform(D3DTRANSFORMSTATETYPE, CONST D3DMATRIX*) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE SetViewport(CONST D3DVIEWPORT9*) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE GetViewport(D3DVIEWPORT9*) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE SetMaterial(CONST D3DMATERIAL9*) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE GetMaterial(D3DMATERIAL9*) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE SetLight(DWORD, CONST D3DLIGHT9*) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE GetLight(DWORD, D3DLIGHT9*) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE LightEnable(DWORD, BOOL) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE GetLightEnable(DWORD, BOOL*) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE SetClipPlane(DWORD, CONST float*) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE GetClipPlane(DWORD, float*) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE SetRenderState(D3DRENDERSTATETYPE, DWORD) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE GetRenderState(D3DRENDERSTATETYPE, DWORD*) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE CreateStateBlock(D3DSTATEBLOCKTYPE, IDirect3DStateBlock9**) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE BeginStateBlock() override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE EndStateBlock(IDirect3DStateBlock9**) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE SetClipStatus(CONST D3DCLIPSTATUS9*) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE GetClipStatus(D3DCLIPSTATUS9*) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE GetTexture(DWORD, IDirect3DBaseTexture9**) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE SetTexture(DWORD, IDirect3DBaseTexture9*) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE GetTextureStageState(DWORD, D3DTEXTURESTAGESTATETYPE, DWORD*) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE SetTextureStageState(DWORD, D3DTEXTURESTAGESTATETYPE, DWORD) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE GetSamplerState(DWORD, D3DSAMPLERSTATETYPE, DWORD*) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE SetSamplerState(DWORD, D3DSAMPLERSTATETYPE, DWORD) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE ValidateDevice(DWORD*) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE SetPaletteEntries(UINT, CONST PALETTEENTRY*) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE GetPaletteEntries(UINT, PALETTEENTRY*) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE SetCurrentTexturePalette(UINT) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE GetCurrentTexturePalette(UINT*) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE SetScissorRect(CONST RECT*) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE GetScissorRect(RECT*) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE SetSoftwareVertexProcessing(BOOL) override { return D3D_OK; }
    BOOL STDMETHODCALLTYPE GetSoftwareVertexProcessing() override { return FALSE; }
    HRESULT STDMETHODCALLTYPE SetNPatchMode(float) override { return D3D_OK; }
    float STDMETHODCALLTYPE GetNPatchMode() override { return 0.0f; }
    HRESULT STDMETHODCALLTYPE DrawPrimitive(D3DPRIMITIVETYPE, UINT, UINT) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE DrawIndexedPrimitive(D3DPRIMITIVETYPE, INT, UINT, UINT, UINT, UINT) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE DrawPrimitiveUP(D3DPRIMITIVETYPE, UINT, CONST void*, UINT) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE DrawIndexedPrimitiveUP(D3DPRIMITIVETYPE, UINT, UINT, UINT, CONST void*, D3DFORMAT, CONST void*, UINT) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE ProcessVertices(UINT, UINT, UINT, IDirect3DVertexBuffer9*, IDirect3DVertexDeclaration9*, DWORD) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE CreateVertexDeclaration(CONST D3DVERTEXELEMENT9*, IDirect3DVertexDeclaration9**) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE SetVertexDeclaration(IDirect3DVertexDeclaration9*) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE GetVertexDeclaration(IDirect3DVertexDeclaration9**) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE SetFVF(DWORD) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE GetFVF(DWORD*) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE CreateVertexShader(CONST DWORD*, IDirect3DVertexShader9**) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE SetVertexShader(IDirect3DVertexShader9*) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE GetVertexShader(IDirect3DVertexShader9**) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE SetVertexShaderConstantF(UINT, CONST float*, UINT) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE GetVertexShaderConstantF(UINT, float*, UINT) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE SetVertexShaderConstantI(UINT, CONST int*, UINT) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE GetVertexShaderConstantI(UINT, int*, UINT) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE SetVertexShaderConstantB(UINT, CONST BOOL*, UINT) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE GetVertexShaderConstantB(UINT, BOOL*, UINT) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE SetStreamSource(UINT, IDirect3DVertexBuffer9*, UINT, UINT) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE GetStreamSource(UINT, IDirect3DVertexBuffer9**, UINT*, UINT*) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE SetStreamSourceFreq(UINT, UINT) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE GetStreamSourceFreq(UINT, UINT*) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE SetIndices(IDirect3DIndexBuffer9*) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE GetIndices(IDirect3DIndexBuffer9**) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE CreatePixelShader(CONST DWORD*, IDirect3DPixelShader9**) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE SetPixelShader(IDirect3DPixelShader9*) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE GetPixelShader(IDirect3DPixelShader9**) override { return D3D_OK; }
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

TEST_CASE("GodFunctions: StateManager modular SetDefaultState execution")
{
    MockD3D9DeviceForGodFunctions* pMockDev = new MockD3D9DeviceForGodFunctions();
    CStateManager stateMgr(pMockDev);

    stateMgr.SetDefaultState();

    // Weryfikacja ze bForce wraca do false
    CHECK(stateMgr.GetForce() == false);

    // Weryfikacja kluczowych domyslnych stanow renderowania
    CHECK(stateMgr.GetRenderState(D3DRS_DITHERENABLE) == TRUE);
    CHECK(stateMgr.GetRenderState(D3DRS_SPECULARENABLE) == FALSE);
    CHECK(stateMgr.GetRenderState(D3DRS_ZENABLE) == TRUE);
    CHECK(stateMgr.GetRenderState(D3DRS_ZWRITEENABLE) == TRUE);
    CHECK(stateMgr.GetRenderState(D3DRS_ZFUNC) == D3DCMP_LESSEQUAL);
    CHECK(stateMgr.GetRenderState(D3DRS_CULLMODE) == D3DCULL_CW);
    CHECK(stateMgr.GetRenderState(D3DRS_ALPHATESTENABLE) == FALSE);
    CHECK(stateMgr.GetRenderState(D3DRS_ALPHABLENDENABLE) == FALSE);
    CHECK(stateMgr.GetRenderState(D3DRS_LIGHTING) == FALSE);
}

TEST_CASE("GodFunctions: StateManager modular subsystems can be invoked independently")
{
    MockD3D9DeviceForGodFunctions* pMockDev = new MockD3D9DeviceForGodFunctions();
    CStateManager stateMgr(pMockDev);

    // 1. Samodzielne wolanie SetDefaultRenderStates
    stateMgr.SetDefaultRenderStates();
    CHECK(stateMgr.GetRenderState(D3DRS_ZENABLE) == TRUE);
    CHECK(stateMgr.GetRenderState(D3DRS_CULLMODE) == D3DCULL_CW);

    // 2. Samodzielne wolanie SetDefaultSamplerStates
    stateMgr.SetDefaultSamplerStates();
    for (DWORD stage = 0; stage < STATEMANAGER_MAX_STAGES; ++stage)
    {
        DWORD minFilter = 0, magFilter = 0, mipFilter = 0, addressU = 0, addressV = 0;
        stateMgr.GetSamplerState(stage, D3DSAMP_MINFILTER, &minFilter);
        stateMgr.GetSamplerState(stage, D3DSAMP_MAGFILTER, &magFilter);
        stateMgr.GetSamplerState(stage, D3DSAMP_MIPFILTER, &mipFilter);
        stateMgr.GetSamplerState(stage, D3DSAMP_ADDRESSU, &addressU);
        stateMgr.GetSamplerState(stage, D3DSAMP_ADDRESSV, &addressV);

        CHECK(minFilter == D3DTEXF_LINEAR);
        CHECK(magFilter == D3DTEXF_LINEAR);
        CHECK(mipFilter == D3DTEXF_LINEAR);
        CHECK(addressU == D3DTADDRESS_WRAP);
        CHECK(addressV == D3DTADDRESS_WRAP);
    }

    // 3. Samodzielne wolanie SetDefaultTextureStageStates
    stateMgr.SetDefaultTextureStageStates();
    for (DWORD stage = 0; stage < STATEMANAGER_MAX_STAGES; ++stage)
    {
        LPDIRECT3DBASETEXTURE9 pTex = nullptr;
        stateMgr.GetTexture(stage, &pTex);
        CHECK(pTex == nullptr);

        DWORD texCoordIndex = 999;
        stateMgr.GetTextureStageState(stage, D3DTSS_TEXCOORDINDEX, &texCoordIndex);
        CHECK(texCoordIndex == stage);
    }
}
