#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest/doctest.h>
#include "Client/Vegetation/SpeedTreeBillboardRenderer.h"

using namespace Client::Vegetation;

TEST_CASE("SpeedTreeBillboardRenderer - Empty Generation") {
    SpeedTreeBillboardRenderer renderer;
    auto result = renderer.GenerateQuads({0.0f, 0.0f, 1.0f}, {0.0f, 1.0f, 0.0f});
    REQUIRE(result.has_value());
    CHECK(result.value().empty());
}

TEST_CASE("SpeedTreeBillboardRenderer - Invalid Camera Vectors") {
    SpeedTreeBillboardRenderer renderer;
    renderer.AddBillboard({{0.0f, 0.0f, 0.0f}, 10.0f, 10.0f, TextureId(1)});
    
    SUBCASE("Zero camera dir") {
        auto result = renderer.GenerateQuads({0.0f, 0.0f, 0.0f}, {0.0f, 1.0f, 0.0f});
        REQUIRE(!result.has_value());
        CHECK(result.error() == RenderError::InvalidCameraVector);
    }
    
    SUBCASE("Zero world up") {
        auto result = renderer.GenerateQuads({0.0f, 0.0f, 1.0f}, {0.0f, 0.0f, 0.0f});
        REQUIRE(!result.has_value());
        CHECK(result.error() == RenderError::InvalidCameraVector);
    }
    
    SUBCASE("Collinear vectors") {
        auto result = renderer.GenerateQuads({0.0f, 1.0f, 0.0f}, {0.0f, 1.0f, 0.0f});
        REQUIRE(!result.has_value());
        CHECK(result.error() == RenderError::InvalidCameraVector);
    }
}

TEST_CASE("SpeedTreeBillboardRenderer - Single Quad Generation") {
    SpeedTreeBillboardRenderer renderer;
    
    Vector3 pos{10.0f, 0.0f, 20.0f};
    float width = 10.0f;
    float height = 20.0f;
    TextureId tex(42);
    
    renderer.AddBillboard({pos, width, height, tex});
    
    Vector3 cameraDir{0.0f, 0.0f, 1.0f}; // Looking forward +Z
    Vector3 worldUp{0.0f, 1.0f, 0.0f};   // Up is +Y
    
    auto result = renderer.GenerateQuads(cameraDir, worldUp);
    REQUIRE(result.has_value());
    REQUIRE(result.value().size() == 1);
    
    const auto& quad = result.value()[0];
    CHECK(quad.textureId == tex);
    
    // cameraDir is +Z, worldUp is +Y
    // right = worldUp x cameraDir = (+Y) x (+Z) = +X
    // up = cameraDir x right = (+Z) x (+X) = +Y
    // halfWidth = 5.0, halfHeight = 10.0
    // rightScaled = (5.0, 0, 0), upScaled = (0, 10.0, 0)
    // v0 (bottom-left) = pos - rightScaled - upScaled = (10 - 5, 0 - 10, 20) = (5.0, -10.0, 20.0)
    // v1 (top-left) = pos - rightScaled + upScaled = (10 - 5, 0 + 10, 20) = (5.0, 10.0, 20.0)
    // v2 (top-right) = pos + rightScaled + upScaled = (10 + 5, 0 + 10, 20) = (15.0, 10.0, 20.0)
    // v3 (bottom-right) = pos + rightScaled - upScaled = (10 + 5, 0 - 10, 20) = (15.0, -10.0, 20.0)

    CHECK(quad.vertices[0].position == Vector3{5.0f, -10.0f, 20.0f});
    CHECK(quad.vertices[0].texCoord == TexCoord{0.0f, 1.0f});
    
    CHECK(quad.vertices[1].position == Vector3{5.0f, 10.0f, 20.0f});
    CHECK(quad.vertices[1].texCoord == TexCoord{0.0f, 0.0f});
    
    CHECK(quad.vertices[2].position == Vector3{15.0f, 10.0f, 20.0f});
    CHECK(quad.vertices[2].texCoord == TexCoord{1.0f, 0.0f});
    
    CHECK(quad.vertices[3].position == Vector3{15.0f, -10.0f, 20.0f});
    CHECK(quad.vertices[3].texCoord == TexCoord{1.0f, 1.0f});
}

TEST_CASE("SpeedTreeBillboardRenderer - Multiple Quads Sorted By Texture") {
    SpeedTreeBillboardRenderer renderer;
    
    renderer.AddBillboard({{0.0f, 0.0f, 0.0f}, 10.0f, 10.0f, TextureId(3)});
    renderer.AddBillboard({{1.0f, 0.0f, 0.0f}, 10.0f, 10.0f, TextureId(1)});
    renderer.AddBillboard({{2.0f, 0.0f, 0.0f}, 10.0f, 10.0f, TextureId(2)});
    renderer.AddBillboard({{3.0f, 0.0f, 0.0f}, 10.0f, 10.0f, TextureId(1)});
    
    auto result = renderer.GenerateQuads({0.0f, 0.0f, 1.0f}, {0.0f, 1.0f, 0.0f});
    REQUIRE(result.has_value());
    REQUIRE(result.value().size() == 4);
    
    CHECK(result.value()[0].textureId == TextureId(1));
    CHECK(result.value()[1].textureId == TextureId(1));
    CHECK(result.value()[2].textureId == TextureId(2));
    CHECK(result.value()[3].textureId == TextureId(3));
}

TEST_CASE("SpeedTreeBillboardRenderer - Clear") {
    SpeedTreeBillboardRenderer renderer;
    renderer.AddBillboard({{0.0f, 0.0f, 0.0f}, 10.0f, 10.0f, TextureId(1)});
    renderer.Clear();
    
    auto result = renderer.GenerateQuads({0.0f, 0.0f, 1.0f}, {0.0f, 1.0f, 0.0f});
    REQUIRE(result.has_value());
    CHECK(result.value().empty());
}
