#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest/doctest.h"
#include "Client/Input/IMECompositionManager.h"

using namespace Client::Input;

TEST_CASE("IMECompositionManager: Composition string and caret management") {
    IMECompositionManager ime;
    CHECK_FALSE(ime.HasComposition());
    ime.SetCompositionString("hangul", 6);
    CHECK(ime.HasComposition());
    CHECK(ime.GetString() == "hangul");
    CHECK(ime.GetCaretPos() == 6);
    
    ime.SetCompositionString("hangul", 100);
    CHECK(ime.GetCaretPos() == 6);
    
    ime.SetCompositionString("hangul", -10);
    CHECK(ime.GetCaretPos() == 0);
    
    ime.Clear();
    CHECK_FALSE(ime.HasComposition());
}

TEST_CASE("IMECompositionManager: Reading string management") {
    IMECompositionManager ime;
    CHECK_FALSE(ime.HasReading());
    ime.SetReadingString("reading");
    CHECK(ime.HasReading());
    CHECK(ime.GetReadingString() == "reading");
    
    ime.Clear();
    CHECK_FALSE(ime.HasReading());
}

TEST_CASE("IMECompositionManager: Clause management") {
    IMECompositionManager ime;
    ime.SetCompositionString("hello world", 11);
    
    std::vector<uint32_t> clauseLengths = {5, 1, 5};
    ime.SetClauses(clauseLengths);
    
    CHECK(ime.GetClauseCount() == 3);
    
    auto clauses = ime.GetClauses();
    CHECK(clauses[0].startPos == 0);
    CHECK(clauses[0].endPos == 5);
    
    auto text1 = ime.GetClauseText(0);
    REQUIRE(text1.has_value());
    CHECK(text1.value() == "hello");
    
    auto text3 = ime.GetClauseText(2);
    REQUIRE(text3.has_value());
    CHECK(text3.value() == "world");
    
    auto text4 = ime.GetClauseText(3);
    CHECK_FALSE(text4.has_value());
    CHECK(text4.error() == IMEError::ClauseIndexOutOfBounds);
}

TEST_CASE("IMECompositionManager: UTF-8 conversion") {
    IMECompositionManager ime;
    ime.SetCompositionString("Zażółć gęślą jaźń", 0);
    
    auto lenZ = IMECompositionManager::GetCodepointLength('Z');
    REQUIRE(lenZ.has_value());
    CHECK(lenZ.value() == 1);
    
    auto bOffset1 = ime.CodepointOffsetToByteOffset(1);
    REQUIRE(bOffset1.has_value());
    CHECK(bOffset1.value() == 1);
}
