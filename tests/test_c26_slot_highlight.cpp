#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest.h>
#include "Client/UI/SlotHighlightRenderer.h"
#include "Client/UI/UIRenderBatcher.h"

using namespace Client::UI;

TEST_SUITE("SlotHighlightRenderer") {

    TEST_CASE("Out of bounds rendering returns error") {
        UIRenderBatcher batcher(100);
        SlotHighlightRenderer renderer;
        GridConfig config{0.0f, 0.0f, 32.0f, 32.0f, 5, 5};

        auto result = renderer.RenderHighlight(batcher, config, 6, 0, 1, 1, Color{255, 0, 0, 255});
        REQUIRE_FALSE(result.has_value());
        CHECK(result.error() == SlotHighlightError::OutOfBounds);

        auto result2 = renderer.RenderHighlight(batcher, config, 0, 6, 1, 1, Color{255, 0, 0, 255});
        REQUIRE_FALSE(result2.has_value());
        CHECK(result2.error() == SlotHighlightError::OutOfBounds);

        auto result3 = renderer.RenderHighlight(batcher, config, 4, 4, 2, 2, Color{255, 0, 0, 255});
        REQUIRE_FALSE(result3.has_value());
        CHECK(result3.error() == SlotHighlightError::OutOfBounds);
    }

    TEST_CASE("Invalid dimensions return error") {
        UIRenderBatcher batcher(100);
        SlotHighlightRenderer renderer;
        GridConfig config{0.0f, 0.0f, 32.0f, 32.0f, 5, 5};

        auto result = renderer.RenderHighlight(batcher, config, 0, 0, 0, 1, Color{255, 0, 0, 255});
        REQUIRE_FALSE(result.has_value());
        CHECK(result.error() == SlotHighlightError::InvalidDimensions);

        auto result2 = renderer.RenderHighlight(batcher, config, 0, 0, 1, 0, Color{255, 0, 0, 255});
        REQUIRE_FALSE(result2.has_value());
        CHECK(result2.error() == SlotHighlightError::InvalidDimensions);
    }

    TEST_CASE("Successful rendering with color") {
        UIRenderBatcher batcher(100);
        SlotHighlightRenderer renderer;
        GridConfig config{10.0f, 20.0f, 32.0f, 32.0f, 5, 5};
        Color color{0, 255, 0, 255};

        auto result = renderer.RenderHighlight(batcher, config, 1, 2, 2, 1, color);
        REQUIRE(result.has_value());

        auto batches = batcher.GetBatches();
        REQUIRE(batches.size() == 1);
        CHECK(batches[0].type == PrimitiveType::TriangleList);

        auto vertices = batcher.GetVertices();
        // A rect usually adds 6 vertices (2 triangles)
        REQUIRE(vertices.size() >= 6); 
        
        // Check top-left vertex position and color
        float expectedX = 10.0f + 1 * 32.0f; // 42.0
        float expectedY = 20.0f + 2 * 32.0f; // 84.0
        
        // Since UIRenderBatcher creates 2 triangles, we just check the first vertex
        CHECK(vertices[0].x == doctest::Approx(expectedX));
        CHECK(vertices[0].y == doctest::Approx(expectedY));
        CHECK(vertices[0].color == color);
    }

    TEST_CASE("Successful rendering with texture") {
        UIRenderBatcher batcher(100);
        SlotHighlightRenderer renderer;
        GridConfig config{0.0f, 0.0f, 32.0f, 32.0f, 5, 5};
        Color color{255, 255, 255, 255};
        TextureId tex{42};

        auto result = renderer.RenderHighlight(batcher, config, 0, 0, 1, 1, color, tex);
        REQUIRE(result.has_value());

        auto batches = batcher.GetBatches();
        REQUIRE(batches.size() == 1);
        CHECK(batches[0].texture == tex);
    }
    
    TEST_CASE("UIRenderBatcher error maps to BatchError") {
        // Create batcher with minimal capacity to force overflow
        UIRenderBatcher batcher(3); 
        SlotHighlightRenderer renderer;
        GridConfig config{0.0f, 0.0f, 32.0f, 32.0f, 5, 5};

        auto result = renderer.RenderHighlight(batcher, config, 0, 0, 1, 1, Color{255, 255, 255, 255});
        REQUIRE_FALSE(result.has_value());
        CHECK(result.error() == SlotHighlightError::BatchError);
    }
}
