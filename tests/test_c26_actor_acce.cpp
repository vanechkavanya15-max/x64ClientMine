#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest.h>

#include "Client/Actors/ActorAcceComponent.h"
#include "Client/Core/StrongTypes.h"

using namespace Client::Actors;
using namespace Client::Core;

TEST_CASE("ActorAcceComponent initialization") {
    ActorAcceComponent acceComponent;

    SUBCASE("Initially no acce is equipped") {
        CHECK_FALSE(acceComponent.IsAcceEquipped());
        CHECK_FALSE(acceComponent.GetEquippedAcce().has_value());
    }
}

TEST_CASE("ActorAcceComponent EquipAcce") {
    ActorAcceComponent acceComponent;
    ItemVnum validAcceId(85001);
    ItemVnum invalidAcceId(0);

    SUBCASE("Equipping a valid acce succeeds") {
        auto result = acceComponent.EquipAcce(validAcceId);
        CHECK(result.has_value());
        CHECK(acceComponent.IsAcceEquipped());
        CHECK(acceComponent.GetEquippedAcce().has_value());
        CHECK(acceComponent.GetEquippedAcce().value() == validAcceId);
    }

    SUBCASE("Equipping an invalid acce fails") {
        auto result = acceComponent.EquipAcce(invalidAcceId);
        CHECK_FALSE(result.has_value());
        CHECK(result.error() == AcceError::InvalidAcce);
        CHECK_FALSE(acceComponent.IsAcceEquipped());
    }

    SUBCASE("Equipping an acce when one is already equipped fails") {
        auto result1 = acceComponent.EquipAcce(validAcceId);
        CHECK(result1.has_value());

        ItemVnum anotherAcceId(85002);
        auto result2 = acceComponent.EquipAcce(anotherAcceId);
        CHECK_FALSE(result2.has_value());
        CHECK(result2.error() == AcceError::AlreadyEquipped);
        
        CHECK(acceComponent.IsAcceEquipped());
        CHECK(acceComponent.GetEquippedAcce().value() == validAcceId);
    }
}

TEST_CASE("ActorAcceComponent UnequipAcce") {
    ActorAcceComponent acceComponent;
    ItemVnum validAcceId(85001);

    SUBCASE("Unequipping when an acce is equipped succeeds") {
        auto equipResult = acceComponent.EquipAcce(validAcceId);
        CHECK(equipResult.has_value());

        auto unequipResult = acceComponent.UnequipAcce();
        CHECK(unequipResult.has_value());
        CHECK_FALSE(acceComponent.IsAcceEquipped());
        CHECK_FALSE(acceComponent.GetEquippedAcce().has_value());
    }

    SUBCASE("Unequipping when no acce is equipped fails") {
        auto unequipResult = acceComponent.UnequipAcce();
        CHECK_FALSE(unequipResult.has_value());
        CHECK(unequipResult.error() == AcceError::NotEquipped);
        CHECK_FALSE(acceComponent.IsAcceEquipped());
    }
}

TEST_CASE("AcceError to_string") {
    CHECK(to_string(AcceError::None) == "AcceError::None");
    CHECK(to_string(AcceError::AlreadyEquipped) == "AcceError::AlreadyEquipped");
    CHECK(to_string(AcceError::NotEquipped) == "AcceError::NotEquipped");
    CHECK(to_string(AcceError::InvalidAcce) == "AcceError::InvalidAcce");
    CHECK(to_string(static_cast<AcceError>(99)) == "AcceError::Unknown");
}

TEST_CASE("AcceError formatter") {
    CHECK(std::format("{}", AcceError::None) == "AcceError::None");
    CHECK(std::format("{}", AcceError::AlreadyEquipped) == "AcceError::AlreadyEquipped");
    CHECK(std::format("{}", AcceError::NotEquipped) == "AcceError::NotEquipped");
    CHECK(std::format("{}", AcceError::InvalidAcce) == "AcceError::InvalidAcce");
    CHECK(std::format("{}", static_cast<AcceError>(99)) == "AcceError::Unknown");
}
