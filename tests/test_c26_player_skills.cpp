#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include "Client/Player/PlayerSkillController.h"
#include "Client/Gameplay/SkillDomain.h"
#include "Client/Gameplay/IPlayerStatsService.h"

using namespace Client::Player;
using namespace Client::Gameplay;
using namespace Client::Core;

class MockPlayerStatsService : public IPlayerStatsService {
public:
    int64_t mockSP = 0;

    void SetPoint(uint32_t type, int64_t value) override {}
    int64_t GetPoint(uint32_t type) const override {
        if (type == 7) return mockSP; // POINT_SP
        return 0;
    }
    const PlayerPointsView& GetPoints() const override {
        static PlayerPointsView pv;
        return pv;
    }
    void Clear() override {}
};

TEST_CASE("PlayerSkillController - Valid skill use returns correct payload") {
    SkillDomain domain;
    SkillData sd;
    sd.baseSPCost = 10;
    domain.DefineSkill(1, sd);
    domain.RegisterSkill(1, 1);
    
    MockPlayerStatsService statsService;
    statsService.mockSP = 100;

    PlayerSkillController controller(domain, statsService);
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
    
    MockPlayerStatsService statsService;
    statsService.mockSP = 100;

    PlayerSkillController controller(domain, statsService);
    auto result = controller.UseSkill(1, 1234);
    
    REQUIRE_FALSE(result.has_value());
    REQUIRE(result.error() == CommandError::OnCooldown);
}

TEST_CASE("PlayerSkillController - Unknown skill maps to CommandError::SkillNotFound") {
    SkillDomain domain;
    
    MockPlayerStatsService statsService;
    statsService.mockSP = 100;

    PlayerSkillController controller(domain, statsService);
    auto result = controller.UseSkill(999, 1234);
    
    REQUIRE_FALSE(result.has_value());
    REQUIRE(result.error() == CommandError::SkillNotFound);
}

TEST_CASE("PlayerSkillController - Insufficient SP maps to CommandError::NotEnoughSP") {
    SkillDomain domain;
    SkillData sd;
    sd.baseSPCost = 50;
    domain.DefineSkill(1, sd);
    domain.RegisterSkill(1, 1);
    
    MockPlayerStatsService statsService;
    statsService.mockSP = 10; // Not enough SP

    PlayerSkillController controller(domain, statsService);
    auto result = controller.UseSkill(1, 1234);
    
    REQUIRE_FALSE(result.has_value());
    REQUIRE(result.error() == CommandError::NotEnoughSP);
}
