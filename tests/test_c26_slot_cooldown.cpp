#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include "Client/UI/SlotCoolDownAnimationRenderer.h"

using namespace Client::UI;

TEST_SUITE("SlotCoolDownAnimationRenderer") {

    TEST_CASE("Progress 1.0 generates no vertices") {
        auto result = SlotCoolDownAnimationRenderer::GenerateRadialWipe(0.0f, 0.0f, 32.0f, 32.0f, 1.0f, Color{255, 0, 0, 255});
        REQUIRE(result.has_value());
        CHECK_EQ(result.value().size(), 0u);
    }

    TEST_CASE("Progress > 1.0 generates no vertices") {
        auto result = SlotCoolDownAnimationRenderer::GenerateRadialWipe(0.0f, 0.0f, 32.0f, 32.0f, 1.5f, Color{255, 0, 0, 255});
        REQUIRE(result.has_value());
        CHECK_EQ(result.value().size(), 0u);
    }

    TEST_CASE("Progress 0.0 generates full quad (2 triangles, but maybe more due to radial fan)") {
        auto result = SlotCoolDownAnimationRenderer::GenerateRadialWipe(0.0f, 0.0f, 32.0f, 32.0f, 0.0f, Color{255, 0, 0, 255});
        REQUIRE(result.has_value());
        auto& verts = result.value();
        
        // At progress 0.0, we have {0, -h/2}, {w/2, -h/2}, {w/2, h/2}, {-w/2, h/2}, {-w/2, -h/2}, {0, -h/2}
        // That is 6 points on the perimeter, creating 5 triangles => 15 vertices.
        CHECK_EQ(verts.size(), 15u);
        
        // Check that all triangles are multiples of 3
        CHECK_EQ(verts.size() % 3u, 0u);
        
        // Check center vertices
        float cx = 16.0f, cy = 16.0f;
        for (std::size_t i = 0; i < verts.size(); i += 3) {
            CHECK_EQ(verts[i].x, cx);
            CHECK_EQ(verts[i].y, cy);
            CHECK_EQ(verts[i].color.r, 255u);
        }
    }

    TEST_CASE("Progress < 0.0 acts as 0.0") {
        auto result = SlotCoolDownAnimationRenderer::GenerateRadialWipe(0.0f, 0.0f, 32.0f, 32.0f, -0.5f, Color{255, 0, 0, 255});
        REQUIRE(result.has_value());
        CHECK_EQ(result.value().size(), 15u);
    }

    TEST_CASE("Progress 0.5 generates exactly half of the area") {
        auto result = SlotCoolDownAnimationRenderer::GenerateRadialWipe(0.0f, 0.0f, 32.0f, 32.0f, 0.5f, Color{255, 0, 0, 255});
        REQUIRE(result.has_value());
        auto& verts = result.value();
        
        // Progress 0.5 starts at angle pi.
        // The points should be: bottom-center {0, h/2}, bottom-left {-w/2, h/2}, top-left {-w/2, -h/2}, top-center {0, -h/2}
        // Which is 4 points -> 3 triangles -> 9 vertices.
        CHECK_EQ(verts.size(), 9u);
        CHECK_EQ(verts.size() % 3u, 0u);
    }

    TEST_CASE("Progress 0.25 generates 3/4 of the area") {
        auto result = SlotCoolDownAnimationRenderer::GenerateRadialWipe(0.0f, 0.0f, 32.0f, 32.0f, 0.25f, Color{255, 0, 0, 255});
        REQUIRE(result.has_value());
        auto& verts = result.value();
        
        // Progress 0.25 starts at angle pi/2.
        // Points: right-center {w/2, 0}, bottom-right {w/2, h/2}, bottom-left {-w/2, h/2}, top-left {-w/2, -h/2}, top-center {0, -h/2}
        // 5 points -> 4 triangles -> 12 vertices.
        CHECK_EQ(verts.size(), 12u);
        CHECK_EQ(verts.size() % 3u, 0u);
    }
}
