#include "StdAfx.h"
#include "StateManager.h"
#include "GrpLightManager.h"
#include "RenderBypassGuard.h"
#include "RenderQueueManager.h"

//#define StateManager_Assert(a) if (!(a)) puts("assert"#a)
#define StateManager_Assert(a) assert(a)

static DWORD gs_DefaultRenderStates[STATEMANAGER_MAX_RENDERSTATES];

struct SLightData
{
	D3DLIGHT9 m_akD3DLight[8];
	bool m_abLightValid[8];
	BOOL m_abLightEnabled[8];
} m_kLightData;

void CStateManager::SetLight(DWORD index, CONST D3DLIGHT9* pLight)
{
	assert(index < 8);
	if (index >= 8 || !pLight)
		return;

	if (!m_bForce && m_kLightData.m_abLightValid[index])
	{
		if (memcmp(&m_kLightData.m_akD3DLight[index], pLight, sizeof(D3DLIGHT9)) == 0)
			return;
	}

	m_kLightData.m_akD3DLight[index] = *pLight;
	m_kLightData.m_abLightValid[index] = true;

	if (m_lpD3DDev)
		m_lpD3DDev->SetLight(index, pLight);
}

void CStateManager::GetLight(DWORD index, D3DLIGHT9* pLight)
{
	assert(index < 8);
	if (index >= 8 || !pLight)
		return;

	*pLight = m_kLightData.m_akD3DLight[index];
}

void CStateManager::LightEnable(DWORD index, BOOL bEnable)
{
	assert(index < 8);
	if (index >= 8)
		return;

	if (!m_bForce && m_kLightData.m_abLightEnabled[index] == bEnable)
		return;

	m_kLightData.m_abLightEnabled[index] = bEnable;

	if (m_lpD3DDev)
		m_lpD3DDev->LightEnable(index, bEnable);
}

BOOL CStateManager::IsLightEnabled(DWORD index) const
{
	if (index >= 8)
		return FALSE;
	return m_kLightData.m_abLightEnabled[index];
}

void CStateManager::SetViewport(const D3DVIEWPORT9* pViewport)
{
	if (!pViewport)
		return;

	m_Viewport = *pViewport;
	m_bViewportValid = true;

	if (m_lpD3DDev)
		m_lpD3DDev->SetViewport(pViewport);
}

void CStateManager::GetViewport(D3DVIEWPORT9* pViewport)
{
	if (!pViewport)
		return;

	if (m_bViewportValid)
	{
		*pViewport = m_Viewport;
		return;
	}

	if (m_lpD3DDev)
	{
		m_lpD3DDev->GetViewport(pViewport);
		m_Viewport = *pViewport;
		m_bViewportValid = true;
	}
}

void CStateManager::SetGammaRamp(DWORD dwFlags, const D3DGAMMARAMP* pRamp)
{
	if (m_lpD3DDev && pRamp)
		m_lpD3DDev->SetGammaRamp(0, dwFlags, pRamp);
}

HRESULT CStateManager::GetDeviceCaps(D3DCAPS9* pCaps)
{
	if (!m_lpD3DDev || !pCaps)
		return E_FAIL;
	return m_lpD3DDev->GetDeviceCaps(pCaps);
}

DWORD CStateManager::GetAvailableTextureMem()
{
	if (!m_lpD3DDev)
		return 0;
	return m_lpD3DDev->GetAvailableTextureMem();
}

void CStateManager::SetScissorRect(const RECT& c_rRect)
{
	if (!m_bForce && m_bScissorRectValid &&
		m_CurrentScissorRect.left == c_rRect.left &&
		m_CurrentScissorRect.top == c_rRect.top &&
		m_CurrentScissorRect.right == c_rRect.right &&
		m_CurrentScissorRect.bottom == c_rRect.bottom)
	{
		return;
	}

	if (m_lpD3DDev)
		m_lpD3DDev->SetScissorRect(&c_rRect);
	m_CurrentScissorRect = c_rRect;
	m_bScissorRectValid = true;
}

void CStateManager::GetScissorRect(RECT* pRect)
{
	if (!pRect)
		return;

	if (m_bScissorRectValid)
	{
		*pRect = m_CurrentScissorRect;
	}
	else
	{
		if (m_lpD3DDev)
		{
			m_lpD3DDev->GetScissorRect(pRect);
			m_CurrentScissorRect = *pRect;
			m_bScissorRectValid = true;
		}
	}
}

void CStateManagerState::ResetState()
{
	DWORD i, y;

	for (i = 0; i < STATEMANAGER_MAX_RENDERSTATES; i++)
		m_RenderStates[i] = gs_DefaultRenderStates[i];

	for (i = 0; i < STATEMANAGER_MAX_STAGES; i++)
		for (y = 0; y < STATEMANAGER_MAX_TEXTURESTATES; y++)
			m_TextureStates[i][y] = 0x7FFFFFFF;

	for (i = 0; i < STATEMANAGER_MAX_STAGES; i++)
		for (y = 0; y < STATEMANAGER_MAX_TEXTURESTATES; y++)
			m_SamplerStates[i][y] = 0xFFFFFFFF;

	for (i = 0; i < STATEMANAGER_MAX_STREAMS; i++)
		m_StreamData[i] = CStreamData();

	m_IndexData = CIndexData();

	for (i = 0; i < STATEMANAGER_MAX_STAGES; i++)
		m_Textures[i] = NULL;

	// Matrices and constants are not cached, just restored.  It's silly to check all the 
	// data elements (by which time the driver could have been sent them).
	for (i = 0; i < STATEMANAGER_MAX_TRANSFORMSTATES; i++)
		D3DXMatrixIdentity(&m_Matrices[i]);

	m_dwPixelShader = 0;
	m_dwVertexShader = 0;
	m_dwVertexDeclaration = 0;
	m_dwFVF = D3DFVF_XYZ;
	m_bVertexProcessing = FALSE;
}

void CStateManager::ResetState()
{
	m_CurrentState.ResetState();
	m_bScissorRectValid = false;
	ZeroMemory(&m_kLightData, sizeof(m_kLightData));
}

bool CStateManager::BeginScene()
{
	m_bScene = true;

	m_lpD3DDev->SetTransform(D3DTS_WORLD, &m_CurrentState.m_Matrices[D3DTS_WORLD]);
	m_lpD3DDev->SetTransform(D3DTS_PROJECTION, &m_CurrentState.m_Matrices[D3DTS_PROJECTION]);
	m_lpD3DDev->SetTransform(D3DTS_VIEW, &m_CurrentState.m_Matrices[D3DTS_VIEW]);

	if (FAILED(m_lpD3DDev->BeginScene()))
		return false;
	return true;
}

void CStateManager::EndScene()
{
	m_lpD3DDev->EndScene();
	m_bScene = false;
}

CStateManager::CStateManager(LPDIRECT3DDEVICE9EX lpDevice) : m_lpD3DDev(NULL)
{
	m_bScene = false;
	m_dwBestMinFilter = D3DTEXF_ANISOTROPIC;
	m_dwBestMagFilter = D3DTEXF_ANISOTROPIC;

	for (int i = 0; i < STATEMANAGER_MAX_RENDERSTATES; i++)
		lpDevice->GetRenderState((D3DRENDERSTATETYPE)i, &gs_DefaultRenderStates[i]);

	SetDevice(lpDevice);

	m_iDrawCallCount = 0;
	m_iLastDrawCallCount = 0;
	m_dwPrimitiveCount = 0;
	m_dwLastPrimitiveCount = 0;
	m_currentRenderCategory = RenderOwnership::ERenderCategory::Count;
}

CStateManager::~CStateManager()
{
	if (m_lpD3DDev)
	{
		m_lpD3DDev->Release();
		m_lpD3DDev = NULL;
	}
}

void CStateManager::SetDevice(LPDIRECT3DDEVICE9EX lpDevice)
{
	StateManager_Assert(lpDevice);
	lpDevice->AddRef();

	if (m_lpD3DDev)
	{
		m_lpD3DDev->Release();
		m_lpD3DDev = NULL;
	}

	m_lpD3DDev = lpDevice;

	SetDefaultState();
}

void CStateManager::SetBestFiltering(DWORD dwStage)
{
	SetSamplerState(dwStage, D3DSAMP_MINFILTER, m_dwBestMinFilter);
	SetSamplerState(dwStage, D3DSAMP_MAGFILTER, m_dwBestMagFilter);
	SetSamplerState(dwStage, D3DSAMP_MIPFILTER, D3DTEXF_LINEAR);
}

namespace
{
	struct StageDefaultConfig
	{
		DWORD colorOp;
		DWORD colorArg1;
		DWORD colorArg2;
		DWORD alphaOp;
		DWORD alphaArg1;
		DWORD alphaArg2;
	};

	constexpr StageDefaultConfig c_StageDefaults[STATEMANAGER_MAX_STAGES] = {
		{ D3DTOP_MODULATE, D3DTA_TEXTURE, D3DTA_CURRENT, D3DTOP_SELECTARG1, D3DTA_TEXTURE, D3DTA_CURRENT },
		{ D3DTOP_DISABLE,  D3DTA_TEXTURE, D3DTA_DIFFUSE, D3DTOP_DISABLE,    D3DTA_TEXTURE, D3DTA_DIFFUSE },
		{ D3DTOP_DISABLE,  D3DTA_TEXTURE, D3DTA_DIFFUSE, D3DTOP_DISABLE,    D3DTA_TEXTURE, D3DTA_DIFFUSE },
		{ D3DTOP_DISABLE,  D3DTA_TEXTURE, D3DTA_DIFFUSE, D3DTOP_DISABLE,    D3DTA_TEXTURE, D3DTA_DIFFUSE },
		{ D3DTOP_DISABLE,  D3DTA_TEXTURE, D3DTA_DIFFUSE, D3DTOP_DISABLE,    D3DTA_TEXTURE, D3DTA_DIFFUSE },
		{ D3DTOP_DISABLE,  D3DTA_TEXTURE, D3DTA_DIFFUSE, D3DTOP_DISABLE,    D3DTA_TEXTURE, D3DTA_DIFFUSE },
		{ D3DTOP_DISABLE,  D3DTA_TEXTURE, D3DTA_DIFFUSE, D3DTOP_DISABLE,    D3DTA_TEXTURE, D3DTA_DIFFUSE },
		{ D3DTOP_DISABLE,  D3DTA_TEXTURE, D3DTA_DIFFUSE, D3DTOP_DISABLE,    D3DTA_TEXTURE, D3DTA_DIFFUSE },
	};

	struct RenderStateDefaultConfig
	{
		D3DRENDERSTATETYPE state;
		DWORD value;
	};

	constexpr RenderStateDefaultConfig c_DefaultRenderStates[] = {
		{ D3DRS_DIFFUSEMATERIALSOURCE, D3DMCS_MATERIAL },
		{ D3DRS_SPECULARMATERIALSOURCE, D3DMCS_MATERIAL },
		{ D3DRS_AMBIENTMATERIALSOURCE, D3DMCS_MATERIAL },
		{ D3DRS_EMISSIVEMATERIALSOURCE, D3DMCS_MATERIAL },
		{ D3DRS_LASTPIXEL, TRUE },
		{ D3DRS_ALPHAREF, 1 },
		{ D3DRS_ALPHAFUNC, D3DCMP_GREATEREQUAL },
		{ D3DRS_FOGSTART, 0 },
		{ D3DRS_FOGEND, 0 },
		{ D3DRS_FOGDENSITY, 0 },
		{ D3DRS_STENCILWRITEMASK, 0xFFFFFFFF },
		{ D3DRS_AMBIENT, 0x00000000 },
		{ D3DRS_LOCALVIEWER, TRUE },
		{ D3DRS_NORMALIZENORMALS, FALSE },
		{ D3DRS_VERTEXBLEND, D3DVBF_DISABLE },
		{ D3DRS_CLIPPLANEENABLE, 0 },
		{ D3DRS_SCISSORTESTENABLE, FALSE },
		{ D3DRS_MULTISAMPLEANTIALIAS, TRUE },
		{ D3DRS_MULTISAMPLEMASK, 0xFFFFFFFF },
		{ D3DRS_PATCHEDGESTYLE, D3DPATCHEDGE_CONTINUOUS },
		{ D3DRS_INDEXEDVERTEXBLENDENABLE, FALSE },
		{ D3DRS_COLORWRITEENABLE, 0xFFFFFFFF },
		{ D3DRS_FILLMODE, D3DFILL_SOLID },
		{ D3DRS_SHADEMODE, D3DSHADE_GOURAUD },
		{ D3DRS_CULLMODE, D3DCULL_CW },
		{ D3DRS_ALPHABLENDENABLE, FALSE },
		{ D3DRS_BLENDOP, D3DBLENDOP_ADD },
		{ D3DRS_SRCBLEND, D3DBLEND_SRCALPHA },
		{ D3DRS_DESTBLEND, D3DBLEND_INVSRCALPHA },
		{ D3DRS_FOGENABLE, FALSE },
		{ D3DRS_FOGCOLOR, 0xFF000000 },
		{ D3DRS_FOGTABLEMODE, D3DFOG_NONE },
		{ D3DRS_FOGVERTEXMODE, D3DFOG_LINEAR },
		{ D3DRS_RANGEFOGENABLE, FALSE },
		{ D3DRS_ZENABLE, TRUE },
		{ D3DRS_ZFUNC, D3DCMP_LESSEQUAL },
		{ D3DRS_ZWRITEENABLE, TRUE },
		{ D3DRS_DITHERENABLE, TRUE },
		{ D3DRS_STENCILENABLE, FALSE },
		{ D3DRS_ALPHATESTENABLE, FALSE },
		{ D3DRS_CLIPPING, TRUE },
		{ D3DRS_LIGHTING, FALSE },
		{ D3DRS_SPECULARENABLE, FALSE },
		{ D3DRS_COLORVERTEX, FALSE },
	};
}

void CStateManager::SetDefaultState()
{
	ResetState();

	for (auto& stack : m_RenderStateStack)
		stack.clear();

	for (auto& stageStacks : m_SamplerStateStack)
		for (auto& stack : stageStacks)
			stack.clear();

	for (auto& stageStacks : m_TextureStageStateStack)
		for (auto& stack : stageStacks)
			stack.clear();

	for (auto& stack : m_TransformStack)
		stack.clear();

	for (auto& stack : m_TextureStack)
		stack.clear();

	for (auto& stack : m_StreamStack)
		stack.clear();

	m_MaterialStack.clear();
	m_FVFStack.clear();
	m_PixelShaderStack.clear();
	m_VertexShaderStack.clear();
	m_VertexDeclarationStack.clear();
	m_VertexProcessingStack.clear();
	m_IndexStack.clear();

	m_bScene = false;
	m_bForce = true;

	D3DXMATRIX matIdentity;
	D3DXMatrixIdentity(&matIdentity);

	SetTransform(D3DTS_WORLD, &matIdentity);
	SetTransform(D3DTS_VIEW, &matIdentity);
	SetTransform(D3DTS_PROJECTION, &matIdentity);

	D3DMATERIAL9 DefaultMat{};
	DefaultMat.Diffuse = { 1.0f, 1.0f, 1.0f, 1.0f };
	DefaultMat.Ambient = { 1.0f, 1.0f, 1.0f, 1.0f };
	SetMaterial(&DefaultMat);

	for (const auto& item : c_DefaultRenderStates)
		SetRenderState(item.state, item.value);

	SaveVertexProcessing(FALSE);

	for (DWORD stage = 0; stage < STATEMANAGER_MAX_STAGES; ++stage)
	{
		const auto& cfg = c_StageDefaults[stage];
		SetTextureStageState(stage, D3DTSS_COLOROP, cfg.colorOp);
		SetTextureStageState(stage, D3DTSS_COLORARG1, cfg.colorArg1);
		SetTextureStageState(stage, D3DTSS_COLORARG2, cfg.colorArg2);
		SetTextureStageState(stage, D3DTSS_ALPHAOP, cfg.alphaOp);
		SetTextureStageState(stage, D3DTSS_ALPHAARG1, cfg.alphaArg1);
		SetTextureStageState(stage, D3DTSS_ALPHAARG2, cfg.alphaArg2);
		SetTextureStageState(stage, D3DTSS_TEXCOORDINDEX, stage);
		SetTextureStageState(stage, D3DTSS_TEXTURETRANSFORMFLAGS, 0);

		SetSamplerState(stage, D3DSAMP_MINFILTER, D3DTEXF_LINEAR);
		SetSamplerState(stage, D3DSAMP_MAGFILTER, D3DTEXF_LINEAR);
		SetSamplerState(stage, D3DSAMP_MIPFILTER, D3DTEXF_LINEAR);
		SetSamplerState(stage, D3DSAMP_ADDRESSU, D3DTADDRESS_WRAP);
		SetSamplerState(stage, D3DSAMP_ADDRESSV, D3DTADDRESS_WRAP);

		SetTexture(stage, NULL);
		SetRenderState(static_cast<D3DRENDERSTATETYPE>(D3DRS_WRAP0 + stage), 0);
	}

	SetVertexShader(0);
	SetVertexDeclaration(0);
	SetPixelShader(0);
	SetFVF(D3DFVF_XYZ);

	D3DXVECTOR4 av4Null[STATEMANAGER_MAX_VCONSTANTS];
	memset(av4Null, 0, sizeof(av4Null));
	SetVertexShaderConstant(0, av4Null, STATEMANAGER_MAX_VCONSTANTS);
	SetPixelShaderConstant(0, av4Null, STATEMANAGER_MAX_PCONSTANTS);

	m_bForce = false;
}

// Material
void CStateManager::SaveMaterial()
{
	m_MaterialStack.push_back(m_CurrentState.m_D3DMaterial);
}

void CStateManager::SaveMaterial(const D3DMATERIAL9* pMaterial)
{
	m_MaterialStack.push_back(m_CurrentState.m_D3DMaterial);
	SetMaterial(pMaterial);
}

void CStateManager::RestoreMaterial()
{
	if (m_MaterialStack.empty())
		return;

	SetMaterial(&m_MaterialStack.back());
	m_MaterialStack.pop_back();
}

void CStateManager::SetMaterial(const D3DMATERIAL9* pMaterial)
{
	if (!pMaterial)
		return;

	if (!m_bForce && memcmp(&m_CurrentState.m_D3DMaterial, pMaterial, sizeof(D3DMATERIAL9)) == 0)
		return;

	m_CurrentState.m_D3DMaterial = *pMaterial;
	if (m_lpD3DDev)
		m_lpD3DDev->SetMaterial(&m_CurrentState.m_D3DMaterial);
}

void CStateManager::GetMaterial(D3DMATERIAL9* pMaterial)
{
	// Set the renderstate and remember it.
	*pMaterial = m_CurrentState.m_D3DMaterial;
}

// Renderstates
DWORD CStateManager::GetRenderState(D3DRENDERSTATETYPE Type)
{
	return m_CurrentState.m_RenderStates[Type];
}

LPDIRECT3DDEVICE9EX CStateManager::GetDevice()
{
	return m_lpD3DDev;
}

void CStateManager::SaveRenderState(D3DRENDERSTATETYPE Type, DWORD dwValue)
{
	m_RenderStateStack[Type].push_back(m_CurrentState.m_RenderStates[Type]);
	SetRenderState(Type, dwValue);
}

void CStateManager::RestoreRenderState(D3DRENDERSTATETYPE Type)
{
#ifdef _DEBUG
	if (m_RenderStateStack[Type].empty())
	{
		Tracef(" CStateManager::RestoreRenderState - This render state was not saved [%d]\n", Type);
		StateManager_Assert(!" This render state was not saved!");
	}
#endif // _DEBUG

	if (m_RenderStateStack[Type].empty())
		return;

	SetRenderState(Type, m_RenderStateStack[Type].back());
	m_RenderStateStack[Type].pop_back();
}

void CStateManager::SetRenderState(D3DRENDERSTATETYPE Type, DWORD Value)
{
	if (!m_bForce && m_CurrentState.m_RenderStates[Type] == Value)
		return;

	if (m_lpD3DDev)
		m_lpD3DDev->SetRenderState(Type, Value);
	m_CurrentState.m_RenderStates[Type] = Value;
}

void CStateManager::GetRenderState(D3DRENDERSTATETYPE Type, DWORD* pdwValue)
{
	*pdwValue = m_CurrentState.m_RenderStates[Type];
}

// Textures
bool CStateManager::IsValidTexturePointer(LPDIRECT3DBASETEXTURE9 pTexture)
{
	if (!pTexture)
		return false;

	const uintptr_t uPtr = reinterpret_cast<uintptr_t>(pTexture);
	// Zakres przestrzeni adresowej uzytkownika w architekturze x64 / x86
	if (uPtr < 0x10000 || uPtr >= 0x00007FFFFFFFFFFF)
		return false;

	// Wskaznik na interfejs COM musi byc naturalnie wyrownany w pamieci
	if ((uPtr & (sizeof(void*) - 1)) != 0)
		return false;

	// Weryfikacja czy wskaznik wskazuje na zaalokowany blok pamieci z prawem odczytu (bez wyzwalania wyjatkow CPU i VEH)
	MEMORY_BASIC_INFORMATION mbi;
	if (VirtualQuery(reinterpret_cast<const void*>(uPtr), &mbi, sizeof(mbi)) == 0)
		return false;

	if (mbi.State != MEM_COMMIT)
		return false;

	if (mbi.Protect & (PAGE_NOACCESS | PAGE_GUARD))
		return false;

	constexpr DWORD kReadableFlags = PAGE_READONLY | PAGE_READWRITE | PAGE_WRITECOPY |
	                                 PAGE_EXECUTE_READ | PAGE_EXECUTE_READWRITE | PAGE_EXECUTE_WRITECOPY;
	if ((mbi.Protect & kReadableFlags) == 0)
		return false;

	__try
	{
		// Pierwsze pole interfejsu COM to wskaznik na tablice funkcji wirtualnych (vtable)
		void* const* pVtbl = *reinterpret_cast<void* const* const*>(uPtr);
		const uintptr_t uVtbl = reinterpret_cast<uintptr_t>(pVtbl);
		if (!pVtbl || uVtbl < 0x10000 || uVtbl >= 0x00007FFFFFFFFFFF || (uVtbl & (sizeof(void*) - 1)) != 0)
			return false;

		// Sprawdzenie czy poczatek vtable (QueryInterface, AddRef, Release) lezy w zaalokowanej pamieci
		MEMORY_BASIC_INFORMATION mbiVtbl;
		if (VirtualQuery(reinterpret_cast<const void*>(uVtbl), &mbiVtbl, sizeof(mbiVtbl)) == 0)
			return false;

		if (mbiVtbl.State != MEM_COMMIT || (mbiVtbl.Protect & (PAGE_NOACCESS | PAGE_GUARD)))
			return false;

		if ((mbiVtbl.Protect & kReadableFlags) == 0)
			return false;

		// Wskaznik na pierwsza metode w vtable musi wskazywac na kod wykonywalny w przestrzeni uzytkownika
		const uintptr_t uFn = reinterpret_cast<uintptr_t>(pVtbl[0]);
		if (uFn < 0x10000 || uFn >= 0x00007FFFFFFFFFFF)
			return false;

		return true;
	}
	__except (EXCEPTION_EXECUTE_HANDLER)
	{
		return false;
	}
}

void CStateManager::SaveTexture(DWORD dwStage, LPDIRECT3DBASETEXTURE9 pTexture)
{
	if (dwStage >= STATEMANAGER_MAX_STAGES)
		return;

	m_TextureStack[dwStage].push_back(m_CurrentState.m_Textures[dwStage]);
	SetTexture(dwStage, pTexture);
}

void CStateManager::RestoreTexture(DWORD dwStage)
{
	if (dwStage >= STATEMANAGER_MAX_STAGES)
		return;

	if (m_TextureStack[dwStage].empty())
		return;

	SetTexture(dwStage, m_TextureStack[dwStage].back());
	m_TextureStack[dwStage].pop_back();
}

void CStateManager::SetTexture(DWORD dwStage, LPDIRECT3DBASETEXTURE9 pTexture)
{
	if (dwStage >= STATEMANAGER_MAX_STAGES)
		return;

	if (!m_lpD3DDev)
		return;

	// Ochrona przed nullem, uszkodzonym lub wiszacym wskaznikiem tekstury COM
	if (pTexture != nullptr && !IsValidTexturePointer(pTexture))
	{
		pTexture = nullptr;
	}

	if (!m_bForce && pTexture == m_CurrentState.m_Textures[dwStage])
		return;

	__try
	{
		HRESULT hr = m_lpD3DDev->SetTexture(dwStage, pTexture);
		if (FAILED(hr))
		{
			m_lpD3DDev->SetTexture(dwStage, nullptr);
			m_CurrentState.m_Textures[dwStage] = nullptr;
			return;
		}
		m_CurrentState.m_Textures[dwStage] = pTexture;
	}
	__except (EXCEPTION_EXECUTE_HANDLER)
	{
		// W razie wyjatku sprzetowego w d3d9.dll bezpiecznie odpinamy teksture
		__try
		{
			m_lpD3DDev->SetTexture(dwStage, nullptr);
		}
		__except (EXCEPTION_EXECUTE_HANDLER)
		{
		}
		m_CurrentState.m_Textures[dwStage] = nullptr;
	}
}

void CStateManager::GetTexture(DWORD dwStage, LPDIRECT3DBASETEXTURE9* ppTexture)
{
	if (dwStage >= STATEMANAGER_MAX_STAGES || !ppTexture)
		return;

	*ppTexture = m_CurrentState.m_Textures[dwStage];
}

// Texture stage states
void CStateManager::SaveTextureStageState(DWORD dwStage, D3DTEXTURESTAGESTATETYPE Type, DWORD dwValue)
{
	m_TextureStageStateStack[dwStage][Type].push_back(m_CurrentState.m_TextureStates[dwStage][Type]);
	SetTextureStageState(dwStage, Type, dwValue);
}

void CStateManager::RestoreTextureStageState(DWORD dwStage, D3DTEXTURESTAGESTATETYPE Type)
{
#ifdef _DEBUG
	if (m_TextureStageStateStack[dwStage][Type].empty())
	{
		Tracef(" CStateManager::RestoreTextureStageState - This texture stage state was not saved [%d, %d]\n", dwStage, Type);
		StateManager_Assert(!" This texture stage state was not saved!");
	}
#endif // _DEBUG

	if (m_TextureStageStateStack[dwStage][Type].empty())
		return;

	SetTextureStageState(dwStage, Type, m_TextureStageStateStack[dwStage][Type].back());
	m_TextureStageStateStack[dwStage][Type].pop_back();
}

void CStateManager::SetTextureStageState(DWORD dwStage, D3DTEXTURESTAGESTATETYPE Type, DWORD dwValue)
{
	if (!m_bForce && m_CurrentState.m_TextureStates[dwStage][Type] == dwValue)
		return;

	if (m_lpD3DDev)
		m_lpD3DDev->SetTextureStageState(dwStage, Type, dwValue);
	m_CurrentState.m_TextureStates[dwStage][Type] = dwValue;
}

void CStateManager::GetTextureStageState(DWORD dwStage, D3DTEXTURESTAGESTATETYPE Type, DWORD* pdwValue)
{
	*pdwValue = m_CurrentState.m_TextureStates[dwStage][Type];
}

// Sampler states
void CStateManager::SaveSamplerState(DWORD dwStage, D3DSAMPLERSTATETYPE Type, DWORD dwValue)
{
	m_SamplerStateStack[dwStage][Type].push_back(m_CurrentState.m_SamplerStates[dwStage][Type]);
	SetSamplerState(dwStage, Type, dwValue);
}
void CStateManager::RestoreSamplerState(DWORD dwStage, D3DSAMPLERSTATETYPE Type)
{
#ifdef _DEBUG
	if (m_SamplerStateStack[dwStage][Type].empty())
	{
		Tracef(" CStateManager::RestoreSamplerState - This sampler state was not saved [%d, %d]\n", dwStage, Type);
		StateManager_Assert(!" This sampler state was not saved!");
	}
#endif // _DEBUG

	if (m_SamplerStateStack[dwStage][Type].empty())
		return;

	SetSamplerState(dwStage, Type, m_SamplerStateStack[dwStage][Type].back());
	m_SamplerStateStack[dwStage][Type].pop_back();
}
void CStateManager::SetSamplerState(DWORD dwStage, D3DSAMPLERSTATETYPE Type, DWORD dwValue)
{
	if (!m_bForce && m_CurrentState.m_SamplerStates[dwStage][Type] == dwValue)
		return;
	if (m_lpD3DDev)
		m_lpD3DDev->SetSamplerState(dwStage, Type, dwValue);
	m_CurrentState.m_SamplerStates[dwStage][Type] = dwValue;
}
void CStateManager::GetSamplerState(DWORD dwStage, D3DSAMPLERSTATETYPE Type, DWORD* pdwValue)
{
	*pdwValue = m_CurrentState.m_SamplerStates[dwStage][Type];
}

// Vertex Shader
void CStateManager::SaveVertexShader(LPDIRECT3DVERTEXSHADER9 dwShader)
{
	m_VertexShaderStack.push_back(m_CurrentState.m_dwVertexShader);
	SetVertexShader(dwShader);
}

void CStateManager::RestoreVertexShader()
{
	if (m_VertexShaderStack.empty())
		return;

	SetVertexShader(m_VertexShaderStack.back());
	m_VertexShaderStack.pop_back();
}

void CStateManager::SetVertexShader(LPDIRECT3DVERTEXSHADER9 dwShader)
{
	if (!m_bForce && m_CurrentState.m_dwVertexShader == dwShader)
		return;

	if (m_lpD3DDev)
		m_lpD3DDev->SetVertexShader(dwShader);
	m_CurrentState.m_dwVertexShader = dwShader;
}

void CStateManager::GetVertexShader(LPDIRECT3DVERTEXSHADER9* pdwShader)
{
	*pdwShader = m_CurrentState.m_dwVertexShader;
}

// Vertex Processing
void CStateManager::SaveVertexProcessing(BOOL IsON)
{
	m_VertexProcessingStack.push_back(m_CurrentState.m_bVertexProcessing);
	m_lpD3DDev->SetSoftwareVertexProcessing(IsON);
	m_CurrentState.m_bVertexProcessing = IsON;
}
// Vertex Declaration
void CStateManager::SaveVertexDeclaration(LPDIRECT3DVERTEXDECLARATION9 dwShader)
{
	m_VertexDeclarationStack.push_back(m_CurrentState.m_dwVertexDeclaration);
	SetVertexDeclaration(dwShader);
}
void CStateManager::RestoreVertexDeclaration()
{
	if (m_VertexDeclarationStack.empty())
		return;

	SetVertexDeclaration(m_VertexDeclarationStack.back());
	m_VertexDeclarationStack.pop_back();
}
void CStateManager::SetVertexDeclaration(LPDIRECT3DVERTEXDECLARATION9 dwShader)
{
	if (!m_bForce && m_CurrentState.m_dwVertexDeclaration == dwShader && m_CurrentState.m_dwFVF == 0)
		return;

	if (m_CurrentState.m_dwFVF != 0)
	{
		if (m_lpD3DDev)
			m_lpD3DDev->SetFVF(0);
		m_CurrentState.m_dwFVF = 0;
	}
	if (dwShader != NULL)
	{
		if (m_lpD3DDev)
			m_lpD3DDev->SetVertexDeclaration(dwShader);
	}
	m_CurrentState.m_dwVertexDeclaration = dwShader;
}
void CStateManager::GetVertexDeclaration(LPDIRECT3DVERTEXDECLARATION9* pdwShader)
{
	*pdwShader = m_CurrentState.m_dwVertexDeclaration;
}
// FVF
void CStateManager::SaveFVF(DWORD dwShader)
{
	m_FVFStack.push_back(m_CurrentState.m_dwFVF);
	SetFVF(dwShader);
}
void CStateManager::RestoreFVF()
{
	if (m_FVFStack.empty())
		return;

	SetFVF(m_FVFStack.back());
	m_FVFStack.pop_back();
}
void CStateManager::SetFVF(DWORD dwShader)
{
	if (!m_bForce && m_CurrentState.m_dwFVF == dwShader && m_CurrentState.m_dwVertexDeclaration == NULL)
		return;

	// NOTE: In Direct3D 9, SetFVF automatically unbinds any active vertex declaration on hardware.
	// Calling SetVertexDeclaration(NULL) on IDirect3DDevice9 is illegal and returns D3DERR_INVALIDCALL.
	m_CurrentState.m_dwVertexDeclaration = NULL;

	if (m_CurrentState.m_dwVertexShader != NULL)
	{
		if (m_lpD3DDev)
			m_lpD3DDev->SetVertexShader(NULL);
		m_CurrentState.m_dwVertexShader = NULL;
	}

	if (m_lpD3DDev)
		m_lpD3DDev->SetFVF(dwShader);
	m_CurrentState.m_dwFVF = dwShader;
}
void CStateManager::GetFVF(DWORD* pdwShader)
{
	*pdwShader = m_CurrentState.m_dwFVF;
}

// Pixel Shader
void CStateManager::SavePixelShader(LPDIRECT3DPIXELSHADER9 dwShader)
{
	m_PixelShaderStack.push_back(m_CurrentState.m_dwPixelShader);
	SetPixelShader(dwShader);
}

void CStateManager::RestorePixelShader()
{
	if (m_PixelShaderStack.empty())
		return;

	SetPixelShader(m_PixelShaderStack.back());
	m_PixelShaderStack.pop_back();
}

void CStateManager::SetPixelShader(LPDIRECT3DPIXELSHADER9 dwShader)
{
	if (!m_bForce && m_CurrentState.m_dwPixelShader == dwShader)
		return;

	if (m_lpD3DDev)
		m_lpD3DDev->SetPixelShader(dwShader);
	m_CurrentState.m_dwPixelShader = dwShader;
}

void CStateManager::GetPixelShader(LPDIRECT3DPIXELSHADER9* pdwShader)
{
	*pdwShader = m_CurrentState.m_dwPixelShader;
}

// *** These states are cached, but not protected from multiple sends of the same value.
// Transform
void CStateManager::SaveTransform(D3DTRANSFORMSTATETYPE Type, const D3DXMATRIX* pMatrix)
{
	m_TransformStack[Type].push_back(m_CurrentState.m_Matrices[Type]);
	SetTransform(Type, pMatrix);
}

void CStateManager::RestoreTransform(D3DTRANSFORMSTATETYPE Type)
{
#ifdef _DEBUG
	if (m_TransformStack[Type].empty())
	{
		Tracef(" CStateManager::RestoreTransform - This transform was not saved [%d]\n", Type);
		StateManager_Assert(!" This render state was not saved!");
	}
#endif // _DEBUG

	if (m_TransformStack[Type].empty())
		return;

	SetTransform(Type, &m_TransformStack[Type].back());
	m_TransformStack[Type].pop_back();
}

// Cache-check transformation matrices to prevent redundant driver calls
void CStateManager::SetTransform(D3DTRANSFORMSTATETYPE Type, const D3DXMATRIX* pMatrix)
{
	if (!pMatrix)
		return;

	if (Type < STATEMANAGER_MAX_TRANSFORMSTATES)
	{
		if (!m_bForce)
		{
			if (memcmp(&m_CurrentState.m_Matrices[Type], pMatrix, sizeof(D3DXMATRIX)) == 0)
				return;
		}

		m_CurrentState.m_Matrices[Type] = *pMatrix;
	}

	if (m_lpD3DDev)
	{
		m_lpD3DDev->SetTransform(Type, pMatrix);
	}
}

void CStateManager::GetTransform(D3DTRANSFORMSTATETYPE Type, D3DXMATRIX* pMatrix)
{
	*pMatrix = m_CurrentState.m_Matrices[Type];
}

void CStateManager::SetVertexShaderConstant(DWORD dwRegister, CONST void* pConstantData, DWORD dwConstantCount)
{
	m_lpD3DDev->SetVertexShaderConstantF(dwRegister, (const float*)pConstantData, dwConstantCount);
}

void CStateManager::SetPixelShaderConstant(DWORD dwRegister, CONST void* pConstantData, DWORD dwConstantCount)
{
	m_lpD3DDev->SetPixelShaderConstantF(dwRegister, (const float*)pConstantData, dwConstantCount);
}

void CStateManager::SaveStreamSource(UINT StreamNumber, LPDIRECT3DVERTEXBUFFER9 pStreamData, UINT Stride)
{
	m_StreamStack[StreamNumber].push_back(m_CurrentState.m_StreamData[StreamNumber]);
	SetStreamSource(StreamNumber, pStreamData, Stride);
}

void CStateManager::RestoreStreamSource(UINT StreamNumber)
{
	if (m_StreamStack[StreamNumber].empty())
		return;

	const auto& topStream = m_StreamStack[StreamNumber].back();
	SetStreamSource(StreamNumber,
		topStream.m_lpStreamData,
		topStream.m_Stride);
	m_StreamStack[StreamNumber].pop_back();
}

void CStateManager::SetStreamSource(UINT StreamNumber, LPDIRECT3DVERTEXBUFFER9 pStreamData, UINT Stride)
{
	if (StreamNumber >= STATEMANAGER_MAX_STREAMS)
		return;

	CStreamData kStreamData(pStreamData, Stride);
	if (!m_bForce && m_CurrentState.m_StreamData[StreamNumber] == kStreamData)
		return;

	if (m_lpD3DDev)
		m_lpD3DDev->SetStreamSource(StreamNumber, pStreamData, 0, Stride);
	m_CurrentState.m_StreamData[StreamNumber] = kStreamData;
}

void CStateManager::SaveIndices(LPDIRECT3DINDEXBUFFER9 pIndexData, UINT BaseVertexIndex)
{
	m_IndexStack.push_back(m_CurrentState.m_IndexData);
	SetIndices(pIndexData, BaseVertexIndex);
}

void CStateManager::RestoreIndices()
{
	if (m_IndexStack.empty())
		return;

	const auto& topIndex = m_IndexStack.back();
	SetIndices(topIndex.m_lpIndexData, topIndex.m_BaseVertexIndex);
	m_IndexStack.pop_back();
}

void CStateManager::SetIndices(LPDIRECT3DINDEXBUFFER9 pIndexData, UINT BaseVertexIndex)
{
	CIndexData kIndexData(pIndexData, BaseVertexIndex);

	if (!m_bForce && m_CurrentState.m_IndexData == kIndexData)
		return;

	if (m_lpD3DDev)
		m_lpD3DDev->SetIndices(pIndexData);
	m_CurrentState.m_IndexData = kIndexData;
}

HRESULT CStateManager::DrawPrimitive(D3DPRIMITIVETYPE PrimitiveType, UINT StartVertex, UINT PrimitiveCount)
{
	++m_iDrawCallCount;
	m_dwPrimitiveCount += PrimitiveCount;
	RecordDrawCall("DrawPrimitive");
	CRenderQueueManager::Instance().RecordDrawCall(m_currentRenderCategory, PrimitiveCount);

	return (m_lpD3DDev->DrawPrimitive(PrimitiveType, StartVertex, PrimitiveCount));
}

HRESULT CStateManager::DrawPrimitiveUP(D3DPRIMITIVETYPE PrimitiveType, UINT PrimitiveCount, const void* pVertexStreamZeroData, UINT VertexStreamZeroStride)
{
	++m_iDrawCallCount;
	m_dwPrimitiveCount += PrimitiveCount;
	RecordDrawCall("DrawPrimitiveUP");
	CRenderQueueManager::Instance().RecordDrawCall(m_currentRenderCategory, PrimitiveCount);

	m_CurrentState.m_StreamData[0] = NULL;
	return (m_lpD3DDev->DrawPrimitiveUP(PrimitiveType, PrimitiveCount, pVertexStreamZeroData, VertexStreamZeroStride));
}

HRESULT CStateManager::DrawIndexedPrimitive(D3DPRIMITIVETYPE PrimitiveType, UINT minIndex, UINT NumVertices, UINT startIndex, UINT primCount)
{
	++m_iDrawCallCount;
	m_dwPrimitiveCount += primCount;
	RecordDrawCall("DrawIndexedPrimitive");
	CRenderQueueManager::Instance().RecordDrawCall(m_currentRenderCategory, primCount);

	return (m_lpD3DDev->DrawIndexedPrimitive(PrimitiveType, 0, minIndex, NumVertices, startIndex, primCount));
}

HRESULT CStateManager::DrawIndexedPrimitive(D3DPRIMITIVETYPE PrimitiveType, INT baseVertexIndex, UINT minIndex, UINT NumVertices, UINT startIndex, UINT primCount)
{
	++m_iDrawCallCount;
	m_dwPrimitiveCount += primCount;
	RecordDrawCall("DrawIndexedPrimitive");
	CRenderQueueManager::Instance().RecordDrawCall(m_currentRenderCategory, primCount);

	return (m_lpD3DDev->DrawIndexedPrimitive(PrimitiveType, baseVertexIndex, minIndex, NumVertices, startIndex, primCount));
}

HRESULT CStateManager::DrawIndexedPrimitiveUP(D3DPRIMITIVETYPE PrimitiveType, UINT MinVertexIndex, UINT NumVertexIndices, UINT PrimitiveCount, CONST void* pIndexData, D3DFORMAT IndexDataFormat, CONST void* pVertexStreamZeroData, UINT VertexStreamZeroStride)
{
	++m_iDrawCallCount;
	m_dwPrimitiveCount += PrimitiveCount;
	RecordDrawCall("DrawIndexedPrimitiveUP");
	CRenderQueueManager::Instance().RecordDrawCall(m_currentRenderCategory, PrimitiveCount);

	m_CurrentState.m_IndexData = NULL;
	m_CurrentState.m_StreamData[0] = NULL;
	return (m_lpD3DDev->DrawIndexedPrimitiveUP(PrimitiveType, MinVertexIndex, NumVertexIndices, PrimitiveCount, pIndexData, IndexDataFormat, pVertexStreamZeroData, VertexStreamZeroStride));
}


void CStateManager::ResetDrawCallCounter()
{
	m_iLastDrawCallCount = m_iDrawCallCount;
	m_iDrawCallCount = 0;
}

int CStateManager::GetDrawCallCount() const
{
	return m_iLastDrawCallCount;
}

void CStateManager::ResetPrimitiveCounter()
{
	m_dwLastPrimitiveCount = m_dwPrimitiveCount;
	m_dwPrimitiveCount = 0;
}

uint32_t CStateManager::GetPrimitiveCount() const
{
	return m_dwLastPrimitiveCount;
}

void CStateManager::SetCurrentRenderCategory(RenderOwnership::ERenderCategory category)
{
	m_currentRenderCategory = category;
}

RenderOwnership::ERenderCategory CStateManager::GetCurrentRenderCategory() const
{
	return m_currentRenderCategory;
}

void CStateManager::RecordDrawCall(const char* siteName)
{
	if (m_currentRenderCategory != RenderOwnership::ERenderCategory::Count)
	{
		CRenderBypassGuard* pGuard = CRenderBypassGuard::InstancePtr();
		if (pGuard && pGuard->IsEnabled())
		{
			pGuard->ValidateDraw(m_currentRenderCategory, siteName);
		}
	}
}

