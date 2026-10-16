#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest/doctest.h"
#include "Client/Player/PlayerMainActorPositionTracker.h"
#include <cmath>

using namespace Client::Player;
using namespace Client::Core;

TEST_CASE("PlayerMainActorPositionTracker: ForcePosition and basic state") {
    TrackerConfig config;
    PlayerMainActorPositionTracker tracker(config);

    CHECK(tracker.GetState() == InterpolationState::Idle);

    auto result = tracker.ForcePosition({100.0f, 200.0f, 30.0f});
    CHECK(result.has_value());

    auto pos = tracker.NEW_GetMainActorPosition();
    CHECK(pos.x == 100.0f);
    CHECK(pos.y == 200.0f);
    CHECK(pos.z == 30.0f);
    CHECK(tracker.GetState() == InterpolationState::Idle);

    // Test NaN validation
    auto badResult = tracker.ForcePosition({std::nanf(""), 0.0f, 0.0f});
    CHECK(!badResult.has_value());
    CHECK(badResult.error() == TrackerError::InvalidInput);
}

TEST_CASE("PlayerMainActorPositionTracker: Linear Interpolation") {
    TrackerConfig config;
    config.baseMoveSpeed = 100.0f; // 100 units per second (0.1 units per ms)
    PlayerMainActorPositionTracker tracker(config);

    tracker.ForcePosition({0.0f, 0.0f, 0.0f});
    
    // Push waypoint at distance 100 on X axis
    auto result = tracker.PushNetworkWaypoint({100.0f, 0.0f, 0.0f}, 1000);
    CHECK(result.has_value());
    CHECK(tracker.GetState() == InterpolationState::Linear);

    // Advance 500ms (should move 50 units)
    tracker.Update(500);
    auto pos = tracker.NEW_GetMainActorPosition();
    CHECK(std::abs(pos.x - 50.0f) < 0.1f);
    CHECK(tracker.GetState() == InterpolationState::Linear);

    // Advance another 600ms (should cap at 100 and consume waypoint)
    tracker.Update(600);
    pos = tracker.NEW_GetMainActorPosition();
    CHECK(pos.x == 100.0f);
    CHECK(tracker.GetState() == InterpolationState::Idle);
}

TEST_CASE("PlayerMainActorPositionTracker: Spline Interpolation") {
    TrackerConfig config;
    config.splineActivationThreshold = 3;
    PlayerMainActorPositionTracker tracker(config);

    tracker.ForcePosition({0.0f, 0.0f, 0.0f});
    
    tracker.PushNetworkWaypoint({100.0f, 0.0f, 0.0f}, 1000);
    tracker.PushNetworkWaypoint({200.0f, 50.0f, 0.0f}, 2000);
    tracker.PushNetworkWaypoint({300.0f, 0.0f, 0.0f}, 3000);

    CHECK(tracker.GetState() == InterpolationState::Spline);

    // Advance halfway through first segment (500ms)
    // t = 0.5 between p1 (100,0,0) and p2 (200,50,0).
    // Initial pos p0 approx (0,0,0)
    tracker.Update(500);
    auto pos = tracker.NEW_GetMainActorPosition();
    
    // Ensure we moved forward
    CHECK(pos.x > 10.0f);
    CHECK(tracker.GetState() == InterpolationState::Spline);
}

TEST_CASE("PlayerMainActorPositionTracker: Camera Sync Position") {
    TrackerConfig config;
    config.cameraLagMs = 100;
    config.baseMoveSpeed = 100.0f;
    PlayerMainActorPositionTracker tracker(config);

    tracker.ForcePosition({0.0f, 0.0f, 0.0f});
    tracker.PushNetworkWaypoint({200.0f, 0.0f, 0.0f}, 1000);

    // Update 50ms, current pos moves to 5.0. Total time 50.
    // Camera target time = 0 (since 50 < 100). Should return 0.
    tracker.Update(50);
    auto camPos = tracker.GetCameraSyncPosition();
    CHECK(std::abs(camPos.x - 0.0f) < 0.1f);

    // Update another 100ms. Total time 150. Current pos = 15.
    // Camera target time = 150 - 100 = 50.
    tracker.Update(100);
    camPos = tracker.GetCameraSyncPosition();
    CHECK(std::abs(camPos.x - 5.0f) < 0.1f); // Camera is 100ms behind, so it's where player was at 50ms
}

TEST_CASE("PlayerMainActorPositionTracker: Queue Overflow") {
    TrackerConfig config;
    config.maxQueueSize = 2;
    PlayerMainActorPositionTracker tracker(config);

    tracker.ForcePosition({0.0f, 0.0f, 0.0f});
    
    auto r1 = tracker.PushNetworkWaypoint({10.0f, 0.0f, 0.0f}, 100);
    auto r2 = tracker.PushNetworkWaypoint({20.0f, 0.0f, 0.0f}, 200);
    auto r3 = tracker.PushNetworkWaypoint({30.0f, 0.0f, 0.0f}, 300);

    CHECK(r1.has_value());
    CHECK(r2.has_value());
    CHECK(!r3.has_value());
    CHECK(r3.error() == TrackerError::QueueOverflow);
}
