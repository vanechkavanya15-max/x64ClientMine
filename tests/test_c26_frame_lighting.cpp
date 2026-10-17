#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"

#include <windows.h>
#include <d3d9.h>
#include <d3dx9.h>
#include <cstdint>
#include <cstring>
#include <memory>

#include "EterLib/RenderPacketTypes.h"
#include "EterLib/RenderQueueManager.h"
#include "EterLib/CFrameSnapshot.h"
#include "EterLib/D3D9RenderBackend.h"
#include "EterLib/StateManager.h"

namespace
{
// Mock urzadzenia D3D9 do weryfikacji aplikacji stanow przez backend
class MockD3D9DeviceForLighting final : public IDirect3DDevice9Ex
{
public:
	ULONG m_refCount{ 1 };
	DWORD m_renderStates[256]{};
	D3DLIGHT9 m_lights[8]{};
	BOOL m_lightEnabled[8]{};
	D3DVIEWPORT9 m_viewport{};

	MockD3D9DeviceForLighting()
	{
		std::memset(m_renderStates, 0, sizeof(m_renderStates));
		std::memset(m_lights, 0, sizeof(m_lights));
		std::memset(m_lightEnabled, 0, sizeof(m_lightEnabled));
		m_viewport = D3DVIEWPORT9{ 0, 0, 800, 600, 0.0f, 1.0f };
	}

	HRESULT STDMETHODCALLTYPE QueryInterface(REFIID riid, void** ppvObj) override
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
	UINT STDMETHODCALLTYPE GetAvailableTextureMem() override { return 1024 * 1024 * 512; }
	HRESULT STDMETHODCALLTYPE EvictManagedResources() override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE GetDirect3D(IDirect3D9** ppD3D9) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE GetDeviceCaps(D3DCAPS9* pCaps) override
	{
		if (pCaps)
		{
			std::memset(pCaps, 0, sizeof(D3DCAPS9));
			pCaps->Caps2 = D3DCAPS2_FULLSCREENGAMMA;
		}
		return D3D_OK;
	}
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
	HRESULT STDMETHODCALLTYPE CreateTexture(UINT Width, UINT Height, UINT Levels, DWORD Usage, D3DFORMAT Format, D3DPOOL Pool, IDirect3DTexture9** ppTexture, HANDLE* pSharedHandle) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE CreateVolumeTexture(UINT Width, UINT Height, UINT Depth, UINT Levels, DWORD Usage, D3DFORMAT Format, D3DPOOL Pool, IDirect3DVolumeTexture9** ppVolumeTexture, HANDLE* pSharedHandle) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE CreateCubeTexture(UINT EdgeLength, UINT Levels, DWORD Usage, D3DFORMAT Format, D3DPOOL Pool, IDirect3DCubeTexture9** ppCubeTexture, HANDLE* pSharedHandle) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE CreateVertexBuffer(UINT Length, DWORD Usage, DWORD FVF, D3DPOOL Pool, IDirect3DVertexBuffer9** ppVertexBuffer, HANDLE* pSharedHandle) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE CreateIndexBuffer(UINT Length, DWORD Usage, D3DFORMAT Format, D3DPOOL Pool, IDirect3DIndexBuffer9** ppIndexBuffer, HANDLE* pSharedHandle) override { return D3D_OK; }
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
	HRESULT STDMETHODCALLTYPE GetTransform(D3DTRANSFORMSTATETYPE State, D3DMATRIX* pMatrix) override
	{
		if (pMatrix) D3DXMatrixIdentity(reinterpret_cast<D3DXMATRIX*>(pMatrix));
		return D3D_OK;
	}
	HRESULT STDMETHODCALLTYPE MultiplyTransform(D3DTRANSFORMSTATETYPE State, CONST D3DMATRIX* pMatrix) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE SetViewport(CONST D3DVIEWPORT9* pViewport) override
	{
		if (pViewport) m_viewport = *pViewport;
		return D3D_OK;
	}
	HRESULT STDMETHODCALLTYPE GetViewport(D3DVIEWPORT9* pViewport) override
	{
		if (pViewport) *pViewport = m_viewport;
		return D3D_OK;
	}
	HRESULT STDMETHODCALLTYPE SetMaterial(CONST D3DMATERIAL9* pMaterial) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE GetMaterial(D3DMATERIAL9* pMaterial) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE SetLight(DWORD Index, CONST D3DLIGHT9* pLight) override
	{
		if (Index < 8 && pLight) m_lights[Index] = *pLight;
		return D3D_OK;
	}
	HRESULT STDMETHODCALLTYPE GetLight(DWORD Index, D3DLIGHT9* pLight) override
	{
		if (Index < 8 && pLight) *pLight = m_lights[Index];
		return D3D_OK;
	}
	HRESULT STDMETHODCALLTYPE LightEnable(DWORD Index, BOOL Enable) override
	{
		if (Index < 8) m_lightEnabled[Index] = Enable;
		return D3D_OK;
	}
	HRESULT STDMETHODCALLTYPE GetLightEnable(DWORD Index, BOOL* pEnable) override
	{
		if (Index < 8 && pEnable) *pEnable = m_lightEnabled[Index];
		return D3D_OK;
	}
	HRESULT STDMETHODCALLTYPE SetClipPlane(DWORD Index, CONST float* pPlane) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE GetClipPlane(DWORD Index, float* pPlane) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE SetRenderState(D3DRENDERSTATETYPE State, DWORD Value) override
	{
		if (State < 256) m_renderStates[State] = Value;
		return D3D_OK;
	}
	HRESULT STDMETHODCALLTYPE GetRenderState(D3DRENDERSTATETYPE State, DWORD* pValue) override
	{
		if (pValue && State < 256) *pValue = m_renderStates[State];
		return D3D_OK;
	}
	HRESULT STDMETHODCALLTYPE CreateStateBlock(D3DSTATEBLOCKTYPE Type, IDirect3DStateBlock9** ppSB) override { return D3D_OK; }
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
	HRESULT STDMETHODCALLTYPE DrawIndexedPrimitive(D3DPRIMITIVETYPE, INT, UINT, UINT, UINT, UINT) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE DrawPrimitiveUP(D3DPRIMITIVETYPE, UINT, CONST void*, UINT) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE DrawIndexedPrimitiveUP(D3DPRIMITIVETYPE, UINT, UINT, UINT, CONST void*, D3DFORMAT, CONST void*, UINT) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE ProcessVertices(UINT, UINT, UINT, IDirect3DVertexBuffer9*, IDirect3DVertexDeclaration9*, DWORD) override { return D3D_OK; }
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

	// IDirect3DDevice9Ex
	HRESULT STDMETHODCALLTYPE SetConvolutionMonoKernel(UINT, UINT, float*, float*) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE ComposeRects(IDirect3DSurface9*, IDirect3DSurface9*, IDirect3DVertexBuffer9*, UINT, IDirect3DVertexBuffer9*, D3DCOMPOSERECTSOP, int, int) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE PresentEx(CONST RECT*, CONST RECT*, HWND, CONST RGNDATA*, DWORD) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE GetGPUThreadPriority(INT* pPriority) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE SetGPUThreadPriority(INT Priority) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE WaitForVBlank(UINT iSwapChain) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE CheckResourceResidency(IDirect3DResource9** pResourceArray, UINT32 NumResources) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE SetMaximumFrameLatency(UINT MaxLatency) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE GetMaximumFrameLatency(UINT* pMaxLatency) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE CheckDeviceState(HWND hWindow) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE CreateRenderTargetEx(UINT Width, UINT Height, D3DFORMAT Format, D3DMULTISAMPLE_TYPE MultiSample, DWORD MultisampleQuality, BOOL Lockable, IDirect3DSurface9** ppSurface, HANDLE* pSharedHandle, DWORD Usage) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE CreateOffscreenPlainSurfaceEx(UINT Width, UINT Height, D3DFORMAT Format, D3DPOOL Pool, IDirect3DSurface9** ppSurface, HANDLE* pSharedHandle, DWORD Usage) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE CreateDepthStencilSurfaceEx(UINT Width, UINT Height, D3DFORMAT Format, D3DMULTISAMPLE_TYPE MultiSample, DWORD MultisampleQuality, BOOL Discard, IDirect3DSurface9** ppSurface, HANDLE* pSharedHandle, DWORD Usage) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE ResetEx(D3DPRESENT_PARAMETERS* pPresentationParameters, D3DDISPLAYMODEEX* pFullscreenDisplayMode) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE GetDisplayModeEx(UINT iSwapChain, D3DDISPLAYMODEEX* pMode, D3DDISPLAYROTATION* pRotation) override { return D3D_OK; }
};
}

TEST_SUITE("FrameLightingTests")
{
	TEST_CASE("FrameLighting - Packing Sun Light and Ambient into TRenderFrameUniforms")
	{
		TRenderFrameUniforms uniforms{};
		uniforms.vSunLightDir = D3DXVECTOR3(0.577f, -0.577f, 0.577f);
		uniforms.vSunLightColor = D3DXCOLOR(0.9f, 0.85f, 0.8f, 1.0f);
		uniforms.vAmbientColor = D3DXCOLOR(0.2f, 0.25f, 0.3f, 1.0f);

		CHECK(uniforms.vSunLightDir.x == doctest::Approx(0.577f));
		CHECK(uniforms.vSunLightDir.y == doctest::Approx(-0.577f));
		CHECK(uniforms.vSunLightDir.z == doctest::Approx(0.577f));

		// Sprawdzenie mapowania unii wektorowej vec4
		CHECK(uniforms.v4SunColor.x == doctest::Approx(0.9f));
		CHECK(uniforms.v4SunColor.y == doctest::Approx(0.85f));
		CHECK(uniforms.v4SunColor.z == doctest::Approx(0.8f));
		CHECK(uniforms.v4SunColor.w == doctest::Approx(1.0f));

		CHECK(uniforms.v4AmbientColor.x == doctest::Approx(0.2f));
		CHECK(uniforms.v4AmbientColor.y == doctest::Approx(0.25f));
		CHECK(uniforms.v4AmbientColor.z == doctest::Approx(0.3f));
		CHECK(uniforms.v4AmbientColor.w == doctest::Approx(1.0f));

		CRenderQueueManager::Instance().SetFrameUniforms(uniforms);
		const TRenderFrameUniforms& retrieved = CRenderQueueManager::Instance().GetFrameUniforms();
		CHECK(retrieved.vSunLightDir.x == doctest::Approx(0.577f));
		CHECK(retrieved.vSunLightColor.r == doctest::Approx(0.9f));
		CHECK(retrieved.vAmbientColor.g == doctest::Approx(0.25f));
	}

	TEST_CASE("FrameLighting - Packing Fog Parameters into TRenderFrameUniforms")
	{
		TRenderFrameUniforms uniforms{};
		const float fNear = 3500.0f;
		const float fFar = 11500.0f;
		const float fInvRange = 1.0f / (fFar - fNear);
		const D3DXCOLOR cFog(0.4f, 0.5f, 0.6f, 1.0f);

		uniforms.fFogNear = fNear;
		uniforms.fFogFar = fFar;
		uniforms.vFogColor = cFog;
		uniforms.v4FogParams = D3DXVECTOR4(fNear, fFar, fInvRange, 1.0f);

		CHECK(uniforms.fFogNear == doctest::Approx(3500.0f));
		CHECK(uniforms.fFogFar == doctest::Approx(11500.0f));
		CHECK(uniforms.fFogInvRange == doctest::Approx(fInvRange));
		CHECK(uniforms.fFogEnable == doctest::Approx(1.0f));
		CHECK(uniforms.vFogColor.r == doctest::Approx(0.4f));

		// Test wylaczonej mgly (fFogEnable = 0.0f, ale fFogNear i fFogFar zachowane)
		uniforms.v4FogParams = D3DXVECTOR4(fNear, fFar, fInvRange, 0.0f);
		CHECK(uniforms.fFogEnable == doctest::Approx(0.0f));
		CHECK(uniforms.fFogNear == doctest::Approx(3500.0f));
		CHECK(uniforms.fFogFar == doctest::Approx(11500.0f));
	}

	TEST_CASE("FrameLighting - CFrameSnapshot Synchronization for Background and Character Lights")
	{
		CFrameSnapshot::Reset();

		TFrameSnapshot snap = CFrameSnapshot::Current();

		D3DLIGHT9 sunLight{};
		sunLight.Type = D3DLIGHT_DIRECTIONAL;
		sunLight.Direction = D3DXVECTOR3(0.0f, -1.0f, 0.0f);
		sunLight.Diffuse = D3DCOLORVALUE{ 1.0f, 0.95f, 0.9f, 1.0f };
		sunLight.Ambient = D3DCOLORVALUE{ 0.15f, 0.15f, 0.2f, 1.0f };

		D3DLIGHT9 chrLight{};
		chrLight.Type = D3DLIGHT_DIRECTIONAL;
		chrLight.Direction = D3DXVECTOR3(-0.3f, -0.7f, 0.5f);
		chrLight.Diffuse = D3DCOLORVALUE{ 0.8f, 0.8f, 0.8f, 1.0f };
		chrLight.Ambient = D3DCOLORVALUE{ 0.3f, 0.3f, 0.3f, 1.0f };

		snap.lights[0] = sunLight;
		snap.lights[1] = chrLight;
		snap.v4FogParams = D3DXVECTOR4(2000.0f, 8000.0f, 1.0f / 6000.0f, 1.0f);

		CFrameSnapshot::SetCurrent(snap);

		const TFrameSnapshot& cur = CFrameSnapshot::Current();
		CHECK(cur.lights[0].Type == D3DLIGHT_DIRECTIONAL);
		CHECK(cur.lights[0].Diffuse.r == doctest::Approx(1.0f));
		CHECK(cur.lights[1].Type == D3DLIGHT_DIRECTIONAL);
		CHECK(cur.lights[1].Direction.x == doctest::Approx(-0.3f));
		CHECK(cur.v4FogParams.w == doctest::Approx(1.0f));
	}

	TEST_CASE("FrameLighting - Backend Playback of Lights in BeginFrame")
	{
		auto pMockDev = new MockD3D9DeviceForLighting();
		auto pStateMgr = std::make_unique<CStateManager>(pMockDev);

		// Przygotuj uniforms
		TRenderFrameUniforms uniforms{};
		D3DXMatrixIdentity(&uniforms.matView);
		D3DXMatrixIdentity(&uniforms.matProj);
		D3DXMatrixIdentity(&uniforms.matViewProj);

		uniforms.vSunLightDir = D3DXVECTOR3(0.5f, -0.5f, 0.707f);
		uniforms.vSunLightColor = D3DXCOLOR(0.95f, 0.9f, 0.85f, 1.0f);
		uniforms.vAmbientColor = D3DXCOLOR(0.2f, 0.2f, 0.25f, 1.0f);
		uniforms.fFogEnable = 0.0f; // mgla wylaczona w tym tescie

		// Przygotuj swiatlo postaci w snapshot
		TFrameSnapshot snap{};
		D3DXMatrixIdentity(&snap.matView);
		D3DXMatrixIdentity(&snap.matProj);
		D3DXMatrixIdentity(&snap.matViewProj);
		snap.lights[1].Type = D3DLIGHT_DIRECTIONAL;
		snap.lights[1].Direction = D3DXVECTOR3(0.0f, 0.0f, 1.0f);
		snap.lights[1].Diffuse = D3DCOLORVALUE{ 0.7f, 0.75f, 0.8f, 1.0f };
		snap.lights[1].Ambient = D3DCOLORVALUE{ 0.1f, 0.1f, 0.1f, 1.0f };
		CFrameSnapshot::SetCurrent(snap);

		// Wywolaj centralne BeginFrame
		CD3D9RenderBackend::Instance().BeginFrame(uniforms);

		// 1. Sprawdz stan oswietlenia D3DRS_LIGHTING
		DWORD dwLighting = 0;
		STATEMANAGER.GetRenderState(D3DRS_LIGHTING, &dwLighting);
		CHECK(dwLighting == TRUE);

		// 2. Sprawdz ambient D3DRS_AMBIENT
		DWORD dwAmbient = 0;
		STATEMANAGER.GetRenderState(D3DRS_AMBIENT, &dwAmbient);
		D3DCOLOR expectedAmbient = D3DCOLOR_COLORVALUE(0.2f, 0.2f, 0.25f, 1.0f);
		CHECK(dwAmbient == expectedAmbient);

		// 3. Sprawdz Light 0 (Sun Light)
		D3DLIGHT9 light0{};
		STATEMANAGER.GetLight(0, &light0);
		CHECK(light0.Type == D3DLIGHT_DIRECTIONAL);
		CHECK(light0.Direction.x == doctest::Approx(0.5f));
		CHECK(light0.Direction.y == doctest::Approx(-0.5f));
		CHECK(light0.Direction.z == doctest::Approx(0.707f));
		CHECK(light0.Diffuse.r == doctest::Approx(0.95f));
		CHECK(light0.Ambient.b == doctest::Approx(0.25f));
		CHECK(STATEMANAGER.IsLightEnabled(0) == TRUE);

		// 4. Sprawdz Light 1 (Character Light ze snapshotu)
		D3DLIGHT9 light1{};
		STATEMANAGER.GetLight(1, &light1);
		CHECK(light1.Type == D3DLIGHT_DIRECTIONAL);
		CHECK(light1.Direction.z == doctest::Approx(1.0f));
		CHECK(light1.Diffuse.g == doctest::Approx(0.75f));
		CHECK(STATEMANAGER.IsLightEnabled(1) == TRUE);
	}

	TEST_CASE("FrameLighting - Backend Playback of Fog in BeginFrame (Active & Disabled)")
	{
		auto pMockDev = new MockD3D9DeviceForLighting();
		auto pStateMgr = std::make_unique<CStateManager>(pMockDev);

		// 1. Test aktywnej mgly
		TRenderFrameUniforms uniforms{};
		D3DXMatrixIdentity(&uniforms.matView);
		D3DXMatrixIdentity(&uniforms.matProj);
		D3DXMatrixIdentity(&uniforms.matViewProj);

		const float fNear = 2500.0f;
		const float fFar = 8500.0f;
		uniforms.fFogNear = fNear;
		uniforms.fFogFar = fFar;
		uniforms.fFogInvRange = 1.0f / (fFar - fNear);
		uniforms.fFogEnable = 1.0f;
		uniforms.vFogColor = D3DXCOLOR(0.6f, 0.65f, 0.7f, 1.0f);

		CD3D9RenderBackend::Instance().BeginFrame(uniforms);

		DWORD dwFogEnable = 0;
		STATEMANAGER.GetRenderState(D3DRS_FOGENABLE, &dwFogEnable);
		CHECK(dwFogEnable == TRUE);

		DWORD dwFogColor = 0;
		STATEMANAGER.GetRenderState(D3DRS_FOGCOLOR, &dwFogColor);
		D3DCOLOR expectedFogColor = D3DCOLOR_COLORVALUE(0.6f, 0.65f, 0.7f, 1.0f);
		CHECK(dwFogColor == expectedFogColor);

		DWORD dwFogVertexMode = 0;
		STATEMANAGER.GetRenderState(D3DRS_FOGVERTEXMODE, &dwFogVertexMode);
		CHECK(dwFogVertexMode == D3DFOG_LINEAR);

		DWORD dwRangeFog = 0;
		STATEMANAGER.GetRenderState(D3DRS_RANGEFOGENABLE, &dwRangeFog);
		CHECK(dwRangeFog == TRUE);

		DWORD dwFogStart = 0;
		STATEMANAGER.GetRenderState(D3DRS_FOGSTART, &dwFogStart);
		float fActualNear = *reinterpret_cast<float*>(&dwFogStart);
		CHECK(fActualNear == doctest::Approx(2500.0f));

		DWORD dwFogEnd = 0;
		STATEMANAGER.GetRenderState(D3DRS_FOGEND, &dwFogEnd);
		float fActualFar = *reinterpret_cast<float*>(&dwFogEnd);
		CHECK(fActualFar == doctest::Approx(8500.0f));

		// 2. Test wylaczenia mgly
		uniforms.fFogEnable = 0.0f;
		uniforms.v4FogParams.w = 0.0f;
		CFrameSnapshot::Reset();

		CD3D9RenderBackend::Instance().BeginFrame(uniforms);

		STATEMANAGER.GetRenderState(D3DRS_FOGENABLE, &dwFogEnable);
		CHECK(dwFogEnable == FALSE);
	}

	TEST_CASE("FrameLighting - StateManager Light & Viewport Decoupling")
	{
		auto pMockDev = new MockD3D9DeviceForLighting();
		auto pStateMgr = std::make_unique<CStateManager>(pMockDev);

		// Weryfikacja LightEnable i IsLightEnabled
		STATEMANAGER.LightEnable(2, TRUE);
		CHECK(STATEMANAGER.IsLightEnabled(2) == TRUE);
		CHECK(pMockDev->m_lightEnabled[2] == TRUE);

		STATEMANAGER.LightEnable(2, FALSE);
		CHECK(STATEMANAGER.IsLightEnabled(2) == FALSE);
		CHECK(pMockDev->m_lightEnabled[2] == FALSE);

		// Weryfikacja Viewport
		D3DVIEWPORT9 vp{ 10, 20, 1024, 768, 0.0f, 1.0f };
		STATEMANAGER.SetViewport(&vp);
		CHECK(pMockDev->m_viewport.X == 10);
		CHECK(pMockDev->m_viewport.Width == 1024);

		D3DVIEWPORT9 readBack{};
		STATEMANAGER.GetViewport(&readBack);
		CHECK(readBack.X == 10);
		CHECK(readBack.Y == 20);
		CHECK(readBack.Width == 1024);
		CHECK(readBack.Height == 768);

		// Weryfikacja GetDeviceCaps i GetAvailableTextureMem
		D3DCAPS9 caps{};
		HRESULT hr = STATEMANAGER.GetDeviceCaps(&caps);
		CHECK(SUCCEEDED(hr));
		CHECK((caps.Caps2 & D3DCAPS2_FULLSCREENGAMMA) != 0);

		DWORD dwTexMem = STATEMANAGER.GetAvailableTextureMem();
		CHECK(dwTexMem > 0);
	}
}
