#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest/doctest.h"
#include "Client/Actors/ActorPixelMovementIntegrator.h"

TEST_CASE("ActorPixelMovementIntegrator: Movement integration and obstacle avoidance") {
    Client::Actors::ActorPixelMovementIntegrator mover;
    mover.SetPosition(100.0f, 100.0f);

    SUBCASE("Basic step integration") {
        mover.SetMovement(1.0f, 0.0f, 100.0f); // 100 px/s w prawo
        CHECK(mover.IsMoving());
        mover.IntegrateStep(0.5f);
        auto pos = mover.GetPosition();
        CHECK(pos.x == doctest::Approx(150.0f));
        CHECK(pos.y == doctest::Approx(100.0f));
    }
    SUBCASE("Wall collision blocks step") {
        mover.SetMovement(0.0f, 1.0f, 50.0f);
        CHECK_FALSE(mover.IntegrateStep(0.5f, true)); // blocked
        auto pos = mover.GetPosition();
        CHECK(pos.x == doctest::Approx(100.0f));
        CHECK(pos.y == doctest::Approx(100.0f));
    }
}
