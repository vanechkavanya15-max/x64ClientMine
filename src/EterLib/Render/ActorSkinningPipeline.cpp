#ifndef TEST_MODE_DISABLE_STDAFX
#include "../StdAfx.h"
#endif

#include "ActorSkinningPipeline.h"
#include "../StateManager.h"
#include <algorithm>
#include <cstring>
#include <cmath>

#pragma comment(lib, "d3dx9.lib")

namespace EterLib { namespace Render {

static const char* s_szEmbeddedActorSkinningHLSL = R"HLSL(
float4x4 g_matViewProj       : register(c0);
float4x4 g_matWorld          : register(c4);
float4   g_vLightDir         : register(c8);
float4   g_vLightDiffuse     : register(c9);
float4   g_vLightAmbient     : register(c10);
float4   g_vMaterialColor    : register(c11);
float4   g_vFogParams        : register(c12);
float4   g_vSkinningInfo     : register(c13);
float4   g_matBones[180]     : register(c16);

sampler2D DiffuseTexture     : register(s0);

struct VS_INPUT
{
    float3 vPosition     : POSITION0;
    float3 vNormal       : NORMAL0;
    float2 vTexCoord0    : TEXCOORD0;
    float2 vTexCoord1    : TEXCOORD1;
    float4 vBlendIndices : BLENDINDICES0;
    float4 vBlendWeights : BLENDWEIGHT0;
};

struct VS_OUTPUT
{
    float4 vPosition     : POSITION0;
    float2 vTexCoord     : TEXCOORD0;
    float4 vColor        : COLOR0;
    float  fFog          : FOG;
};

float3 TransformBonePosition(float3 pos, int boneIdx)
{
    int baseReg = boneIdx * 3;
    float4 r0 = g_matBones[baseReg + 0];
    float4 r1 = g_matBones[baseReg + 1];
    float4 r2 = g_matBones[baseReg + 2];
    return float3(dot(float4(pos, 1.0f), r0), dot(float4(pos, 1.0f), r1), dot(float4(pos, 1.0f), r2));
}

float3 TransformBoneNormal(float3 norm, int boneIdx)
{
    int baseReg = boneIdx * 3;
    float3 r0 = g_matBones[baseReg + 0].xyz;
    float3 r1 = g_matBones[baseReg + 1].xyz;
    float3 r2 = g_matBones[baseReg + 2].xyz;
    return float3(dot(norm, r0), dot(norm, r1), dot(norm, r2));
}

VS_OUTPUT vs_main(VS_INPUT input)
{
    VS_OUTPUT output;
    float3 localPos = input.vPosition;
    float3 localNormal = input.vNormal;

    float4 weights = max(input.vBlendWeights, 0.0f);
    float weightSum = weights.x + weights.y + weights.z + weights.w;

    float3 skinnedPos = float3(0.0f, 0.0f, 0.0f);
    float3 skinnedNormal = float3(0.0f, 0.0f, 0.0f);

    if (g_vSkinningInfo.x > 0.5f && weightSum > 0.0001f)
    {
        weights /= weightSum;
        int4 indices = int4(round(input.vBlendIndices));

        [unroll]
        for (int i = 0; i < 4; ++i)
        {
            float w = weights[i];
            if (w > 0.0001f)
            {
                int boneIdx = clamp(indices[i], 0, 59);
                skinnedPos    += TransformBonePosition(localPos, boneIdx) * w;
                skinnedNormal += TransformBoneNormal(localNormal, boneIdx) * w;
            }
        }
    }
    else
    {
        skinnedPos = localPos;
        skinnedNormal = localNormal;
    }

    float4 worldPos = mul(float4(skinnedPos, 1.0f), g_matWorld);
    float3 worldNormal = normalize(mul(skinnedNormal, (float3x3)g_matWorld));

    output.vPosition = mul(worldPos, g_matViewProj);
    output.vTexCoord = input.vTexCoord0;

    float nDotL = max(0.0f, dot(worldNormal, -normalize(g_vLightDir.xyz)));
    float3 diffuseLight = saturate(g_vLightAmbient.rgb + (g_vLightDiffuse.rgb * nDotL));
    output.vColor = float4(diffuseLight * g_vMaterialColor.rgb, g_vMaterialColor.a);

    if (g_vFogParams.w > 0.5f)
    {
        float depth = output.vPosition.w;
        output.fFog = saturate((g_vFogParams.y - depth) * g_vFogParams.z);
    }
    else
    {
        output.fFog = 1.0f;
    }

    return output;
}

struct PS_INPUT
{
    float2 vTexCoord     : TEXCOORD0;
    float4 vColor        : COLOR0;
};

float4 ps_main(PS_INPUT input) : COLOR0
{
    float4 texColor = tex2D(DiffuseTexture, input.vTexCoord);
    return texColor * input.vColor;
}
)HLSL";

ActorSkinningPipeline& ActorSkinningPipeline::Instance()
{
    static ActorSkinningPipeline s_instance;
    return s_instance;
}

ActorSkinningPipeline::ActorSkinningPipeline()
    : m_pDevice(nullptr)
    , m_pVertexDecl(nullptr)
    , m_pVertexShader(nullptr)
    , m_pPixelShader(nullptr)
    , m_pPrevVertexDecl(nullptr)
    , m_pPrevVertexShader(nullptr)
    , m_pPrevPixelShader(nullptr)
    , m_bInitialized(false)
    , m_bActivePass(false)
{
}

ActorSkinningPipeline::~ActorSkinningPipeline()
{
    Shutdown();
}

bool ActorSkinningPipeline::Initialize(LPDIRECT3DDEVICE9 pDevice)
{
    if (m_bInitialized && m_pDevice == pDevice)
        return true;

    Shutdown();

    if (!pDevice)
        return false;

    m_pDevice = pDevice;

    D3DVERTEXELEMENT9 elements[] =
    {
        { 0, 0,  D3DDECLTYPE_FLOAT3, D3DDECLMETHOD_DEFAULT, D3DDECLUSAGE_POSITION,     0 },
        { 0, 12, D3DDECLTYPE_FLOAT3, D3DDECLMETHOD_DEFAULT, D3DDECLUSAGE_NORMAL,       0 },
        { 0, 24, D3DDECLTYPE_FLOAT2, D3DDECLMETHOD_DEFAULT, D3DDECLUSAGE_TEXCOORD,     0 },
        { 0, 32, D3DDECLTYPE_FLOAT2, D3DDECLMETHOD_DEFAULT, D3DDECLUSAGE_TEXCOORD,     1 },
        { 0, 40, D3DDECLTYPE_FLOAT4, D3DDECLMETHOD_DEFAULT, D3DDECLUSAGE_BLENDINDICES, 0 },
        { 0, 56, D3DDECLTYPE_FLOAT4, D3DDECLMETHOD_DEFAULT, D3DDECLUSAGE_BLENDWEIGHT,  0 },
        D3DDECL_END()
    };

    if (FAILED(m_pDevice->CreateVertexDeclaration(elements, &m_pVertexDecl)))
    {
        Shutdown();
        return false;
    }

    LPD3DXBUFFER pVSCode = nullptr;
    LPD3DXBUFFER pPSCode = nullptr;
    LPD3DXBUFFER pErrorMsgs = nullptr;

    HRESULT hrVS = D3DXCompileShader(
        s_szEmbeddedActorSkinningHLSL,
        (UINT)std::strlen(s_szEmbeddedActorSkinningHLSL),
        nullptr, nullptr,
        "vs_main", "vs_3_0",
        0, &pVSCode, &pErrorMsgs, nullptr);

    if (pErrorMsgs)
    {
        pErrorMsgs->Release();
        pErrorMsgs = nullptr;
    }

    if (SUCCEEDED(hrVS) && pVSCode)
    {
        m_pDevice->CreateVertexShader((const DWORD*)pVSCode->GetBufferPointer(), &m_pVertexShader);
        pVSCode->Release();
    }

    HRESULT hrPS = D3DXCompileShader(
        s_szEmbeddedActorSkinningHLSL,
        (UINT)std::strlen(s_szEmbeddedActorSkinningHLSL),
        nullptr, nullptr,
        "ps_main", "ps_3_0",
        0, &pPSCode, &pErrorMsgs, nullptr);

    if (pErrorMsgs)
    {
        pErrorMsgs->Release();
        pErrorMsgs = nullptr;
    }

    if (SUCCEEDED(hrPS) && pPSCode)
    {
        m_pDevice->CreatePixelShader((const DWORD*)pPSCode->GetBufferPointer(), &m_pPixelShader);
        pPSCode->Release();
    }

    if (!m_pVertexShader || !m_pPixelShader)
    {
        Shutdown();
        return false;
    }

    m_bInitialized = true;
    return true;
}

void ActorSkinningPipeline::Shutdown()
{
    if (m_pPixelShader)
    {
        m_pPixelShader->Release();
        m_pPixelShader = nullptr;
    }

    if (m_pVertexShader)
    {
        m_pVertexShader->Release();
        m_pVertexShader = nullptr;
    }

    if (m_pVertexDecl)
    {
        m_pVertexDecl->Release();
        m_pVertexDecl = nullptr;
    }

    m_pDevice = nullptr;
    m_bInitialized = false;
    m_bActivePass = false;
}

void ActorSkinningPipeline::PackBoneMatricesTo4x3Registers(
    const float* pSourceMatrices4x4,
    UINT boneCount,
    std::vector<float>& outRegisters4x3)
{
    const UINT clampedCount = std::min(boneCount, ActorSkinningShaderConstants::kMaxBones);
    outRegisters4x3.resize(clampedCount * 12, 0.0f);

    if (!pSourceMatrices4x4 || clampedCount == 0)
        return;

    for (UINT b = 0; b < clampedCount; ++b)
    {
        const float* m = pSourceMatrices4x4 + (b * 16);
        float* dst = outRegisters4x3.data() + (b * 12);

        const bool isRowMajorTranslation =
            (std::abs(m[12]) > 1e-6f || std::abs(m[13]) > 1e-6f || std::abs(m[14]) > 1e-6f) &&
            (std::abs(m[3]) < 1e-6f && std::abs(m[7]) < 1e-6f && std::abs(m[11]) < 1e-6f);

        if (isRowMajorTranslation)
        {
            // Transpose columns into rows for dot(float4(pos, 1), r_i)
            dst[0]  = m[0]; dst[1]  = m[4]; dst[2]  = m[8];  dst[3]  = m[12];
            dst[4]  = m[1]; dst[5]  = m[5]; dst[6]  = m[9];  dst[7]  = m[13];
            dst[8]  = m[2]; dst[9]  = m[6]; dst[10] = m[10]; dst[11] = m[14];
        }
        else
        {
            // Standard row0, row1, row2 where translation is in [0][3], [1][3], [2][3]
            dst[0]  = m[0]; dst[1]  = m[1]; dst[2]  = m[2];  dst[3]  = m[3];
            dst[4]  = m[4]; dst[5]  = m[5]; dst[6]  = m[6];  dst[7]  = m[7];
            dst[8]  = m[8]; dst[9]  = m[9]; dst[10] = m[10]; dst[11] = m[11];
        }
    }
}

bool ActorSkinningPipeline::Begin(
    const D3DXMATRIX& matWorld,
    const D3DXMATRIX& matViewProj,
    const ActorLightingState* pLighting)
{
    if (!m_bInitialized || !m_pDevice || !m_pVertexShader || !m_pPixelShader)
        return false;

    m_pDevice->GetVertexDeclaration(&m_pPrevVertexDecl);
    m_pDevice->GetVertexShader(&m_pPrevVertexShader);
    m_pDevice->GetPixelShader(&m_pPrevPixelShader);

    m_pDevice->SetVertexDeclaration(m_pVertexDecl);
    m_pDevice->SetVertexShader(m_pVertexShader);
    m_pDevice->SetPixelShader(m_pPixelShader);

    D3DXMATRIX matVPTransposed, matWorldTransposed;
    D3DXMatrixTranspose(&matVPTransposed, &matViewProj);
    D3DXMatrixTranspose(&matWorldTransposed, &matWorld);

    m_pDevice->SetVertexShaderConstantF(ActorSkinningShaderConstants::kRegViewProj, (const float*)&matVPTransposed, 4);
    m_pDevice->SetVertexShaderConstantF(ActorSkinningShaderConstants::kRegWorld, (const float*)&matWorldTransposed, 4);

    ActorLightingState defaultLighting;
    const ActorLightingState& light = pLighting ? *pLighting : defaultLighting;

    m_pDevice->SetVertexShaderConstantF(ActorSkinningShaderConstants::kRegLightDir, (const float*)&light.lightDir, 1);
    m_pDevice->SetVertexShaderConstantF(ActorSkinningShaderConstants::kRegLightDiffuse, (const float*)&light.lightDiffuse, 1);
    m_pDevice->SetVertexShaderConstantF(ActorSkinningShaderConstants::kRegLightAmbient, (const float*)&light.lightAmbient, 1);
    m_pDevice->SetVertexShaderConstantF(ActorSkinningShaderConstants::kRegMaterialColor, (const float*)&light.materialColor, 1);
    m_pDevice->SetVertexShaderConstantF(ActorSkinningShaderConstants::kRegFogParams, (const float*)&light.fogParams, 1);

    D3DXVECTOR4 skinInfo(0.0f, 1.0f, 0.0f, 0.0f);
    m_pDevice->SetVertexShaderConstantF(ActorSkinningShaderConstants::kRegSkinningInfo, (const float*)&skinInfo, 1);

    m_bActivePass = true;
    return true;
}

void ActorSkinningPipeline::UploadBonePalette(const void* pMatrices4x4, UINT boneCount)
{
    if (!m_bActivePass || !m_pDevice)
        return;

    const UINT clampedBones = std::min(boneCount, ActorSkinningShaderConstants::kMaxBones);
    if (!pMatrices4x4 || clampedBones == 0)
    {
        D3DXVECTOR4 skinInfo(0.0f, 0.0f, 0.0f, 0.0f);
        m_pDevice->SetVertexShaderConstantF(ActorSkinningShaderConstants::kRegSkinningInfo, (const float*)&skinInfo, 1);
        return;
    }

    std::vector<float> packedRegs;
    PackBoneMatricesTo4x3Registers(static_cast<const float*>(pMatrices4x4), clampedBones, packedRegs);

    m_pDevice->SetVertexShaderConstantF(
        ActorSkinningShaderConstants::kRegBonePalette,
        packedRegs.data(),
        clampedBones * ActorSkinningShaderConstants::kRegsPerBone);

    D3DXVECTOR4 skinInfo((float)clampedBones, 1.0f, 0.0f, 0.0f);
    m_pDevice->SetVertexShaderConstantF(ActorSkinningShaderConstants::kRegSkinningInfo, (const float*)&skinInfo, 1);
}

void ActorSkinningPipeline::End()
{
    if (!m_bActivePass || !m_pDevice)
        return;

    m_pDevice->SetVertexShader(m_pPrevVertexShader);
    m_pDevice->SetPixelShader(m_pPrevPixelShader);
    m_pDevice->SetVertexDeclaration(m_pPrevVertexDecl);

    if (m_pPrevVertexShader)
    {
        m_pPrevVertexShader->Release();
        m_pPrevVertexShader = nullptr;
    }
    if (m_pPrevPixelShader)
    {
        m_pPrevPixelShader->Release();
        m_pPrevPixelShader = nullptr;
    }
    if (m_pPrevVertexDecl)
    {
        m_pPrevVertexDecl->Release();
        m_pPrevVertexDecl = nullptr;
    }

    m_bActivePass = false;
}

}} // namespace EterLib::Render
