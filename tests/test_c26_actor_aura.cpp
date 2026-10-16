#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest/doctest.h>
#include "Client/Actors/ActorAuraComponent.h"
#include "Client/Core/StrongTypes.h"

using namespace Client::Actors;
using namespace Client::Core;

TEST_SUITE("ActorAuraComponent") {
    TEST_CASE("Initial state is inactive") {
        ActorAuraComponent auraComp;
        CHECK(auraComp.IsAuraActive() == false);
        CHECK(auraComp.GetActiveAura().has_value() == false);
    }

    TEST_CASE("Activate valid aura") {
        ActorAuraComponent auraComp;
        ItemVnum validAura(1001);
        
        auto result = auraComp.ActivateAura(validAura);
        CHECK(result.has_value() == true);
        CHECK(auraComp.IsAuraActive() == true);
        
        auto activeAura = auraComp.GetActiveAura();
        REQUIRE(activeAura.has_value());
        CHECK(activeAura.value() == validAura);
    }

    TEST_CASE("Cannot activate zero aura ID") {
        ActorAuraComponent auraComp;
        ItemVnum invalidAura(0);
        
        auto result = auraComp.ActivateAura(invalidAura);
        CHECK(result.has_value() == false);
        CHECK(result.error() == AuraError::InvalidAura);
        CHECK(auraComp.IsAuraActive() == false);
    }

    TEST_CASE("Cannot activate aura if one is already active") {
        ActorAuraComponent auraComp;
        ItemVnum aura1(1001);
        ItemVnum aura2(1002);
        
        CHECK(auraComp.ActivateAura(aura1).has_value() == true);
        
        auto result = auraComp.ActivateAura(aura2);
        CHECK(result.has_value() == false);
        CHECK(result.error() == AuraError::AlreadyActive);
        
        auto activeAura = auraComp.GetActiveAura();
        REQUIRE(activeAura.has_value());
        CHECK(activeAura.value() == aura1);
    }

    TEST_CASE("Deactivate active aura") {
        ActorAuraComponent auraComp;
        ItemVnum validAura(1001);
        
        auraComp.ActivateAura(validAura);
        CHECK(auraComp.IsAuraActive() == true);
        
        auto result = auraComp.DeactivateAura();
        CHECK(result.has_value() == true);
        CHECK(auraComp.IsAuraActive() == false);
        CHECK(auraComp.GetActiveAura().has_value() == false);
    }

    TEST_CASE("Cannot deactivate if no aura is active") {
        ActorAuraComponent auraComp;
        
        auto result = auraComp.DeactivateAura();
        CHECK(result.has_value() == false);
        CHECK(result.error() == AuraError::NotActive);
    }
}
