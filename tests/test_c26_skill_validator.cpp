#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include "Client/Skills/SkillRequirementValidator.h"

using namespace Client::Skills;

TEST_CASE("SkillRequirementValidator - Basic Validations") {
    SkillRequirements reqs{
        .requiredSP = 50,
        .requiredHP = 0,
        .minLevel = 10,
        .requiredWeaponSubtype = std::nullopt,
        .requiresTarget = false
    };

    SkillCastContext ctx{
        .currentSP = 100,
        .currentHP = 1000,
        .actorLevel = 15,
        .isOnCooldown = false,
        .equippedWeaponSubtype = std::nullopt,
        .hasTarget = false,
        .isSkillLearned = true
    };

    SUBCASE("Successful validation") {
        auto result = SkillRequirementValidator::Validate(ctx, reqs);
        CHECK(result.has_value());
    }

    SUBCASE("Skill not learned") {
        ctx.isSkillLearned = false;
        auto result = SkillRequirementValidator::Validate(ctx, reqs);
        CHECK_FALSE(result.has_value());
        CHECK(result.error() == SkillValidationError::SkillNotLearned);
    }

    SUBCASE("On cooldown") {
        ctx.isOnCooldown = true;
        auto result = SkillRequirementValidator::Validate(ctx, reqs);
        CHECK_FALSE(result.has_value());
        CHECK(result.error() == SkillValidationError::OnCooldown);
    }

    SUBCASE("Level too low") {
        ctx.actorLevel = 5;
        auto result = SkillRequirementValidator::Validate(ctx, reqs);
        CHECK_FALSE(result.has_value());
        CHECK(result.error() == SkillValidationError::LevelTooLow);
    }

    SUBCASE("Not enough SP") {
        ctx.currentSP = 40;
        auto result = SkillRequirementValidator::Validate(ctx, reqs);
        CHECK_FALSE(result.has_value());
        CHECK(result.error() == SkillValidationError::NotEnoughSP);
    }

    SUBCASE("Not enough HP") {
        reqs.requiredHP = 200;
        ctx.currentHP = 100;
        auto result = SkillRequirementValidator::Validate(ctx, reqs);
        CHECK_FALSE(result.has_value());
        CHECK(result.error() == SkillValidationError::NotEnoughHP);
    }
}

TEST_CASE("SkillRequirementValidator - Weapon and Target Validations") {
    SkillRequirements reqs{
        .requiredSP = 10,
        .requiredHP = 0,
        .minLevel = 1,
        .requiredWeaponSubtype = 3, // e.g., Bow
        .requiresTarget = true
    };

    SkillCastContext ctx{
        .currentSP = 100,
        .currentHP = 100,
        .actorLevel = 10,
        .isOnCooldown = false,
        .equippedWeaponSubtype = 3,
        .hasTarget = true,
        .isSkillLearned = true
    };

    SUBCASE("Successful weapon and target validation") {
        auto result = SkillRequirementValidator::Validate(ctx, reqs);
        CHECK(result.has_value());
    }

    SUBCASE("Missing target") {
        ctx.hasTarget = false;
        auto result = SkillRequirementValidator::Validate(ctx, reqs);
        CHECK_FALSE(result.has_value());
        CHECK(result.error() == SkillValidationError::InvalidTarget);
    }

    SUBCASE("Invalid weapon subtype") {
        ctx.equippedWeaponSubtype = 1; // e.g., Sword
        auto result = SkillRequirementValidator::Validate(ctx, reqs);
        CHECK_FALSE(result.has_value());
        CHECK(result.error() == SkillValidationError::InvalidWeapon);
    }

    SUBCASE("No weapon equipped when required") {
        ctx.equippedWeaponSubtype = std::nullopt;
        auto result = SkillRequirementValidator::Validate(ctx, reqs);
        CHECK_FALSE(result.has_value());
        CHECK(result.error() == SkillValidationError::InvalidWeapon);
    }
}
