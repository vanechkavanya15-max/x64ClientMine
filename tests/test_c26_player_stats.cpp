#include "doctest.h"
#include "Client/Player/PlayerStatsController.h"

using namespace Client::Player;

TEST_CASE("PlayerStatsController: Initialization") {
    PlayerStatsController ctrl;
    ctrl.Initialize(10, 10, 10, 10);
    
    CHECK(ctrl.GetStat(StatType::VIT) == 10);
    CHECK(ctrl.GetStat(StatType::INT) == 10);
    CHECK(ctrl.GetStat(StatType::STR) == 10);
    CHECK(ctrl.GetStat(StatType::DEX) == 10);

    // MaxHP = 100 + 10 * 40 = 500
    CHECK(ctrl.GetMaxHP() == 500);
    CHECK(ctrl.GetHP() == 500);

    // MaxSP = 50 + 10 * 20 = 250
    CHECK(ctrl.GetMaxSP() == 250);
    CHECK(ctrl.GetSP() == 250);
}

TEST_CASE("PlayerStatsController: Stat Point Allocation") {
    PlayerStatsController ctrl;
    ctrl.Initialize(10, 10, 10, 10);
    ctrl.SetAvailablePoints(2);

    SUBCASE("Successful allocation") {
        auto result = ctrl.AllocateStatPoint(StatType::VIT);
        CHECK(result.has_value());
        CHECK(ctrl.GetStat(StatType::VIT) == 11);
        CHECK(ctrl.GetAvailablePoints() == 1);
        CHECK(ctrl.GetMaxHP() == 540);
        CHECK(ctrl.GetHP() == 540);

        result = ctrl.AllocateStatPoint(StatType::INT);
        CHECK(result.has_value());
        CHECK(ctrl.GetStat(StatType::INT) == 11);
        CHECK(ctrl.GetAvailablePoints() == 0);
        CHECK(ctrl.GetMaxSP() == 270);
        CHECK(ctrl.GetSP() == 270);
    }

    SUBCASE("Not enough points") {
        ctrl.SetAvailablePoints(0);
        auto result = ctrl.AllocateStatPoint(StatType::STR);
        CHECK(!result.has_value());
        CHECK(result.error() == PlayerStatError::NotEnoughPoints);
        CHECK(ctrl.GetStat(StatType::STR) == 10);
    }

    SUBCASE("Stat maxed out") {
        ctrl.Initialize(90, 10, 10, 10);
        ctrl.SetAvailablePoints(1);
        auto result = ctrl.AllocateStatPoint(StatType::VIT);
        CHECK(!result.has_value());
        CHECK(result.error() == PlayerStatError::StatMaxedOut);
        CHECK(ctrl.GetStat(StatType::VIT) == 90);
        CHECK(ctrl.GetAvailablePoints() == 1); // Point not consumed
    }
}

TEST_CASE("PlayerStatsController: Combat and Consumables") {
    PlayerStatsController ctrl;
    ctrl.Initialize(10, 10, 10, 10);

    SUBCASE("Take damage and heal") {
        ctrl.TakeDamage(100);
        CHECK(ctrl.GetHP() == 400);

        ctrl.HealHP(50);
        CHECK(ctrl.GetHP() == 450);

        ctrl.HealHP(100);
        CHECK(ctrl.GetHP() == 500); // Should not exceed max HP
    }

    SUBCASE("Death and healing restriction") {
        ctrl.TakeDamage(600); // More than MaxHP
        CHECK(ctrl.GetHP() == 0);

        ctrl.HealHP(100);
        CHECK(ctrl.GetHP() == 0); // Cannot heal if dead
    }

    SUBCASE("Consume and restore SP") {
        auto result = ctrl.ConsumeSP(50);
        CHECK(result.has_value());
        CHECK(ctrl.GetSP() == 200);

        ctrl.RestoreSP(20);
        CHECK(ctrl.GetSP() == 220);

        ctrl.RestoreSP(100);
        CHECK(ctrl.GetSP() == 250); // Should not exceed max SP
    }

    SUBCASE("Consume more SP than available") {
        auto result = ctrl.ConsumeSP(300); // More than MaxSP
        CHECK(!result.has_value());
        CHECK(result.error() == PlayerStatError::NotEnoughSP);
        CHECK(ctrl.GetSP() == 250); // SP unchanged
    }

    SUBCASE("Cannot restore SP if dead") {
        ctrl.TakeDamage(500); // Dead
        ctrl.ConsumeSP(50);
        ctrl.RestoreSP(50);
        CHECK(ctrl.GetSP() == 200); // Still 200
    }
}

TEST_CASE("PlayerStatsController: Error to_string and formatting") {
    CHECK(to_string(PlayerStatError::None) == "PlayerStatError::None");
    CHECK(to_string(PlayerStatError::NotEnoughPoints) == "PlayerStatError::NotEnoughPoints - Brak punktów do rozdania");
    CHECK(to_string(PlayerStatError::StatMaxedOut) == "PlayerStatError::StatMaxedOut - Statystyka osiągnęła maksymalny poziom");
    CHECK(to_string(PlayerStatError::NotEnoughSP) == "PlayerStatError::NotEnoughSP - Niewystarczająca ilość punktów magii (SP)");
    CHECK(to_string(static_cast<PlayerStatError>(99)) == "PlayerStatError::Unknown");
    
    CHECK(std::format("{}", PlayerStatError::None) == "PlayerStatError::None");
}
