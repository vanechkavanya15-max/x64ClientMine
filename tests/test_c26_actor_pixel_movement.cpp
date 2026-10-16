#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest/doctest.h"
#include "Client/Actors/ActorPixelMovementIntegrator.h"

using namespace Client::Actors;

class TestTerrainEnvironment : public ITerrainEnvironment {
public:
    bool IsWall(float x, float y) const noexcept override {
        return x > 140.0f;
    }
    
    bool IsBlockObject(float x, float y) const noexcept override {
        // Only block if y is between 140 and 150.
        // This leaves y > 150 completely free of objects for the cliff test
        return y > 140.0f && y < 150.0f;
    }
    
    bool IsWater(float x, float y) const noexcept override {
        return x < 50.0f;
    }
    
    bool AvoidObject(float startX, float startY, float& targetX, float& targetY) const noexcept override {
        if (targetX > 140.0f && startX <= 140.0f) {
            targetX = 140.0f;
            return true;
        }
        if (targetY > 140.0f && targetY < 150.0f && startY <= 140.0f) {
            targetY = 140.0f;
            return true;
        }
        return false;
    }
    
    float GetHeight(float x, float y) const noexcept override {
        if (x > 120.0f && x <= 140.0f && y >= 150.0f) {
            return 100.0f; // Cliff
        }
        return 0.0f;
    }
};

TEST_CASE("ActorPixelMovementIntegrator: Comprehensive physics logic") {
    ActorPixelMovementIntegrator mover;
    TestTerrainEnvironment env;
    mover.SetEnvironment(&env);
    mover.SetPosition(100.0f, 100.0f, 0.0f);
    mover.SetMicroStepSize(5.0f);
    mover.SetMaxVerticalStep(50.0f);

    SUBCASE("Basic step integration without obstacles") {
        mover.SetMovement(1.0f, 0.0f, 20.0f);
        CHECK(mover.IsMoving());
        auto result = mover.IntegrateStep(0.5f);
        REQUIRE(result.has_value());
        auto pos = result.value();
        CHECK(pos.x == doctest::Approx(110.0f));
        CHECK(pos.y == doctest::Approx(100.0f));
        CHECK(pos.z == doctest::Approx(0.0f));
    }
    
    SUBCASE("Wall collision stops movement (without slide if perpendicular)") {
        mover.SetMovement(1.0f, 0.0f, 100.0f);
        auto result = mover.IntegrateStep(0.5f);
        REQUIRE(result.has_value());
        auto pos = result.value();
        CHECK(pos.x == doctest::Approx(140.0f));
        CHECK(pos.y == doctest::Approx(100.0f));
    }

    SUBCASE("Wall collision with sliding") {
        mover.SetMovement(1.0f, 1.0f, 100.0f);
        auto result = mover.IntegrateStep(1.0f); 
        REQUIRE(result.has_value());
        auto pos = result.value();
        CHECK(pos.x == doctest::Approx(140.0f));
        CHECK(pos.y > 100.0f);
    }

    SUBCASE("Water collision prevents movement") {
        mover.SetMovement(-1.0f, 0.0f, 100.0f);
        auto result = mover.IntegrateStep(1.0f);
        REQUIRE_FALSE(result.has_value());
        CHECK(result.error() == MovementError::BlockedByWater);
    }
    
    SUBCASE("Vertical step height limit blocks movement") {
        mover.SetPosition(115.0f, 160.0f, 0.0f);
        mover.SetMovement(1.0f, 0.0f, 20.0f);
        auto result = mover.IntegrateStep(1.0f);
        REQUIRE_FALSE(result.has_value());
        CHECK(result.error() == MovementError::VerticalStepTooHigh);
    }

    SUBCASE("Error handling for missing environment") {
        ActorPixelMovementIntegrator badMover;
        badMover.SetMovement(1.0f, 0.0f, 10.0f);
        auto result = badMover.IntegrateStep(1.0f);
        REQUIRE_FALSE(result.has_value());
        CHECK(result.error() == MovementError::EnvironmentNotSet);
    }

    SUBCASE("Error handling for not moving") {
        mover.SetMovement(0.0f, 0.0f, 0.0f);
        auto result = mover.IntegrateStep(1.0f);
        REQUIRE_FALSE(result.has_value());
        CHECK(result.error() == MovementError::NotMoving);
    }
}
