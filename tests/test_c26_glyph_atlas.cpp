#include "doctest.h"
#include "Client/Text/TextGlyphAtlasManager.h"
#include <vector>

using namespace Client::Text;

TEST_CASE("TextGlyphAtlasManager - Initialization and Defaults") {
    TextGlyphAtlasManager manager(256, 128);
    
    CHECK(manager.GetAtlasWidth() == 256);
    CHECK(manager.GetAtlasHeight() == 128);
    
    auto pixelData = manager.GetAtlasPixelData();
    CHECK(pixelData.size() == 256 * 128);
    
    // Check if initial buffer is zeroed out
    bool allZero = true;
    for (uint8_t pixel : pixelData) {
        if (pixel != 0) {
            allZero = false;
            break;
        }
    }
    CHECK(allZero);
}

TEST_CASE("TextGlyphAtlasManager - Add Valid Glyph") {
    TextGlyphAtlasManager manager(128, 128);
    std::vector<uint8_t> glyphData(16 * 16, 255); // 16x16 white block
    
    auto result = manager.AddGlyph(16, 16, glyphData);
    REQUIRE(result.has_value());
    
    auto rect = result.value();
    CHECK(rect.x == 0);
    CHECK(rect.y == 0);
    CHECK(rect.width == 16);
    CHECK(rect.height == 16);
    
    // Verify pixel data was copied correctly
    auto atlasData = manager.GetAtlasPixelData();
    CHECK(atlasData[0] == 255);
    CHECK(atlasData[15] == 255);
    CHECK(atlasData[16] == 0); // Out of glyph bounds (width is 128)
}

TEST_CASE("TextGlyphAtlasManager - Invalid Arguments") {
    TextGlyphAtlasManager manager(128, 128);
    std::vector<uint8_t> glyphData(16 * 16, 255);
    
    SUBCASE("Invalid Dimensions - Zero Width/Height") {
        auto res1 = manager.AddGlyph(0, 16, glyphData);
        CHECK_FALSE(res1.has_value());
        CHECK(res1.error() == GlyphAtlasError::InvalidDimensions);
        
        auto res2 = manager.AddGlyph(16, 0, glyphData);
        CHECK_FALSE(res2.has_value());
        CHECK(res2.error() == GlyphAtlasError::InvalidDimensions);
    }
    
    SUBCASE("Glyph Too Large") {
        std::vector<uint8_t> hugeData(256 * 256, 255);
        auto res = manager.AddGlyph(256, 256, hugeData);
        CHECK_FALSE(res.has_value());
        CHECK(res.error() == GlyphAtlasError::GlyphTooLarge);
    }
    
    SUBCASE("Invalid Pixel Data Size") {
        std::vector<uint8_t> smallData(10, 255); // Not enough for 16x16
        auto res = manager.AddGlyph(16, 16, smallData);
        CHECK_FALSE(res.has_value());
        CHECK(res.error() == GlyphAtlasError::InvalidPixelDataSize);
    }
}

TEST_CASE("TextGlyphAtlasManager - Shelf Packing and Atlas Full") {
    TextGlyphAtlasManager manager(32, 32);
    std::vector<uint8_t> glyphData(16 * 16, 255);
    
    // 1st glyph
    auto res1 = manager.AddGlyph(16, 16, glyphData);
    REQUIRE(res1.has_value());
    CHECK(res1.value().x == 0);
    CHECK(res1.value().y == 0);
    
    // 2nd glyph (fits on same shelf)
    auto res2 = manager.AddGlyph(16, 16, glyphData);
    REQUIRE(res2.has_value());
    CHECK(res2.value().x == 16);
    CHECK(res2.value().y == 0);
    
    // 3rd glyph (moves to next shelf)
    auto res3 = manager.AddGlyph(16, 16, glyphData);
    REQUIRE(res3.has_value());
    CHECK(res3.value().x == 0);
    CHECK(res3.value().y == 16);
    
    // 4th glyph (fits on second shelf)
    auto res4 = manager.AddGlyph(16, 16, glyphData);
    REQUIRE(res4.has_value());
    CHECK(res4.value().x == 16);
    CHECK(res4.value().y == 16);
    
    // 5th glyph (atlas full)
    auto res5 = manager.AddGlyph(16, 16, glyphData);
    CHECK_FALSE(res5.has_value());
    CHECK(res5.error() == GlyphAtlasError::AtlasFull);
}

TEST_CASE("TextGlyphAtlasManager - Clear") {
    TextGlyphAtlasManager manager(32, 32);
    std::vector<uint8_t> glyphData(16 * 16, 255);
    
    auto res1 = manager.AddGlyph(16, 16, glyphData);
    REQUIRE(res1.has_value());
    
    manager.Clear();
    
    // Adding again should put it back at 0,0
    auto res2 = manager.AddGlyph(16, 16, glyphData);
    REQUIRE(res2.has_value());
    CHECK(res2.value().x == 0);
    CHECK(res2.value().y == 0);
    
    // Atlas memory should be cleared initially (verified by 0s around new placement)
    auto atlasData = manager.GetAtlasPixelData();
    CHECK(atlasData[0] == 255); // Top left of new glyph
    
    // Ensure memory after the first glyph is zeroed out by Clear
    CHECK(atlasData[16] == 0); // Past the first 16 pixels
}
