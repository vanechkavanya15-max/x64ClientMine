#include <doctest/doctest.h>
#include "../src/Client/Terrain/TerrainFrustumCuller.h"

using namespace Client::Terrain;

TEST_CASE("TerrainFrustumCuller: Error handling") {
    TerrainFrustumCuller culler;
    
    SUBCASE("Planes not set") {
        TerrainAABB box = { -50.0f, -50.0f, 50.0f, 50.0f, 50.0f, 150.0f };
        
        auto res1 = culler.IsBoxVisible(box);
        CHECK_FALSE(res1.has_value());
        CHECK(res1.error() == CullerError::PlanesNotSet);
        
        std::vector<uint32_t> visibleIds;
        auto res2 = culler.CullQuadTree(box, visibleIds);
        CHECK_FALSE(res2.has_value());
        CHECK(res2.error() == CullerError::PlanesNotSet);
    }
    
    SUBCASE("Invalid planes count") {
        std::vector<FrustumPlane> badPlanes(5);
        auto res = culler.SetFrustumPlanes(badPlanes);
        CHECK_FALSE(res.has_value());
        CHECK(res.error() == CullerError::InvalidPlanesCount);
    }
}

TEST_CASE("TerrainFrustumCuller: Visibility tests") {
    TerrainFrustumCuller culler;
    
    std::array<FrustumPlane, 6> planes = {{
        { -1.0f,  0.0f,  0.0f, 100.0f }, // Right plane (x < 100)
        {  1.0f,  0.0f,  0.0f, 100.0f }, // Left plane (x > -100)
        {  0.0f, -1.0f,  0.0f, 100.0f }, // Top plane (y < 100)
        {  0.0f,  1.0f,  0.0f, 100.0f }, // Bottom plane (y > -100)
        {  0.0f,  0.0f, -1.0f, 1000.0f}, // Far plane (z < 1000)
        {  0.0f,  0.0f,  1.0f, -10.0f }  // Near plane (z > 10)
    }};

    auto res = culler.SetFrustumPlanes(planes);
    REQUIRE(res.has_value());

    SUBCASE("AABB fully inside frustum") {
        TerrainAABB insideBox = { -50.0f, -50.0f, 50.0f, 50.0f, 50.0f, 150.0f };
        auto visibleRes = culler.IsBoxVisible(insideBox);
        REQUIRE(visibleRes.has_value());
        CHECK(visibleRes.value() == true);
        
        std::vector<uint32_t> visibleIds;
        auto cullRes = culler.CullQuadTree(insideBox, visibleIds);
        REQUIRE(cullRes.has_value());
        CHECK(visibleIds.size() == 256);
    }

    SUBCASE("AABB completely outside frustum") {
        TerrainAABB outsideBox = { -50.0f, -50.0f, -200.0f, 50.0f, 50.0f, -100.0f };
        auto visibleRes = culler.IsBoxVisible(outsideBox);
        REQUIRE(visibleRes.has_value());
        CHECK(visibleRes.value() == false);
        
        std::vector<uint32_t> visibleIds;
        auto cullRes = culler.CullQuadTree(outsideBox, visibleIds);
        REQUIRE(cullRes.has_value());
        CHECK(visibleIds.size() == 0);
    }
    
    SUBCASE("Partial intersection") {
        TerrainAABB partialBox = { -200.0f, -50.0f, 50.0f, 0.0f, 50.0f, 150.0f };
        auto visibleRes = culler.IsBoxVisible(partialBox);
        REQUIRE(visibleRes.has_value());
        CHECK(visibleRes.value() == true);
        
        std::vector<uint32_t> visibleIds;
        auto cullRes = culler.CullQuadTree(partialBox, visibleIds);
        REQUIRE(cullRes.has_value());
        CHECK(visibleIds.size() == 144);
    }
}
