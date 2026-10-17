#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"

#include <windows.h>
#include <d3d9.h>
#include <d3dx9.h>
#include <cstdint>
#include <cstring>
#include <vector>
#include <memory>

#include "EterLib/RenderPacketTypes.h"
#include "EterLib/RenderOwnership.h"
#include "EterLib/RenderQueueManager.h"
#include "EterLib/RenderResourceRegistry.h"
#include "EterLib/StateManager.h"
#include "EterLib/D3D9RenderBackend.h"
#include "EterLib/Render/ActorSkinningPipeline.h"
#include "GameLib/DungeonBlockRenderExtractor.h"
#include "GameLib/DungeonBlock.h"

// ============================================================================
// Mock IDirect3DDevice9Ex i mock tekstur na potrzeby testu parzystosci lochow
// ============================================================================
namespace
{
class TestMockVertexDecl final : public IDirect3DVertexDeclaration9
{
public:
	ULONG m_refCount{ 1 };

	HRESULT STDMETHODCALLTYPE QueryInterface(REFIID, void** ppvObj) override
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
	HRESULT STDMETHODCALLTYPE GetDevice(IDirect3DDevice9**) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE GetDeclaration(D3DVERTEXELEMENT9*, UINT*) override { return D3D_OK; }
};

class TestMockTexture final : public IDirect3DTexture9
{
public:
	ULONG m_refCount{ 1 };
	D3DSURFACE_DESC m_desc{};

	TestMockTexture(UINT w = 128, UINT h = 128, D3DFORMAT fmt = D3DFMT_A8R8G8B8)
	{
		std::memset(&m_desc, 0, sizeof(m_desc));
		m_desc.Width = w;
		m_desc.Height = h;
		m_desc.Format = fmt;
		m_desc.Type = D3DRTYPE_TEXTURE;
	}

	HRESULT STDMETHODCALLTYPE QueryInterface(REFIID, void** ppvObj) override
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
		if (!pDesc) return E_POINTER;
		*pDesc = m_desc;
		return D3D_OK;
	}
	HRESULT STDMETHODCALLTYPE GetSurfaceLevel(UINT, IDirect3DSurface9**) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE LockRect(UINT, D3DLOCKED_RECT*, CONST RECT*, DWORD) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE UnlockRect(UINT) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE AddDirtyRect(CONST RECT*) override { return D3D_OK; }
};

struct SDrawSnapshot
{
	DWORD colorOp1{ 0 };
	DWORD alphaOp1{ 0 };
	DWORD alphaBlendEnable{ 0 };
	DWORD srcBlend{ 0 };
	DWORD destBlend{ 0 };
	IDirect3DBaseTexture9* pTexture0{ nullptr };
	IDirect3DBaseTexture9* pTexture1{ nullptr };
	UINT stride{ 0 };
};

class TestMockDungeonDevice final : public IDirect3DDevice9Ex
{
public:
	ULONG m_refCount{ 1 };
	DWORD m_hwRenderStates[256]{};
	DWORD m_hwStageStates[8][64]{};
	IDirect3DBaseTexture9* m_hwTextures[8]{};
	UINT m_lastStreamSourceStride{ 0 };
	IDirect3DVertexDeclaration9* m_lastVertexDecl{ nullptr };
	uint32_t m_drawCalls{ 0 };
	std::vector<SDrawSnapshot> m_snapshots;

	TestMockDungeonDevice()
	{
		for (int i = 0; i < 256; ++i)
			m_hwRenderStates[i] = 0;
		for (int s = 0; s < 8; ++s)
		{
			for (int t = 0; t < 64; ++t)
				m_hwStageStates[s][t] = 0;
			m_hwTextures[s] = nullptr;
		}
	}

	HRESULT STDMETHODCALLTYPE QueryInterface(REFIID, void** ppvObj) override
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

	HRESULT STDMETHODCALLTYPE SetTexture(DWORD Stage, IDirect3DBaseTexture9* pTexture) override
	{
		if (Stage < 8)
			m_hwTextures[Stage] = pTexture;
		return D3D_OK;
	}
	HRESULT STDMETHODCALLTYPE GetTexture(DWORD Stage, IDirect3DBaseTexture9** ppTexture) override
	{
		if (ppTexture && Stage < 8)
			*ppTexture = m_hwTextures[Stage];
		return D3D_OK;
	}

	HRESULT STDMETHODCALLTYPE SetTextureStageState(DWORD Stage, D3DTEXTURESTAGESTATETYPE Type, DWORD Value) override
	{
		if (Stage < 8 && Type < 64)
			m_hwStageStates[Stage][Type] = Value;
		return D3D_OK;
	}
	HRESULT STDMETHODCALLTYPE GetTextureStageState(DWORD Stage, D3DTEXTURESTAGESTATETYPE Type, DWORD* pValue) override
	{
		if (pValue && Stage < 8 && Type < 64)
			*pValue = m_hwStageStates[Stage][Type];
		return D3D_OK;
	}

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

	HRESULT STDMETHODCALLTYPE SetStreamSource(UINT, IDirect3DVertexBuffer9*, UINT, UINT Stride) override
	{
		m_lastStreamSourceStride = Stride;
		return D3D_OK;
	}
	HRESULT STDMETHODCALLTYPE SetIndices(IDirect3DIndexBuffer9*) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE BeginScene() override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE EndScene() override { return D3D_OK; }

	void RecordSnapshot(UINT Stride)
	{
		SDrawSnapshot snap;
		snap.colorOp1 = m_hwStageStates[1][D3DTSS_COLOROP];
		snap.alphaOp1 = m_hwStageStates[1][D3DTSS_ALPHAOP];
		snap.alphaBlendEnable = m_hwRenderStates[D3DRS_ALPHABLENDENABLE];
		snap.srcBlend = m_hwRenderStates[D3DRS_SRCBLEND];
		snap.destBlend = m_hwRenderStates[D3DRS_DESTBLEND];
		snap.pTexture0 = m_hwTextures[0];
		snap.pTexture1 = m_hwTextures[1];
		snap.stride = Stride;
		m_snapshots.push_back(snap);
	}

	HRESULT STDMETHODCALLTYPE DrawPrimitive(D3DPRIMITIVETYPE, UINT, UINT) override
	{
		m_drawCalls++;
		RecordSnapshot(m_lastStreamSourceStride);
		return D3D_OK;
	}
	HRESULT STDMETHODCALLTYPE DrawPrimitiveUP(D3DPRIMITIVETYPE, UINT, CONST void*, UINT Stride) override
	{
		m_lastStreamSourceStride = Stride;
		m_drawCalls++;
		RecordSnapshot(Stride);
		return D3D_OK;
	}
	HRESULT STDMETHODCALLTYPE DrawIndexedPrimitive(D3DPRIMITIVETYPE, INT, UINT, UINT, UINT, UINT) override
	{
		m_drawCalls++;
		RecordSnapshot(m_lastStreamSourceStride);
		return D3D_OK;
	}
	HRESULT STDMETHODCALLTYPE DrawIndexedPrimitiveUP(D3DPRIMITIVETYPE, UINT, UINT, UINT, CONST void*, D3DFORMAT, CONST void*, UINT Stride) override
	{
		m_lastStreamSourceStride = Stride;
		m_drawCalls++;
		RecordSnapshot(Stride);
		return D3D_OK;
	}

	HRESULT STDMETHODCALLTYPE SetVertexDeclaration(IDirect3DVertexDeclaration9* pDecl) override
	{
		m_lastVertexDecl = pDecl;
		return D3D_OK;
	}
	HRESULT STDMETHODCALLTYPE GetVertexDeclaration(IDirect3DVertexDeclaration9** pp) override
	{
		if (pp) *pp = m_lastVertexDecl;
		return D3D_OK;
	}
	HRESULT STDMETHODCALLTYPE SetFVF(DWORD) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE GetFVF(DWORD* p) override { if (p) *p = 0; return D3D_OK; }
	HRESULT STDMETHODCALLTYPE SetVertexShader(IDirect3DVertexShader9*) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE GetVertexShader(IDirect3DVertexShader9** pp) override { if (pp) *pp = nullptr; return D3D_OK; }
	HRESULT STDMETHODCALLTYPE SetPixelShader(IDirect3DPixelShader9*) override { return D3D_OK; }
	HRESULT STDMETHODCALLTYPE GetPixelShader(IDirect3DPixelShader9** pp) override { if (pp) *pp = nullptr; return D3D_OK; }

	// Stubs
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
	HRESULT STDMETHODCALLTYPE CreateVertexDeclaration(CONST D3DVERTEXELEMENT9*, IDirect3DVertexDeclaration9** pp) override
	{
		if (pp) *pp = new TestMockVertexDecl();
		return D3D_OK;
	}
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
} // namespace

// ============================================================================
// TEST 1: TDungeonBlockPacket - ABI, rozmiar, wyrownanie i offsety pol
// ============================================================================
TEST_CASE("S6.14 - TDungeonBlockPacket ABI and Layout")
{
	CHECK(sizeof(TDungeonBlockPacket) == 160u);
	CHECK(alignof(TDungeonBlockPacket) == 16u);

	CHECK(offsetof(TDungeonBlockPacket, matWorld) == 0u);
	CHECK(offsetof(TDungeonBlockPacket, dwDiffuseTextureID) == 64u);
	CHECK(offsetof(TDungeonBlockPacket, dwLightmapTextureID) == 72u);
	CHECK(offsetof(TDungeonBlockPacket, pDiffuseTexture) == 80u);
	CHECK(offsetof(TDungeonBlockPacket, pLightmapTexture) == 88u);
	CHECK(offsetof(TDungeonBlockPacket, pVertexBuffer) == 96u);
	CHECK(offsetof(TDungeonBlockPacket, pIndexBuffer) == 104u);
	CHECK(offsetof(TDungeonBlockPacket, pVertices) == 112u);
	CHECK(offsetof(TDungeonBlockPacket, pIndices) == 120u);
	CHECK(offsetof(TDungeonBlockPacket, dwVertexCount) == 128u);
	CHECK(offsetof(TDungeonBlockPacket, dwIndexCount) == 132u);
	CHECK(offsetof(TDungeonBlockPacket, dwPrimitiveCount) == 136u);
	CHECK(offsetof(TDungeonBlockPacket, dwStartIndex) == 140u);
	CHECK(offsetof(TDungeonBlockPacket, nBaseVertexIndex) == 144u);
	CHECK(offsetof(TDungeonBlockPacket, fAlpha) == 148u);
	CHECK(offsetof(TDungeonBlockPacket, bAlphaBlend) == 152u);
	CHECK(offsetof(TDungeonBlockPacket, bPadDungeon) == 153u);
	CHECK(offsetof(TDungeonBlockPacket, fPad) == 156u);

	// Domyslne inicjalizacje w konstruktorze C++
	TDungeonBlockPacket pkt;
	CHECK(pkt.matWorld._11 == 1.0f);
	CHECK(pkt.matWorld._22 == 1.0f);
	CHECK(pkt.matWorld._33 == 1.0f);
	CHECK(pkt.matWorld._44 == 1.0f);
	CHECK(pkt.fAlpha == 1.0f);
	CHECK(pkt.bAlphaBlend == 0u);
	CHECK(pkt.bPadDungeon[0] == 0u);
}

// ============================================================================
// TEST 2: Single Owner (NB.1) - RenderOwnership i bramka LegacyDraws
// ============================================================================
TEST_CASE("S6.14 - Single Owner (NB.1) and LegacyDraws Gating")
{
	RenderOwnership::ResetDefaults();

	// Domyslnie Dungeon jest pod kontrola potoku D3D9 lub Legacy w zaleznosci od builda
	// Ustawienie D3D9Backend jako jedynego wlasciciela
	RenderOwnership::SetOwner(ERenderCategory::Dungeon, ERenderOwner::D3D9Backend);
	CHECK(RenderOwnership::Owner(ERenderCategory::Dungeon) == ERenderOwner::D3D9Backend);
	CHECK(RenderOwnership::LegacyDraws(ERenderCategory::Dungeon) == false);

	// Przypisanie Legacy jako wlasciciela
	RenderOwnership::SetOwner(ERenderCategory::Dungeon, ERenderOwner::Legacy);
	CHECK(RenderOwnership::Owner(ERenderCategory::Dungeon) == ERenderOwner::Legacy);
	CHECK(RenderOwnership::LegacyDraws(ERenderCategory::Dungeon) == true);

	RenderOwnership::ResetDefaults();
}

// ============================================================================
// TEST 3: DEC-3 Lighting Parity - D3DTOP_MODULATE vs D3DTOP_DISABLE w Stage 1
// ============================================================================
TEST_CASE("S6.14 - DEC-3 Lighting Parity and Texture Stages")
{
	auto* mockDev = new TestMockDungeonDevice();
	auto stateMgr = std::make_unique<CStateManager>(mockDev);

	RenderOwnership::SetOwner(ERenderCategory::Dungeon, ERenderOwner::D3D9Backend);

	TestMockTexture mockDiffTex(64, 64);
	TestMockTexture mockLightmapTex(64, 64);

	TPNT2Vertex sampleVertices[3]{};
	uint16_t sampleIndices[3] = { 0, 1, 2 };

	// 1. Pakiet z obecna lightmapa: Stage 1 MUSI byc w trybie D3DTOP_MODULATE w momencie rysowania (zgodnie z DEC-3 i Area.cpp)
	{
		TDungeonBlockPacket pkt;
		pkt.pDiffuseTexture = &mockDiffTex;
		pkt.pLightmapTexture = &mockLightmapTex;
		pkt.pVertices = sampleVertices;
		pkt.pIndices = sampleIndices;
		pkt.dwVertexCount = 3;
		pkt.dwIndexCount = 3;
		pkt.dwPrimitiveCount = 1;
		pkt.bAlphaBlend = 0;

		CD3D9RenderBackend::Instance().RenderDungeonBlocks(&pkt, 1);

		REQUIRE(!mockDev->m_snapshots.empty());
		const auto& snap = mockDev->m_snapshots.back();
		CHECK(snap.colorOp1 == D3DTOP_MODULATE);
		CHECK(snap.alphaOp1 == D3DTOP_MODULATE);
		CHECK(snap.alphaBlendEnable == FALSE);

		// Weryfikacja, ze po zakonczeniu renderowania stan Stage 1 zostal zresetowany do DISABLE
		CHECK(mockDev->m_hwStageStates[1][D3DTSS_COLOROP] == D3DTOP_DISABLE);
		CHECK(mockDev->m_hwStageStates[1][D3DTSS_ALPHAOP] == D3DTOP_DISABLE);
	}

	// 2. Pakiet bez lightmapy (pLightmapTexture == nullptr): Stage 1 MUSI byc wylaczony (D3DTOP_DISABLE) podczas rysowania
	{
		TDungeonBlockPacket pktNoLm;
		pktNoLm.pDiffuseTexture = &mockDiffTex;
		pktNoLm.pLightmapTexture = nullptr;
		pktNoLm.pVertices = sampleVertices;
		pktNoLm.pIndices = sampleIndices;
		pktNoLm.dwVertexCount = 3;
		pktNoLm.dwIndexCount = 3;
		pktNoLm.dwPrimitiveCount = 1;
		pktNoLm.bAlphaBlend = 0;

		CD3D9RenderBackend::Instance().RenderDungeonBlocks(&pktNoLm, 1);

		REQUIRE(!mockDev->m_snapshots.empty());
		const auto& snap = mockDev->m_snapshots.back();
		CHECK(snap.colorOp1 == D3DTOP_DISABLE);
		CHECK(snap.alphaOp1 == D3DTOP_DISABLE);
	}

	RenderOwnership::ResetDefaults();
}

// ============================================================================
// TEST 4: bAlphaBlend - Dynamiczne przelaczanie przezroczystosci scian
// ============================================================================
TEST_CASE("S6.14 - bAlphaBlend State Switching")
{
	auto* mockDev = new TestMockDungeonDevice();
	auto stateMgr = std::make_unique<CStateManager>(mockDev);

	RenderOwnership::SetOwner(ERenderCategory::Dungeon, ERenderOwner::D3D9Backend);

	TestMockTexture mockDiffTex(64, 64);
	TPNT2Vertex sampleVertices[3]{};
	uint16_t sampleIndices[3] = { 0, 1, 2 };

	// 1. Pakiet przezroczysty (bAlphaBlend = 1)
	TDungeonBlockPacket pktBlend;
	pktBlend.pDiffuseTexture = &mockDiffTex;
	pktBlend.pLightmapTexture = nullptr;
	pktBlend.pVertices = sampleVertices;
	pktBlend.pIndices = sampleIndices;
	pktBlend.dwVertexCount = 3;
	pktBlend.dwIndexCount = 3;
	pktBlend.dwPrimitiveCount = 1;
	pktBlend.bAlphaBlend = 1;

	CD3D9RenderBackend::Instance().RenderDungeonBlocks(&pktBlend, 1);

	REQUIRE(!mockDev->m_snapshots.empty());
	CHECK(mockDev->m_snapshots.back().alphaBlendEnable == TRUE);
	CHECK(mockDev->m_snapshots.back().srcBlend == D3DBLEND_SRCALPHA);
	CHECK(mockDev->m_snapshots.back().destBlend == D3DBLEND_INVSRCALPHA);

	// 2. Nastepny pakiet nieprzezroczysty (bAlphaBlend = 0)
	TDungeonBlockPacket pktOpaque = pktBlend;
	pktOpaque.bAlphaBlend = 0;

	CD3D9RenderBackend::Instance().RenderDungeonBlocks(&pktOpaque, 1);

	REQUIRE(mockDev->m_snapshots.size() >= 2);
	CHECK(mockDev->m_snapshots.back().alphaBlendEnable == FALSE);

	RenderOwnership::ResetDefaults();
}

// ============================================================================
// TEST 5: RenderResourceRegistry i SafeResolveTexture w Dungeon Blocks
// ============================================================================
TEST_CASE("S6.14 - SafeResolveTexture Handle Resolution")
{
	auto* mockDev = new TestMockDungeonDevice();
	auto stateMgr = std::make_unique<CStateManager>(mockDev);
	auto registry = std::make_unique<CRenderResourceRegistry>();

	RenderOwnership::SetOwner(ERenderCategory::Dungeon, ERenderOwner::D3D9Backend);

	TestMockTexture rawDiffTex(128, 128);
	TestMockTexture rawLmTex(128, 128);

	TRenderTextureHandle hDiff = CRenderResourceRegistry::Instance().RegisterTexture(&rawDiffTex, "dungeon_wall");
	TRenderTextureHandle hLm = CRenderResourceRegistry::Instance().RegisterTexture(&rawLmTex, "dungeon_lm");

	REQUIRE(hDiff != kInvalidTextureHandle);
	REQUIRE(hLm != kInvalidTextureHandle);

	TPNT2Vertex sampleVertices[3]{};
	uint16_t sampleIndices[3] = { 0, 1, 2 };

	TDungeonBlockPacket pkt;
	pkt.dwDiffuseTextureID = hDiff;
	pkt.dwLightmapTextureID = hLm;
	pkt.pDiffuseTexture = nullptr;
	pkt.pLightmapTexture = nullptr;
	pkt.pVertices = sampleVertices;
	pkt.pIndices = sampleIndices;
	pkt.dwVertexCount = 3;
	pkt.dwIndexCount = 3;
	pkt.dwPrimitiveCount = 1;

	CD3D9RenderBackend::Instance().RenderDungeonBlocks(&pkt, 1);

	// Backend musial bezpiecznie rozwiazac uchwyty do wlasciwych wskaznikow tekstury D3D9 podczas rysowania
	REQUIRE(!mockDev->m_snapshots.empty());
	CHECK(mockDev->m_snapshots.back().pTexture0 == &rawDiffTex);
	CHECK(mockDev->m_snapshots.back().pTexture1 == &rawLmTex);

	RenderOwnership::ResetDefaults();
}

// ============================================================================
// TEST 6: glTF Stride i Parity z modelem TPNT2Vertex / GpuSkinnedVertex
// ============================================================================
TEST_CASE("S6.14 - glTF Stride and Vertex Declaration Parity")
{
	auto* mockDev = new TestMockDungeonDevice();
	auto stateMgr = std::make_unique<CStateManager>(mockDev);

	RenderOwnership::SetOwner(ERenderCategory::Dungeon, ERenderOwner::D3D9Backend);

	// 1. Granny TPNT2Vertex (40 bajtow)
	{
		TPNT2Vertex verts[3]{};
		uint16_t indices[3] = { 0, 1, 2 };

		TDungeonBlockPacket grannyPkt;
		grannyPkt.pVertices = verts;
		grannyPkt.pIndices = indices;
		grannyPkt.dwVertexCount = 3;
		grannyPkt.dwIndexCount = 3;
		grannyPkt.dwPrimitiveCount = 1;
		grannyPkt.bPadDungeon[0] = 0; // domyslny stride 40B

		CD3D9RenderBackend::Instance().RenderDungeonBlocks(&grannyPkt, 1);
		REQUIRE(!mockDev->m_snapshots.empty());
		CHECK(mockDev->m_snapshots.back().stride == sizeof(TPNT2Vertex));
		CHECK(mockDev->m_snapshots.back().stride == 40u);
	}

	// 2. glTF GpuSkinnedVertex (72 bajty)
	{
		EterLib::Render::GpuSkinnedVertex gltfVerts[3]{};
		uint16_t indices[3] = { 0, 1, 2 };

		TDungeonBlockPacket gltfPkt;
		gltfPkt.pVertices = reinterpret_cast<const TPNT2Vertex*>(gltfVerts);
		gltfPkt.pIndices = indices;
		gltfPkt.dwVertexCount = 3;
		gltfPkt.dwIndexCount = 3;
		gltfPkt.dwPrimitiveCount = 1;
		gltfPkt.bPadDungeon[0] = static_cast<uint8_t>(sizeof(EterLib::Render::GpuSkinnedVertex)); // 72B

		CD3D9RenderBackend::Instance().RenderDungeonBlocks(&gltfPkt, 1);
		REQUIRE(!mockDev->m_snapshots.empty());
		CHECK(mockDev->m_snapshots.back().stride == sizeof(EterLib::Render::GpuSkinnedVertex));
		CHECK(mockDev->m_snapshots.back().stride == 72u);
	}

	RenderOwnership::ResetDefaults();
}

// ============================================================================
// TEST 7: CDungeonBlock - isShow() Early Exit Guard
// ============================================================================
TEST_CASE("S6.14 - CDungeonBlock isShow() Early Exit")
{
	CDungeonBlock block;
	CHECK(block.isShow() == true);

	// Ukrycie bloku
	block.Hide();
	CHECK(block.isShow() == false);

	// Gdy blok jest ukryty, Render() i OnRenderShadow() powinny bezpiecznie zakonczyc dzialanie bez operacji
	block.Render();
	block.OnRenderShadow();

	// Przywrocenie widocznosci
	block.Show();
	CHECK(block.isShow() == true);
}
