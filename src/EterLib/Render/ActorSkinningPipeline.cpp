#include "StdAfx.h"
#include "ActorSkinningPipeline.h"
#include <algorithm>
#include <cmath>
#include <cstring>

#pragma comment(lib, "d3dx9.lib")

namespace EterLib::Render {

static const char* s_szActorSkinningShaderSrc = R"HLSL(
float4x4 g_matViewProj       : register(c0);
float4x3 g_matBones[60]      : register(c4);

float4   g_vLightDir         : register(c184);
float4   g_vLightDiffuse     : register(c185);
float4   g_vLightAmbient     : register(c186);
float4   g_vMaterialColor    : register(c187);
float4   g_vFogParams        : register(c188);
float4x4 g_matWorld          : register(c189);

sampler2D DiffuseTexture     : register(s0);

struct VS_INPUT
{
    float3 vPosition     : POSITION0;
    float3 vNormal       : NORMAL0;
    float2 vTexCoord     : TEXCOORD0;
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

VS_OUTPUT vs_main(VS_INPUT input)
{
    VS_OUTPUT output;

    float4 weights = max(input.vBlendWeights, 0.0f);
    float weightSum = weights.x + weights.y + weights.z + weights.w;

    float3 skinnedPos = float3(0.0f, 0.0f, 0.0f);
    float3 skinnedNormal = float3(0.0f, 0.0f, 0.0f);

    if (weightSum > 0.0001f)
    {
        weights /= weightSum;
        int4 indices = int4(round(input.vBlendIndices));

        [unroll]
        for (int i = 0; i < 4; ++i)
        {
            int boneIdx = clamp(indices[i], 0, 59);
            float w = weights[i];

            if (w > 0.0001f)
            {
                float3 localPos = mul(float4(input.vPosition, 1.0f), g_matBones[boneIdx]);
                skinnedPos += localPos * w;

                float3 localNorm = mul(input.vNormal, (float3x3)g_matBones[boneIdx]);
                skinnedNormal += localNorm * w;
            }
        }
    }
    else
    {
        skinnedPos = input.vPosition;
        skinnedNormal = input.vNormal;
    }

    float4 worldPos = mul(float4(skinnedPos, 1.0f), g_matWorld);
    skinnedNormal = normalize(mul(skinnedNormal, (float3x3)g_matWorld));

    output.vPosition = mul(worldPos, g_matViewProj);
    output.vTexCoord = input.vTexCoord;

    float nDotL = max(0.0f, dot(skinnedNormal, -g_vLightDir.xyz));
    float3 diffuseLight = saturate(g_vLightAmbient.rgb + (g_vLightDiffuse.rgb * nDotL));
    output.vColor = float4(diffuseLight * g_vMaterialColor.rgb, g_vMaterialColor.a);

    if (g_vFogParams.w > 0.5f)
    {
        float depth = output.vPosition.z;
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

ActorSkinningPipeline& ActorSkinningPipeline::Instance() noexcept {
    static ActorSkinningPipeline s_instance;
    return s_instance;
}

ActorSkinningPipeline::~ActorSkinningPipeline() {
    Shutdown();
}

bool ActorSkinningPipeline::Initialize(LPDIRECT3DDEVICE9 dev) {
    if (IsReady() && m_device == dev) {
        return true;
    }

    Shutdown();

    if (!dev) {
        return false;
    }

    m_device = dev;

    static const D3DVERTEXELEMENT9 kVertexElements[] = {
        { 0, 0,  D3DDECLTYPE_FLOAT3, D3DDECLMETHOD_DEFAULT, D3DDECLUSAGE_POSITION,     0 },
        { 0, 12, D3DDECLTYPE_FLOAT3, D3DDECLMETHOD_DEFAULT, D3DDECLUSAGE_NORMAL,       0 },
        { 0, 24, D3DDECLTYPE_FLOAT2, D3DDECLMETHOD_DEFAULT, D3DDECLUSAGE_TEXCOORD,     0 },
        { 0, 32, D3DDECLTYPE_FLOAT2, D3DDECLMETHOD_DEFAULT, D3DDECLUSAGE_TEXCOORD,     1 },
        { 0, 40, D3DDECLTYPE_FLOAT4, D3DDECLMETHOD_DEFAULT, D3DDECLUSAGE_BLENDINDICES, 0 },
        { 0, 56, D3DDECLTYPE_FLOAT4, D3DDECLMETHOD_DEFAULT, D3DDECLUSAGE_BLENDWEIGHT,  0 },
        D3DDECL_END()
    };

    if (FAILED(m_device->CreateVertexDeclaration(kVertexElements, &m_vertexDecl))) {
        Shutdown();
        return false;
    }

    LPD3DXBUFFER vsCode = nullptr;
    LPD3DXBUFFER psCode = nullptr;
    LPD3DXBUFFER errBuf = nullptr;

    HRESULT hrVS = D3DXCompileShader(
        s_szActorSkinningShaderSrc,
        static_cast<UINT>(std::strlen(s_szActorSkinningShaderSrc)),
        nullptr,
        nullptr,
        "vs_main",
        "vs_3_0",
        0,
        &vsCode,
        &errBuf,
        nullptr);

    if (errBuf) {
        errBuf->Release();
        errBuf = nullptr;
    }

    if (SUCCEEDED(hrVS) && vsCode) {
        m_device->CreateVertexShader(reinterpret_cast<const DWORD*>(vsCode->GetBufferPointer()), &m_vertexShader);
        vsCode->Release();
    }

    HRESULT hrPS = D3DXCompileShader(
        s_szActorSkinningShaderSrc,
        static_cast<UINT>(std::strlen(s_szActorSkinningShaderSrc)),
        nullptr,
        nullptr,
        "ps_main",
        "ps_3_0",
        0,
        &psCode,
        &errBuf,
        nullptr);

    if (errBuf) {
        errBuf->Release();
        errBuf = nullptr;
    }

    if (SUCCEEDED(hrPS) && psCode) {
        m_device->CreatePixelShader(reinterpret_cast<const DWORD*>(psCode->GetBufferPointer()), &m_pixelShader);
        psCode->Release();
    }

    if (!IsReady()) {
        Shutdown();
        return false;
    }

    return true;
}

void ActorSkinningPipeline::Shutdown() noexcept {
    if (m_inPass) {
        End();
    }
    if (m_pixelShader) {
        m_pixelShader->Release();
        m_pixelShader = nullptr;
    }
    if (m_vertexShader) {
        m_vertexShader->Release();
        m_vertexShader = nullptr;
    }
    if (m_vertexDecl) {
        m_vertexDecl->Release();
        m_vertexDecl = nullptr;
    }
    m_device = nullptr;
}

void ActorSkinningPipeline::PackBonePaletteTo4x3Registers(
    const void* pMatrices,
    UINT count,
    std::vector<float>& outRegisters)
{
    const UINT clampedCount = std::min(count, ActorSkinningShaderConstants::kMaxBones);
    if (!pMatrices || clampedCount == 0) {
        outRegisters.assign(12, 0.0f);
        outRegisters[0]  = 1.0f;
        outRegisters[5]  = 1.0f;
        outRegisters[10] = 1.0f;
        return;
    }

    outRegisters.resize(clampedCount * 12, 0.0f);
    const float* src = static_cast<const float*>(pMatrices);

    for (UINT i = 0; i < clampedCount; ++i) {
        const float* m = src + (i * 16);
        float* dst = outRegisters.data() + (i * 12);

        // Detect whether translation is stored in row 3 (m[12..14], D3DX row-major)
        // or column 3 (m[3], m[7], m[11], column-vector convention).
        const bool hasRow3Translation =
            (std::fabs(m[12]) > 1e-6f || std::fabs(m[13]) > 1e-6f || std::fabs(m[14]) > 1e-6f) &&
            (std::fabs(m[3]) < 1e-6f && std::fabs(m[7]) < 1e-6f && std::fabs(m[11]) < 1e-6f);

        if (hasRow3Translation) {
            // Transpose 4x3 into 3 column registers for HLSL mul(float4(pos,1), g_matBones[i])
            dst[0]  = m[0]; dst[1]  = m[4]; dst[2]  = m[8];  dst[3]  = m[12];
            dst[4]  = m[1]; dst[5]  = m[5]; dst[6]  = m[9];  dst[7]  = m[13];
            dst[8]  = m[2]; dst[9]  = m[6]; dst[10] = m[10]; dst[11] = m[14];
        } else {
            // Rows 0..2 already hold (basis.xyz, translation.w)
            dst[0]  = m[0]; dst[1]  = m[1]; dst[2]  = m[2];  dst[3]  = m[3];
            dst[4]  = m[4]; dst[5]  = m[5]; dst[6]  = m[6];  dst[7]  = m[7];
            dst[8]  = m[8]; dst[9]  = m[9]; dst[10] = m[10]; dst[11] = m[11];
        }
    }
}

bool ActorSkinningPipeline::Begin(const ActorSkinningBindDesc& desc) noexcept {
    return Begin(desc.worldMatrix, desc.viewProjMatrix, &desc);
}

bool ActorSkinningPipeline::Begin(
    const D3DXMATRIX& matWorld,
    const D3DXMATRIX& matViewProj,
    const ActorSkinningBindDesc* pOptionalLighting) noexcept
{
    if (!IsReady() || !m_device) {
        return false;
    }

    m_device->GetVertexDeclaration(&m_prevVertexDecl);
    m_device->GetVertexShader(&m_prevVertexShader);
    m_device->GetPixelShader(&m_prevPixelShader);

    m_device->SetVertexDeclaration(m_vertexDecl);
    m_device->SetVertexShader(m_vertexShader);
    m_device->SetPixelShader(m_pixelShader);

    D3DXMATRIX matVPTransposed;
    D3DXMATRIX matWorldTransposed;
    D3DXMatrixTranspose(&matVPTransposed, &matViewProj);
    D3DXMatrixTranspose(&matWorldTransposed, &matWorld);

    m_device->SetVertexShaderConstantF(
        ActorSkinningShaderConstants::kRegViewProj,
        reinterpret_cast<const float*>(&matVPTransposed),
        4);

    m_device->SetVertexShaderConstantF(
        ActorSkinningShaderConstants::kRegWorld,
        reinterpret_cast<const float*>(&matWorldTransposed),
        4);

    ActorSkinningBindDesc defaultDesc;
    const ActorSkinningBindDesc& light = pOptionalLighting ? *pOptionalLighting : defaultDesc;

    m_device->SetVertexShaderConstantF(
        ActorSkinningShaderConstants::kRegLightDir,
        reinterpret_cast<const float*>(&light.lightDir),
        1);
    m_device->SetVertexShaderConstantF(
        ActorSkinningShaderConstants::kRegLightDiffuse,
        reinterpret_cast<const float*>(&light.lightDiffuse),
        1);
    m_device->SetVertexShaderConstantF(
        ActorSkinningShaderConstants::kRegLightAmbient,
        reinterpret_cast<const float*>(&light.lightAmbient),
        1);
    m_device->SetVertexShaderConstantF(
        ActorSkinningShaderConstants::kRegMaterialColor,
        reinterpret_cast<const float*>(&light.materialColor),
        1);
    m_device->SetVertexShaderConstantF(
        ActorSkinningShaderConstants::kRegFogParams,
        reinterpret_cast<const float*>(&light.fogParams),
        1);

    m_inPass = true;
    return true;
}

void ActorSkinningPipeline::UploadBonePalette(const void* pMatrices, UINT count) noexcept {
    if (!m_inPass || !m_device) {
        return;
    }

    std::vector<float> packedRegisters;
    PackBonePaletteTo4x3Registers(pMatrices, count, packedRegisters);

    const UINT numBonesUploaded = static_cast<UINT>(packedRegisters.size() / 12);
    m_device->SetVertexShaderConstantF(
        ActorSkinningShaderConstants::kRegBonePalette,
        packedRegisters.data(),
        numBonesUploaded * ActorSkinningShaderConstants::kRegsPerBone);
}

void ActorSkinningPipeline::End() noexcept {
    if (!m_inPass || !m_device) {
        return;
    }

    m_device->SetVertexShader(m_prevVertexShader);
    m_device->SetPixelShader(m_prevPixelShader);
    m_device->SetVertexDeclaration(m_prevVertexDecl);

    if (m_prevVertexShader) {
        m_prevVertexShader->Release();
        m_prevVertexShader = nullptr;
    }
    if (m_prevPixelShader) {
        m_prevPixelShader->Release();
        m_prevPixelShader = nullptr;
    }
    if (m_prevVertexDecl) {
        m_prevVertexDecl->Release();
        m_prevVertexDecl = nullptr;
    }

    m_inPass = false;
}

} // namespace EterLib::Render
