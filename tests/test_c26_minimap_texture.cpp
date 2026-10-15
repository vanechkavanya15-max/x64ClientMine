#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include "Client/MiniMap/MiniMapTextureRenderer.h"

using namespace Client::MiniMap;

TEST_CASE("MiniMapTextureRenderer - GenerateQuads") {
    MiniMapTextureRenderer renderer;

    SUBCASE("Invalid scale returns error") {
        MiniMapRenderContext ctx{};
        ctx.scale = 0.0f;
        ctx.cellScale = 1.0f;
        
        auto result = renderer.GenerateQuads(ctx);
        CHECK(result.has_value() == false);
        CHECK(result.error() == "Invalid map scale");
    }

    SUBCASE("Invalid cell scale returns error") {
        MiniMapRenderContext ctx{};
        ctx.scale = 1.0f;
        ctx.cellScale = 0.0f;
        
        auto result = renderer.GenerateQuads(ctx);
        CHECK(result.has_value() == false);
        CHECK(result.error() == "Invalid cell scale");
    }

    SUBCASE("Valid configuration generates correct number of batches with center offset") {
        MiniMapRenderContext ctx{};
        ctx.scale = 2.0f;
        ctx.cellScale = 256.0f; // std cell scale
        ctx.screenX = 500.0f;
        ctx.screenY = 500.0f;
        // Panning the map by half a cell
        ctx.mapCenterX = 128.0f;
        ctx.mapCenterY = 64.0f;
        
        // Mock texture handles (1, 2, 3... 9)
        for (int i = 0; i < 9; ++i) {
            ctx.textures[i] = reinterpret_cast<TextureHandle>(static_cast<uintptr_t>(i + 1));
        }

        auto result = renderer.GenerateQuads(ctx);
        REQUIRE(result.has_value() == true);
        
        const auto& batches = result.value();
        REQUIRE(batches.size() == 9);

        // Check the first batch properties
        const auto& firstBatch = batches[0];
        CHECK(firstBatch.texture == reinterpret_cast<TextureHandle>(static_cast<uintptr_t>(1)));
        REQUIRE(firstBatch.vertices.size() == 6);

        // Verify correct offset calculations for top-left quad (col 0, row 0)
        float expectedScaledCellSize = 2.0f * 256.0f; // 512.0f
        float expectedHalfWidth = (3.0f * 512.0f) / 2.0f; // 768.0f
        
        // offset inside a cell:
        // mapCenter = 128 -> fmod(128, 256) = 128 -> * scale(2) = 256
        // mapCenter = 64 -> fmod(64, 256) = 64 -> * scale(2) = 128
        float offsetX = 256.0f;
        float offsetY = 128.0f;
        
        float expectedStartX = 500.0f - 768.0f - offsetX; // 500 - 768 - 256 = -524.0f
        float expectedStartY = 500.0f - 768.0f - offsetY; // 500 - 768 - 128 = -396.0f

        CHECK(firstBatch.vertices[0].x == doctest::Approx(expectedStartX)); // TL x
        CHECK(firstBatch.vertices[0].y == doctest::Approx(expectedStartY)); // TL y
        CHECK(firstBatch.vertices[0].u == doctest::Approx(0.0f));
        CHECK(firstBatch.vertices[0].v == doctest::Approx(0.0f));
    }
    
    SUBCASE("Missing textures are skipped") {
        MiniMapRenderContext ctx{};
        ctx.scale = 1.0f;
        ctx.cellScale = 1.0f;
        ctx.mapCenterX = 0.0f;
        ctx.mapCenterY = 0.0f;
        
        for (int i = 0; i < 9; ++i) {
            ctx.textures[i] = nullptr;
        }
        
        // Add only one valid texture at index 4 (center)
        ctx.textures[4] = reinterpret_cast<TextureHandle>(static_cast<uintptr_t>(99));

        auto result = renderer.GenerateQuads(ctx);
        REQUIRE(result.has_value() == true);
        
        const auto& batches = result.value();
        REQUIRE(batches.size() == 1);
        CHECK(batches[0].texture == reinterpret_cast<TextureHandle>(static_cast<uintptr_t>(99)));
    }
}
