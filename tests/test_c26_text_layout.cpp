#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include "../src/Client/Text/TextLayoutEngine.h"
#include <string_view>

using namespace Client::Text;

class MockFontMetrics : public IFontMetricsProvider {
public:
    float GetGlyphAdvance(char32_t codepoint) const override {
        if (codepoint == 'W') return 15.0f;
        if (codepoint == 'i') return 5.0f;
        if (codepoint == 'l') return 5.0f;
        return 10.0f; // Default 10px per char
    }
    float GetLineHeight() const override {
        return 20.0f;
    }
};

TEST_SUITE("TextLayoutEngine") {
    TEST_CASE("Basic text layout without tags") {
        MockFontMetrics metrics;
        LayoutConfig config;
        
        auto result = TextLayoutEngine::BuildLayout("Hello", metrics, config);
        
        REQUIRE(result.has_value());
        // H(10) e(10) l(5) l(5) o(10) = 40.0f
        CHECK(result->width == 40.0f);
        CHECK(result->height == 20.0f);
        REQUIRE(result->lines.size() == 1);
        CHECK(result->lines[0].glyphs.size() == 5);
        CHECK(result->lines[0].glyphs[0].content == "H");
        CHECK(result->lines[0].glyphs[0].x == 0.0f);
        CHECK(result->lines[0].glyphs[4].x == 30.0f); // 10+10+5+5 = 30
    }
    
    TEST_CASE("Text layout with color tags") {
        MockFontMetrics metrics;
        LayoutConfig config;
        config.defaultColor = 0xFFFFFFFF;
        
        auto result = TextLayoutEngine::BuildLayout("A|cFFFF0000B|rC", metrics, config);
        
        REQUIRE(result.has_value());
        REQUIRE(result->lines.size() == 1);
        REQUIRE(result->lines[0].glyphs.size() == 3);
        
        CHECK(result->lines[0].glyphs[0].content == "A");
        CHECK(result->lines[0].glyphs[0].color == 0xFFFFFFFF);
        
        CHECK(result->lines[0].glyphs[1].content == "B");
        CHECK(result->lines[0].glyphs[1].color == 0xFFFF0000);
        
        CHECK(result->lines[0].glyphs[2].content == "C");
        CHECK(result->lines[0].glyphs[2].color == 0xFFFFFFFF);
    }
    
    TEST_CASE("Text layout with hyperlink") {
        MockFontMetrics metrics;
        LayoutConfig config;
        
        auto result = TextLayoutEngine::BuildLayout("Click |Hitem:123|hhere|h!", metrics, config);
        
        REQUIRE(result.has_value());
        REQUIRE(result->hyperlinks.size() == 1);
        
        CHECK(result->hyperlinks[0].text == "item:123");
        // C(10) l(5) i(5) c(10) k(10)  (10) = 50.0f
        CHECK(result->hyperlinks[0].sx == 50.0f);
        // h(10) e(10) r(10) e(10) = 40.0f -> ex = 90.0f
        CHECK(result->hyperlinks[0].ex == 90.0f);
        CHECK(result->hyperlinks[0].sy == 0.0f);
        CHECK(result->hyperlinks[0].ey == 20.0f);
    }
    
    TEST_CASE("Multiline wrapping") {
        MockFontMetrics metrics;
        LayoutConfig config;
        config.multiLine = true;
        config.limitWidth = 45.0f; // Max 4 chars per line (40px)
        
        auto result = TextLayoutEngine::BuildLayout("123456789", metrics, config);
        
        REQUIRE(result.has_value());
        REQUIRE(result->lines.size() == 3);
        
        CHECK(result->lines[0].glyphs.size() == 4); // "1234"
        CHECK(result->lines[1].glyphs.size() == 4); // "5678"
        CHECK(result->lines[2].glyphs.size() == 1); // "9"
        
        CHECK(result->width == 40.0f);
        CHECK(result->height == 60.0f);
        
        // Check Y coordinates
        CHECK(result->lines[0].glyphs[0].y == 0.0f);
        CHECK(result->lines[1].glyphs[0].y == 20.0f);
        CHECK(result->lines[2].glyphs[0].y == 40.0f);
    }
    
    TEST_CASE("Alignment") {
        MockFontMetrics metrics;
        
        SUBCASE("Center Horizontal") {
            LayoutConfig config;
            config.horizontalAlign = HorizontalAlignment::Center;
            config.limitWidth = 100.0f;
            
            auto result = TextLayoutEngine::BuildLayout("Hello", metrics, config); // 40px width
            REQUIRE(result.has_value());
            // offsetX should be (100 - 40) / 2 = 30px
            CHECK(result->lines[0].glyphs[0].x == 30.0f);
        }
        
        SUBCASE("Right Horizontal") {
            LayoutConfig config;
            config.horizontalAlign = HorizontalAlignment::Right;
            config.limitWidth = 100.0f;
            
            auto result = TextLayoutEngine::BuildLayout("Hello", metrics, config); // 40px width
            REQUIRE(result.has_value());
            // offsetX should be 100 - 40 = 60px
            CHECK(result->lines[0].glyphs[0].x == 60.0f);
        }
        
        SUBCASE("Center Vertical") {
            LayoutConfig config;
            config.verticalAlign = VerticalAlignment::Center;
            
            auto result = TextLayoutEngine::BuildLayout("A", metrics, config); // 20px height
            REQUIRE(result.has_value());
            // startY should be -20 / 2 = -10px
            CHECK(result->lines[0].glyphs[0].y == -10.0f);
        }
        
        SUBCASE("Bottom Vertical") {
            LayoutConfig config;
            config.verticalAlign = VerticalAlignment::Bottom;
            
            auto result = TextLayoutEngine::BuildLayout("A", metrics, config); // 20px height
            REQUIRE(result.has_value());
            // startY should be -20px
            CHECK(result->lines[0].glyphs[0].y == -20.0f);
        }
    }
    
    TEST_CASE("RTL Direction") {
        MockFontMetrics metrics;
        LayoutConfig config;
        config.textDirection = TextDirection::RTL;
        config.limitWidth = 100.0f;
        
        auto result = TextLayoutEngine::BuildLayout("AB", metrics, config); // 20px width
        REQUIRE(result.has_value());
        
        CHECK(result->isRtlComputed == true);
        
        // offsetX should be 100 - 20 = 80px (for Left align in RTL with limit width)
        // A is placed at 80 + 20 - 10 = 90
        // B is placed at 80 + 20 - 10 - 10 = 80
        CHECK(result->lines[0].glyphs[0].content == "A");
        CHECK(result->lines[0].glyphs[0].x == 90.0f);
        CHECK(result->lines[0].glyphs[1].content == "B");
        CHECK(result->lines[0].glyphs[1].x == 80.0f);
    }
}
