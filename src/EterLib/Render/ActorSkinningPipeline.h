#pragma once

#include <d3d9.h>
#include <d3dx9.h>
#include <cstdint>
#include <vector>
#include <span>
#include <array>

namespace EterLib::Render {

struct ActorSkinningShaderConstants {
    static constexpr UINT kRegViewProj      = 0;   // c0..c3     (4 regs)
    static constexpr UINT kRegBonePalette   = 4;   // c4..c243   (80 bones * 3 regs = 240 regs)
    static constexpr UINT kMaxBones         = 80;
    static constexpr UINT kRegsPerBone      = 3;
    static constexpr UINT kLastBoneReg      = kRegBonePalette + (kMaxBones * kRegsPerBone) - 1; // 243
    static constexpr UINT kRegLightDir      = 244; // c244       (1 reg)
    static constexpr UINT kRegLightDiffuse  = 245; // c245       (1 reg)
    static constexpr UINT kRegLightAmbient  = 246; // c246       (1 reg)
    static constexpr UINT kRegMaterialColor = 247; // c247       (1 reg)
    static constexpr UINT kRegFogParams     = 248; // c248       (1 reg)
    static constexpr UINT kRegWorld         = 249; // c249..c252 (4 regs)
};

// 72-byte hardware vertex structure matching GltfVertex size and offsets
struct GpuSkinnedVertex {
    float position[3];      // offset 0  (12B)
    float normal[3];        // offset 12 (12B)
    float uv0[2];           // offset 24 (8B)
    float uv1[2];           // offset 32 (8B)
    float jointIndices[4];  // offset 40 (16B) - float4 representation of GltfVertex::jointIndices
    float jointWeights[4];  // offset 56 (16B)
};
static_assert(sizeof(GpuSkinnedVertex) == 72, "GpuSkinnedVertex size must match GltfVertex (72 bytes)");

struct ActorSkinningBindDesc {
    D3DXMATRIX worldMatrix;
    D3DXMATRIX viewProjMatrix;
    D3DXVECTOR4 lightDir{0.0f, -0.7071f, -0.7071f, 0.0f};
    D3DXVECTOR4 lightDiffuse{0.8f, 0.8f, 0.8f, 1.0f};
    D3DXVECTOR4 lightAmbient{0.35f, 0.35f, 0.35f, 1.0f};
    D3DXVECTOR4 materialColor{1.0f, 1.0f, 1.0f, 1.0f};
    D3DXVECTOR4 fogParams{1000.0f, 10000.0f, 1.0f / 9000.0f, 0.0f};
};

using ActorLightingState = ActorSkinningBindDesc;

class ActorSkinningPipeline {
public:
    static ActorSkinningPipeline& Instance() noexcept;

    ActorSkinningPipeline() = default;
    ~ActorSkinningPipeline();

    ActorSkinningPipeline(const ActorSkinningPipeline&) = delete;
    ActorSkinningPipeline& operator=(const ActorSkinningPipeline&) = delete;

    bool Initialize(LPDIRECT3DDEVICE9 dev);
    void Shutdown() noexcept;

    bool Begin(const ActorSkinningBindDesc& desc) noexcept;
    bool Begin(
        const D3DXMATRIX& matWorld,
        const D3DXMATRIX& matViewProj,
        const ActorSkinningBindDesc* pOptionalLighting = nullptr) noexcept;

    void UploadBonePalette(const void* pMatrices, UINT count) noexcept;
    void End() noexcept;

    [[nodiscard]] bool IsReady() const noexcept {
        return m_vertexShader != nullptr && m_pixelShader != nullptr && m_vertexDecl != nullptr;
    }
    [[nodiscard]] bool IsInitialized() const noexcept { return IsReady(); }
    [[nodiscard]] bool HasValidShaders() const noexcept { return m_vertexShader != nullptr && m_pixelShader != nullptr; }

    [[nodiscard]] LPDIRECT3DVERTEXDECLARATION9 GetVertexDeclaration() const noexcept {
        return m_vertexDecl;
    }

    // Converts 4x4 bone matrices into 4x3 column-major HLSL register triplets (3 * float4 per bone)
    static void PackBonePaletteTo4x3Registers(
        const void* pMatrices,
        UINT count,
        std::vector<float>& outRegisters);

    static void PackBonePaletteTo4x3Registers(
        const void* pMatrices,
        UINT count,
        std::span<float> outRegisters) noexcept;

    static void PackBoneMatricesTo4x3Registers(
        const float* pSourceMatrices4x4,
        UINT boneCount,
        std::vector<float>& outRegisters4x3)
    {
        PackBonePaletteTo4x3Registers(pSourceMatrices4x4, boneCount, outRegisters4x3);
    }

private:
    std::array<float, ActorSkinningShaderConstants::kMaxBones * 12> m_packedBonesCache{};
    LPDIRECT3DDEVICE9 m_device{nullptr};
    IDirect3DVertexDeclaration9* m_vertexDecl{nullptr};
    IDirect3DVertexShader9* m_vertexShader{nullptr};
    IDirect3DPixelShader9* m_pixelShader{nullptr};

    IDirect3DVertexDeclaration9* m_prevVertexDecl{nullptr};
    IDirect3DVertexShader9* m_prevVertexShader{nullptr};
    IDirect3DPixelShader9* m_prevPixelShader{nullptr};
    bool m_inPass{false};
};

} // namespace EterLib::Render
