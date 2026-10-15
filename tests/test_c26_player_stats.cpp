#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest/doctest.h>
#include "../src/Client/Player/PlayerStatsController.h"
#include "../src/Client/Gameplay/PlayerStatsDomain.h"

using namespace Client::Player;
using namespace Client::Gameplay;

TEST_CASE("PlayerStatsController - Initialization and Errors") {
    PlayerStatsController controller;

    SUBCASE("Fails when domain is not attached") {
        auto result = controller.UpdateHP(100);
        CHECK_FALSE(result.has_value());
        CHECK(result.error() == PlayerStatsError::DomainNotAttached);
    }
}

TEST_CASE("PlayerStatsController - Stats Modification") {
    PlayerStatsDomain domain;
    PlayerStatsController controller(&domain);

    SUBCASE("Update HP and SP") {
        auto hpRes = controller.UpdateHP(500);
        CHECK(hpRes.has_value());
        CHECK(domain.GetHP() == 500);

        auto maxHpRes = controller.UpdateMaxHP(1000);
        CHECK(maxHpRes.has_value());
        CHECK(domain.GetMaxHP() == 1000);

        auto spRes = controller.UpdateSP(200);
        CHECK(spRes.has_value());
        CHECK(domain.GetSP() == 200);

        auto maxSpRes = controller.UpdateMaxSP(400);
        CHECK(maxSpRes.has_value());
        CHECK(domain.GetMaxSP() == 400);
    }

    SUBCASE("Level Up and Errors") {
        auto lvlRes = controller.SetLevel(5);
        CHECK(lvlRes.has_value());
        CHECK(domain.GetLevel() == 5);

        auto badLvl = controller.SetLevel(0);
        CHECK_FALSE(badLvl.has_value());
        CHECK(badLvl.error() == PlayerStatsError::InvalidLevel);
    }

    SUBCASE("Experience") {
        auto expRes = controller.AddExperience(1500);
        CHECK(expRes.has_value());
        CHECK(domain.GetExp() == 1500);

        controller.AddExperience(500);
        CHECK(domain.GetExp() == 2000);
    }

    SUBCASE("Status Points") {
        auto stRes = controller.SetStatusPoint(10, 55);
        CHECK(stRes.has_value());
        CHECK(domain.GetStatusPoint(10) == 55);

        auto badSt = controller.SetStatusPoint(300, 10);
        CHECK_FALSE(badSt.has_value());
        CHECK(badSt.error() == PlayerStatsError::InvalidStatPoint);
    }

    SUBCASE("Gold") {
        auto goldRes = controller.SetGold(999999);
        CHECK(goldRes.has_value());
        CHECK(domain.GetGold() == 999999);
    }
}
