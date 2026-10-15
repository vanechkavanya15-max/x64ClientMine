#pragma once

#include <cstdint>
#include <vector>
#include <span>
#include <expected>

namespace Client::Terrain {

    // Struktura reprezentujaca wektor 3D dla wektorow normalnych
    struct Vector3 {
        float x;
        float y;
        float z;
    };

    // Domenowe bledy dla samplera terenu
    enum class TerrainError : uint8_t {
        OutOfBounds,
        InvalidGridSize,
        DataNotLoaded
    };

    // Klasa odpowiedzialna za probkowanie wysokosci terenu i obliczanie wektorow normalnych
    class TerrainHeightSampler {
    public:
        TerrainHeightSampler() = default;
        ~TerrainHeightSampler() = default;

        /**
         * Inicjalizuje sampler terenu wymiarami siatki i danymi wysokosci.
         */
        std::expected<void, TerrainError> Initialize(uint32_t width, uint32_t height, std::span<const float> heights);

        /**
         * Pobiera interpolowana wysokosc w danym punkcie 2D.
         */
        [[nodiscard]] std::expected<float, TerrainError> GetHeight(float x, float y) const noexcept;

        /**
         * Oblicza wektor normalny powierzchni w danym punkcie 2D.
         */
        [[nodiscard]] std::expected<Vector3, TerrainError> GetNormal(float x, float y) const noexcept;

    private:
        [[nodiscard]] bool IsInBoundsFloat(float x, float y) const noexcept;
        [[nodiscard]] float GetRawHeight(uint32_t gridX, uint32_t gridY) const noexcept;

        uint32_t m_width = 0;
        uint32_t m_height = 0;
        std::vector<float> m_gridHeights;
    };

} // namespace Client::Terrain
