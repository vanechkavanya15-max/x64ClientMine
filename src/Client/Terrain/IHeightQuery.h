#pragma once

#include <optional>
#include <expected>
#include <system_error>

namespace Client::Terrain
{
    enum class HeightQueryError
    {
        InvalidCoordinate = 1,
        TerrainNotLoaded
    };

    struct HeightQueryResult
    {
        float height;
    };

    class IHeightQuery
    {
    public:
        virtual ~IHeightQuery() = default;

        // Metody izolujące zapytania o wysokość terenu, wolne od D3D
        virtual std::expected<HeightQueryResult, HeightQueryError> GetTerrainHeight(float x, float y) const noexcept = 0;
        virtual std::expected<HeightQueryResult, HeightQueryError> GetCacheHeight(float x, float y) const noexcept = 0;
    };
} // namespace Client::Terrain
