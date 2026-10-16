#include "doctest.h"

#include "../src/Client/Text/TextFontGlyphMetrics.h"

using namespace Client::Text;

TEST_CASE("TextFontGlyphMetrics - ASCII Lookups") {
    std::vector<GlyphAdvanceMapping> mappings = {
        {'A', 10.5f},
        {'B', 11.0f},
        {'a', 8.5f},
        {' ', 5.0f}
    };

    TextFontGlyphMetrics metrics(15.0f, mappings);

    CHECK(metrics.GetGlyphAdvance('A') == 10.5f);
    CHECK(metrics.GetGlyphAdvance('B') == 11.0f);
    CHECK(metrics.GetGlyphAdvance('a') == 8.5f);
    CHECK(metrics.GetGlyphAdvance(' ') == 5.0f);
}

TEST_CASE("TextFontGlyphMetrics - Unicode Lookups") {
    std::vector<GlyphAdvanceMapping> mappings = {
        {0x0104, 12.0f}, // 'Ą'
        {0x0105, 11.5f}, // 'ą'
        {0x20AC, 14.0f}  // '€'
    };

    TextFontGlyphMetrics metrics(16.0f, mappings);

    CHECK(metrics.GetGlyphAdvance(0x0104) == 12.0f);
    CHECK(metrics.GetGlyphAdvance(0x0105) == 11.5f);
    CHECK(metrics.GetGlyphAdvance(0x20AC) == 14.0f);
}

TEST_CASE("TextFontGlyphMetrics - Missing Glyph Advances") {
    std::vector<GlyphAdvanceMapping> mappings = {
        {'A', 10.0f}
    };

    TextFontGlyphMetrics metrics(15.0f, mappings);

    // ASCII not in mappings should be 0.0f
    CHECK(metrics.GetGlyphAdvance('B') == 0.0f);
    
    // Unicode not in mappings should be 0.0f
    CHECK(metrics.GetGlyphAdvance(0x0104) == 0.0f);
}

TEST_CASE("TextFontGlyphMetrics - Line Height") {
    std::vector<GlyphAdvanceMapping> mappings;

    TextFontGlyphMetrics metrics(22.5f, mappings);
    
    CHECK(metrics.GetLineHeight() == 22.5f);
}
