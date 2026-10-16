#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest/doctest.h"
#include "Client/Skills/SkillRequirementEvaluator.h"
#include <string>

namespace doctest {
    template<> struct StringMaker<Client::Skills::SkillEvaluationError> {
        static String convert(const Client::Skills::SkillEvaluationError& err) {
            return ToString(err).data();
        }
    };
    template<typename T, typename E> struct StringMaker<std::expected<T, E>> {
        static String convert(const std::expected<T, E>& val) {
            if (val.has_value()) {
                return "ExpectedResult(Success)";
            } else {
                return String("ExpectedResult(Error: ") + StringMaker<E>::convert(val.error()) + ")";
            }
        }
    };
}

using namespace Client::Skills;

TEST_CASE("SpHpRequirementRule Evaluation") {
    SpHpRequirementRule rule;
    SkillEvaluationContext ctx;
    ctx.character.currentSP = EvalManaPoints(100);
    ctx.character.currentHP = EvalHealthPoints(500);
    ctx.skill.requiredSP = EvalManaPoints(50);
    ctx.skill.requiredHP = EvalHealthPoints(100);

    SUBCASE("Sufficient SP and HP") {
        CHECK(rule.Evaluate(ctx).has_value());
    }
    SUBCASE("Exact SP and HP") {
        ctx.character.currentSP = EvalManaPoints(50);
        ctx.character.currentHP = EvalHealthPoints(100);
        CHECK(rule.Evaluate(ctx).has_value());
    }
    SUBCASE("Insufficient SP") {
        ctx.character.currentSP = EvalManaPoints(49);
        auto res = rule.Evaluate(ctx);
        CHECK_FALSE(res.has_value());
        CHECK_EQ(res.error(), SkillEvaluationError::NotEnoughSP);
    }
    SUBCASE("Insufficient HP") {
        ctx.character.currentHP = EvalHealthPoints(99);
        auto res = rule.Evaluate(ctx);
        CHECK_FALSE(res.has_value());
        CHECK_EQ(res.error(), SkillEvaluationError::NotEnoughHP);
    }
}

TEST_CASE("LevelRequirementRule Evaluation") {
    LevelRequirementRule rule;
    SkillEvaluationContext ctx;
    ctx.character.actorLevel = EvalLevel(10);
    ctx.skill.minLevel = EvalLevel(5);
    ctx.skill.isLearned = true;

    SUBCASE("Level high enough and skill learned") {
        CHECK(rule.Evaluate(ctx).has_value());
    }
    SUBCASE("Level too low") {
        ctx.character.actorLevel = EvalLevel(4);
        auto res = rule.Evaluate(ctx);
        CHECK_FALSE(res.has_value());
        CHECK_EQ(res.error(), SkillEvaluationError::LevelTooLow);
    }
    SUBCASE("Skill not learned") {
        ctx.skill.isLearned = false;
        auto res = rule.Evaluate(ctx);
        CHECK_FALSE(res.has_value());
        CHECK_EQ(res.error(), SkillEvaluationError::SkillNotLearned);
    }
}

TEST_CASE("CooldownRequirementRule Evaluation") {
    CooldownRequirementRule rule;
    SkillEvaluationContext ctx;

    SUBCASE("Not on cooldown") {
        ctx.skill.isOnCooldown = false;
        CHECK(rule.Evaluate(ctx).has_value());
    }
    SUBCASE("On cooldown") {
        ctx.skill.isOnCooldown = true;
        auto res = rule.Evaluate(ctx);
        CHECK_FALSE(res.has_value());
        CHECK_EQ(res.error(), SkillEvaluationError::OnCooldown);
    }
}

TEST_CASE("WeaponRequirementRule Evaluation") {
    WeaponRequirementRule rule;
    SkillEvaluationContext ctx;

    SUBCASE("No weapon required") {
        ctx.skill.requiredWeaponSubtype = std::nullopt;
        CHECK(rule.Evaluate(ctx).has_value());
    }
    SUBCASE("Weapon required and correctly equipped") {
        ctx.skill.requiredWeaponSubtype = EvalWeaponSubtype(1);
        ctx.character.equippedWeaponSubtype = EvalWeaponSubtype(1);
        CHECK(rule.Evaluate(ctx).has_value());
    }
    SUBCASE("Weapon required but not equipped") {
        ctx.skill.requiredWeaponSubtype = EvalWeaponSubtype(1);
        ctx.character.equippedWeaponSubtype = std::nullopt;
        auto res = rule.Evaluate(ctx);
        CHECK_FALSE(res.has_value());
        CHECK_EQ(res.error(), SkillEvaluationError::InvalidWeapon);
    }
    SUBCASE("Weapon required but wrong type equipped") {
        ctx.skill.requiredWeaponSubtype = EvalWeaponSubtype(1);
        ctx.character.equippedWeaponSubtype = EvalWeaponSubtype(2);
        auto res = rule.Evaluate(ctx);
        CHECK_FALSE(res.has_value());
        CHECK_EQ(res.error(), SkillEvaluationError::InvalidWeapon);
    }
}

TEST_CASE("TargetRequirementRule Evaluation") {
    TargetRequirementRule rule;
    SkillEvaluationContext ctx;

    SUBCASE("No target required") {
        ctx.skill.requiresTarget = false;
        CHECK(rule.Evaluate(ctx).has_value());
    }
    SUBCASE("Target required and valid") {
        ctx.skill.requiresTarget = true;
        ctx.target.hasTarget = true;
        ctx.target.isTargetValid = true;
        ctx.skill.maxRange = 100.0f;
        ctx.target.distanceToTarget = 50.0f;
        CHECK(rule.Evaluate(ctx).has_value());
    }
    SUBCASE("Target required but no target") {
        ctx.skill.requiresTarget = true;
        ctx.target.hasTarget = false;
        auto res = rule.Evaluate(ctx);
        CHECK_FALSE(res.has_value());
        CHECK_EQ(res.error(), SkillEvaluationError::InvalidTarget);
    }
    SUBCASE("Target required but target invalid") {
        ctx.skill.requiresTarget = true;
        ctx.target.hasTarget = true;
        ctx.target.isTargetValid = false;
        auto res = rule.Evaluate(ctx);
        CHECK_FALSE(res.has_value());
        CHECK_EQ(res.error(), SkillEvaluationError::InvalidTarget);
    }
    SUBCASE("Target required but out of range") {
        ctx.skill.requiresTarget = true;
        ctx.target.hasTarget = true;
        ctx.target.isTargetValid = true;
        ctx.skill.maxRange = 50.0f;
        ctx.target.distanceToTarget = 51.0f;
        auto res = rule.Evaluate(ctx);
        CHECK_FALSE(res.has_value());
        CHECK_EQ(res.error(), SkillEvaluationError::OutOfRange);
    }
}

TEST_CASE("StateRestrictionRule Evaluation") {
    StateRestrictionRule rule;
    SkillEvaluationContext ctx;

    SUBCASE("No restrictions") {
        ctx.character.inSafeZone = false;
        ctx.character.isPolymorphed = false;
        ctx.character.isRiding = false;
        CHECK(rule.Evaluate(ctx).has_value());
    }
    SUBCASE("In safe zone") {
        ctx.character.inSafeZone = true;
        auto res = rule.Evaluate(ctx);
        CHECK_FALSE(res.has_value());
        CHECK_EQ(res.error(), SkillEvaluationError::InSafeZone);
    }
    SUBCASE("Is polymorphed") {
        ctx.character.isPolymorphed = true;
        auto res = rule.Evaluate(ctx);
        CHECK_FALSE(res.has_value());
        CHECK_EQ(res.error(), SkillEvaluationError::CannotCastWhilePolymorphed);
    }
    SUBCASE("Is riding and skill not castable while riding") {
        ctx.character.isRiding = true;
        ctx.skill.castableWhileRiding = false;
        auto res = rule.Evaluate(ctx);
        CHECK_FALSE(res.has_value());
        CHECK_EQ(res.error(), SkillEvaluationError::CannotCastWhileRiding);
    }
    SUBCASE("Is riding and skill castable while riding") {
        ctx.character.isRiding = true;
        ctx.skill.castableWhileRiding = true;
        CHECK(rule.Evaluate(ctx).has_value());
    }
}

TEST_CASE("StandardEvaluator Integration") {
    auto evaluator = SkillRequirementEvaluator::CreateStandardEvaluator();
    SkillEvaluationContext ctx;
    ctx.character.actorLevel = EvalLevel(10);
    ctx.character.currentSP = EvalManaPoints(100);
    ctx.character.currentHP = EvalHealthPoints(100);
    ctx.skill.minLevel = EvalLevel(5);
    ctx.skill.isLearned = true;
    ctx.skill.requiredSP = EvalManaPoints(50);
    ctx.skill.requiredHP = EvalHealthPoints(50);

    SUBCASE("All requirements met") {
        CHECK(evaluator.CanCast(ctx).has_value());
    }
    SUBCASE("Fails state restriction (Safe Zone)") {
        ctx.character.inSafeZone = true;
        auto res = evaluator.CanCast(ctx);
        CHECK_FALSE(res.has_value());
        CHECK_EQ(res.error(), SkillEvaluationError::InSafeZone);
    }
    SUBCASE("Fails level requirement") {
        ctx.character.actorLevel = EvalLevel(1);
        auto res = evaluator.CanCast(ctx);
        CHECK_FALSE(res.has_value());
        CHECK_EQ(res.error(), SkillEvaluationError::LevelTooLow);
    }
    SUBCASE("Fails cooldown requirement") {
        ctx.skill.isOnCooldown = true;
        auto res = evaluator.CanCast(ctx);
        CHECK_FALSE(res.has_value());
        CHECK_EQ(res.error(), SkillEvaluationError::OnCooldown);
    }
    SUBCASE("Fails weapon requirement") {
        ctx.skill.requiredWeaponSubtype = EvalWeaponSubtype(1);
        auto res = evaluator.CanCast(ctx);
        CHECK_FALSE(res.has_value());
        CHECK_EQ(res.error(), SkillEvaluationError::InvalidWeapon);
    }
    SUBCASE("Fails SP requirement") {
        ctx.character.currentSP = EvalManaPoints(10);
        auto res = evaluator.CanCast(ctx);
        CHECK_FALSE(res.has_value());
        CHECK_EQ(res.error(), SkillEvaluationError::NotEnoughSP);
    }
    SUBCASE("Fails target requirement") {
        ctx.skill.requiresTarget = true;
        auto res = evaluator.CanCast(ctx);
        CHECK_FALSE(res.has_value());
        CHECK_EQ(res.error(), SkillEvaluationError::InvalidTarget);
    }
}
