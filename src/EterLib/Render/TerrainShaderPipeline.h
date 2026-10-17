#pragma once

#include <d3d9.h>
#include <array>
#include <cstdint>

namespace EterLib::Render {

struct TerrainSectorBindDesc {
    float sectorOriginX{0.0f};
    float sectorOriginY{0.0f};
    float invSectorSize{1.0f / 25600.0f};
    float detailTileScale{64.0f};
    LPDIRECT3DTEXTURE9 splatTexture{nullptr};
    std::array<LPDIRECT3DTEXTURE9, 4> layerTextures{nullptr, nullptr, nullptr, nullptr};
};

/**
 * @brief Jednoprzebiegowy potok renderowania terenu (Single-Pass Splatting HLSL vs_3_0 / ps_3_0).
 * Redukuje renderowanie sektora/patcha z N przejsc alfa (RAW_AllocateSplats) do dokladnie 1 DrawCall.
 * Rozdziela stale klatkowe (BeginFrame) od stalych sektora (BindSector).
 */
class TerrainShaderPipeline {
public:
    TerrainShaderPipeline() = default;
    ~TerrainShaderPipeline();

    TerrainShaderPipeline(const TerrainShaderPipeline&) = delete;
    TerrainShaderPipeline& operator=(const TerrainShaderPipeline&) = delete;

    bool Initialize(LPDIRECT3DDEVICE9 dev);
    void Shutdown() noexcept;

    // Wywolywane RAZ na klatke przed petla rysowania sektorow (stale macierzy, swiatla i mgly)
    void BeginFrame(
        LPDIRECT3DDEVICE9 dev,
        const D3DMATRIX& viewProj,
        const D3DVECTOR& lightDir,
        const D3DVECTOR& ambientColor,
        const float (&fogColorAndRange)[4]) noexcept;

    // Wywolywane per-sektor/patch (1 Draw Call) - minimalny narzut sterownika
    void BindSector(LPDIRECT3DDEVICE9 dev, const TerrainSectorBindDesc& desc) noexcept;

    void EndFrame(LPDIRECT3DDEVICE9 dev) noexcept;

    [[nodiscard]] bool IsReady() const noexcept {
        return m_vertexShader != nullptr && m_pixelShader != nullptr && m_vertexDecl != nullptr;
    }

private:
    IDirect3DVertexDeclaration9* m_vertexDecl{nullptr};
    IDirect3DVertexShader9* m_vertexShader{nullptr};
    IDirect3DPixelShader9* m_pixelShader{nullptr};
    LPDIRECT3DTEXTURE9 m_fallbackWhiteTex{nullptr};
    bool m_inFrame{false};
};

} // namespace EterLib::Render
