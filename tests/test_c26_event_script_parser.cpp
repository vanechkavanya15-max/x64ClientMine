#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest/doctest.h>
#include "Client/Events/EventScriptParser.h"

using namespace Client::Events;

TEST_CASE("EventScriptParser - Valid Parsing") {
    SUBCASE("Single command without arguments") {
        std::string_view script = "command";
        auto result = EventScriptParser::Parse(script);
        REQUIRE(result.has_value());
        REQUIRE(result->size() == 1);
        CHECK(result->at(0).name == "command");
        CHECK(result->at(0).arguments.empty());
    }

    SUBCASE("Single command with arguments") {
        std::string_view script = "cmd arg1 arg2";
        auto result = EventScriptParser::Parse(script);
        REQUIRE(result.has_value());
        REQUIRE(result->size() == 1);
        CHECK(result->at(0).name == "cmd");
        REQUIRE(result->at(0).arguments.size() == 2);
        CHECK(result->at(0).arguments[0].value == "arg1");
        CHECK(result->at(0).arguments[1].value == "arg2");
    }

    SUBCASE("Arguments with quotes") {
        std::string_view script = "cmd \"arg with spaces\" arg2";
        auto result = EventScriptParser::Parse(script);
        REQUIRE(result.has_value());
        REQUIRE(result->size() == 1);
        CHECK(result->at(0).name == "cmd");
        REQUIRE(result->at(0).arguments.size() == 2);
        CHECK(result->at(0).arguments[0].value == "arg with spaces");
        CHECK(result->at(0).arguments[1].value == "arg2");
    }
    
    SUBCASE("Command name with quotes") {
        std::string_view script = "\"quoted cmd\" arg";
        auto result = EventScriptParser::Parse(script);
        REQUIRE(result.has_value());
        REQUIRE(result->size() == 1);
        CHECK(result->at(0).name == "quoted cmd");
        REQUIRE(result->at(0).arguments.size() == 1);
        CHECK(result->at(0).arguments[0].value == "arg");
    }

    SUBCASE("Multiple commands") {
        std::string_view script = "cmd1 arg1\ncmd2 \"arg 2\"\n\ncmd3";
        auto result = EventScriptParser::Parse(script);
        REQUIRE(result.has_value());
        REQUIRE(result->size() == 3);
        
        CHECK(result->at(0).name == "cmd1");
        REQUIRE(result->at(0).arguments.size() == 1);
        CHECK(result->at(0).arguments[0].value == "arg1");
        
        CHECK(result->at(1).name == "cmd2");
        REQUIRE(result->at(1).arguments.size() == 1);
        CHECK(result->at(1).arguments[0].value == "arg 2");
        
        CHECK(result->at(2).name == "cmd3");
        CHECK(result->at(2).arguments.empty());
    }

    SUBCASE("Whitespace handling") {
        std::string_view script = "  \t cmd1 \t arg1 \t \n  cmd2  arg2 \r\n  cmd3   ";
        auto result = EventScriptParser::Parse(script);
        REQUIRE(result.has_value());
        REQUIRE(result->size() == 3);
        
        CHECK(result->at(0).name == "cmd1");
        REQUIRE(result->at(0).arguments.size() == 1);
        CHECK(result->at(0).arguments[0].value == "arg1");
        
        CHECK(result->at(1).name == "cmd2");
        REQUIRE(result->at(1).arguments.size() == 1);
        CHECK(result->at(1).arguments[0].value == "arg2");
        
        CHECK(result->at(2).name == "cmd3");
        CHECK(result->at(2).arguments.empty());
    }
}

TEST_CASE("EventScriptParser - Error Conditions") {
    SUBCASE("Empty input") {
        std::string_view script = "";
        auto result = EventScriptParser::Parse(script);
        REQUIRE(!result.has_value());
        CHECK(result.error() == ParserError::EmptyInput);
    }
    
    SUBCASE("Whitespace only input") {
        std::string_view script = "   \n\t  \r\n ";
        auto result = EventScriptParser::Parse(script);
        REQUIRE(!result.has_value());
        CHECK(result.error() == ParserError::EmptyInput);
    }

    SUBCASE("Unmatched quotes") {
        std::string_view script = "cmd \"arg without closing quote";
        auto result = EventScriptParser::Parse(script);
        REQUIRE(!result.has_value());
        CHECK(result.error() == ParserError::UnmatchedQuotes);
    }
    
    SUBCASE("Empty quotes as command name") {
        std::string_view script = "\"\" arg";
        auto result = EventScriptParser::Parse(script);
        REQUIRE(!result.has_value());
        CHECK(result.error() == ParserError::EmptyCommandName);
    }
}
