#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include "Client/Actors/ActorMovementComponent.h"
#include <cmath>

using namespace Client::Actor;
using namespace Client::Core;
using namespace EterBase;

TEST_SUITE("ActorMovementComponent") {
    TEST_CASE("Initialization") {
        ActorMovementComponent component;
        CHECK(component.GetCurrentPosition().x == 0.0f);
        CHECK(component.GetCurrentPosition().y == 0.0f);
        CHECK(component.GetCurrentPosition().z == 0.0f);
        CHECK_FALSE(component.IsMoving());

        MapCoords startPos{10.0f, 20.0f, 30.0f};
        ActorMovementComponent componentInit(startPos);
        CHECK(componentInit.GetCurrentPosition() == startPos);
        CHECK(componentInit.GetTargetPosition() == startPos);
        CHECK_FALSE(componentInit.IsMoving());
    }

    TEST_CASE("MoveTo Command") {
        ActorMovementComponent component({0.0f, 0.0f, 0.0f});
        MapCoords target{10.0f, 0.0f, 0.0f};

        SUBCASE("Valid movement") {
            auto result = component.MoveTo(target, 5.0f);
            CHECK(result.has_value());
            CHECK(component.IsMoving());
            CHECK(component.GetTargetPosition() == target);
        }

        SUBCASE("Invalid speed") {
            auto result = component.MoveTo(target, -1.0f);
            CHECK_FALSE(result.has_value());
            CHECK(result.error() == EntityError::InvalidType);
            CHECK_FALSE(component.IsMoving());
        }

        SUBCASE("Move to current position") {
            auto result = component.MoveTo({0.0f, 0.0f, 0.0f}, 5.0f);
            CHECK(result.has_value());
            CHECK_FALSE(component.IsMoving());
        }
    }

    TEST_CASE("Update Interpolation") {
        ActorMovementComponent component({0.0f, 0.0f, 0.0f});
        MapCoords target{10.0f, 0.0f, 0.0f};
        (void)component.MoveTo(target, 5.0f); // 5 units per second

        SUBCASE("Partial movement") {
            auto result = component.Update(1.0f); // Move 1 second
            CHECK(result.has_value());
            CHECK(result.value() == MapCoords{5.0f, 0.0f, 0.0f});
            CHECK(component.GetCurrentPosition() == MapCoords{5.0f, 0.0f, 0.0f});
            CHECK(component.IsMoving());
        }

        SUBCASE("Reach target") {
            auto result = component.Update(2.0f); // Move 2 seconds (10 units)
            CHECK(result.has_value());
            CHECK(result.value() == target);
            CHECK(component.GetCurrentPosition() == target);
            CHECK_FALSE(component.IsMoving());
        }

        SUBCASE("Overshoot target") {
            auto result = component.Update(3.0f); // Move 3 seconds (15 units)
            CHECK(result.has_value());
            CHECK(result.value() == target);
            CHECK(component.GetCurrentPosition() == target);
            CHECK_FALSE(component.IsMoving());
        }

        SUBCASE("Zero delta time") {
            auto result = component.Update(0.0f);
            CHECK_FALSE(result.has_value());
            CHECK(component.GetCurrentPosition() == MapCoords{0.0f, 0.0f, 0.0f});
            CHECK(component.IsMoving());
        }
    }

    TEST_CASE("Stop and SetPosition") {
        ActorMovementComponent component({0.0f, 0.0f, 0.0f});
        (void)component.MoveTo({10.0f, 0.0f, 0.0f}, 5.0f);

        SUBCASE("Stop explicitly") {
            component.Stop();
            CHECK_FALSE(component.IsMoving());
            CHECK(component.GetTargetPosition() == component.GetCurrentPosition());
        }

        SUBCASE("SetPosition interrupts movement") {
            MapCoords newPos{5.0f, 5.0f, 5.0f};
            component.SetPosition(newPos);
            CHECK_FALSE(component.IsMoving());
            CHECK(component.GetCurrentPosition() == newPos);
            CHECK(component.GetTargetPosition() == newPos);
        }
    }

    TEST_CASE("3D Movement Interpolation") {
        ActorMovementComponent component({0.0f, 0.0f, 0.0f});
        MapCoords target{10.0f, 10.0f, 10.0f};
        
        // Długość wektora (10,10,10) to ok. 17.3205
        float distance = std::sqrt(10.0f*10.0f + 10.0f*10.0f + 10.0f*10.0f);
        float speed = distance / 2.0f; // Powinno dotrzeć do celu w 2 sekundy
        
        (void)component.MoveTo(target, speed);
        
        auto result = component.Update(1.0f); // W połowie drogi po 1 sek.
        CHECK(result.has_value());
        
        // Z powodu zaokrągleń ułamków używamy przybliżenia
        auto currentPos = component.GetCurrentPosition();
        CHECK(std::abs(currentPos.x - 5.0f) < 0.001f);
        CHECK(std::abs(currentPos.y - 5.0f) < 0.001f);
        CHECK(std::abs(currentPos.z - 5.0f) < 0.001f);
        CHECK(component.IsMoving());
        
        component.Update(1.01f); // Reszta drogi + lekki zapas
        CHECK_FALSE(component.IsMoving());
        CHECK(component.GetCurrentPosition() == target);
    }
}
