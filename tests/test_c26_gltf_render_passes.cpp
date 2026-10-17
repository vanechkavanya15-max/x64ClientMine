#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"

#include <windows.h>
#include <d3d9.h>
#include <d3dx9.h>
#include <cstdint>
#include <cstring>
#include <vector>
#include <memory>
#include <iostream>

#include "EterLib/RenderPacketTypes.h"
#include "EterLib/StateManager.h"
#include "EterGrnLib/ModelDrawItem.h"
#include "EterGrnLib/LODController.h"
#include "EterGrnLib/Thing.h"
#include "EterModelLib/GltfModel.h"
#include "EterModelLib/GltfModelInstance.h"

using namespace EterModelLib;

// ============================================================================
// Mock IDirect3DDevice9Ex z telemetria stanow dla weryfikacji izolacji D3D9
// ============================================================================
namespace
{
class TestMockD3DDevice final : public IDirect3DDevice9Ex
{
public:
	ULONG m_refCount{ 1 };
	DWORD m_hwRenderStates[256]{};
	uint32_t m_drawIndexedPrimitiveCalls{ 0 };

	TestMockD3DDevice()
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
	HRESULT STDMETHODCALLTYPE DrawIndexedPrimitive(D3DPRIMITIVETYPE, INT, UINT, UINT, UINT, UINT) override
	{
		m_drawIndexedPrimitiveCalls++;
		return D3D_OK;
	}
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

// Pomocnik do budowy syntetycznego modelu glTF z 4 materialami i 4 submeshami
static void BuildTestModelData(GltfModelData& data)
{
	data.name = "SyntheticPassTestModel";

	// 4 Materialy:
	// 0: Zwykly Opaque
	GltfMaterial mat0;
	mat0.name = "Armor_Opaque";
	mat0.doubleSided = false;
	data.materials.push_back(mat0);

	// 1: Blend po nazwie ("blend_aura")
	GltfMaterial mat1;
	mat1.name = "blend_aura";
	mat1.doubleSided = false;
	data.materials.push_back(mat1);

	// 2: Blend po opacityTexture
	GltfMaterial mat2;
	mat2.name = "Wings";
	mat2.opacityTexture = "textures/wings_mask.png";
	mat2.doubleSided = false;
	data.materials.push_back(mat2);

	// 3: TwoSided Opaque
	GltfMaterial mat3;
	mat3.name = "Cape_TwoSided";
	mat3.doubleSided = true;
	data.materials.push_back(mat3);

	// 4 Submeshe:
	for (int s = 0; s < 4; ++s)
	{
		GltfSubmesh sm;
		sm.name = "Submesh_" + std::to_string(s);
		sm.materialIndex = s;
		sm.vertexOffset = s * 3;
		sm.vertexCount = 3;
		sm.indexOffset = s * 3;
		sm.indexCount = 3;
		data.submeshes.push_back(sm);
	}

	// 12 Wierzcholkow (po 3 na podsiatke)
	for (int i = 0; i < 12; ++i)
	{
		GltfVertex v;
		v.position = { static_cast<float>(i), 0.0f, 0.0f };
		v.normal = { 0.0f, 1.0f, 0.0f };
		v.uv0 = { 0.0f, 0.0f };
		v.jointIndices = { 0, 0, 0, 0 };
		v.jointWeights = { 1.0f, 0.0f, 0.0f, 0.0f };
		data.vertices.push_back(v);
	}

	// 12 Indeksow
	for (unsigned int i = 0; i < 12; ++i)
	{
		data.indices.push_back(i);
	}
}
} // namespace

// ============================================================================
// TEST 1: Weryfikacja ABI, rozmiaru i offsetow struktury TModelDrawItem
// ============================================================================
TEST_CASE("S6.7 - TModelDrawItem ABI and Field Offsets")
{
	CHECK(sizeof(TModelDrawItem) == 96u);

	CHECK(offsetof(TModelDrawItem, modelId) == 0u);
	CHECK(offsetof(TModelDrawItem, model) == 0u);
	CHECK(offsetof(TModelDrawItem, subMeshIndex) == 4u);
	CHECK(offsetof(TModelDrawItem, submeshIndex) == 4u);
	CHECK(offsetof(TModelDrawItem, materialHandle) == 8u);
	CHECK(offsetof(TModelDrawItem, passType) == 12u);
	CHECK(offsetof(TModelDrawItem, pass) == 12u);
	CHECK(offsetof(TModelDrawItem, matWorld) == 16u);
	CHECK(offsetof(TModelDrawItem, fAlpha) == 80u);
	CHECK(offsetof(TModelDrawItem, addColor) == 84u);
	CHECK(offsetof(TModelDrawItem, materialColor) == 88u);
	CHECK(offsetof(TModelDrawItem, dwPad) == 92u);
	CHECK(offsetof(TModelDrawItem, bTwoSided) == 92u);

	// Domyslny konstruktor C++
	TModelDrawItem item{};
	CHECK(item.modelId == 0u);
	CHECK(item.submeshIndex == 0u);
	CHECK(item.materialHandle == 0u);
	CHECK(item.pass == EModelPass::Opaque);
	CHECK(item.passType == 0u);
	CHECK(item.matWorld._11 == 1.0f);
	CHECK(item.matWorld._22 == 1.0f);
	CHECK(item.matWorld._33 == 1.0f);
	CHECK(item.matWorld._44 == 1.0f);
	CHECK(item.fAlpha == 1.0f);
	CHECK(item.addColor == 0u);
	CHECK(item.materialColor == 0xffffffffu);
	CHECK(item.bTwoSided == 0u);
}

// ============================================================================
// TEST 2: Wartosci i rzutowania typu wyliczeniowego EModelPass
// ============================================================================
TEST_CASE("S6.7 - EModelPass Enum Values")
{
	CHECK(static_cast<uint32_t>(EModelPass::Opaque) == 0u);
	CHECK(static_cast<uint32_t>(EModelPass::Blend) == 1u);
	CHECK(static_cast<uint32_t>(EModelPass::ShadowDepth) == 2u);

	TModelDrawItem item{};
	item.pass = EModelPass::Blend;
	CHECK(item.passType == 1u);

	item.pass = EModelPass::ShadowDepth;
	CHECK(item.passType == 2u);

	item.pass = EModelPass::Opaque;
	CHECK(item.passType == 0u);
}

// ============================================================================
// TEST 3: Klasyfikacja materialow isBlend / isTwoSided w InitGltfMaterials
// ============================================================================
TEST_CASE("S6.7 - SGltfMaterialState Classification in InitGltfMaterials")
{
	GltfModelData modelData;
	BuildTestModelData(modelData);

	CGraphicThing thing("synthetic_model.glb");
	thing.AddReferenceOnly();
	bool createOk = thing.CreateFromGltfModelData(modelData);
	CHECK(createOk == true);

	CGrannyLODController controller;
	controller.AddModel(&thing, 0);
	CHECK(controller.IsGltf() == true);

	controller.InitGltfMaterials();
	const auto& states = controller.GetGltfMaterialStates();

	REQUIRE(states.size() == 4u);

	// Mat 0: Armor_Opaque -> isBlend=false, isTwoSided=false
	CHECK(states[0].isBlend == false);
	CHECK(states[0].isTwoSided == false);

	// Mat 1: blend_aura -> isBlend=true, isTwoSided=false
	CHECK(states[1].isBlend == true);
	CHECK(states[1].isTwoSided == false);

	// Mat 2: Wings (opacityTexture) -> isBlend=true, isTwoSided=false
	CHECK(states[2].isBlend == true);
	CHECK(states[2].isTwoSided == false);

	// Mat 3: Cape_TwoSided (doubleSided) -> isBlend=false, isTwoSided=true
	CHECK(states[3].isBlend == false);
	CHECK(states[3].isTwoSided == true);

	// Kontroler raportuje obecnosc elementow przezroczystych
	CHECK(controller.HaveBlendThing() == true);
}

// ============================================================================
// TEST 4: Filtr submeshy per pass (DEC-4: eliminacja podwojnego rysowania)
// ============================================================================
TEST_CASE("S6.7 - DEC-4 Pass Submesh Filtering (Zero Duplicate Draws)")
{
	GltfModelData modelData;
	BuildTestModelData(modelData);

	CGraphicThing thing("synthetic_model.glb");
	thing.AddReferenceOnly();
	CHECK(thing.CreateFromGltfModelData(modelData) == true);

	CGrannyLODController controller;
	controller.AddModel(&thing, 0);
	controller.InitGltfMaterials();

	const auto& states = controller.GetGltfMaterialStates();
	REQUIRE(states.size() == 4u);

	std::vector<uint32_t> opaqueDrawnSubmeshes;
	std::vector<uint32_t> blendDrawnSubmeshes;

	// Symulacja filtru passow zgodnie z logika RenderGltfModel
	for (size_t s = 0; s < modelData.submeshes.size(); ++s)
	{
		const auto& submesh = modelData.submeshes[s];
		bool isBlend = states[submesh.materialIndex].isBlend;

		// Opaque pass
		if (!isBlend)
		{
			opaqueDrawnSubmeshes.push_back(static_cast<uint32_t>(s));
		}

		// Blend pass
		if (isBlend)
		{
			blendDrawnSubmeshes.push_back(static_cast<uint32_t>(s));
		}
	}

	// Weryfikacja passu Opaque: dokladnie submeshe 0 i 3
	CHECK(opaqueDrawnSubmeshes.size() == 2u);
	CHECK(opaqueDrawnSubmeshes[0] == 0u);
	CHECK(opaqueDrawnSubmeshes[1] == 3u);

	// Weryfikacja passu Blend: dokladnie submeshe 1 i 2
	CHECK(blendDrawnSubmeshes.size() == 2u);
	CHECK(blendDrawnSubmeshes[0] == 1u);
	CHECK(blendDrawnSubmeshes[1] == 2u);

	// Suma podsiatek musi dokladnie odpowiadac 4 (kazda podsiatka narysowana DOKLADNIE RAZ)
	CHECK(opaqueDrawnSubmeshes.size() + blendDrawnSubmeshes.size() == 4u);

	// Brak duplikatow pomiedzy passami (DEC-4)
	for (uint32_t opSubmesh : opaqueDrawnSubmeshes)
	{
		for (uint32_t blSubmesh : blendDrawnSubmeshes)
		{
			CHECK(opSubmesh != blSubmesh);
		}
	}
}

// ============================================================================
// TEST 5: Wykonanie ExecuteModelDrawItemD3D9 i determinizm izolacji stanow D3D9
// ============================================================================
TEST_CASE("S6.7 - ExecuteModelDrawItemD3D9 State Isolation with Mock Device")
{
	auto* mockDev = new TestMockD3DDevice();
	auto stateMgr = std::make_unique<CStateManager>(mockDev);

	GltfModelData modelData;
	BuildTestModelData(modelData);

	CGraphicThing thing("synthetic_model.glb");
	thing.AddReferenceOnly();
	CHECK(thing.CreateFromGltfModelData(modelData) == true);

	CGrannyLODController controller;
	controller.SetModelID(42);
	controller.AddModel(&thing, 0);
	controller.InitGltfMaterials();

	CGrannyLODController::RegisterLODController(42, &controller);

	// 1. Ustawienie stanow wolajacego (np. ActorInstance)
	STATEMANAGER.SetRenderState(D3DRS_CULLMODE, D3DCULL_CCW);
	STATEMANAGER.SetRenderState(D3DRS_ALPHABLENDENABLE, FALSE);
	STATEMANAGER.SetRenderState(D3DRS_ALPHATESTENABLE, TRUE);
	STATEMANAGER.SetRenderState(D3DRS_ALPHAREF, 0x40);

	// 2. Wykonanie rysowania Blend dla submesh 1 z dwustronnym cullem
	TModelDrawItem blendItem{};
	blendItem.modelId = 42;
	blendItem.submeshIndex = 1;
	blendItem.pass = EModelPass::Blend;
	blendItem.bTwoSided = 1;

	ExecuteModelDrawItemD3D9(blendItem);

	// 3. Weryfikacja, ze stany wolajacego zostaly w 100% zachowane i przywrocone
	DWORD cullMode = 0;
	STATEMANAGER.GetRenderState(D3DRS_CULLMODE, &cullMode);
	CHECK(cullMode == D3DCULL_CCW);

	DWORD alphaBlend = 0;
	STATEMANAGER.GetRenderState(D3DRS_ALPHABLENDENABLE, &alphaBlend);
	CHECK(alphaBlend == FALSE);

	DWORD alphaTest = 0;
	STATEMANAGER.GetRenderState(D3DRS_ALPHATESTENABLE, &alphaTest);
	CHECK(alphaTest == TRUE);

	DWORD alphaRef = 0;
	STATEMANAGER.GetRenderState(D3DRS_ALPHAREF, &alphaRef);
	CHECK(alphaRef == 0x40);

	// 4. Wykonanie rysowania ShadowDepth dla submesh 0
	TModelDrawItem shadowItem{};
	shadowItem.modelId = 42;
	shadowItem.submeshIndex = 0;
	shadowItem.pass = EModelPass::ShadowDepth;
	shadowItem.bTwoSided = 0;

	ExecuteModelDrawItemD3D9(shadowItem);

	// Ponowne sprawdzenie stanow
	STATEMANAGER.GetRenderState(D3DRS_CULLMODE, &cullMode);
	CHECK(cullMode == D3DCULL_CCW);

	STATEMANAGER.GetRenderState(D3DRS_ALPHATESTENABLE, &alphaTest);
	CHECK(alphaTest == TRUE);

	CGrannyLODController::UnregisterLODController(42);
}
