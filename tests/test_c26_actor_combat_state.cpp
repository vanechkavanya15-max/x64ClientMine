#include "../src/Client/Actors/ActorCombatStateComponent.h"
#include <iostream>
#include <cassert>

using namespace Client;

void TestInitialization() {
    Actors::CombatStateConfig config{1000, 500};
    Actors::ActorCombatStateComponent combatState(config);

    assert(combatState.GetHp() == 1000);
    assert(combatState.GetMaxHp() == 1000);
    assert(combatState.GetSp() == 500);
    assert(combatState.GetMaxSp() == 500);
    assert(!combatState.IsDead());
    assert(!combatState.IsInCombat());
    assert(!combatState.GetTarget().has_value());
    
    std::cout << "[OK] TestInitialization\n";
}

void TestTakeDamage() {
    Actors::CombatStateConfig config{100, 50};
    Actors::ActorCombatStateComponent combatState(config);
    combatState.SetTarget(Client::Core::EntityVid{1});
    combatState.SetInCombat(true);

    auto result = combatState.TakeDamage(30);
    assert(result.has_value());
    assert(combatState.GetHp() == 70);
    assert(!combatState.IsDead());
    assert(combatState.IsInCombat());
    assert(combatState.GetTarget().has_value());

    auto lethalResult = combatState.TakeDamage(80);
    assert(lethalResult.has_value());
    assert(combatState.GetHp() == 0);
    assert(combatState.IsDead());
    assert(!combatState.IsInCombat());
    assert(!combatState.GetTarget().has_value());

    auto deadResult = combatState.TakeDamage(10);
    assert(!deadResult.has_value());
    assert(deadResult.error() == Client::Core::EntityError::Dead);

    std::cout << "[OK] TestTakeDamage\n";
}

void TestHeal() {
    Actors::CombatStateConfig config{100, 50};
    Actors::ActorCombatStateComponent combatState(config);
    
    [[maybe_unused]] auto res1 = combatState.TakeDamage(40);
    assert(combatState.GetHp() == 60);

    auto healRes = combatState.Heal(20);
    assert(healRes.has_value());
    assert(combatState.GetHp() == 80);

    auto overHealRes = combatState.Heal(50);
    assert(overHealRes.has_value());
    assert(combatState.GetHp() == 100);

    [[maybe_unused]] auto res2 = combatState.TakeDamage(100); // dies
    auto deadHealRes = combatState.Heal(50);
    assert(!deadHealRes.has_value());
    assert(deadHealRes.error() == Client::Core::EntityError::Dead);

    std::cout << "[OK] TestHeal\n";
}

void TestSp() {
    Actors::CombatStateConfig config{100, 50};
    Actors::ActorCombatStateComponent combatState(config);

    auto consRes = combatState.ConsumeSp(20);
    assert(consRes.has_value());
    assert(combatState.GetSp() == 30);

    auto failCons = combatState.ConsumeSp(40);
    assert(!failCons.has_value());
    assert(failCons.error() == Client::Core::EntityError::OutOfRange);
    assert(combatState.GetSp() == 30); // unchanged

    auto recRes = combatState.RecoverSp(10);
    assert(recRes.has_value());
    assert(combatState.GetSp() == 40);

    [[maybe_unused]] auto res1 = combatState.RecoverSp(100); // overheal
    assert(combatState.GetSp() == 50);

    [[maybe_unused]] auto res2 = combatState.TakeDamage(100);
    assert(!combatState.ConsumeSp(10).has_value());
    assert(!combatState.RecoverSp(10).has_value());

    std::cout << "[OK] TestSp\n";
}

void TestReviveAndConfigUpdate() {
    Actors::CombatStateConfig config{100, 50};
    Actors::ActorCombatStateComponent combatState(config);
    
    [[maybe_unused]] auto res1 = combatState.TakeDamage(100);
    assert(combatState.IsDead());

    combatState.Revive(50); // revive with 50% HP
    assert(!combatState.IsDead());
    assert(combatState.GetHp() == 50);

    combatState.UpdateMaxStats(200, 100);
    assert(combatState.GetMaxHp() == 200);
    assert(combatState.GetMaxSp() == 100);
    assert(combatState.GetHp() == 50); // doesn't auto-fill, just changes cap

    [[maybe_unused]] auto res2 = combatState.Heal(300);
    assert(combatState.GetHp() == 200);

    combatState.UpdateMaxStats(100, 50);
    assert(combatState.GetMaxHp() == 100);
    assert(combatState.GetHp() == 100); // capped

    std::cout << "[OK] TestReviveAndConfigUpdate\n";
}

void TestConstexpr() {
    constexpr Actors::CombatStateConfig cfg{100, 50};
    constexpr Actors::ActorCombatStateComponent comp(cfg);
    static_assert(comp.GetMaxHp() == 100);
    static_assert(!comp.IsDead());
    
    std::cout << "[OK] TestConstexpr\n";
}

int main() {
    TestInitialization();
    TestTakeDamage();
    TestHeal();
    TestSp();
    TestReviveAndConfigUpdate();
    TestConstexpr();
    
    std::cout << "All Combat State Component tests passed successfully!\n";
    return 0;
}
