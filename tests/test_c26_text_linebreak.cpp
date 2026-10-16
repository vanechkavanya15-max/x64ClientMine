#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest/doctest.h>
#include "../src/Client/Text/TextLineBreakFormatter.h"
#include <string>

using namespace Client::Text;

class MockMetricsProvider : public ILineBreakMetricsProvider {
public:
    float GetGlyphAdvance(char32_t codepoint) const override {
        // Simple mock: assume all characters are width 10.0f
        return 10.0f;
    }
};

TEST_CASE("TextLineBreakFormatter - Empty Input") {
    MockMetricsProvider metrics;
    auto result = TextLineBreakFormatter::BreakLines("", metrics, 100.0f);
    REQUIRE(result.has_value());
    REQUIRE(result.value().size() == 1);
    CHECK(result.value()[0].content == "");
    CHECK(result.value()[0].width == 0.0f);
}

TEST_CASE("TextLineBreakFormatter - Fits in one line") {
    MockMetricsProvider metrics;
    // "Hello" is 5 chars * 10 = 50.0f width.
    auto result = TextLineBreakFormatter::BreakLines("Hello", metrics, 100.0f);
    REQUIRE(result.has_value());
    REQUIRE(result.value().size() == 1);
    CHECK(result.value()[0].content == "Hello");
    CHECK(result.value()[0].width == 50.0f);
}

TEST_CASE("TextLineBreakFormatter - Word break") {
    MockMetricsProvider metrics;
    // "Hello World"
    // "Hello " is 6 chars * 10 = 60.0f width.
    // "World" is 5 chars * 10 = 50.0f width.
    // If max is 60.0f, "Hello" fits, "World" goes to next line.
    auto result = TextLineBreakFormatter::BreakLines("Hello World", metrics, 60.0f);
    REQUIRE(result.has_value());
    auto lines = result.value();
    REQUIRE(lines.size() == 2);
    CHECK(lines[0].content == "Hello");
    CHECK(lines[0].width == 50.0f);
    CHECK(lines[1].content == "World");
    CHECK(lines[1].width == 50.0f);
}

TEST_CASE("TextLineBreakFormatter - Explicit newlines") {
    MockMetricsProvider metrics;
    auto result = TextLineBreakFormatter::BreakLines("Line1\nLine2", metrics, 100.0f);
    REQUIRE(result.has_value());
    auto lines = result.value();
    REQUIRE(lines.size() == 2);
    CHECK(lines[0].content == "Line1");
    CHECK(lines[0].width == 50.0f);
    CHECK(lines[1].content == "Line2");
    CHECK(lines[1].width == 50.0f);
}

TEST_CASE("TextLineBreakFormatter - Mid-word break") {
    MockMetricsProvider metrics;
    // "Superlongword"
    // 13 chars * 10 = 130.0f
    // If max is 50.0f, it should break into chunks of 5 chars.
    auto result = TextLineBreakFormatter::BreakLines("Superlongword", metrics, 50.0f);
    REQUIRE(result.has_value());
    auto lines = result.value();
    REQUIRE(lines.size() == 3);
    CHECK(lines[0].content == "Super");
    CHECK(lines[0].width == 50.0f);
    CHECK(lines[1].content == "longw");
    CHECK(lines[1].width == 50.0f);
    CHECK(lines[2].content == "ord");
    CHECK(lines[2].width == 30.0f);
}

TEST_CASE("TextLineBreakFormatter - Tags ignore width") {
    MockMetricsProvider metrics;
    // "|cffffffffHello|r"
    // "Hello" is 50.0f. Tags take 0.0f.
    // If max is 60.0f, the whole string should fit on one line and report width 50.0f.
    auto result = TextLineBreakFormatter::BreakLines("|cffffffffHello|r", metrics, 60.0f);
    REQUIRE(result.has_value());
    auto lines = result.value();
    REQUIRE(lines.size() == 1);
    CHECK(lines[0].content == "|cffffffffHello|r");
    CHECK(lines[0].width == 50.0f);
}

TEST_CASE("TextLineBreakFormatter - Tags with line break") {
    MockMetricsProvider metrics;
    // "|cffffffffHello World|r"
    // Max 60.0f
    // line 1: "|cffffffffHello" (width 50.0f)
    // line 2: "World|r" (width 50.0f)
    auto result = TextLineBreakFormatter::BreakLines("|cffffffffHello World|r", metrics, 60.0f);
    REQUIRE(result.has_value());
    auto lines = result.value();
    REQUIRE(lines.size() == 2);
    CHECK(lines[0].content == "|cffffffffHello");
    CHECK(lines[0].width == 50.0f);
    CHECK(lines[1].content == "World|r");
    CHECK(lines[1].width == 50.0f);
}

TEST_CASE("TextLineBreakFormatter - UTF-8 character width") {
    class PolishMetricsProvider : public ILineBreakMetricsProvider {
    public:
        float GetGlyphAdvance(char32_t codepoint) const override {
            if (codepoint == U'ą' || codepoint == U'ę') {
                return 15.0f;
            }
            return 10.0f;
        }
    };
    
    PolishMetricsProvider metrics;
    // "Cześć" -> C z e ś ć -> let's say ś, ć not special, but we test 'ą' or 'ę'
    // "Mąka" -> M (10) ą (15) k (10) a (10) = 45.0f
    auto result = TextLineBreakFormatter::BreakLines("Mąka", metrics, 100.0f);
    REQUIRE(result.has_value());
    auto lines = result.value();
    REQUIRE(lines.size() == 1);
    CHECK(lines[0].content == "Mąka");
    CHECK(lines[0].width == 45.0f);
}

TEST_CASE("TextLineBreakFormatter - Hyperlink Tags") {
    MockMetricsProvider metrics;
    // "|Hitem:123|h[Item Name]|h"
    // "[Item Name]" is 11 chars = 110.0f
    // Tag should be ignored. Max 200.0f
    auto result = TextLineBreakFormatter::BreakLines("|Hitem:123|h[Item Name]|h", metrics, 200.0f);
    REQUIRE(result.has_value());
    auto lines = result.value();
    REQUIRE(lines.size() == 1);
    CHECK(lines[0].content == "|Hitem:123|h[Item Name]|h");
    CHECK(lines[0].width == 110.0f);
}
