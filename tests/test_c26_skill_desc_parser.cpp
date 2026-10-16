#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest/doctest.h"
#include "Client/Skills/SkillDescriptionParser.h"

using namespace Client::Skills;

TEST_CASE("SkillDescriptionParser: Basic evaluation and formatting") {
    SkillDescriptionContext ctx;
    ctx.variables["atk"] = 150.0;
    ctx.variables["def"] = 50.0;

    SUBCASE("Simple variable") {
        auto res = SkillDescriptionParser::Parse("Atk: {atk}", ctx);
        REQUIRE(res.has_value());
        CHECK(res.value() == "Atk: 150");
    }

    SUBCASE("Math expression") {
        auto res = SkillDescriptionParser::Parse("Dmg: {atk * 2 + def}", ctx);
        REQUIRE(res.has_value());
        CHECK(res.value() == "Dmg: 350");
    }

    SUBCASE("Format: integer") {
        ctx.variables["val"] = 12.8;
        auto res = SkillDescriptionParser::Parse("Val: {val:d}", ctx);
        REQUIRE(res.has_value());
        CHECK(res.value() == "Val: 13");
    }

    SUBCASE("Format: percentage") {
        ctx.variables["val"] = 45.6;
        auto res = SkillDescriptionParser::Parse("Chance: {val:%}", ctx);
        REQUIRE(res.has_value());
        CHECK(res.value() == "Chance: 46%");
    }

    SUBCASE("Format: time") {
        ctx.variables["t1"] = 45.0;
        ctx.variables["t2"] = 125.0;
        CHECK(SkillDescriptionParser::Parse("T1: {t1:time}", ctx).value() == "T1: 45s");
        CHECK(SkillDescriptionParser::Parse("T2: {t2:time}", ctx).value() == "T2: 2m 5s");
    }
}

TEST_CASE("SkillDescriptionParser: Conditional blocks") {
    SkillDescriptionContext ctx;

    SUBCASE("Simple IF true") {
        ctx.variables["val"] = 1.0;
        auto res = SkillDescriptionParser::Parse("[IF val]Positive[ENDIF]", ctx);
        REQUIRE(res.has_value());
        CHECK(res.value() == "Positive");
    }

    SUBCASE("Simple IF false with ELSE") {
        ctx.variables["val"] = 0.0;
        auto res = SkillDescriptionParser::Parse("[IF val]Yes[ELSE]No[ENDIF]", ctx);
        REQUIRE(res.has_value());
        CHECK(res.value() == "No");
    }
}
