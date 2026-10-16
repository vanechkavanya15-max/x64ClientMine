#pragma once

#include <d3d9.h>
#include <array>
#include <cstdint>
#include <span>
#include <vector>

namespace EterLib::Render {

/**
 * @brief 4-kanalowa mapa wag splattingu terenu (RGBA) zgodna z D3DFMT_A8R8G8B8.
 * Likwiduje archaiczne wielokrotne narzucanie tekstur alfa (RAW_AllocateSplats).
 * Przeprowadza analize czestosci tile.raw i wyznacza 4 dominujace warstwy.
 */
class TerrainSplatWeightMap {
public:
    TerrainSplatWeightMap() = default;
    ~TerrainSplatWeightMap();

    TerrainSplatWeightMap(const TerrainSplatWeightMap&) = delete;
    TerrainSplatWeightMap& operator=(const TerrainSplatWeightMap&) = delete;
    TerrainSplatWeightMap(TerrainSplatWeightMap&& other) noexcept;
    TerrainSplatWeightMap& operator=(TerrainSplatWeightMap&& other) noexcept;

    void Initialize(uint32_t width, uint32_t height);
    void ReleaseGPU() noexcept;

    void SetLayerWeight(uint32_t layerIdx, uint8_t weight);
    void SetLayerWeight(uint32_t x, uint32_t y, uint32_t layerIdx, uint8_t weight);

    [[nodiscard]] uint32_t GetPackedWeight(uint32_t x, uint32_t y) const noexcept;
    [[nodiscard]] std::span<const uint32_t> GetRawData() const noexcept;

    // Generuje 4-kanalowa mape wag z tile.raw z histogramem 4 dominujacych warstw (bez modulo 4)
    bool GenerateFromTileMap(
        const uint8_t* pTileMap,
        uint32_t rawWidth,
        uint32_t rawHeight,
        uint32_t outWidth = 256,
        uint32_t outHeight = 256);

    // Przesyla wygenerowana mape wag do tekstury GPU (D3DFMT_A8R8G8B8)
    bool UploadToGPU(LPDIRECT3DDEVICE9 dev);

    [[nodiscard]] LPDIRECT3DTEXTURE9 GetD3DTexture() const noexcept { return m_gpuTexture; }
    [[nodiscard]] const std::array<uint32_t, 4>& GetDominantLayers() const noexcept { return m_dominantLayers; }
    [[nodiscard]] uint32_t GetWidth() const noexcept { return m_width; }
    [[nodiscard]] uint32_t GetHeight() const noexcept { return m_height; }
    [[nodiscard]] bool IsDirty() const noexcept { return m_isDirty; }

private:
    // Pakowanie Little-Endian dla D3DFMT_A8R8G8B8: B(0), G(8), R(16), A(24)
    static constexpr uint32_t PackD3DColor(uint8_t r, uint8_t g, uint8_t b, uint8_t a) noexcept {
        return (static_cast<uint32_t>(b)) |
               (static_cast<uint32_t>(g) << 8) |
               (static_cast<uint32_t>(r) << 16) |
               (static_cast<uint32_t>(a) << 24);
    }

    static constexpr void UnpackD3DColor(uint32_t packed, uint8_t (&outW)[4]) noexcept {
        outW[0] = static_cast<uint8_t>((packed >> 16) & 0xFF); // R - Layer 0
        outW[1] = static_cast<uint8_t>((packed >> 8) & 0xFF);  // G - Layer 1
        outW[2] = static_cast<uint8_t>(packed & 0xFF);         // B - Layer 2
        outW[3] = static_cast<uint8_t>((packed >> 24) & 0xFF); // A - Layer 3
    }

    uint32_t m_width{0};
    uint32_t m_height{0};
    std::vector<uint32_t> m_weights;
    std::array<uint32_t, 4> m_dominantLayers{0, 0, 0, 0};
    LPDIRECT3DTEXTURE9 m_gpuTexture{nullptr};
    bool m_isDirty{false};
};

} // namespace EterLib::Render
