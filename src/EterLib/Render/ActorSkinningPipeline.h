#pragma once

#include <d3d9.h>
#include <d3dx9.h>
#include <cstdint>
#include <vector>

namespace EterLib { namespace Render {

// Constant register layout matching src/EterLib/actor_skinning.hlsl
struct ActorSkinningShaderConstants
{
    static constexpr UINT kRegViewProj      = 0;   // c0..c3   (4 regs)
    static constexpr UINT kRegWorld         = 4;   // c4..c7   (4 regs)
    static constexpr UINT kRegLightDir      = 8;   // c8       (1 reg)
    static constexpr UINT kRegLightDiffuse  = 9;   // c9       (1 reg)
    static constexpr UINT kRegLightAmbient  = 10;  // c10      (1 reg)
    static constexpr UINT kRegMaterialColor = 11;  // c11      (1 reg)
    static constexpr UINT kRegFogParams     = 12;  // c12      (1 reg)
    static constexpr UINT kRegSkinningInfo  = 13;  // c13      (1 reg)
    static constexpr UINT kRegBonePalette   = 16;  // c16..c195 (60 * 3 = 180 regs)
    static constexpr UINT kMaxBones         = 60;
    static constexpr UINT kRegsPerBone      = 3;
    static constexpr UINT kLastBoneReg      = kRegBonePalette + (kMaxBones * kRegsPerBone) - 1; // 195
};

// GPU-friendly vertex matching VS_INPUT (72 bytes, 1:1 size with GltfVertex)
struct GpuSkinnedVertex
{
    float position[3];      // offset 0  (12B)
    float normal[3];        // offset 12 (12B)
    float uv0[2];           // offset 24 (8B)
    float uv1[2];           // offset 32 (8B)
    float jointIndices[4];  // offset 40 (16B) - converted from uint4 for D3DDECLTYPE_FLOAT4
    float jointWeights[4];  // offset 56 (16B)
};
static_assert(sizeof(GpuSkinnedVertex) == 72, "GpuSkinnedVertex must be 72 bytes");

struct ActorLightingState
{
    D3DXVECTOR4 lightDir      = D3DXVECTOR4(0.0f, -0.7071f, -0.7071f, 0.0f);
    D3DXVECTOR4 lightDiffuse  = D3DXVECTOR4(0.8f, 0.8f, 0.8f, 1.0f);
    D3DXVECTOR4 lightAmbient  = D3DXVECTOR4(0.35f, 0.35f, 0.35f, 1.0f);
    D3DXVECTOR4 materialColor = D3DXVECTOR4(1.0f, 1.0f, 1.0f, 1.0f);
    D3DXVECTOR4 fogParams     = D3DXVECTOR4(1000.0f, 10000.0f, 1.0f / 9000.0f, 0.0f);
};

class ActorSkinningPipeline
{
public:
    static ActorSkinningPipeline& Instance();

    ActorSkinningPipeline();
    ~ActorSkinningPipeline();

    bool Initialize(LPDIRECT3DDEVICE9 pDevice);
    void Shutdown();

    bool IsInitialized() const { return m_bInitialized; }
    bool HasValidShaders() const { return m_pVertexShader != nullptr && m_pPixelShader != nullptr; }

    bool Begin(
        const D3DXMATRIX& matWorld,
        const D3DXMATRIX& matViewProj,
        const ActorLightingState* pLighting = nullptr);

    void UploadBonePalette(const void* pMatrices4x4, UINT boneCount);
    void End();

    LPDIRECT3DVERTEXDECLARATION9 GetVertexDeclaration() const { return m_pVertexDecl; }

    // Pure math helper for unit testing and CPU->GPU packing verification
    static void PackBoneMatricesTo4x3Registers(
        const float* pSourceMatrices4x4,
        UINT boneCount,
        std::vector<float>& outRegisters4x3);

private:
    LPDIRECT3DDEVICE9            m_pDevice;
    LPDIRECT3DVERTEXDECLARATION9 m_pVertexDecl;
    LPDIRECT3DVERTEXSHADER9      m_pVertexShader;
    LPDIRECT3DPIXELSHADER9       m_pPixelShader;

    LPDIRECT3DVERTEXDECLARATION9 m_pPrevVertexDecl;
    LPDIRECT3DVERTEXSHADER9      m_pPrevVertexShader;
    LPDIRECT3DPIXELSHADER9       m_pPrevPixelShader;

    bool                         m_bInitialized;
    bool                         m_bActivePass;
};

}} // namespace EterLib::Render
