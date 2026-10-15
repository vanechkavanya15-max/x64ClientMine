#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include "../src/Client/Text/TextStyleParser.h"

using namespace Client::Text;

TEST_CASE("TextStyleParser - ParseTag") {
    SUBCASE("Plain text") {
        auto res = TextStyleParser::ParseTag(L"Hello");
        CHECK(res.type == TagType::Plain);
        CHECK(res.tagLength == 1);
    }
    
    SUBCASE("Color tag") {
        auto res = TextStyleParser::ParseTag(L"|cFF00FF00Text");
        CHECK(res.type == TagType::Color);
        CHECK(res.tagLength == 10);
        CHECK(res.extraInfo == L"FF00FF00");
    }
    
    SUBCASE("Escape pipe") {
        auto res = TextStyleParser::ParseTag(L"||Text");
        CHECK(res.type == TagType::Tag);
        CHECK(res.tagLength == 2);
    }
    
    SUBCASE("Restore color") {
        auto res = TextStyleParser::ParseTag(L"|rText");
        CHECK(res.type == TagType::RestoreColor);
        CHECK(res.tagLength == 2);
    }
    
    SUBCASE("Hyperlink start") {
        auto res = TextStyleParser::ParseTag(L"|Hitem:1234");
        CHECK(res.type == TagType::HyperlinkStart);
        CHECK(res.tagLength == 2);
    }
    
    SUBCASE("Hyperlink end") {
        auto res = TextStyleParser::ParseTag(L"|h");
        CHECK(res.type == TagType::HyperlinkEnd);
        CHECK(res.tagLength == 2);
    }
}

TEST_CASE("TextStyleParser - GetOutputString") {
    SUBCASE("No tags") {
        CHECK(TextStyleParser::GetOutputString(L"Hello") == L"Hello");
    }
    
    SUBCASE("With color") {
        CHECK(TextStyleParser::GetOutputString(L"|cFFFF0000Red|rText") == L"RedText");
    }
    
    SUBCASE("With escaped pipe") {
        CHECK(TextStyleParser::GetOutputString(L"A||B") == L"A|B");
    }
    
    SUBCASE("With hyperlink") {
        CHECK(TextStyleParser::GetOutputString(L"|Hitem:1234|hSword|h") == L"Sword");
    }
}

TEST_CASE("TextStyleParser - GetOutputLen") {
    SUBCASE("No tags") {
        CHECK(TextStyleParser::GetOutputLen(L"Hello") == 5);
    }
    
    SUBCASE("With color") {
        CHECK(TextStyleParser::GetOutputLen(L"|cFFFF0000Red|rText") == 7); 
    }
    
    SUBCASE("With escaped pipe") {
        CHECK(TextStyleParser::GetOutputLen(L"A||B") == 3); 
    }
    
    SUBCASE("With hyperlink") {
        CHECK(TextStyleParser::GetOutputLen(L"|Hitem:1234|hSword|h") == 5); 
    }
}

TEST_CASE("TextStyleParser - GetInternalPosFromRenderPos") {
    SUBCASE("No tags") {
        CHECK(TextStyleParser::GetInternalPosFromRenderPos(L"Hello", 2) == 2);
    }
    
    SUBCASE("With color") {
        CHECK(TextStyleParser::GetInternalPosFromRenderPos(L"|cFFFF0000Red|rText", 1) == 11);
    }
    
    SUBCASE("Color internal pos fallback") {
        CHECK(TextStyleParser::GetInternalPosFromRenderPos(L"|cFFFF0000Text|r", 0) == 10);
    }
}

TEST_CASE("TextStyleParser - FindColorTagStartPosition") {
    SUBCASE("Valid case") {
        std::wstring str = L"Hello|cFFFF0000World|r";
        CHECK(TextStyleParser::FindColorTagStartPosition(str) == 16); 
    }
    
    SUBCASE("Invalid case") {
        std::wstring str = L"Hello|r";
        CHECK(TextStyleParser::FindColorTagStartPosition(str) == 7);
    }
}

TEST_CASE("TextStyleParser - FindColorTagEndPosition") {
    SUBCASE("Valid case") {
        std::wstring str = L"|cFFFF0000World|r";
        CHECK(TextStyleParser::FindColorTagEndPosition(str) == 16); 
    }
    
    SUBCASE("Invalid case") {
        std::wstring str = L"|cFFFF0000World";
        CHECK(TextStyleParser::FindColorTagEndPosition(str) == 0);
    }
}
