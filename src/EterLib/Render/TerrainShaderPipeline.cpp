#ifndef TEST_MODE_DISABLE_STDAFX
#include "../StdAfx.h"
#endif

#include "TerrainShaderPipeline.h"
#include "../StateManager.h"
#include <d3dx9.h>
#include <string_view>

namespace EterLib::Render {

namespace {

// row_major gwarantuje pelna zgodnosc z ukladem pamieci D3DMATRIX w C++ bez narzutu transpozycji
const char* g_terrainShaderHLSL = R"(
row_major float4x4 ViewProj : register(c0);
float4 SectorParams : register(c8); // xy = sectorOriginWorld, z = invSectorSize, w = detailTileScale

struct VS_INPUT {
    float3 Pos    : POSITION;
    float3 Normal : NORMAL;
};

struct VS_OUTPUT {
    float4 Pos         : POSITION;
    float2 SplatUV     : TEXCOORD0;
    float2 DetailUV    : TEXCOORD1;
    float3 NormalWorld : TEXCOORD2;
    float  ViewDepth   : TEXCOORD3;
};

VS_OUTPUT VS_Main(VS_INPUT input) {
    VS_OUTPUT output;
    float4 worldPos = float4(input.Pos, 1.0f);
    output.Pos = mul(worldPos, ViewProj);

    // Lokalna wspolrzedna UV [0..1] wzgledem poczatku sektora
    float2 localPos = float2(input.Pos.x - SectorParams.x, -input.Pos.y - SectorParams.y);
    output.SplatUV = localPos * SectorParams.z;
    output.DetailUV = output.SplatUV * SectorParams.w;

    output.NormalWorld = input.Normal;
    output.ViewDepth = output.Pos.w;
    return output;
}

sampler2D TexSplat  : register(s0);
sampler2D TexLayer0 : register(s1);
sampler2D TexLayer1 : register(s2);
sampler2D TexLayer2 : register(s3);
sampler2D TexLayer3 : register(s4);

float4 PS_LightDir     : register(c0);
float4 PS_AmbientColor : register(c1);
float4 PS_FogParams    : register(c2); // rgb = fogColor, w = invFogEnd

float4 PS_Main(VS_OUTPUT input) : COLOR {
    float4 weights = tex2D(TexSplat, input.SplatUV);

    // Normalizacja wag zapobiega ciemnym szwom na granicach filtrowania
    float weightSum = dot(weights, float4(1.0f, 1.0f, 1.0f, 1.0f));
    weights = (weightSum > 0.001f) ? (weights / weightSum) : float4(1.0f, 0.0f, 0.0f, 0.0f);

    float4 c0 = tex2D(TexLayer0, input.DetailUV);
    float4 c1 = tex2D(TexLayer1, input.DetailUV);
    float4 c2 = tex2D(TexLayer2, input.DetailUV);
    float4 c3 = tex2D(TexLayer3, input.DetailUV);

    float4 albedo = c0 * weights.r + c1 * weights.g + c2 * weights.b + c3 * weights.a;

    float3 normal = normalize(input.NormalWorld);
    float nDotL = saturate(dot(normal, -PS_LightDir.xyz));
    float3 litColor = albedo.rgb * saturate(PS_AmbientColor.rgb + nDotL);

    // Liniowa mgla odleglosciowa
    float fogFactor = saturate(input.ViewDepth * PS_FogParams.w);
    float3 finalRgb = lerp(litColor, PS_FogParams.rgb, fogFactor);

    return float4(finalRgb, 1.0f);
}
)";

} // namespace

TerrainShaderPipeline::~TerrainShaderPipeline() {
    Shutdown();
}

void TerrainShaderPipeline::Shutdown() noexcept {
    m_inFrame = false;
    if (m_fallbackWhiteTex) {
        m_fallbackWhiteTex->Release();
        m_fallbackWhiteTex = nullptr;
    }
    if (m_vertexShader) {
        m_vertexShader->Release();
        m_vertexShader = nullptr;
    }
    if (m_pixelShader) {
        m_pixelShader->Release();
        m_pixelShader = nullptr;
    }
    if (m_vertexDecl) {
        m_vertexDecl->Release();
        m_vertexDecl = nullptr;
    }
}

bool TerrainShaderPipeline::Initialize(LPDIRECT3DDEVICE9 dev) {
    if (!dev) return false;
    if (IsReady()) return true;

    Shutdown();

    constexpr D3DVERTEXELEMENT9 declElements[] = {
        {0, 0,  D3DDECLTYPE_FLOAT3, D3DDECLMETHOD_DEFAULT, D3DDECLUSAGE_POSITION, 0},
        {0, 12, D3DDECLTYPE_FLOAT3, D3DDECLMETHOD_DEFAULT, D3DDECLUSAGE_NORMAL,   0},
        D3DDECL_END()
    };

    if (FAILED(dev->CreateVertexDeclaration(declElements, &m_vertexDecl))) {
        Shutdown();
        return false;
    }

    ID3DXBuffer* vsCode = nullptr;
    ID3DXBuffer* psCode = nullptr;
    ID3DXBuffer* errorMsgs = nullptr;
    const UINT srcLen = static_cast<UINT>(std::string_view(g_terrainShaderHLSL).length());

    if (FAILED(D3DXCompileShader(g_terrainShaderHLSL, srcLen, nullptr, nullptr, "VS_Main", "vs_3_0", 0, &vsCode, &errorMsgs, nullptr))) {
        if (errorMsgs) errorMsgs->Release();
        Shutdown();
        return false;
    }
    const HRESULT hrVS = dev->CreateVertexShader(static_cast<const DWORD*>(vsCode->GetBufferPointer()), &m_vertexShader);
    vsCode->Release();
    if (FAILED(hrVS)) {
        Shutdown();
        return false;
    }

    if (FAILED(D3DXCompileShader(g_terrainShaderHLSL, srcLen, nullptr, nullptr, "PS_Main", "ps_3_0", 0, &psCode, &errorMsgs, nullptr))) {
        if (errorMsgs) errorMsgs->Release();
        Shutdown();
        return false;
    }
    const HRESULT hrPS = dev->CreatePixelShader(static_cast<const DWORD*>(psCode->GetBufferPointer()), &m_pixelShader);
    psCode->Release();
    if (FAILED(hrPS)) {
        Shutdown();
        return false;
    }

    // Tekstura zapasowa 1x1 (R=255)
    if (SUCCEEDED(dev->CreateTexture(1, 1, 1, D3DUSAGE_DYNAMIC, D3DFMT_A8R8G8B8, D3DPOOL_DEFAULT, &m_fallbackWhiteTex, nullptr))) {
        D3DLOCKED_RECT lr{};
        if (SUCCEEDED(m_fallbackWhiteTex->LockRect(0, &lr, nullptr, 0))) {
            *static_cast<uint32_t*>(lr.pBits) = 0x00FF0000u; // R=1, G=0, B=0, A=0
            m_fallbackWhiteTex->UnlockRect(0);
        }
    }

    return true;
}

void TerrainShaderPipeline::BeginFrame(
    LPDIRECT3DDEVICE9 dev,
    const D3DMATRIX& viewProj,
    const D3DVECTOR& lightDir,
    const D3DVECTOR& ambientColor,
    const float (&fogColorAndRange)[4]) noexcept {
    if (!dev || !IsReady()) return;

    if (m_inFrame) {
        EndFrame(dev);
    }

    STATEMANAGER.SaveVertexDeclaration(m_vertexDecl);
    STATEMANAGER.SaveVertexShader(m_vertexShader);
    STATEMANAGER.SavePixelShader(m_pixelShader);

    STATEMANAGER.SetVertexShaderConstant(0, reinterpret_cast<const float*>(&viewProj), 4);

    const float lightData[4] = {lightDir.x, lightDir.y, lightDir.z, 0.0f};
    const float ambientData[4] = {ambientColor.x, ambientColor.y, ambientColor.z, 1.0f};
    STATEMANAGER.SetPixelShaderConstant(0, lightData, 1);
    STATEMANAGER.SetPixelShaderConstant(1, ambientData, 1);
    STATEMANAGER.SetPixelShaderConstant(2, fogColorAndRange, 1);

    // Sampler s0: Splatmapa (CLAMP)
    STATEMANAGER.SaveSamplerState(0, D3DSAMP_ADDRESSU, D3DTADDRESS_CLAMP);
    STATEMANAGER.SaveSamplerState(0, D3DSAMP_ADDRESSV, D3DTADDRESS_CLAMP);
    STATEMANAGER.SaveSamplerState(0, D3DSAMP_MINFILTER, D3DTEXF_LINEAR);
    STATEMANAGER.SaveSamplerState(0, D3DSAMP_MAGFILTER, D3DTEXF_LINEAR);

    // Samplery s1..s4: Warstwy szczegolowe (WRAP)
    for (DWORD i = 1; i <= 4; ++i) {
        STATEMANAGER.SaveSamplerState(i, D3DSAMP_ADDRESSU, D3DTADDRESS_WRAP);
        STATEMANAGER.SaveSamplerState(i, D3DSAMP_ADDRESSV, D3DTADDRESS_WRAP);
        STATEMANAGER.SaveSamplerState(i, D3DSAMP_MINFILTER, D3DTEXF_LINEAR);
        STATEMANAGER.SaveSamplerState(i, D3DSAMP_MAGFILTER, D3DTEXF_LINEAR);
        STATEMANAGER.SaveSamplerState(i, D3DSAMP_MIPFILTER, D3DTEXF_LINEAR);
    }

    // Zabezpieczenie slotow tekstur 0..4
    for (DWORD i = 0; i < 5; ++i) {
        STATEMANAGER.SaveTexture(i, nullptr);
    }

    m_inFrame = true;
}

void TerrainShaderPipeline::BindSector(LPDIRECT3DDEVICE9 dev, const TerrainSectorBindDesc& desc) noexcept {
    if (!dev || !IsReady() || !m_inFrame) return;

    const float sectorParams[4] = {
        desc.sectorOriginX,
        desc.sectorOriginY,
        desc.invSectorSize,
        desc.detailTileScale
    };
    STATEMANAGER.SetVertexShaderConstant(8, sectorParams, 1);

    LPDIRECT3DTEXTURE9 splatTex = desc.splatTexture ? desc.splatTexture : m_fallbackWhiteTex;
    STATEMANAGER.SetTexture(0, splatTex);

    for (DWORD i = 0; i < 4; ++i) {
        LPDIRECT3DTEXTURE9 layerTex = desc.layerTextures[i] ? desc.layerTextures[i] : m_fallbackWhiteTex;
        STATEMANAGER.SetTexture(i + 1, layerTex);
    }
}

void TerrainShaderPipeline::EndFrame(LPDIRECT3DDEVICE9 dev) noexcept {
    if (!dev || !m_inFrame) return;

    // 1. Przywrocenie tekstur w odwrotnej kolejnosci
    for (int i = 4; i >= 0; --i) {
        STATEMANAGER.RestoreTexture(static_cast<DWORD>(i));
    }

    // 2. Przywrocenie samplerow s1..s4 w odwrotnej kolejnosci
    for (DWORD i = 4; i >= 1; --i) {
        STATEMANAGER.RestoreSamplerState(i, D3DSAMP_MIPFILTER);
        STATEMANAGER.RestoreSamplerState(i, D3DSAMP_MAGFILTER);
        STATEMANAGER.RestoreSamplerState(i, D3DSAMP_MINFILTER);
        STATEMANAGER.RestoreSamplerState(i, D3DSAMP_ADDRESSV);
        STATEMANAGER.RestoreSamplerState(i, D3DSAMP_ADDRESSU);
    }

    // 3. Przywrocenie samplera s0 w odwrotnej kolejnosci
    STATEMANAGER.RestoreSamplerState(0, D3DSAMP_MAGFILTER);
    STATEMANAGER.RestoreSamplerState(0, D3DSAMP_MINFILTER);
    STATEMANAGER.RestoreSamplerState(0, D3DSAMP_ADDRESSV);
    STATEMANAGER.RestoreSamplerState(0, D3DSAMP_ADDRESSU);

    // 4. Przywrocenie shaderow i deklaracji wierzcholkow
    STATEMANAGER.RestorePixelShader();
    STATEMANAGER.RestoreVertexShader();
    STATEMANAGER.RestoreVertexDeclaration();

    m_inFrame = false;
}

} // namespace EterLib::Render
