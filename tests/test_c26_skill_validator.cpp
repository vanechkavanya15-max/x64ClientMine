#include <doctest.h>
#include "../src/Client/Skills/SkillRequirementValidator.h"

using namespace Client::Skills;
using namespace Client::Core;
using namespace Client::Gameplay;

TEST_CASE("SkillRequirementValidator - Success") {
    SkillRequirementContext ctx{
        .currentHP = 100,
        .currentSP = 50,
        .currentLevel = 10,
        .currentWeapon = WeaponType::Sword
    };
    
    SkillRequirements reqs{
        .minLevel = 5,
        .hpCost = 10,
        .spCost = 20,
        .requiresWeapon = true,
        .allowedWeaponsMask = (1u << static_cast<uint32_t>(WeaponType::Sword))
    };
    
    auto result = SkillRequirementValidator::Validate(ctx, reqs);
    CHECK(result.has_value() == true);
}

TEST_CASE("SkillRequirementValidator - Not enough SP") {
    SkillRequirementContext ctx{
        .currentHP = 100,
        .currentSP = 10, // Not enough SP
        .currentLevel = 10,
        .currentWeapon = WeaponType::Sword
    };
    
    SkillRequirements reqs{
        .minLevel = 5,
        .hpCost = 10,
        .spCost = 20,
        .requiresWeapon = true,
        .allowedWeaponsMask = (1u << static_cast<uint32_t>(WeaponType::Sword))
    };
    
    auto result = SkillRequirementValidator::Validate(ctx, reqs);
    CHECK(result.has_value() == false);
    CHECK(result.error() == SkillError::NotEnoughSP);
}

TEST_CASE("SkillRequirementValidator - Requirement Not Met (HP)") {
    SkillRequirementContext ctx{
        .currentHP = 10, // HP would reach 0, meaning dead, so usually we require > hpCost
        .currentSP = 50,
        .currentLevel = 10,
        .currentWeapon = WeaponType::Sword
    };
    
    SkillRequirements reqs{
        .minLevel = 5,
        .hpCost = 10,
        .spCost = 20,
        .requiresWeapon = false
    };
    
    auto result = SkillRequirementValidator::Validate(ctx, reqs);
    CHECK(result.has_value() == false);
    CHECK(result.error() == SkillError::RequirementNotMet);
}

TEST_CASE("SkillRequirementValidator - Requirement Not Met (Level)") {
    SkillRequirementContext ctx{
        .currentHP = 100,
        .currentSP = 50,
        .currentLevel = 4, // Too low
        .currentWeapon = WeaponType::Sword
    };
    
    SkillRequirements reqs{
        .minLevel = 5,
        .hpCost = 10,
        .spCost = 20,
        .requiresWeapon = false
    };
    
    auto result = SkillRequirementValidator::Validate(ctx, reqs);
    CHECK(result.has_value() == false);
    CHECK(result.error() == SkillError::RequirementNotMet);
}

TEST_CASE("SkillRequirementValidator - Requirement Not Met (Missing Weapon)") {
    SkillRequirementContext ctx{
        .currentHP = 100,
        .currentSP = 50,
        .currentLevel = 10,
        .currentWeapon = std::nullopt // Missing weapon
    };
    
    SkillRequirements reqs{
        .minLevel = 5,
        .hpCost = 10,
        .spCost = 20,
        .requiresWeapon = true,
        .allowedWeaponsMask = (1u << static_cast<uint32_t>(WeaponType::Sword))
    };
    
    auto result = SkillRequirementValidator::Validate(ctx, reqs);
    CHECK(result.has_value() == false);
    CHECK(result.error() == SkillError::RequirementNotMet);
}

TEST_CASE("SkillRequirementValidator - Requirement Not Met (Wrong Weapon)") {
    SkillRequirementContext ctx{
        .currentHP = 100,
        .currentSP = 50,
        .currentLevel = 10,
        .currentWeapon = WeaponType::Bow // Wrong weapon
    };
    
    SkillRequirements reqs{
        .minLevel = 5,
        .hpCost = 10,
        .spCost = 20,
        .requiresWeapon = true,
        .allowedWeaponsMask = (1u << static_cast<uint32_t>(WeaponType::Sword))
    };
    
    auto result = SkillRequirementValidator::Validate(ctx, reqs);
    CHECK(result.has_value() == false);
    CHECK(result.error() == SkillError::RequirementNotMet);
}

TEST_CASE("SkillRequirementValidator - Allowed with Multiple Weapons") {
    SkillRequirementContext ctx{
        .currentHP = 100,
        .currentSP = 50,
        .currentLevel = 10,
        .currentWeapon = WeaponType::Dagger // Dagger is allowed
    };
    
    SkillRequirements reqs{
        .minLevel = 5,
        .hpCost = 10,
        .spCost = 20,
        .requiresWeapon = true,
        .allowedWeaponsMask = (1u << static_cast<uint32_t>(WeaponType::Sword)) | (1u << static_cast<uint32_t>(WeaponType::Dagger))
    };
    
    auto result = SkillRequirementValidator::Validate(ctx, reqs);
    CHECK(result.has_value() == true);
}
