#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include "Client/Skills/SkillTreeNavigator.h"
#include "Client/Core/StrongTypes.h"
#include "Client/Core/DomainErrors.h"
#include <vector>

using namespace Client::Skills;
using namespace Client::Core;

TEST_CASE("SkillTreeNavigator::CanLearnSkill") {
    std::vector<SkillTableRecord> skills = {
        {
            SkillId{1}, "Aura of Sword", 1, 1, 20, 5, 0, "", "", "", "", "", "", "", 0, 0, 0, "", "", 0, 
            SkillId{0}, 0, 1, 1, "", 0, 0
        },
        {
            SkillId{2}, "Advanced Aura", 1, 1, 10, 30, 0, "", "", "", "", "", "", "", 0, 0, 0, "", "", 0, 
            SkillId{1}, 20, 1, 1, "", 0, 0
        }
    };

    SkillTreeNavigator navigator(skills);

    SUBCASE("Skill not found") {
        SkillTreeContext ctx;
        ctx.availableSkillPoints = 1;
        auto result = navigator.CanLearnSkill(SkillId{999}, ctx);
        CHECK(!result.has_value());
        CHECK(result.error() == SkillError::SkillNotFound);
    }

    SUBCASE("No available skill points") {
        SkillTreeContext ctx;
        ctx.availableSkillPoints = 0;
        ctx.actorLevel = 10;
        ctx.currentSkillLevel = 0;
        auto result = navigator.CanLearnSkill(SkillId{1}, ctx);
        CHECK(!result.has_value());
        CHECK(result.error() == SkillError::RequirementNotMet);
    }

    SUBCASE("Actor level too low") {
        SkillTreeContext ctx;
        ctx.availableSkillPoints = 1;
        ctx.actorLevel = 4; // limit is 5
        ctx.currentSkillLevel = 0;
        auto result = navigator.CanLearnSkill(SkillId{1}, ctx);
        CHECK(!result.has_value());
        CHECK(result.error() == SkillError::RequirementNotMet);
    }

    SUBCASE("Max skill level reached") {
        SkillTreeContext ctx;
        ctx.availableSkillPoints = 1;
        ctx.actorLevel = 10;
        ctx.currentSkillLevel = 20; // max is 20
        auto result = navigator.CanLearnSkill(SkillId{1}, ctx);
        CHECK(!result.has_value());
        CHECK(result.error() == SkillError::RequirementNotMet);
    }

    SUBCASE("Prerequisite not met - no prerequisite level") {
        SkillTreeContext ctx;
        ctx.availableSkillPoints = 1;
        ctx.actorLevel = 35; // limit is 30
        ctx.currentSkillLevel = 0;
        ctx.prerequisiteSkillCurrentLevel = std::nullopt; // prerequisite level is missing
        auto result = navigator.CanLearnSkill(SkillId{2}, ctx);
        CHECK(!result.has_value());
        CHECK(result.error() == SkillError::RequirementNotMet);
    }

    SUBCASE("Prerequisite not met - prerequisite level too low") {
        SkillTreeContext ctx;
        ctx.availableSkillPoints = 1;
        ctx.actorLevel = 35; // limit is 30
        ctx.currentSkillLevel = 0;
        ctx.prerequisiteSkillCurrentLevel = 19; // needs 20
        auto result = navigator.CanLearnSkill(SkillId{2}, ctx);
        CHECK(!result.has_value());
        CHECK(result.error() == SkillError::RequirementNotMet);
    }

    SUBCASE("Can learn base skill") {
        SkillTreeContext ctx;
        ctx.availableSkillPoints = 1;
        ctx.actorLevel = 10;
        ctx.currentSkillLevel = 0;
        auto result = navigator.CanLearnSkill(SkillId{1}, ctx);
        CHECK(result.has_value());
    }

    SUBCASE("Can learn advanced skill") {
        SkillTreeContext ctx;
        ctx.availableSkillPoints = 1;
        ctx.actorLevel = 35;
        ctx.currentSkillLevel = 0;
        ctx.prerequisiteSkillCurrentLevel = 20;
        auto result = navigator.CanLearnSkill(SkillId{2}, ctx);
        CHECK(result.has_value());
    }
}

TEST_CASE("SkillTreeNavigator::GetSkillRecord and GetPrerequisiteSkill") {
    std::vector<SkillTableRecord> skills = {
        {
            SkillId{1}, "Aura of Sword", 1, 1, 20, 5, 0, "", "", "", "", "", "", "", 0, 0, 0, "", "", 0, 
            SkillId{0}, 0, 1, 1, "", 0, 0
        },
        {
            SkillId{2}, "Advanced Aura", 1, 1, 10, 30, 0, "", "", "", "", "", "", "", 0, 0, 0, "", "", 0, 
            SkillId{1}, 20, 1, 1, "", 0, 0
        }
    };

    SkillTreeNavigator navigator(skills);

    SUBCASE("GetSkillRecord") {
        auto recordOpt1 = navigator.GetSkillRecord(SkillId{1});
        REQUIRE(recordOpt1.has_value());
        CHECK(recordOpt1->strName == "Aura of Sword");

        auto recordOpt2 = navigator.GetSkillRecord(SkillId{999});
        CHECK(!recordOpt2.has_value());
    }

    SUBCASE("GetPrerequisiteSkill") {
        auto preqOpt1 = navigator.GetPrerequisiteSkill(SkillId{1});
        CHECK(!preqOpt1.has_value()); // No prerequisite

        auto preqOpt2 = navigator.GetPrerequisiteSkill(SkillId{2});
        REQUIRE(preqOpt2.has_value());
        CHECK(preqOpt2->strName == "Aura of Sword");
        CHECK(preqOpt2->dwVnum == SkillId{1});
    }
}
