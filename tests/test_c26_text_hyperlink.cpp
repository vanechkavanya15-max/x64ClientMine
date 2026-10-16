#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include <vector>
#include "../src/Client/Text/TextHyperlinkDetector.h"

using namespace Client::Text;

TEST_CASE("TextHyperlinkDetector - GetHyperlinkAtPosition") {
    std::vector<TextLayoutHyperlinkBounds> bounds = {
        {10.0f, 50.0f, 20.0f, 30.0f, "link1"},
        {60.0f, 100.0f, 20.0f, 30.0f, "link2"},
        {10.0f, 50.0f, 40.0f, 50.0f, "link3"}
    };

    SUBCASE("Detects hyperlink inside bounds") {
        auto result = TextHyperlinkDetector::GetHyperlinkAtPosition(bounds, 25.0f, 25.0f);
        REQUIRE(result.has_value());
        CHECK(result.value() == "link1");
        
        result = TextHyperlinkDetector::GetHyperlinkAtPosition(bounds, 80.0f, 25.0f);
        REQUIRE(result.has_value());
        CHECK(result.value() == "link2");
        
        result = TextHyperlinkDetector::GetHyperlinkAtPosition(bounds, 30.0f, 45.0f);
        REQUIRE(result.has_value());
        CHECK(result.value() == "link3");
    }

    SUBCASE("Detects hyperlink exactly on boundaries") {
        auto result1 = TextHyperlinkDetector::GetHyperlinkAtPosition(bounds, 10.0f, 20.0f); // Top-left
        REQUIRE(result1.has_value());
        CHECK(result1.value() == "link1");

        auto result2 = TextHyperlinkDetector::GetHyperlinkAtPosition(bounds, 50.0f, 30.0f); // Bottom-right
        REQUIRE(result2.has_value());
        CHECK(result2.value() == "link1");
        
        auto result3 = TextHyperlinkDetector::GetHyperlinkAtPosition(bounds, 10.0f, 30.0f); // Bottom-left
        REQUIRE(result3.has_value());
        CHECK(result3.value() == "link1");
        
        auto result4 = TextHyperlinkDetector::GetHyperlinkAtPosition(bounds, 50.0f, 20.0f); // Top-right
        REQUIRE(result4.has_value());
        CHECK(result4.value() == "link1");
    }

    SUBCASE("Returns nullopt outside of any bounds") {
        auto result = TextHyperlinkDetector::GetHyperlinkAtPosition(bounds, 5.0f, 25.0f);
        CHECK_FALSE(result.has_value());

        result = TextHyperlinkDetector::GetHyperlinkAtPosition(bounds, 25.0f, 15.0f);
        CHECK_FALSE(result.has_value());
        
        result = TextHyperlinkDetector::GetHyperlinkAtPosition(bounds, 55.0f, 25.0f);
        CHECK_FALSE(result.has_value());
        
        result = TextHyperlinkDetector::GetHyperlinkAtPosition(bounds, 25.0f, 35.0f);
        CHECK_FALSE(result.has_value());
    }

    SUBCASE("Handles empty bounds span") {
        std::vector<TextLayoutHyperlinkBounds> empty_bounds;
        auto result = TextHyperlinkDetector::GetHyperlinkAtPosition(empty_bounds, 25.0f, 25.0f);
        CHECK_FALSE(result.has_value());
    }
}
