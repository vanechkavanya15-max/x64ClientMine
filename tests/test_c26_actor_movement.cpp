#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"

#include "Client/Actors/ActorMovementComponent.h"
#include "Client/Core/StrongTypes.h"
#include <cmath>

using namespace Client::Actor;
using namespace Client::Core;

TEST_SUITE("ActorMovementComponent") {
    TEST_CASE("Initial state is valid") {
        ActorMovementComponent movement;
        
        CHECK(movement.GetPosition().x == doctest::Approx(0.0f));
        CHECK(movement.GetPosition().y == doctest::Approx(0.0f));
        CHECK(movement.GetPosition().z == doctest::Approx(0.0f));
        
        CHECK_FALSE(movement.IsMoving());
    }

    TEST_CASE("SetPosition changes current position immediately") {
        ActorMovementComponent movement;
        MapCoords pos{100.0f, 200.0f, 300.0f};
        
        movement.SetPosition(pos);
        
        CHECK(movement.GetPosition().x == doctest::Approx(100.0f));
        CHECK(movement.GetPosition().y == doctest::Approx(200.0f));
        CHECK(movement.GetPosition().z == doctest::Approx(300.0f));
        CHECK_FALSE(movement.IsMoving());
    }

    TEST_CASE("MoveTo fails with zero speed") {
        ActorMovementComponent movement;
        movement.SetPosition({0.0f, 0.0f, 0.0f});
        
        auto result = movement.MoveTo({100.0f, 100.0f, 100.0f}, 0.0f);
        
        REQUIRE_FALSE(result.has_value());
        CHECK(result.error() == MovementError::SpeedZero);
        CHECK_FALSE(movement.IsMoving());
    }

    TEST_CASE("MoveTo fails if already at destination") {
        ActorMovementComponent movement;
        movement.SetPosition({100.0f, 200.0f, 300.0f});
        
        auto result = movement.MoveTo({100.0f, 200.0f, 300.0f}, 5.0f);
        
        REQUIRE_FALSE(result.has_value());
        CHECK(result.error() == MovementError::AlreadyAtDestination);
        CHECK_FALSE(movement.IsMoving());
    }

    TEST_CASE("Update fails if not moving") {
        ActorMovementComponent movement;
        
        auto result = movement.Update(1.0f);
        
        REQUIRE_FALSE(result.has_value());
        CHECK(result.error() == MovementError::NotMoving);
    }
    
    TEST_CASE("Update fails with zero or negative delta time") {
        ActorMovementComponent movement;
        movement.MoveTo({100.0f, 0.0f, 0.0f}, 5.0f);
        
        auto result = movement.Update(0.0f);
        REQUIRE_FALSE(result.has_value());
        CHECK(result.error() == MovementError::ZeroDuration);
        
        auto result2 = movement.Update(-1.0f);
        REQUIRE_FALSE(result2.has_value());
        CHECK(result2.error() == MovementError::ZeroDuration);
    }

    TEST_CASE("Moving towards destination over time") {
        ActorMovementComponent movement;
        movement.SetPosition({0.0f, 0.0f, 0.0f});
        
        auto result = movement.MoveTo({100.0f, 0.0f, 0.0f}, 10.0f);
        REQUIRE(result.has_value());
        CHECK(movement.IsMoving());
        
        // 5 seconds * 10 units/sec = 50 units
        auto updateResult = movement.Update(5.0f);
        REQUIRE(updateResult.has_value());
        
        CHECK(movement.GetPosition().x == doctest::Approx(50.0f));
        CHECK(movement.IsMoving());
        
        // Next 5 seconds * 10 units/sec = 50 units (total 100) -> arrives at destination
        updateResult = movement.Update(5.0f);
        REQUIRE(updateResult.has_value());
        
        CHECK(movement.GetPosition().x == doctest::Approx(100.0f));
        CHECK_FALSE(movement.IsMoving());
        
        // Overshooting time should still cap at destination
        movement.SetPosition({0.0f, 0.0f, 0.0f});
        movement.MoveTo({10.0f, 0.0f, 0.0f}, 5.0f);
        updateResult = movement.Update(5.0f); // Move 25 units, max distance is 10
        REQUIRE(updateResult.has_value());
        CHECK(movement.GetPosition().x == doctest::Approx(10.0f));
        CHECK_FALSE(movement.IsMoving());
    }

    TEST_CASE("Stop interrupts movement") {
        ActorMovementComponent movement;
        movement.SetPosition({0.0f, 0.0f, 0.0f});
        
        movement.MoveTo({100.0f, 0.0f, 0.0f}, 10.0f);
        movement.Update(2.0f); // moves 20 units
        
        CHECK(movement.GetPosition().x == doctest::Approx(20.0f));
        CHECK(movement.IsMoving());
        
        movement.Stop();
        
        CHECK_FALSE(movement.IsMoving());
        CHECK(movement.GetPosition().x == doctest::Approx(20.0f));
        CHECK(movement.GetDestination().x == doctest::Approx(20.0f));
        
        auto result = movement.Update(1.0f);
        REQUIRE_FALSE(result.has_value());
        CHECK(result.error() == MovementError::NotMoving);
    }
}
