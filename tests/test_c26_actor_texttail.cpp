#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest.h>
#include "Client/Actors/ActorTextTailComponent.h"
#include "Client/Actor/ActorComponentStore.h"

using namespace Client::Actor;

TEST_CASE("ActorTextTailComponent basic properties") {
    ActorTextTailComponent comp;
    comp.name = "TestActor";
    comp.guildName = "TestGuild";
    comp.title = "TestTitle";
    comp.karma = AlignmentKarma::Chivalric;
    comp.modelHeight = 150.0f;
    comp.showHpBar = true;
    comp.hpPercentage = 0.75f;
    
    CHECK(comp.name == "TestActor");
    CHECK(comp.guildName.has_value());
    CHECK(comp.guildName.value() == "TestGuild");
    CHECK(comp.title.has_value());
    CHECK(comp.title.value() == "TestTitle");
    CHECK(comp.karma == AlignmentKarma::Chivalric);
    CHECK(comp.modelHeight == doctest::Approx(150.0f));
    CHECK(comp.showHpBar == true);
    CHECK(comp.hpPercentage == doctest::Approx(0.75f));
}

TEST_CASE("ActorTextTailComponent integration with ActorComponentStore via separate store (Zero-Conflict)") {
    ComponentStore<ActorTextTailComponent> textTailStore;
    
    EterBase::EntityId id1{100};
    EterBase::EntityId id2{200};
    
    SUBCASE("Add and Get TextTail Component") {
        ActorTextTailComponent comp1;
        comp1.name = "PlayerOne";
        comp1.guildName = "Warriors";
        
        auto addResult = textTailStore.Add(id1, comp1);
        CHECK(addResult.has_value());
        
        auto getResult = textTailStore.Get(id1);
        CHECK(getResult.has_value());
        CHECK(getResult.value()->name == "PlayerOne");
        CHECK(getResult.value()->guildName.value() == "Warriors");
    }
    
    SUBCASE("Update HP Bar values") {
        ActorTextTailComponent comp2;
        comp2.name = "Monster";
        comp2.showHpBar = true;
        comp2.hpPercentage = 1.0f;
        
        auto res = textTailStore.Add(id2, comp2);
        CHECK(res.has_value());
        
        auto getResult = textTailStore.Get(id2);
        CHECK(getResult.has_value());
        
        getResult.value()->hpPercentage = 0.5f;
        
        auto getResult2 = textTailStore.Get(id2);
        CHECK(getResult2.value()->hpPercentage == doctest::Approx(0.5f));
    }
    
    SUBCASE("Remove Component") {
        ActorTextTailComponent comp1;
        comp1.name = "NPC";
        
        auto res = textTailStore.Add(id1, comp1);
        CHECK(res.has_value());
        CHECK(textTailStore.Get(id1).has_value());
        
        auto remResult = textTailStore.Remove(id1);
        CHECK(remResult.has_value());
        
        auto getResult = textTailStore.Get(id1);
        CHECK_FALSE(getResult.has_value());
        CHECK(getResult.error() == EterBase::EntityError::NotFound);
    }
}
