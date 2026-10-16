#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest/doctest.h>

#include "src/Client/UI/SlotCountTextRenderer.h"
#include "src/Client/UI/UIRenderBatcher.h"
#include "src/Client/UI/UITooltipRenderer.h"

using namespace Client::UI;

class MockFontProvider : public IFontProvider {
public:
    std::expected<GlyphMetrics, TooltipRenderError> GetGlyphMetrics(char c) const override {
        if (c == '?') {
            return std::unexpected(TooltipRenderError::InvalidGlyph);
        }
        // Dummy metrics for testing
        return GlyphMetrics{0.1f, 0.2f, 0.05f, 0.05f, 10.0f, 12.0f};
    }
};

TEST_CASE("SlotCountTextRenderer Rendering") {
    MockFontProvider fontProvider;
    SlotCountTextRenderer renderer(&fontProvider);
    UIRenderBatcher batcher(1024);
    
    SlotCountLayoutConfig config{10.0f, 20.0f, TextureId{1}, Color{255, 255, 255, 255}};

    SUBCASE("Render valid count") {
        auto res = renderer.RenderCount(batcher, 123, config);
        CHECK(res.has_value() == true);
        
        auto batches = batcher.GetBatches();
        CHECK(batches.size() == 1); // Should create a single batch
        CHECK(batches[0].vertexCount == 18); // 3 chars * 6 vertices per char
    }

    SUBCASE("Render zero count") {
        auto res = renderer.RenderCount(batcher, 0, config);
        CHECK(res.has_value() == true);
        
        auto batches = batcher.GetBatches();
        CHECK(batches.size() == 1);
        CHECK(batches[0].vertexCount == 6); // '0' is 1 char, 6 vertices
    }
}

TEST_CASE("SlotCountTextRenderer Error Handling") {
    MockFontProvider fontProvider;
    SlotCountLayoutConfig config{0.0f, 0.0f, TextureId{1}, Color{255, 255, 255, 255}};

    SUBCASE("Invalid font provider") {
        SlotCountTextRenderer renderer(nullptr);
        UIRenderBatcher batcher(1024);
        
        auto res = renderer.RenderCount(batcher, 10, config);
        CHECK(res.has_value() == false);
        CHECK(res.error() == SlotCountRenderError::InvalidFontProvider);
    }

    SUBCASE("Invalid glyph") {
        SlotCountTextRenderer renderer(&fontProvider);
        UIRenderBatcher batcher(1024);
        
        auto res = renderer.RenderText(batcher, "12?3", config);
        CHECK(res.has_value() == false);
        CHECK(res.error() == SlotCountRenderError::InvalidGlyph);
    }

    SUBCASE("Batcher overflow") {
        SlotCountTextRenderer renderer(&fontProvider);
        UIRenderBatcher batcher(12); // Capacity for 12 vertices (2 chars)
        
        auto res = renderer.RenderCount(batcher, 123, config); // Trying to render 3 chars
        CHECK(res.has_value() == false);
        CHECK(res.error() == SlotCountRenderError::BatcherOverflow);
    }
}
