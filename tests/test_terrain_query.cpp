#include "doctest.h"
#include "../src/Client/Terrain/IHeightQuery.h"

using namespace Client::Terrain;

class MockHeightQuery : public IHeightQuery
{
public:
    std::expected<HeightQueryResult, HeightQueryError> GetTerrainHeight(float x, float y) const noexcept override
    {
        if (x < 0.0f || y < 0.0f) {
            return std::unexpected(HeightQueryError::InvalidCoordinate);
        }
        return HeightQueryResult{10.0f};
    }

    std::expected<HeightQueryResult, HeightQueryError> GetCacheHeight(float x, float y) const noexcept override
    {
        if (x < 0.0f || y < 0.0f) {
            return std::unexpected(HeightQueryError::InvalidCoordinate);
        }
        return HeightQueryResult{5.0f};
    }
};

TEST_CASE("IHeightQuery - GetTerrainHeight") {
    MockHeightQuery query;
    
    SUBCASE("Valid coordinates return expected height") {
        auto result = query.GetTerrainHeight(10.0f, 20.0f);
        REQUIRE(result.has_value());
        CHECK(result->height == 10.0f);
    }

    SUBCASE("Invalid coordinates return error") {
        auto result = query.GetTerrainHeight(-1.0f, 20.0f);
        REQUIRE(!result.has_value());
        CHECK(result.error() == HeightQueryError::InvalidCoordinate);
    }
}

TEST_CASE("IHeightQuery - GetCacheHeight") {
    MockHeightQuery query;
    
    SUBCASE("Valid coordinates return expected height") {
        auto result = query.GetCacheHeight(10.0f, 20.0f);
        REQUIRE(result.has_value());
        CHECK(result->height == 5.0f);
    }

    SUBCASE("Invalid coordinates return error") {
        auto result = query.GetCacheHeight(10.0f, -20.0f);
        REQUIRE(!result.has_value());
        CHECK(result.error() == HeightQueryError::InvalidCoordinate);
    }
}
