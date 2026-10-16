#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include "../src/Client/UI/UITooltipRenderer.h"
#include "../src/Client/UI/UIRenderBatcher.h"

using namespace Client::UI;

class MockFontProvider : public IFontProvider {
public:
    std::expected<GlyphMetrics, TooltipRenderError> GetGlyphMetrics(char c) const override {
        return GlyphMetrics{0.0f, 0.0f, 1.0f, 1.0f, 8.0f, 12.0f};
    }
};

TEST_CASE("UITooltipRenderer - Empty Content Error") {
    UITooltipRenderer renderer;
    UIRenderBatcher batcher(100);
    MockFontProvider fontProvider;
    TooltipLayoutConfig config;
    config.fontProvider = &fontProvider;
    
    auto result = renderer.Render(batcher, config);
    REQUIRE(!result.has_value());
    CHECK(result.error() == TooltipRenderError::EmptyContent);
}

TEST_CASE("UITooltipRenderer - Layout and Render Success") {
    UITooltipRenderer renderer;
    renderer.AddTextLine("Sword +9");
    renderer.AddSeparator();
    renderer.AddTextLine("Damage: 100-120");

    UIRenderBatcher batcher(1000);
    MockFontProvider fontProvider;
    TooltipLayoutConfig config;
    config.x = 10.0f;
    config.y = 10.0f;
    config.fontProvider = &fontProvider;

    auto result = renderer.Render(batcher, config);
    REQUIRE(result.has_value());

    auto vertices = batcher.GetVertices();
    // 1 background quad (6 vertices) + "Sword +9" (8 chars * 6 vertices) + 1 separator line (2 vertices) + "Damage: 100-120" (15 chars * 6 vertices)
    // Total = 6 + 48 + 2 + 90 = 146 vertices
    CHECK(vertices.size() == 146);
}

TEST_CASE("UITooltipRenderer - Batcher Overflow Error") {
    UITooltipRenderer renderer;
    // 12 vertices max (can hold background, but not enough for text)
    UIRenderBatcher batcher(12);
    renderer.AddTextLine("Too long string");

    MockFontProvider fontProvider;
    TooltipLayoutConfig config;
    config.fontProvider = &fontProvider;
    auto result = renderer.Render(batcher, config);
    
    REQUIRE(!result.has_value());
    CHECK(result.error() == TooltipRenderError::BatcherOverflow);
}

TEST_CASE("UITooltipRenderer - Clear Method") {
    UITooltipRenderer renderer;
    renderer.AddTextLine("Test");
    CHECK_FALSE(renderer.IsEmpty());
    renderer.Clear();
    CHECK(renderer.IsEmpty());
}

TEST_CASE("UITooltipRenderer - Missing Font Provider Error") {
    UITooltipRenderer renderer;
    UIRenderBatcher batcher(100);
    renderer.AddTextLine("Sword +9");
    TooltipLayoutConfig config;
    config.fontProvider = nullptr;
    
    auto result = renderer.Render(batcher, config);
    REQUIRE(!result.has_value());
    CHECK(result.error() == TooltipRenderError::InvalidGlyph);
}

