#include "doctest.h"
#include "Client/Skills/SkillBuffManager.h"
#include <chrono>

using namespace Client::Skills;
using namespace Client::Core;
using namespace std::chrono_literals;

TEST_CASE("SkillBuffManager - dodawanie buffa i sprawdzanie istnienia") {
    SkillBuffManager manager;
    SkillId buffId{100};
    
    // Na poczatku nie ma buffa
    CHECK_FALSE(manager.HasBuff(buffId));
    CHECK_FALSE(manager.GetBuffRemainingTime(buffId).has_value());
    
    // Dodawanie buffa
    auto result = manager.AddBuff(buffId, 5000ms);
    CHECK(result.has_value());
    
    // Sprawdzenie czy buff istnieje
    CHECK(manager.HasBuff(buffId));
    auto remainingTime = manager.GetBuffRemainingTime(buffId);
    CHECK(remainingTime.has_value());
    CHECK(remainingTime.value() == 5000ms);
    
    // Proba dodania buffa o id 0 powinna sie nie udac
    SkillId invalidId{0};
    auto resultInvalid = manager.AddBuff(invalidId, 5000ms);
    CHECK_FALSE(resultInvalid.has_value());
    CHECK(resultInvalid.error() == SkillError::SkillNotFound);
}

TEST_CASE("SkillBuffManager - uplyw czasu i wygasanie buffow") {
    SkillBuffManager manager;
    SkillId buffId{101};
    
    manager.AddBuff(buffId, 3000ms);
    CHECK(manager.GetBuffRemainingTime(buffId).value() == 3000ms);
    
    // Uplyw 1000ms
    manager.Update(1000ms);
    CHECK(manager.HasBuff(buffId));
    CHECK(manager.GetBuffRemainingTime(buffId).value() == 2000ms);
    
    // Uplyw kolejnych 2000ms (buff wygasa)
    manager.Update(2000ms);
    CHECK_FALSE(manager.HasBuff(buffId));
    CHECK_FALSE(manager.GetBuffRemainingTime(buffId).has_value());
}

TEST_CASE("SkillBuffManager - reczne usuwanie buffa") {
    SkillBuffManager manager;
    SkillId buffId{102};
    
    manager.AddBuff(buffId, 10000ms);
    CHECK(manager.HasBuff(buffId));
    
    // Usuwanie buffa
    auto result = manager.RemoveBuff(buffId);
    CHECK(result.has_value());
    CHECK_FALSE(manager.HasBuff(buffId));
    
    // Proba usuniecia nieistniejacego buffa
    SkillId invalidId{999};
    auto resultNotFound = manager.RemoveBuff(invalidId);
    CHECK_FALSE(resultNotFound.has_value());
    CHECK(resultNotFound.error() == SkillError::SkillNotFound);
}

TEST_CASE("SkillBuffManager - czyszczenie wszystkich buffow") {
    SkillBuffManager manager;
    SkillId buff1{103};
    SkillId buff2{104};
    
    manager.AddBuff(buff1, 5000ms);
    manager.AddBuff(buff2, 8000ms);
    
    CHECK(manager.HasBuff(buff1));
    CHECK(manager.HasBuff(buff2));
    
    manager.ClearAll();
    
    CHECK_FALSE(manager.HasBuff(buff1));
    CHECK_FALSE(manager.HasBuff(buff2));
}

TEST_CASE("SkillBuffManager - nadpisywanie czasu trwania") {
    SkillBuffManager manager;
    SkillId buffId{105};
    
    manager.AddBuff(buffId, 2000ms);
    CHECK(manager.GetBuffRemainingTime(buffId).value() == 2000ms);
    
    // Nadpisanie czasu trwania tego samego buffa
    manager.AddBuff(buffId, 10000ms);
    CHECK(manager.GetBuffRemainingTime(buffId).value() == 10000ms);
}
