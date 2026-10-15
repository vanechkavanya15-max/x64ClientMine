#pragma once

#include <vector>
#include <span>
#include <cstdint>
#include <cmath>
#include <algorithm>

namespace Client::Terrain {

// Struktura wektora 3D aby uniknac zaleznosci od Windows D3DVECTOR
struct Vector3 {
    float x = 0.0f;
    float y = 0.0f;
    float z = 0.0f;
    
    constexpr bool operator==(const Vector3&) const = default;
};

// Klasa samplera wysokosci terenu
class TerrainHeightSampler {
public:
    TerrainHeightSampler() = default;
    ~TerrainHeightSampler() = default;
    
    // Zapobieganie kopiowaniu
    TerrainHeightSampler(const TerrainHeightSampler&) = delete;
    TerrainHeightSampler& operator=(const TerrainHeightSampler&) = delete;
    
    // Przenoszenie dozwolone
    TerrainHeightSampler(TerrainHeightSampler&&) noexcept = default;
    TerrainHeightSampler& operator=(TerrainHeightSampler&&) noexcept = default;

    // Inicjalizacja danych terenu
    void SetHeightData(std::span<const float> grid, uint32_t width, uint32_t height, float scale);

    // Pobranie wysokosci w zadanym punkcie z interpolacja dwuliniowa
    [[nodiscard]] float GetHeight(float x, float y) const;

    // Pobranie normalnej w zadanym punkcie
    [[nodiscard]] Vector3 GetNormal(float x, float y) const;

private:
    std::vector<float> heightGrid_;
    uint32_t gridWidth_ = 0;
    uint32_t gridHeight_ = 0;
    float gridScale_ = 1.0f;

    // Metoda pomocnicza do pobierania wysokosci z uwzglednieniem clampowania
    [[nodiscard]] float GetClampedHeight(int32_t gridX, int32_t gridY) const;
};

} // namespace Client::Terrain
