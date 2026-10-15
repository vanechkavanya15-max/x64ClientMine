#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include "Client/Player/PlayerSkillController.h"
#include "Client/Gameplay/SkillDomain.h"

using namespace Client::Player;
using namespace Client::Gameplay;
using namespace Client::Core;

TEST_CASE("PlayerSkillController - Valid skill use returns correct payload") {
    SkillDomain domain;
    SkillData sd;
    sd.baseSPCost = 10;
    domain.DefineSkill(1, sd);
    domain.RegisterSkill(1, 1);
    
    PlayerSkillController controller(domain);
    auto result = controller.UseSkill(1, 1234);
    
    REQUIRE(result.has_value());
    REQUIRE_FALSE(result.value().empty());
}

TEST_CASE("PlayerSkillController - Skill cooldown maps to CommandError::OnCooldown") {
    SkillDomain domain;
    SkillData sd;
    sd.baseSPCost = 10;
    domain.DefineSkill(1, sd);
    domain.RegisterSkill(1, 1);
    auto cooldownRes = domain.StartCooldownResult(1, 5000); // Trigger cooldown
    REQUIRE(cooldownRes.has_value());
    
    PlayerSkillController controller(domain);
    auto result = controller.UseSkill(1, 1234);
    
    REQUIRE_FALSE(result.has_value());
    REQUIRE(result.error() == CommandError::OnCooldown);
}

TEST_CASE("PlayerSkillController - Unknown skill maps to CommandError::SkillNotFound") {
    SkillDomain domain;
    
    PlayerSkillController controller(domain);
    auto result = controller.UseSkill(999, 1234);
    
    REQUIRE_FALSE(result.has_value());
    REQUIRE(result.error() == CommandError::SkillNotFound);
}
