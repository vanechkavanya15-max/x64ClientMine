#include <doctest/doctest.h>
#include "Client/Text/TextTagColorParser.h"
#include <string>

using namespace Client::Text;

TEST_SUITE("TextTagColorParser") {
    TEST_CASE("ParseColor string_view - valid") {
        auto result = TextTagColorParser::ParseColor("ffff0000");
        CHECK(result.has_value());
        if (result) CHECK(*result == 4294901760u);
        
        auto result2 = TextTagColorParser::ParseColor("00000000");
        CHECK(result2.has_value());
        if (result2) CHECK(*result2 == 0u);
        
        auto result3 = TextTagColorParser::ParseColor("12345678");
        CHECK(result3.has_value());
        if (result3) CHECK(*result3 == 305419896u);
        
        auto result4 = TextTagColorParser::ParseColor("AABBCCDD");
        CHECK(result4.has_value());
        if (result4) CHECK(*result4 == 2864434397u);
    }
    
    TEST_CASE("ParseColor string_view - longer string valid") {
        auto result = TextTagColorParser::ParseColor("ffff0000123");
        CHECK(result.has_value());
        if (result) CHECK(*result == 4294901760u);
    }
    
    TEST_CASE("ParseColor string_view - errors") {
        auto result = TextTagColorParser::ParseColor("fff");
        CHECK(!result.has_value());
        if (!result) CHECK(static_cast<int>(result.error()) == static_cast<int>(ColorParseError::TooShort));
        
        auto result2 = TextTagColorParser::ParseColor("ffxx0000");
        CHECK(!result2.has_value());
        if (!result2) CHECK(static_cast<int>(result2.error()) == static_cast<int>(ColorParseError::InvalidFormat));
        
        auto result3 = TextTagColorParser::ParseColor("-1ff0000");
        CHECK(!result3.has_value());
        if (!result3) CHECK(static_cast<int>(result3.error()) == static_cast<int>(ColorParseError::InvalidFormat));
    }
    
    TEST_CASE("ParseColor wstring_view - valid") {
        auto result = TextTagColorParser::ParseColor(L"FFff0000");
        CHECK(result.has_value());
        if (result) CHECK(*result == 4294901760u);
        
        auto result2 = TextTagColorParser::ParseColor(L"00000000");
        CHECK(result2.has_value());
        if (result2) CHECK(*result2 == 0u);
        
        auto result3 = TextTagColorParser::ParseColor(L"12345678");
        CHECK(result3.has_value());
        if (result3) CHECK(*result3 == 305419896u);
        
        auto result4 = TextTagColorParser::ParseColor(L"AABBCCDD");
        CHECK(result4.has_value());
        if (result4) CHECK(*result4 == 2864434397u);
    }
    
    TEST_CASE("ParseColor wstring_view - longer string valid") {
        auto result = TextTagColorParser::ParseColor(L"FFff0000123");
        CHECK(result.has_value());
        if (result) CHECK(*result == 4294901760u);
    }
    
    TEST_CASE("ParseColor wstring_view - errors") {
        auto result = TextTagColorParser::ParseColor(L"fff");
        CHECK(!result.has_value());
        if (!result) CHECK(static_cast<int>(result.error()) == static_cast<int>(ColorParseError::TooShort));
        
        auto result2 = TextTagColorParser::ParseColor(L"ffxx0000");
        CHECK(!result2.has_value());
        if (!result2) CHECK(static_cast<int>(result2.error()) == static_cast<int>(ColorParseError::InvalidFormat));
        
        auto result3 = TextTagColorParser::ParseColor(L"-1ff0000");
        CHECK(!result3.has_value());
        if (!result3) CHECK(static_cast<int>(result3.error()) == static_cast<int>(ColorParseError::InvalidFormat));
    }
}
