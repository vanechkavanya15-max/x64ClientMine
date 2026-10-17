#include "StdAfx.h"
#include "GrpDevice.h"
#include "EterBase/Stl.h"
#include "EterBase/Debug.h"

#include <intrin.h>
#include <utf8.h>

bool CPU_HAS_SSE2 = false;
bool GRAPHICS_CAPS_CAN_NOT_DRAW_LINE = false;
bool GRAPHICS_CAPS_CAN_NOT_DRAW_SHADOW = false;
bool GRAPHICS_CAPS_HALF_SIZE_IMAGE = false;
bool GRAPHICS_CAPS_CAN_NOT_TEXTURE_ADDRESS_BORDER = false;
bool GRAPHICS_CAPS_SOFTWARE_TILING = false;

CGraphicDevice::CGraphicDevice()
: m_uBackBufferCount(0)
{
	__Initialize();
}

CGraphicDevice::~CGraphicDevice()
{
	Destroy();
}

void CGraphicDevice::__Initialize()
{
	ms_lpd3d			= NULL;
	ms_lpd3dDevice		= NULL;
	ms_lpd3dMatStack	= NULL;

	ms_dwWavingEndTime = 0;

	m_pStateManager		= NULL;

	__InitializeDefaultIndexBufferList();
	__InitializePDTVertexBufferList();
}

void CGraphicDevice::RegisterWarningString(UINT uiMsg, const char * c_szString)
{
	m_kMap_strWarningMessage[uiMsg] = c_szString;
}

void CGraphicDevice::__WarningMessage(HWND hWnd, UINT uiMsg)
{
	auto it = m_kMap_strWarningMessage.find(uiMsg);
	if (it == m_kMap_strWarningMessage.end())
		return;

	const std::string& msgUtf8 = it->second;

	std::wstring wMsg = Utf8ToWide(msgUtf8);
	std::wstring wCaption = L"Warning"; // static wide literal is fine

	MessageBoxW(hWnd, wMsg.c_str(), wCaption.c_str(), MB_OK | MB_TOPMOST);
}

void CGraphicDevice::__ReleaseDefaultResources()
{
	if (CStateManager::InstancePtr())
	{
		STATEMANAGER.UnbindAllResources();
	}

	__DestroyPDTVertexBufferList();
	__DestroyDefaultIndexBufferList();
}

bool CGraphicDevice::__RecreateDefaultResources()
{
	if (!__CreateDefaultIndexBufferList())
	{
		TraceError("CGraphicDevice::__RecreateDefaultResources: Failed to recreate DefaultIndexBufferList");
		return false;
	}

	if (!__CreatePDTVertexBufferList())
	{
		TraceError("CGraphicDevice::__RecreateDefaultResources: Failed to recreate PDTVertexBufferList");
		return false;
	}

	if (CStateManager::InstancePtr())
	{
		STATEMANAGER.SetDefaultState();
	}

	return true;
}

bool CGraphicDevice::ResizeBackBuffer(UINT uWidth, UINT uHeight)
{
	if (!ms_lpd3dDevice || uWidth == 0 || uHeight == 0)
		return false;

	D3DPRESENT_PARAMETERS& rkD3DPP = ms_d3dPresentParameter;
	if (rkD3DPP.Windowed)
	{
		if (rkD3DPP.BackBufferWidth != uWidth || rkD3DPP.BackBufferHeight != uHeight)
		{
			const UINT oldWidth = rkD3DPP.BackBufferWidth;
			const UINT oldHeight = rkD3DPP.BackBufferHeight;

			rkD3DPP.BackBufferWidth = uWidth;
			rkD3DPP.BackBufferHeight = uHeight;

			// Release all D3DPOOL_DEFAULT resources and unbind textures/shaders in STATEMANAGER
			__ReleaseDefaultResources();

			IDirect3DDevice9Ex& rkD3DDev = *ms_lpd3dDevice;

			HRESULT hr = rkD3DDev.Reset(&rkD3DPP);
			if (FAILED(hr))
			{
				if (hr == D3DERR_DEVICELOST)
				{
					TRACE_ERROR_ONCE("CGraphicDevice::ResizeBackBuffer: Device lost during reset (D3DERR_DEVICELOST)");
				}
				else if (hr == D3DERR_DEVICENOTRESET)
				{
					TRACE_ERROR_ONCE("CGraphicDevice::ResizeBackBuffer: Device not reset (D3DERR_DEVICENOTRESET)");
				}
				else
				{
					TraceError("CGraphicDevice::ResizeBackBuffer: Reset failed (hr=0x%08X)", hr);
				}

				rkD3DPP.BackBufferWidth = oldWidth;
				rkD3DPP.BackBufferHeight = oldHeight;
				return false;
			}

			// Recreate D3DPOOL_DEFAULT resources after successful Reset
			if (!__RecreateDefaultResources())
			{
				TraceError("CGraphicDevice::ResizeBackBuffer: Failed to recreate default resources");
				return false;
			}
		}
	}

	return true;
}

LPDIRECT3DVERTEXDECLARATION9 CGraphicDevice::CreatePNTStreamVertexShader()
{
	assert(ms_lpd3dDevice != NULL);

	LPDIRECT3DVERTEXDECLARATION9 dwShader = NULL;

	D3DVERTEXELEMENT9 pShaderDecl[] = {
		{ 0, 0, D3DDECLTYPE_FLOAT3, D3DDECLMETHOD_DEFAULT, D3DDECLUSAGE_POSITION, 0 },
		{ 0, 12, D3DDECLTYPE_FLOAT3, D3DDECLMETHOD_DEFAULT, D3DDECLUSAGE_NORMAL, 0 },
		{ 0, 24, D3DDECLTYPE_FLOAT2, D3DDECLMETHOD_DEFAULT, D3DDECLUSAGE_TEXCOORD, 0 },
		D3DDECL_END()
	};

	if (ms_lpd3dDevice->CreateVertexDeclaration(pShaderDecl, &dwShader) != D3D_OK)
	{
		wchar_t szError[1024];
		swprintf(szError, L"Failed to create CreatePNTStreamVertexShader");
		MessageBoxW(NULL, szError, L"Vertex Shader Error", MB_ICONSTOP);
	}

	return dwShader;
}

LPDIRECT3DVERTEXDECLARATION9 CGraphicDevice::CreatePNT2StreamVertexShader()
{
	assert(ms_lpd3dDevice != NULL);

	LPDIRECT3DVERTEXDECLARATION9 dwShader = NULL;

	D3DVERTEXELEMENT9 pShaderDecl[] = {
		{ 0, 0, D3DDECLTYPE_FLOAT3, D3DDECLMETHOD_DEFAULT, D3DDECLUSAGE_POSITION, 0 },
		{ 0, 12, D3DDECLTYPE_FLOAT3, D3DDECLMETHOD_DEFAULT, D3DDECLUSAGE_NORMAL, 0 },
		{ 0, 24, D3DDECLTYPE_FLOAT2, D3DDECLMETHOD_DEFAULT, D3DDECLUSAGE_TEXCOORD, 0 },
		{ 0, 32, D3DDECLTYPE_FLOAT2, D3DDECLMETHOD_DEFAULT, D3DDECLUSAGE_TEXCOORD, 1 },
		D3DDECL_END()
	};

	if (ms_lpd3dDevice->CreateVertexDeclaration(pShaderDecl, &dwShader) != D3D_OK)
	{
		wchar_t szError[1024];
		swprintf(szError, L"Failed to create CreatePNT2StreamVertexShader");
		MessageBoxW(NULL, szError, L"Vertex Shader Error", MB_ICONSTOP);
	}

	return dwShader;
}

LPDIRECT3DVERTEXDECLARATION9 CGraphicDevice::CreatePTStreamVertexShader()
{
	assert(ms_lpd3dDevice != NULL);

	LPDIRECT3DVERTEXDECLARATION9 dwShader = NULL;

	D3DVERTEXELEMENT9 pShaderDecl[] = {
		{ 0, 0, D3DDECLTYPE_FLOAT3, D3DDECLMETHOD_DEFAULT, D3DDECLUSAGE_POSITION, 0 },
		{ 1, 0, D3DDECLTYPE_FLOAT2, D3DDECLMETHOD_DEFAULT, D3DDECLUSAGE_TEXCOORD, 0 },
		D3DDECL_END()
	};

	if (ms_lpd3dDevice->CreateVertexDeclaration(pShaderDecl, &dwShader) != D3D_OK)
	{
		wchar_t szError[1024];
		swprintf(szError, L"Failed to create CreatePTStreamVertexShader");
		MessageBoxW(NULL, szError, L"Vertex Shader Error", MB_ICONSTOP);
	}

	return dwShader;
}

CGraphicDevice::EDeviceState CGraphicDevice::GetDeviceState()
{
	if (!ms_lpd3dDevice)
		return DEVICESTATE_NULL;

	HRESULT hr;

	if (FAILED(hr = ms_lpd3dDevice->TestCooperativeLevel()))
	{
		if (D3DERR_DEVICELOST == hr)
			return DEVICESTATE_BROKEN;

		if (D3DERR_DEVICENOTRESET == hr)
			return DEVICESTATE_NEEDS_RESET;

		return DEVICESTATE_BROKEN;
	}
	
	return DEVICESTATE_OK;
}

bool CGraphicDevice::Reset()
{
	return ResetWithParams(ms_d3dPresentParameter);
}

bool CGraphicDevice::ResetWithParams(D3DPRESENT_PARAMETERS& d3dpp)
{
	if (!ms_lpd3dDevice)
		return false;

	// Release all D3DPOOL_DEFAULT resources and unbind textures/shaders in STATEMANAGER
	__ReleaseDefaultResources();

	HRESULT hr = ms_lpd3dDevice->Reset(&d3dpp);
	if (FAILED(hr))
	{
		if (hr == D3DERR_DEVICELOST)
		{
			TRACE_ERROR_ONCE("CGraphicDevice::ResetWithParams: Device lost during reset (D3DERR_DEVICELOST)");
		}
		else if (hr == D3DERR_DEVICENOTRESET)
		{
			TRACE_ERROR_ONCE("CGraphicDevice::ResetWithParams: Device not reset (D3DERR_DEVICENOTRESET)");
		}
		else
		{
			TraceError("CGraphicDevice::ResetWithParams: Reset failed (hr=0x%08X)", hr);
		}
		return false;
	}

	ms_d3dPresentParameter = d3dpp;

	// Recreate D3DPOOL_DEFAULT resources after successful Reset
	if (!__RecreateDefaultResources())
	{
		TraceError("CGraphicDevice::ResetWithParams: Failed to recreate default resources");
		return false;
	}

	return true;
}

static LPDIRECT3DSURFACE9 s_lpStencil;
static DWORD   s_MaxTextureWidth, s_MaxTextureHeight;

BOOL EL3D_ConfirmDevice(D3DCAPS9& rkD3DCaps, UINT uBehavior)
{
	// PUREDEVICE does not support GetTransform / GetViewport.
	if (uBehavior & D3DCREATE_PUREDEVICE)
		return FALSE;

	if (uBehavior & D3DCREATE_HARDWARE_VERTEXPROCESSING)
	{
		// DirectionalLight
		if (!(rkD3DCaps.VertexProcessingCaps & D3DVTXPCAPS_DIRECTIONALLIGHTS))
			return FALSE;

		// PositionalLight
		if (!(rkD3DCaps.VertexProcessingCaps & D3DVTXPCAPS_POSITIONALLIGHTS))
			return FALSE;

		// Shadow/Terrain
		if (!(rkD3DCaps.VertexProcessingCaps & D3DVTXPCAPS_TEXGEN))
			return FALSE;
	}

	s_MaxTextureWidth = rkD3DCaps.MaxTextureWidth;
	s_MaxTextureHeight = rkD3DCaps.MaxTextureHeight;

	return TRUE;
}

DWORD GetMaxTextureWidth()
{
	return s_MaxTextureWidth;
}

DWORD GetMaxTextureHeight()
{
	return s_MaxTextureHeight;
}

int CGraphicDevice::InitD3DInterface(D3DADAPTER_IDENTIFIER9& d3dAdapterId, D3DDISPLAYMODE& d3dDisplayMode, bool windowed)
{
	Direct3DCreate9Ex(D3D_SDK_VERSION, &ms_lpd3d);
	if (!ms_lpd3d)
		return CREATE_NO_DIRECTX;

	ms_lpd3d->GetAdapterIdentifier(D3DADAPTER_DEFAULT, 0, &d3dAdapterId);
	ms_lpd3d->GetAdapterDisplayMode(D3DADAPTER_DEFAULT, &d3dDisplayMode);

	if (windowed &&
		strnicmp(d3dAdapterId.Driver, "3dfx", 4) == 0 &&
		D3DFMT_X8R8G8B8 == d3dDisplayMode.Format)
	{
		return CREATE_FORMAT;
	}

	return CREATE_OK;
}

DWORD CGraphicDevice::DetermineD3DBehavior(const D3DCAPS9& d3dCaps, int& iRet)
{
	DWORD dwBehavior = 0;
	BOOL isFormatConfirmed = FALSE;

	if (d3dCaps.DevCaps & D3DDEVCAPS_HWTRANSFORMANDLIGHT)
	{
		if (d3dCaps.DevCaps & D3DDEVCAPS_PUREDEVICE)
		{
			dwBehavior = D3DCREATE_HARDWARE_VERTEXPROCESSING | D3DCREATE_PUREDEVICE;
			isFormatConfirmed = EL3D_ConfirmDevice(const_cast<D3DCAPS9&>(d3dCaps), dwBehavior);
		}

		if (!isFormatConfirmed)
		{
			dwBehavior = D3DCREATE_HARDWARE_VERTEXPROCESSING;
			isFormatConfirmed = EL3D_ConfirmDevice(const_cast<D3DCAPS9&>(d3dCaps), dwBehavior);
		}

		if (!isFormatConfirmed)
		{
			dwBehavior = D3DCREATE_MIXED_VERTEXPROCESSING;
			isFormatConfirmed = EL3D_ConfirmDevice(const_cast<D3DCAPS9&>(d3dCaps), dwBehavior);
		}
	}

	if (!isFormatConfirmed)
	{
		dwBehavior = D3DCREATE_SOFTWARE_VERTEXPROCESSING;
		isFormatConfirmed = EL3D_ConfirmDevice(const_cast<D3DCAPS9&>(d3dCaps), dwBehavior);
		iRet |= CREATE_NO_TNL;
	}

	return dwBehavior;
}

bool CGraphicDevice::CreateD3DDeviceWithFallback(HWND hWnd, int iHres, int iVres, bool windowed, int iReflashRate, const D3DDISPLAYMODE& d3dDisplayMode, int& iRet)
{
	bool disableMSAA = false;
	const int maxAttempts = 2;

	for (int attempt = 0; attempt < maxAttempts; ++attempt)
	{
		ZeroMemory(&ms_d3dPresentParameter, sizeof(ms_d3dPresentParameter));

		ms_d3dPresentParameter.Windowed							= windowed;
		ms_d3dPresentParameter.BackBufferWidth					= iHres;
		ms_d3dPresentParameter.BackBufferHeight					= iVres;
		ms_d3dPresentParameter.hDeviceWindow					= hWnd;
		ms_d3dPresentParameter.BackBufferFormat					= windowed ? D3DFMT_UNKNOWN : d3dDisplayMode.Format;
		ms_d3dPresentParameter.BackBufferCount					= m_uBackBufferCount;
		ms_d3dPresentParameter.SwapEffect						= D3DSWAPEFFECT_DISCARD;
		ms_d3dPresentParameter.MultiSampleType					= D3DMULTISAMPLE_NONE;

		if (windowed)
		{
			ms_d3dPresentParameter.PresentationInterval			= D3DPRESENT_INTERVAL_DEFAULT;
			ms_d3dPresentParameter.FullScreen_RefreshRateInHz	= 0;
		}
		else
		{
			ms_d3dPresentParameter.PresentationInterval			= D3DPRESENT_INTERVAL_ONE;
			ms_d3dPresentParameter.FullScreen_RefreshRateInHz	= D3DPRESENT_RATE_DEFAULT;
		}

		ms_d3dPresentParameter.EnableAutoDepthStencil			= TRUE;
		ms_d3dPresentParameter.AutoDepthStencilFormat			= D3DFMT_D24S8;

		// Enable MSAA only in fullscreen and only if supported for the chosen backbuffer format
		if (!windowed && !disableMSAA)
		{
			D3DFORMAT msaaCheckFormat = ms_d3dPresentParameter.BackBufferFormat;
			if (SUCCEEDED(ms_lpd3d->CheckDeviceMultiSampleType(
				D3DADAPTER_DEFAULT,
				D3DDEVTYPE_HAL,
				msaaCheckFormat,
				FALSE,
				D3DMULTISAMPLE_2_SAMPLES,
				&ms_d3dPresentParameter.MultiSampleQuality)))
			{
				ms_d3dPresentParameter.MultiSampleType = D3DMULTISAMPLE_2_SAMPLES;
				ms_d3dPresentParameter.MultiSampleQuality = 0;
			}

			if (SUCCEEDED(ms_lpd3d->CheckDeviceMultiSampleType(
				D3DADAPTER_DEFAULT,
				D3DDEVTYPE_HAL,
				msaaCheckFormat,
				FALSE,
				D3DMULTISAMPLE_4_SAMPLES,
				&ms_d3dPresentParameter.MultiSampleQuality)))
			{
				ms_d3dPresentParameter.MultiSampleType = D3DMULTISAMPLE_4_SAMPLES;
				ms_d3dPresentParameter.MultiSampleQuality = 0;
			}
		}

		D3DDISPLAYMODEEX displayModeEx;
		ZeroMemory(&displayModeEx, sizeof(displayModeEx));
		displayModeEx.Size = sizeof(D3DDISPLAYMODEEX);
		displayModeEx.Width = iHres;
		displayModeEx.Height = iVres;
		displayModeEx.RefreshRate = iReflashRate;
		displayModeEx.Format = d3dDisplayMode.Format;
		displayModeEx.ScanLineOrdering = D3DSCANLINEORDERING_PROGRESSIVE;

		D3DDISPLAYMODEEX* pDisplayMode = windowed ? NULL : &displayModeEx;

		ms_hLastResult = ms_lpd3d->CreateDeviceEx(
			D3DADAPTER_DEFAULT,
			D3DDEVTYPE_HAL,
			hWnd,
			ms_dwD3DBehavior,
			&ms_d3dPresentParameter,
			pDisplayMode,
			&ms_lpd3dDevice);

		if (SUCCEEDED(ms_hLastResult))
		{
			return true;
		}

		switch (ms_hLastResult)
		{
		case D3DERR_INVALIDCALL:
			Tracen("IDirect3DDevice.CreateDevice - ERROR D3DERR_INVALIDCALL\nThe method call is invalid. For example, a method's parameter may have an invalid value.");
			break;
		case D3DERR_NOTAVAILABLE:
			Tracen("IDirect3DDevice.CreateDevice - ERROR D3DERR_NOTAVAILABLE\nThis device does not support the queried technique. ");
			break;
		case D3DERR_OUTOFVIDEOMEMORY:
			Tracen("IDirect3DDevice.CreateDevice - ERROR D3DERR_OUTOFVIDEOMEMORY\nDirect3D does not have enough display memory to perform the operation");
			break;
		default:
			Tracenf("IDirect3DDevice.CreateDevice - ERROR %d", ms_hLastResult);
			break;
		}

		// Fallback for next attempt: conservative settings
		disableMSAA = true;
		iReflashRate = 0;
		iRet |= CREATE_REFRESHRATE;
	}

	return false;
}

void CGraphicDevice::CheckTextureFormats(bool windowed, const D3DDISPLAYMODE& d3dDisplayMode)
{
	const D3DFORMAT baseFormatForTextureCheck = windowed ? d3dDisplayMode.Format : ms_d3dPresentParameter.BackBufferFormat;
	if (ms_lpd3d->CheckDeviceFormat(
		D3DADAPTER_DEFAULT,
		D3DDEVTYPE_HAL,
		baseFormatForTextureCheck,
		0,
		D3DRTYPE_TEXTURE,
		D3DFMT_DXT1) == D3DERR_NOTAVAILABLE)
	{
		ms_bSupportDXT = false;
	}

	if (ms_lpd3d->CheckDeviceFormat(
		D3DADAPTER_DEFAULT,
		D3DDEVTYPE_HAL,
		baseFormatForTextureCheck,
		0,
		D3DRTYPE_TEXTURE,
		D3DFMT_DXT3) == D3DERR_NOTAVAILABLE)
	{
		ms_bSupportDXT = false;
	}

	if (ms_lpd3d->CheckDeviceFormat(
		D3DADAPTER_DEFAULT,
		D3DDEVTYPE_HAL,
		baseFormatForTextureCheck,
		0,
		D3DRTYPE_TEXTURE,
		D3DFMT_DXT5) == D3DERR_NOTAVAILABLE)
	{
		ms_bSupportDXT = false;
	}
}

bool CGraphicDevice::InitDeviceResources()
{
	m_pStateManager = new CStateManager(ms_lpd3dDevice);

	D3DXCreateMatrixStack(0, &ms_lpd3dMatStack);
	ms_lpd3dMatStack->LoadIdentity();

	ms_ptVS	= CreatePTStreamVertexShader();
	ms_pntVS = CreatePNTStreamVertexShader();
	ms_pnt2VS = CreatePNT2StreamVertexShader();

	D3DXMatrixIdentity(&ms_matIdentity);
	D3DXMatrixIdentity(&ms_matView);
	D3DXMatrixIdentity(&ms_matProj);
	D3DXMatrixIdentity(&ms_matInverseView);

	D3DXCreateSphere(ms_lpd3dDevice, 1.0f, 32, 32, &ms_lpSphereMesh, NULL);
	D3DXCreateCylinder(ms_lpd3dDevice, 1.0f, 1.0f, 1.0f, 8, 8, &ms_lpCylinderMesh, NULL);

	ms_lpd3dDevice->Clear(0L, NULL, D3DCLEAR_TARGET | D3DCLEAR_ZBUFFER, 0xff000000, 1.0f, 0);

	if (!__CreateDefaultIndexBufferList())
		return false;

	if (!__CreatePDTVertexBufferList())
		return false;

	return true;
}

void CGraphicDevice::DetectHardwareCapsAndQuirks(const D3DADAPTER_IDENTIFIER9& d3dAdapterId)
{
	DWORD dwTexMemSize = GetAvailableTextureMemory();

	if (dwTexMemSize < 64 * 1024 * 1024)
		ms_isLowTextureMemory = true;
	else
		ms_isLowTextureMemory = false;

	if (dwTexMemSize > 100 * 1024 * 1024)
		ms_isHighTextureMemory = true;
	else
		ms_isHighTextureMemory = false;

	if (ms_d3dCaps.TextureAddressCaps & D3DPTADDRESSCAPS_BORDER)
		GRAPHICS_CAPS_CAN_NOT_TEXTURE_ADDRESS_BORDER = false;
	else
		GRAPHICS_CAPS_CAN_NOT_TEXTURE_ADDRESS_BORDER = true;

	if (strnicmp(d3dAdapterId.Driver, "SIS", 3) == 0)
	{
		GRAPHICS_CAPS_CAN_NOT_DRAW_LINE = true;
		GRAPHICS_CAPS_CAN_NOT_DRAW_SHADOW = true;
		GRAPHICS_CAPS_HALF_SIZE_IMAGE = true;
		ms_isLowTextureMemory = true;
	}
	else if (strnicmp(d3dAdapterId.Driver, "3dfx", 4) == 0)
	{
		GRAPHICS_CAPS_CAN_NOT_DRAW_SHADOW = true;
		GRAPHICS_CAPS_HALF_SIZE_IMAGE = true;
		ms_isLowTextureMemory = true;
	}

	// CPU check
	int cpuInfo[4] = { 0 };
	__cpuid(cpuInfo, 1);

	CPU_HAS_SSE2 = cpuInfo[3] & (1 << 26);
}

int CGraphicDevice::Create(HWND hWnd, int iHres, int iVres, bool Windowed, int /*iBit*/, int iReflashRate)
{
	int iRet = CREATE_OK;

	Destroy();

	ms_iWidth	= iHres;
	ms_iHeight	= iVres;

	ms_hWnd		= hWnd;
	ms_hDC		= GetDC(hWnd);

	D3DADAPTER_IDENTIFIER9 d3dAdapterId{};
	D3DDISPLAYMODE d3dDisplayMode{};
	int initRet = InitD3DInterface(d3dAdapterId, d3dDisplayMode, Windowed);
	if (initRet != CREATE_OK)
		return initRet;

	D3DCAPS9 d3dCaps{};
	ms_lpd3d->GetDeviceCaps(D3DADAPTER_DEFAULT, D3DDEVTYPE_HAL, &d3dCaps);
	ms_dwD3DBehavior = DetermineD3DBehavior(d3dCaps, iRet);

	if (!CreateD3DDeviceWithFallback(hWnd, iHres, iVres, Windowed, iReflashRate, d3dDisplayMode, iRet))
		return CREATE_DEVICE;

	CheckTextureFormats(Windowed, d3dDisplayMode);

	if (FAILED((ms_hLastResult = ms_lpd3dDevice->GetDeviceCaps(&ms_d3dCaps))))
	{
		Tracenf("IDirect3DDevice.GetDeviceCaps - ERROR %d", ms_hLastResult);
		return CREATE_GET_DEVICE_CAPS2;
	}

	if (!Windowed)
		SetWindowPos(hWnd, HWND_TOPMOST, 0, 0, iHres, iVres, SWP_SHOWWINDOW);

	ms_lpd3dDevice->GetViewport(&ms_Viewport);

	if (!InitDeviceResources())
		return CREATE_DEVICE;

	DetectHardwareCapsAndQuirks(d3dAdapterId);

	return iRet;
}

void CGraphicDevice::__InitializePDTVertexBufferList()
{
	for (UINT i=0; i<PDT_VERTEXBUFFER_NUM; ++i)
		ms_alpd3dPDTVB[i]=NULL;	
}
		
void CGraphicDevice::__DestroyPDTVertexBufferList()
{
	for (UINT i=0; i<PDT_VERTEXBUFFER_NUM; ++i)
	{
		if (ms_alpd3dPDTVB[i])
		{
			ms_alpd3dPDTVB[i]->Release();
			ms_alpd3dPDTVB[i]=NULL;
		}
	}
}

bool CGraphicDevice::__CreatePDTVertexBufferList()
{
	for (UINT i = 0; i < PDT_VERTEXBUFFER_NUM; ++i)
	{
		HRESULT hr = ms_lpd3dDevice->CreateVertexBuffer(
			sizeof(TPDTVertex) * PDT_VERTEX_NUM, 
			D3DUSAGE_DYNAMIC | D3DUSAGE_WRITEONLY, 
			D3DFVF_XYZ | D3DFVF_DIFFUSE | D3DFVF_TEX1, 
			D3DPOOL_DEFAULT,
			&ms_alpd3dPDTVB[i],
			nullptr);

		if (FAILED(hr))
		{
			TraceError("CGraphicDevice::__CreatePDTVertexBufferList: Failed to create vertex buffer %u (hr=0x%08X)", i, hr);
			__DestroyPDTVertexBufferList();
			return false;
		}
	}
	return true;
}

void CGraphicDevice::__InitializeDefaultIndexBufferList()
{
	for (UINT i = 0; i < DEFAULT_IB_NUM; ++i)
		ms_alpd3dDefIB[i] = NULL;
}

void CGraphicDevice::__DestroyDefaultIndexBufferList()
{
	for (UINT i = 0; i < DEFAULT_IB_NUM; ++i)
	{
		if (ms_alpd3dDefIB[i])
		{
			ms_alpd3dDefIB[i]->Release();
			ms_alpd3dDefIB[i] = NULL;
		}
	}
}

bool CGraphicDevice::__CreateDefaultIndexBuffer(UINT eDefIB, UINT uIdxCount, const WORD* c_awIndices)
{
	assert(eDefIB < DEFAULT_IB_NUM);
	if (eDefIB >= DEFAULT_IB_NUM)
		return false;

	if (ms_alpd3dDefIB[eDefIB])
	{
		ms_alpd3dDefIB[eDefIB]->Release();
		ms_alpd3dDefIB[eDefIB] = nullptr;
	}

	HRESULT hr = ms_lpd3dDevice->CreateIndexBuffer(
		sizeof(WORD) * uIdxCount,
		D3DUSAGE_WRITEONLY,
		D3DFMT_INDEX16,
		D3DPOOL_DEFAULT,
		&ms_alpd3dDefIB[eDefIB],
		nullptr
	);
	if (FAILED(hr))
	{
		TraceError("CGraphicDevice::__CreateDefaultIndexBuffer: Failed to create index buffer %u (hr=0x%08X)", eDefIB, hr);
		return false;
	}
	
	WORD* dstIndices = nullptr;
	hr = ms_alpd3dDefIB[eDefIB]->Lock(0, 0, (void**)&dstIndices, 0);
	if (FAILED(hr))
	{
		TraceError("CGraphicDevice::__CreateDefaultIndexBuffer: Failed to lock index buffer %u (hr=0x%08X)", eDefIB, hr);
		safe_release(ms_alpd3dDefIB[eDefIB]);
		return false;
	}

	memcpy(dstIndices, c_awIndices, sizeof(WORD) * uIdxCount);

	hr = ms_alpd3dDefIB[eDefIB]->Unlock();
	if (FAILED(hr))
	{
		TraceError("CGraphicDevice::__CreateDefaultIndexBuffer: Failed to unlock index buffer %u (hr=0x%08X)", eDefIB, hr);
		safe_release(ms_alpd3dDefIB[eDefIB]);
		return false;
	}

	return true;
}

bool CGraphicDevice::__CreateDefaultIndexBufferList()
{
	static const WORD c_awLineIndices[2] = { 0, 1, };
	static const WORD c_awLineTriIndices[6] = { 0, 1, 0, 2, 1, 2, };
	static const WORD c_awLineRectIndices[8] = { 0, 1, 0, 2, 1, 3, 2, 3,};
	static const WORD c_awLineCubeIndices[24] = { 
		0, 1, 0, 2, 1, 3, 2, 3,
		0, 4, 1, 5, 2, 6, 3, 7,
		4, 5, 4, 6, 5, 7, 6, 7,
	};
	static const WORD c_awFillTriIndices[3]= { 0, 1, 2, };
	static const WORD c_awFillRectIndices[6] = { 0, 2, 1, 2, 3, 1, };
	static const WORD c_awFillCubeIndices[36] = { 
		0, 1, 2, 1, 3, 2,
		2, 0, 6, 0, 4, 6,
		0, 1, 4, 1, 5, 4,
		1, 3, 5, 3, 7, 5,
		3, 2, 7, 2, 6, 7,
		4, 5, 6, 5, 7, 6,
	};
	
	if (!__CreateDefaultIndexBuffer(DEFAULT_IB_LINE, 2, c_awLineIndices) ||
		!__CreateDefaultIndexBuffer(DEFAULT_IB_LINE_TRI, 6, c_awLineTriIndices) ||
		!__CreateDefaultIndexBuffer(DEFAULT_IB_LINE_RECT, 8, c_awLineRectIndices) ||
		!__CreateDefaultIndexBuffer(DEFAULT_IB_LINE_CUBE, 24, c_awLineCubeIndices) ||
		!__CreateDefaultIndexBuffer(DEFAULT_IB_FILL_TRI, 3, c_awFillTriIndices) ||
		!__CreateDefaultIndexBuffer(DEFAULT_IB_FILL_RECT, 6, c_awFillRectIndices) ||
		!__CreateDefaultIndexBuffer(DEFAULT_IB_FILL_CUBE, 36, c_awFillCubeIndices))
	{
		__DestroyDefaultIndexBufferList();
		return false;
	}
	
	return true;
}

void CGraphicDevice::InitBackBufferCount(UINT uBackBufferCount)
{
	m_uBackBufferCount=uBackBufferCount;
}

void CGraphicDevice::Destroy()
{
	__DestroyPDTVertexBufferList();
	__DestroyDefaultIndexBufferList();

	if (ms_hDC)
	{
		ReleaseDC(ms_hWnd, ms_hDC);
		ms_hDC = NULL;
	}

	if (ms_ptVS)
	{	
		ms_ptVS->Release();
		ms_ptVS = nullptr;
	}

	if (ms_pntVS)
	{	
		ms_pntVS->Release();
		ms_pntVS = nullptr;
	}

	if (ms_pnt2VS)
	{	
		ms_pnt2VS->Release();
		ms_pnt2VS = nullptr;
	}

	safe_release(ms_lpSphereMesh);
	safe_release(ms_lpCylinderMesh);

	safe_release(ms_lpd3dMatStack);
	safe_release(ms_lpd3dDevice);
	safe_release(ms_lpd3d);	

	if (m_pStateManager)
	{
		delete m_pStateManager;
		m_pStateManager = NULL;
	}

	__Initialize();
}
