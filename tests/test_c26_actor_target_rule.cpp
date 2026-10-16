#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest/doctest.h"
#include "Client/Actors/ActorTargetRuleValidator.h"

TEST_CASE("ActorTargetRuleValidator: Attack eligibility and empire relations") {
    using namespace Client::Actors;
    
    // Default context for tests
    // vid, level, empire, guildId, partyId, pkMode, alignment, alignmentType, isSafeZone, isDuelOpponent, guildWarState
    ActorRelationContext player1{1, 30, EmpireId::Shinsoo, 10, 0, PkMode::Peace, 0, AlignmentType::Neutral, false, false, GuildWarState::None};
    ActorRelationContext player2{2, 30, EmpireId::Jinno,   20, 0, PkMode::Peace, 0, AlignmentType::Neutral, false, false, GuildWarState::None};
    ActorRelationContext ally{3,    30, EmpireId::Shinsoo, 10, 0, PkMode::Peace, 0, AlignmentType::Neutral, false, false, GuildWarState::None};

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
        
        player2.isSafeZone = false;
        player1.isSafeZone = true;
        CHECK_FALSE(ActorTargetRuleValidator::CanAttackTarget(player1, player2));
    }
    
    SUBCASE("Duel overrides normal rules") {
        ActorRelationContext sameEmpire1{4, 30, EmpireId::Shinsoo, 0, 0, PkMode::Peace, 0, AlignmentType::Neutral, false, true, GuildWarState::None};
        // Normally same empire cannot attack in Peace mode
        CHECK(ActorTargetRuleValidator::CanAttackTarget(player1, sameEmpire1));
    }
    
    SUBCASE("Level < 15 protection") {
        ActorRelationContext lowLevel1{5, 10, EmpireId::Shinsoo, 0, 0, PkMode::Free, 0, AlignmentType::Neutral, false, false, GuildWarState::None};
        ActorRelationContext lowLevel2{6, 12, EmpireId::Shinsoo, 0, 0, PkMode::Free, 0, AlignmentType::Neutral, false, false, GuildWarState::None};
        // Under 15 same empire cannot attack even in Free mode
        CHECK_FALSE(ActorTargetRuleValidator::CanAttackTarget(lowLevel1, lowLevel2));
        
        ActorRelationContext lowLevelEnemy{7, 10, EmpireId::Chunjo, 0, 0, PkMode::Peace, 0, AlignmentType::Neutral, false, false, GuildWarState::None};
        // Under 15 different empire can be attacked
        CHECK(ActorTargetRuleValidator::CanAttackTarget(lowLevel1, lowLevelEnemy));
    }

    SUBCASE("PK Mode: Free") {
        ActorRelationContext freePlayer{8, 30, EmpireId::Shinsoo, 0, 0, PkMode::Free, 0, AlignmentType::Neutral, false, false, GuildWarState::None};
        ActorRelationContext sameEmpire{9, 30, EmpireId::Shinsoo, 0, 0, PkMode::Peace, 0, AlignmentType::Neutral, false, false, GuildWarState::None};
        
        // Free mode can attack same empire
        CHECK(ActorTargetRuleValidator::CanAttackTarget(freePlayer, sameEmpire));
        
        // Free mode cannot attack same guild
        ActorRelationContext guildPlayer{10, 30, EmpireId::Shinsoo, 10, 0, PkMode::Free, 0, AlignmentType::Neutral, false, false, GuildWarState::None};
        CHECK_FALSE(ActorTargetRuleValidator::CanAttackTarget(guildPlayer, ally));
    }
    
    SUBCASE("PK Mode: Revenge") {
        ActorRelationContext revengePlayer{11, 30, EmpireId::Shinsoo, 0, 0, PkMode::Revenge, 0, AlignmentType::Neutral, false, false, GuildWarState::None};
        ActorRelationContext neutralEmpire{12, 30, EmpireId::Shinsoo, 0, 0, PkMode::Peace, 0, AlignmentType::Neutral, false, false, GuildWarState::None};
        ActorRelationContext evilEmpire{13, 30, EmpireId::Shinsoo, 0, 0, PkMode::Peace, -10000, AlignmentType::Evil, false, false, GuildWarState::None};
        
        // Revenge cannot attack neutral same empire
        CHECK_FALSE(ActorTargetRuleValidator::CanAttackTarget(revengePlayer, neutralEmpire));
        
        // Revenge can attack evil same empire
        CHECK(ActorTargetRuleValidator::CanAttackTarget(revengePlayer, evilEmpire));
        
        // Revenge can attack different empire
        CHECK(ActorTargetRuleValidator::CanAttackTarget(revengePlayer, player2));
    }
    
    SUBCASE("PK Mode: Guild (Guild War)") {
        ActorRelationContext guildWarPlayer{14, 30, EmpireId::Shinsoo, 10, 0, PkMode::Guild, 0, AlignmentType::Neutral, false, false, GuildWarState::Battle};
        ActorRelationContext enemyGuildPlayer{15, 30, EmpireId::Shinsoo, 20, 0, PkMode::Guild, 0, AlignmentType::Neutral, false, false, GuildWarState::Battle};
        ActorRelationContext sameEmpireNoGuild{16, 30, EmpireId::Shinsoo, 0, 0, PkMode::Peace, 0, AlignmentType::Neutral, false, false, GuildWarState::None};
        
        // Guild mode in battle can attack
        CHECK(ActorTargetRuleValidator::CanAttackTarget(guildWarPlayer, enemyGuildPlayer));
        
        // Guild mode cannot attack same empire when not in battle with them
        guildWarPlayer.guildWarState = GuildWarState::None;
        CHECK_FALSE(ActorTargetRuleValidator::CanAttackTarget(guildWarPlayer, sameEmpireNoGuild));
    }
}
