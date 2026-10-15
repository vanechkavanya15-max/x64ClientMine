#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include "Client/Actors/ActorCombatStateComponent.h"

using namespace Client::Actors;
using namespace Client::Core;

TEST_CASE("ActorCombatStateComponent - Initialization") {
    ActorCombatStateComponent combatState;
    
    CHECK(combatState.GetHp() == 0);
    CHECK(combatState.GetMaxHp() == 0);
    CHECK(combatState.GetSp() == 0);
    CHECK(combatState.GetMaxSp() == 0);
    CHECK(combatState.IsDead() == false);
    CHECK(combatState.GetTargetVid() == std::nullopt);
}

TEST_CASE("ActorCombatStateComponent - HP and Bounds") {
    ActorCombatStateComponent combatState;
    
    combatState.SetMaxHp(100);
    CHECK(combatState.GetMaxHp() == 100);
    
    combatState.SetHp(50);
    CHECK(combatState.GetHp() == 50);
    CHECK(combatState.IsDead() == false);
    
    // Test capping HP at MaxHP
    combatState.SetHp(150);
    CHECK(combatState.GetHp() == 100);
    
    // Reducing MaxHP should cap current HP
    combatState.SetMaxHp(80);
    CHECK(combatState.GetHp() == 80);
    
    // Setting HP to 0 sets dead state
    combatState.SetHp(0);
    CHECK(combatState.GetHp() == 0);
    CHECK(combatState.IsDead() == true);
    
    // Setting HP above 0 revives
    combatState.SetHp(10);
    CHECK(combatState.GetHp() == 10);
    CHECK(combatState.IsDead() == false);
}

TEST_CASE("ActorCombatStateComponent - SP and Bounds") {
    ActorCombatStateComponent combatState;
    
    combatState.SetMaxSp(50);
    CHECK(combatState.GetMaxSp() == 50);
    
    combatState.SetSp(20);
    CHECK(combatState.GetSp() == 20);
    
    // Test capping SP at MaxSP
    combatState.SetSp(100);
    CHECK(combatState.GetSp() == 50);
    
    // Reducing MaxSP should cap current SP
    combatState.SetMaxSp(30);
    CHECK(combatState.GetSp() == 30);
}

TEST_CASE("ActorCombatStateComponent - Target Management") {
    ActorCombatStateComponent combatState;
    
    combatState.SetTargetVid(EntityVid(123));
    REQUIRE(combatState.GetTargetVid().has_value());
    CHECK(combatState.GetTargetVid()->get() == 123);
    
    combatState.ClearTargetVid();
    CHECK(combatState.GetTargetVid() == std::nullopt);
}

TEST_CASE("ActorCombatStateComponent - Death State") {
    ActorCombatStateComponent combatState;
    combatState.SetMaxHp(100);
    combatState.SetHp(100);
    
    combatState.SetDead(true);
    CHECK(combatState.IsDead() == true);
    CHECK(combatState.GetHp() == 0); // HP should drop to 0 when explicitly dead
    
    combatState.SetDead(false);
    CHECK(combatState.IsDead() == false);
    CHECK(combatState.GetHp() == 0); // HP shouldn't magically regenerate
}

TEST_CASE("ActorCombatStateComponent - Take Damage") {
    ActorCombatStateComponent combatState;
    combatState.SetMaxHp(100);
    combatState.SetHp(100);
    
    SUBCASE("Normal Damage") {
        auto res = combatState.TakeDamage(30);
        REQUIRE(res.has_value());
        CHECK(combatState.GetHp() == 70);
        CHECK(combatState.IsDead() == false);
    }
    
    SUBCASE("Lethal Damage") {
        auto res = combatState.TakeDamage(150);
        REQUIRE(res.has_value());
        CHECK(combatState.GetHp() == 0);
        CHECK(combatState.IsDead() == true);
    }
    
    SUBCASE("Damage when Already Dead") {
        combatState.TakeDamage(150);
        CHECK(combatState.IsDead() == true);
        
        auto res = combatState.TakeDamage(10);
        REQUIRE_FALSE(res.has_value());
        CHECK(res.error() == ActorError::AlreadyDead);
    }
}
