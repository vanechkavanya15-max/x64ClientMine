#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest/doctest.h"
#include "Client/Actors/ActorTargetRuleValidator.h"

TEST_CASE("ActorTargetRuleValidator: Attack eligibility and empire relations") {
    using namespace Client::Actors;
    ActorRelationContext player1{1, EmpireId::Shinsoo, 10, 0, PkMode::Peace, false, false};
    ActorRelationContext player2{2, EmpireId::Jinno, 20, 0, PkMode::Peace, false, false};
    ActorRelationContext ally{3, EmpireId::Shinsoo, 10, 0, PkMode::Peace, false, false};

    SUBCASE("Different empire players can attack") {
        CHECK(ActorTargetRuleValidator::CanAttackTarget(player1, player2));
        CHECK(ActorTargetRuleValidator::IsEnemy(player1, player2));
    }
    SUBCASE("Same guild allies cannot attack") {
        CHECK_FALSE(ActorTargetRuleValidator::CanAttackTarget(player1, ally));
        CHECK(ActorTargetRuleValidator::IsFriendly(player1, ally));
    }
    SUBCASE("SafeZone blocks attack") {
        player2.isSafeZone = true;
        CHECK_FALSE(ActorTargetRuleValidator::CanAttackTarget(player1, player2));
    }
}
