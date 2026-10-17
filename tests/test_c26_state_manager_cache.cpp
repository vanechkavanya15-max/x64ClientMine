#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"

#include <windows.h>
#include <d3d9.h>
#include <d3dx9.h>
#include <memory>
#include <vector>

#include "EterLib/StateManager.h"
#include "EterLib/StateManagerGuards.h"

// ============================================================================
// Mock IDirect3DDevice9Ex z pelnym sledzeniem wywolan sprzetowych
// ============================================================================
class MockD3D9DeviceForCache final : public IDirect3DDevice9Ex
{
public:
    ULONG m_refCount = 1;

    // Liczniki wywolan metod sprzetowych
    uint32_t m_setRenderStateCalls = 0;
    uint32_t m_setSamplerStateCalls = 0;
    uint32_t m_setTextureStageStateCalls = 0;
    uint32_t m_setScissorRectCalls = 0;
    uint32_t m_setLightCalls = 0;
    uint32_t m_setTransformCalls = 0;
    uint32_t m_setVertexShaderCalls = 0;
    uint32_t m_setPixelShaderCalls = 0;
    uint32_t m_setVertexDeclCalls = 0;
    uint32_t m_setFVFCalls = 0;
    uint32_t m_setTextureCalls = 0;

    // Zarejestrowane stany sprzetowe
    DWORD m_hwRenderStates[STATEMANAGER_MAX_RENDERSTATES] = {};
    DWORD m_hwSamplerStates[STATEMANAGER_MAX_STAGES][STATEMANAGER_MAX_TEXTURESTATES] = {};
    DWORD m_hwTextureStageStates[STATEMANAGER_MAX_STAGES][STATEMANAGER_MAX_TEXTURESTATES] = {};
    RECT m_hwScissorRect = { 0, 0, 0, 0 };
    D3DLIGHT9 m_hwLights[8] = {};
    D3DXMATRIX m_hwTransform[STATEMANAGER_MAX_TRANSFORMSTATES] = {};
    IDirect3DVertexDeclaration9* m_hwVertexDecl = nullptr;
    IDirect3DVertexShader9* m_hwVertexShader = nullptr;
    IDirect3DPixelShader9* m_hwPixelShader = nullptr;
    DWORD m_hwFVF = 0;

    MockD3D9DeviceForCache()
    {
        for (int i = 0; i < STATEMANAGER_MAX_RENDERSTATES; ++i)
            m_hwRenderStates[i] = 0;
        for (int s = 0; s < STATEMANAGER_MAX_STAGES; ++s)
            for (int t = 0; t < STATEMANAGER_MAX_TEXTURESTATES; ++t)
            {
                m_hwSamplerStates[s][t] = 0;
                m_hwTextureStageStates[s][t] = 0;
            }
        for (int i = 0; i < STATEMANAGER_MAX_TRANSFORMSTATES; ++i)
            D3DXMatrixIdentity(&m_hwTransform[i]);
    }

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

    HRESULT STDMETHODCALLTYPE SetRenderState(D3DRENDERSTATETYPE State, DWORD Value) override
    {
        m_setRenderStateCalls++;
        if (State < STATEMANAGER_MAX_RENDERSTATES)
            m_hwRenderStates[State] = Value;
        return D3D_OK;
    }
    HRESULT STDMETHODCALLTYPE GetRenderState(D3DRENDERSTATETYPE State, DWORD* pValue) override
    {
        if (pValue && State < STATEMANAGER_MAX_RENDERSTATES)
            *pValue = m_hwRenderStates[State];
        return D3D_OK;
    }

    HRESULT STDMETHODCALLTYPE SetSamplerState(DWORD Sampler, D3DSAMPLERSTATETYPE Type, DWORD Value) override
    {
        m_setSamplerStateCalls++;
        if (Sampler < STATEMANAGER_MAX_STAGES && Type < STATEMANAGER_MAX_TEXTURESTATES)
            m_hwSamplerStates[Sampler][Type] = Value;
        return D3D_OK;
    }
    HRESULT STDMETHODCALLTYPE GetSamplerState(DWORD Sampler, D3DSAMPLERSTATETYPE Type, DWORD* pValue) override
    {
        if (pValue && Sampler < STATEMANAGER_MAX_STAGES && Type < STATEMANAGER_MAX_TEXTURESTATES)
            *pValue = m_hwSamplerStates[Sampler][Type];
        return D3D_OK;
    }

    HRESULT STDMETHODCALLTYPE SetTextureStageState(DWORD Stage, D3DTEXTURESTAGESTATETYPE Type, DWORD Value) override
    {
        m_setTextureStageStateCalls++;
        if (Stage < STATEMANAGER_MAX_STAGES && Type < STATEMANAGER_MAX_TEXTURESTATES)
            m_hwTextureStageStates[Stage][Type] = Value;
        return D3D_OK;
    }
    HRESULT STDMETHODCALLTYPE GetTextureStageState(DWORD Stage, D3DTEXTURESTAGESTATETYPE Type, DWORD* pValue) override
    {
        if (pValue && Stage < STATEMANAGER_MAX_STAGES && Type < STATEMANAGER_MAX_TEXTURESTATES)
            *pValue = m_hwTextureStageStates[Stage][Type];
        return D3D_OK;
    }

    HRESULT STDMETHODCALLTYPE SetScissorRect(CONST RECT* pRect) override
    {
        m_setScissorRectCalls++;
        if (pRect)
            m_hwScissorRect = *pRect;
        return D3D_OK;
    }
    HRESULT STDMETHODCALLTYPE GetScissorRect(RECT* pRect) override
    {
        if (pRect)
            *pRect = m_hwScissorRect;
        return D3D_OK;
    }

    HRESULT STDMETHODCALLTYPE SetLight(DWORD Index, CONST D3DLIGHT9* pLight) override
    {
        m_setLightCalls++;
        if (Index < 8 && pLight)
            m_hwLights[Index] = *pLight;
        return D3D_OK;
    }
    HRESULT STDMETHODCALLTYPE GetLight(DWORD Index, D3DLIGHT9* pLight) override
    {
        if (Index < 8 && pLight)
            *pLight = m_hwLights[Index];
        return D3D_OK;
    }

    HRESULT STDMETHODCALLTYPE SetTransform(D3DTRANSFORMSTATETYPE State, CONST D3DMATRIX* pMatrix) override
    {
        m_setTransformCalls++;
        if (State < STATEMANAGER_MAX_TRANSFORMSTATES && pMatrix)
            m_hwTransform[State] = *reinterpret_cast<const D3DXMATRIX*>(pMatrix);
        return D3D_OK;
    }
    HRESULT STDMETHODCALLTYPE GetTransform(D3DTRANSFORMSTATETYPE State, D3DMATRIX* pMatrix) override
    {
        if (State < STATEMANAGER_MAX_TRANSFORMSTATES && pMatrix)
            *reinterpret_cast<D3DXMATRIX*>(pMatrix) = m_hwTransform[State];
        return D3D_OK;
    }

    HRESULT STDMETHODCALLTYPE SetVertexShader(IDirect3DVertexShader9* pShader) override
    {
        m_setVertexShaderCalls++;
        m_hwVertexShader = pShader;
        return D3D_OK;
    }
    HRESULT STDMETHODCALLTYPE GetVertexShader(IDirect3DVertexShader9** ppShader) override
    {
        if (ppShader) *ppShader = m_hwVertexShader;
        return D3D_OK;
    }

    HRESULT STDMETHODCALLTYPE SetPixelShader(IDirect3DPixelShader9* pShader) override
    {
        m_setPixelShaderCalls++;
        m_hwPixelShader = pShader;
        return D3D_OK;
    }
    HRESULT STDMETHODCALLTYPE GetPixelShader(IDirect3DPixelShader9** ppShader) override
    {
        if (ppShader) *ppShader = m_hwPixelShader;
        return D3D_OK;
    }

    HRESULT STDMETHODCALLTYPE SetVertexDeclaration(IDirect3DVertexDeclaration9* pDecl) override
    {
        m_setVertexDeclCalls++;
        m_hwVertexDecl = pDecl;
        return D3D_OK;
    }
    HRESULT STDMETHODCALLTYPE GetVertexDeclaration(IDirect3DVertexDeclaration9** ppDecl) override
    {
        if (ppDecl) *ppDecl = m_hwVertexDecl;
        return D3D_OK;
    }

    HRESULT STDMETHODCALLTYPE SetFVF(DWORD FVF) override
    {
        m_setFVFCalls++;
        m_hwFVF = FVF;
        if (FVF != 0)
            m_hwVertexDecl = nullptr;
        return D3D_OK;
    }
    HRESULT STDMETHODCALLTYPE GetFVF(DWORD* pFVF) override
    {
        if (pFVF) *pFVF = m_hwFVF;
        return D3D_OK;
    }

    HRESULT STDMETHODCALLTYPE SetTexture(DWORD, IDirect3DBaseTexture9*) override
    {
        m_setTextureCalls++;
        return D3D_OK;
    }
    HRESULT STDMETHODCALLTYPE GetTexture(DWORD, IDirect3DBaseTexture9** pp) override
    {
        if (pp) *pp = nullptr;
        return D3D_OK;
    }

    HRESULT STDMETHODCALLTYPE SetMaterial(CONST D3DMATERIAL9*) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE GetMaterial(D3DMATERIAL9*) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE SetSoftwareVertexProcessing(BOOL) override { return D3D_OK; }
    BOOL STDMETHODCALLTYPE GetSoftwareVertexProcessing() override { return FALSE; }
    HRESULT STDMETHODCALLTYPE SetVertexShaderConstantF(UINT, CONST float*, UINT) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE SetPixelShaderConstantF(UINT, CONST float*, UINT) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE SetStreamSource(UINT, IDirect3DVertexBuffer9*, UINT, UINT) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE SetIndices(IDirect3DIndexBuffer9*) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE BeginScene() override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE EndScene() override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE DrawPrimitive(D3DPRIMITIVETYPE, UINT, UINT) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE DrawPrimitiveUP(D3DPRIMITIVETYPE, UINT, CONST void*, UINT) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE DrawIndexedPrimitive(D3DPRIMITIVETYPE, INT, UINT, UINT, UINT, UINT) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE DrawIndexedPrimitiveUP(D3DPRIMITIVETYPE, UINT, UINT, UINT, CONST void*, D3DFORMAT, CONST void*, UINT) override { return D3D_OK; }
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

struct DummyDeclForCache : IDirect3DVertexDeclaration9
{
    ULONG ref = 1;
    HRESULT STDMETHODCALLTYPE QueryInterface(REFIID, void**) override { return S_OK; }
    ULONG STDMETHODCALLTYPE AddRef() override { return ++ref; }
    ULONG STDMETHODCALLTYPE Release() override { return --ref; }
    HRESULT STDMETHODCALLTYPE GetDevice(IDirect3DDevice9**) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE GetDeclaration(D3DVERTEXELEMENT9*, UINT*) override { return D3D_OK; }
};

struct DummyVSForCache : IDirect3DVertexShader9
{
    ULONG ref = 1;
    HRESULT STDMETHODCALLTYPE QueryInterface(REFIID, void**) override { return S_OK; }
    ULONG STDMETHODCALLTYPE AddRef() override { return ++ref; }
    ULONG STDMETHODCALLTYPE Release() override { return --ref; }
    HRESULT STDMETHODCALLTYPE GetDevice(IDirect3DDevice9**) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE GetFunction(void*, UINT*) override { return D3D_OK; }
};

struct DummyPSForCache : IDirect3DPixelShader9
{
    ULONG ref = 1;
    HRESULT STDMETHODCALLTYPE QueryInterface(REFIID, void**) override { return S_OK; }
    ULONG STDMETHODCALLTYPE AddRef() override { return ++ref; }
    ULONG STDMETHODCALLTYPE Release() override { return --ref; }
    HRESULT STDMETHODCALLTYPE GetDevice(IDirect3DDevice9**) override { return D3D_OK; }
    HRESULT STDMETHODCALLTYPE GetFunction(void*, UINT*) override { return D3D_OK; }
};

// Fixture pomocniczy do tworzenia srodowiska testowego
struct StateManagerTestFixture
{
    MockD3D9DeviceForCache* pDev = nullptr;
    std::unique_ptr<CStateManager> pStateManager;

    StateManagerTestFixture()
    {
        pDev = new MockD3D9DeviceForCache();
        pStateManager = std::make_unique<CStateManager>(pDev);
    }

    ~StateManagerTestFixture()
    {
        pStateManager.reset();
        // pDev jest zwalniany przez CStateManager::~CStateManager (Release)
    }
};

// ============================================================================
// Test A: ResetState uniewaznia samplery, scissor rect oraz swiatla
// ============================================================================
TEST_CASE("ResetState invalidates samplers and scissor rect and lights")
{
    StateManagerTestFixture fixture;
    auto* dev = fixture.pDev;

    // 1. Konfiguracja poczatkowa samplerow
    STATEMANAGER.SetSamplerState(0, D3DSAMP_MINFILTER, D3DTEXF_LINEAR);
    STATEMANAGER.SetSamplerState(1, D3DSAMP_ADDRESSU, D3DTADDRESS_CLAMP);

    DWORD samp0 = 0, samp1 = 0;
    STATEMANAGER.GetSamplerState(0, D3DSAMP_MINFILTER, &samp0);
    STATEMANAGER.GetSamplerState(1, D3DSAMP_ADDRESSU, &samp1);
    CHECK(samp0 == D3DTEXF_LINEAR);
    CHECK(samp1 == D3DTADDRESS_CLAMP);

    // 2. Konfiguracja scissor rect
    RECT testRect = { 10, 20, 300, 400 };
    STATEMANAGER.SetScissorRect(testRect);
    uint32_t scissorCallsBefore = dev->m_setScissorRectCalls;
    CHECK(scissorCallsBefore > 0);

    // Ponowne ustawienie tego samego rect z cache - nie powinno wywolac urzadzenia
    STATEMANAGER.SetScissorRect(testRect);
    CHECK(dev->m_setScissorRectCalls == scissorCallsBefore);

    // 3. Konfiguracja swiatla
    D3DLIGHT9 light{};
    light.Type = D3DLIGHT_DIRECTIONAL;
    light.Diffuse.r = 1.0f;
    STATEMANAGER.SetLight(0, &light);
    uint32_t lightCallsBefore = dev->m_setLightCalls;
    CHECK(lightCallsBefore > 0);

    // Ponowne ustawienie tego samego swiatla - ignorowane przez cache
    STATEMANAGER.SetLight(0, &light);
    CHECK(dev->m_setLightCalls == lightCallsBefore);

    // 4. Wywolanie ResetState()
    STATEMANAGER.ResetState();

    // Weryfikacja: samplery musza miec wartosc 0xFFFFFFFF (UNKNOWN / niezainicjalizowany)
    STATEMANAGER.GetSamplerState(0, D3DSAMP_MINFILTER, &samp0);
    STATEMANAGER.GetSamplerState(1, D3DSAMP_ADDRESSU, &samp1);
    CHECK(samp0 == 0xFFFFFFFF);
    CHECK(samp1 == 0xFFFFFFFF);

    // Po uniewaznieniu, ponowne ustawienie tej samej wartosci samplera MUSI wyslac stan do urzadzenia
    uint32_t samplerCallsBefore = dev->m_setSamplerStateCalls;
    STATEMANAGER.SetSamplerState(0, D3DSAMP_MINFILTER, D3DTEXF_LINEAR);
    CHECK(dev->m_setSamplerStateCalls == samplerCallsBefore + 1);

    // Weryfikacja: scissor rect cache uniewazniony (m_bScissorRectValid == false)
    // Ponowne ustawienie tego samego rect MUSI trafic do urzadzenia
    STATEMANAGER.SetScissorRect(testRect);
    CHECK(dev->m_setScissorRectCalls == scissorCallsBefore + 1);

    // Weryfikacja: swiatla uniewaznione
    // Ponowne ustawienie tego samego swiatla MUSI trafic do urzadzenia
    STATEMANAGER.SetLight(0, &light);
    CHECK(dev->m_setLightCalls == lightCallsBefore + 1);
}

// ============================================================================
// Test B: m_bForce wymusza wyslanie stanu pomimo zgodnosci z cache
// ============================================================================
TEST_CASE("m_bForce forces state transmission to device")
{
    StateManagerTestFixture fixture;
    auto* dev = fixture.pDev;

    STATEMANAGER.SetForce(false);

    // 1. RenderState
    STATEMANAGER.SetRenderState(D3DRS_ZENABLE, D3DZB_TRUE);
    uint32_t rsCalls = dev->m_setRenderStateCalls;
    STATEMANAGER.SetRenderState(D3DRS_ZENABLE, D3DZB_TRUE);
    CHECK(dev->m_setRenderStateCalls == rsCalls); // Brak wywolania, bo cache pasuje

    STATEMANAGER.SetForce(true);
    CHECK(STATEMANAGER.GetForce() == true);
    STATEMANAGER.SetRenderState(D3DRS_ZENABLE, D3DZB_TRUE);
    CHECK(dev->m_setRenderStateCalls == rsCalls + 1); // Wymuszone wyslanie!

    // 2. SamplerState
    STATEMANAGER.SetForce(false);
    STATEMANAGER.SetSamplerState(0, D3DSAMP_MAGFILTER, D3DTEXF_LINEAR);
    uint32_t sampCalls = dev->m_setSamplerStateCalls;
    STATEMANAGER.SetSamplerState(0, D3DSAMP_MAGFILTER, D3DTEXF_LINEAR);
    CHECK(dev->m_setSamplerStateCalls == sampCalls);

    STATEMANAGER.SetForce(true);
    STATEMANAGER.SetSamplerState(0, D3DSAMP_MAGFILTER, D3DTEXF_LINEAR);
    CHECK(dev->m_setSamplerStateCalls == sampCalls + 1);

    // 3. TextureStageState
    STATEMANAGER.SetForce(false);
    STATEMANAGER.SetTextureStageState(0, D3DTSS_COLOROP, D3DTOP_MODULATE);
    uint32_t tssCalls = dev->m_setTextureStageStateCalls;
    STATEMANAGER.SetTextureStageState(0, D3DTSS_COLOROP, D3DTOP_MODULATE);
    CHECK(dev->m_setTextureStageStateCalls == tssCalls);

    STATEMANAGER.SetForce(true);
    STATEMANAGER.SetTextureStageState(0, D3DTSS_COLOROP, D3DTOP_MODULATE);
    CHECK(dev->m_setTextureStageStateCalls == tssCalls + 1);

    // 4. ScissorRect
    STATEMANAGER.SetForce(false);
    RECT rectA = { 0, 0, 800, 600 };
    STATEMANAGER.SetScissorRect(rectA);
    uint32_t scCalls = dev->m_setScissorRectCalls;
    STATEMANAGER.SetScissorRect(rectA);
    CHECK(dev->m_setScissorRectCalls == scCalls);

    STATEMANAGER.SetForce(true);
    STATEMANAGER.SetScissorRect(rectA);
    CHECK(dev->m_setScissorRectCalls == scCalls + 1);

    // 5. Transform
    STATEMANAGER.SetForce(false);
    D3DXMATRIX mat;
    D3DXMatrixIdentity(&mat);
    STATEMANAGER.SetTransform(D3DTS_WORLD, &mat);
    uint32_t transCalls = dev->m_setTransformCalls;
    STATEMANAGER.SetTransform(D3DTS_WORLD, &mat);
    CHECK(dev->m_setTransformCalls == transCalls);

    STATEMANAGER.SetForce(true);
    STATEMANAGER.SetTransform(D3DTS_WORLD, &mat);
    CHECK(dev->m_setTransformCalls == transCalls + 1);

    STATEMANAGER.SetForce(false);
}

// ============================================================================
// Test C: SetTransform poza scena wysyla do urzadzenia dla wszystkich typow transformacji
// ============================================================================
TEST_CASE("SetTransform outside scene sends to device for all transform types")
{
    StateManagerTestFixture fixture;
    auto* dev = fixture.pDev;

    STATEMANAGER.EndScene(); // Poza scena

    D3DXMATRIX matWorld, matProj, matTex0;
    D3DXMatrixIdentity(&matWorld); matWorld._11 = 2.0f;
    D3DXMatrixIdentity(&matProj); matProj._22 = 3.0f;
    D3DXMatrixIdentity(&matTex0); matTex0._33 = 4.0f;

    uint32_t callsBefore = dev->m_setTransformCalls;

    // D3DTS_WORLD poza scena
    STATEMANAGER.SetTransform(D3DTS_WORLD, &matWorld);
    CHECK(dev->m_setTransformCalls == callsBefore + 1);
    CHECK(dev->m_hwTransform[D3DTS_WORLD]._11 == 2.0f);

    // D3DTS_PROJECTION poza scena
    STATEMANAGER.SetTransform(D3DTS_PROJECTION, &matProj);
    CHECK(dev->m_setTransformCalls == callsBefore + 2);
    CHECK(dev->m_hwTransform[D3DTS_PROJECTION]._22 == 3.0f);

    // D3DTS_TEXTURE0 (inny typ transformacji poza scena)
    STATEMANAGER.SetTransform((D3DTRANSFORMSTATETYPE)D3DTS_TEXTURE0, &matTex0);
    CHECK(dev->m_setTransformCalls == callsBefore + 3);
    CHECK(dev->m_hwTransform[D3DTS_TEXTURE0]._33 == 4.0f);
}

// ============================================================================
// Test D: ScopedD3DVertexDeclGuard poprawnie zapisuje i przywraca VertexDecl oraz FVF
// ============================================================================
TEST_CASE("ScopedD3DVertexDeclGuard saves and restores both VertexDecl and FVF")
{
    StateManagerTestFixture fixture;
    auto* dev = fixture.pDev;

    DummyDeclForCache decl1, decl2;
    const DWORD testFVF = D3DFVF_XYZ | D3DFVF_DIFFUSE;

    // Scenariusz 1: FVF aktywny przed wejsciem do straznika
    STATEMANAGER.SetFVF(testFVF);
    CHECK(dev->m_hwFVF == testFVF);
    CHECK(dev->m_hwVertexDecl == nullptr);

    {
        ScopedD3DVertexDeclGuard guard(&decl1);
        LPDIRECT3DVERTEXDECLARATION9 curDecl = nullptr;
        STATEMANAGER.GetVertexDeclaration(&curDecl);
        CHECK(curDecl == &decl1);
        CHECK(dev->m_hwVertexDecl == &decl1);

        DWORD curFVF = 0;
        STATEMANAGER.GetFVF(&curFVF);
        CHECK(curFVF == 0); // SetVertexDeclaration unbinduje FVF
    }

    // Po wyjsciu ze straznika FVF musi zostac przywrocony!
    DWORD restoredFVF = 0;
    STATEMANAGER.GetFVF(&restoredFVF);
    CHECK(restoredFVF == testFVF);
    CHECK(dev->m_hwFVF == testFVF);

    LPDIRECT3DVERTEXDECLARATION9 restoredDecl = nullptr;
    STATEMANAGER.GetVertexDeclaration(&restoredDecl);
    CHECK(restoredDecl == nullptr);

    // Scenariusz 2: VertexDeclaration aktywna przed wejsciem do straznika
    STATEMANAGER.SetVertexDeclaration(&decl1);
    CHECK(dev->m_hwVertexDecl == &decl1);

    {
        ScopedD3DVertexDeclGuard guard(&decl2);
        LPDIRECT3DVERTEXDECLARATION9 curDecl = nullptr;
        STATEMANAGER.GetVertexDeclaration(&curDecl);
        CHECK(curDecl == &decl2);
    }

    STATEMANAGER.GetVertexDeclaration(&restoredDecl);
    CHECK(restoredDecl == &decl1);
    CHECK(dev->m_hwVertexDecl == &decl1);

    // Scenariusz 3: bRestoreOnExit = false (czyste wyjscie do NULL)
    {
        ScopedD3DVertexDeclGuard guard(&decl2, false);
    }
    STATEMANAGER.GetVertexDeclaration(&restoredDecl);
    CHECK(restoredDecl == nullptr);
}

// ============================================================================
// Test E: ScopedD3DFFPGuard przywraca stany w kolejnosci FVF -> VS (D3D9 semantics)
// ============================================================================
TEST_CASE("ScopedD3DFFPGuard restores states in order FVF -> VS")
{
    StateManagerTestFixture fixture;
    auto* dev = fixture.pDev;

    DummyVSForCache vs;
    DummyPSForCache ps;
    const DWORD testFVF = D3DFVF_XYZ | D3DFVF_NORMAL | D3DFVF_TEX1;

    // Ustawiamy stan bazowy: FVF oraz VertexShader i PixelShader
    STATEMANAGER.SetFVF(testFVF);
    STATEMANAGER.SetVertexShader(&vs);
    STATEMANAGER.SetPixelShader(&ps);

    LPDIRECT3DVERTEXSHADER9 checkVS = nullptr;
    LPDIRECT3DPIXELSHADER9 checkPS = nullptr;
    DWORD checkFVF = 0;
    STATEMANAGER.GetVertexShader(&checkVS);
    STATEMANAGER.GetPixelShader(&checkPS);
    STATEMANAGER.GetFVF(&checkFVF);
    CHECK(checkVS == &vs);
    CHECK(checkPS == &ps);
    CHECK(checkFVF == testFVF);

    // Wejscie w straznik FVF (potok terenu / debug)
    const DWORD terrainFVF = D3DFVF_XYZ | D3DFVF_DIFFUSE;
    {
        ScopedD3DFFPGuard ffpGuard(terrainFVF, true);

        LPDIRECT3DVERTEXDECLARATION9 curDecl = nullptr;
        STATEMANAGER.GetVertexDeclaration(&curDecl);
        STATEMANAGER.GetVertexShader(&checkVS);
        STATEMANAGER.GetPixelShader(&checkPS);
        STATEMANAGER.GetFVF(&checkFVF);

        // Wszystkie shadery i deklaracje musza byc wyzerowane (zero-leak)
        CHECK(curDecl == nullptr);
        CHECK(checkVS == nullptr);
        CHECK(checkPS == nullptr);
        CHECK(checkFVF == terrainFVF);

        CHECK(dev->m_hwVertexDecl == nullptr);
        CHECK(dev->m_hwVertexShader == nullptr);
        CHECK(dev->m_hwPixelShader == nullptr);
        CHECK(dev->m_hwFVF == terrainFVF);
    }

    // Po wyjsciu: FVF -> VS kolejnosc gwarantuje, ze SetFVF nie nadpisze przywroconego VertexShadera!
    STATEMANAGER.GetVertexShader(&checkVS);
    STATEMANAGER.GetPixelShader(&checkPS);
    STATEMANAGER.GetFVF(&checkFVF);

    CHECK(checkFVF == testFVF);
    CHECK(checkVS == &vs);
    CHECK(checkPS == &ps);
    CHECK(dev->m_hwVertexShader == &vs);
    CHECK(dev->m_hwPixelShader == &ps);
    CHECK(dev->m_hwFVF == testFVF);

    // Czyste wyjscie (bRestoreOnExit = false)
    {
        ScopedD3DFFPGuard ffpGuard(terrainFVF, false);
    }
    STATEMANAGER.GetVertexShader(&checkVS);
    STATEMANAGER.GetPixelShader(&checkPS);
    CHECK(checkVS == nullptr);
    CHECK(checkPS == nullptr);
}
