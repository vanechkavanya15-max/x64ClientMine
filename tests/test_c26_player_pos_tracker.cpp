#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest/doctest.h"
#include "Client/Player/PlayerMainActorPositionTracker.h"

TEST_CASE("PlayerMainActorPositionTracker: Position updates") {
    Client::Player::PlayerMainActorPositionTracker tracker;
    tracker.UpdatePosition(100.0f, 200.0f, 30.0f);
    auto p = tracker.GetPosition();
    CHECK(p.x == 100.0f);
    CHECK(p.y == 200.0f);
    CHECK(p.z == 30.0f);
}
