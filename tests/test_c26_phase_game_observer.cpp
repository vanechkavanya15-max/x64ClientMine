#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest/doctest.h"
#include "Client/Bridge/PhaseGameObserverBridge.h"
#include <cmath>

using namespace Client::Bridge;
using namespace Client::Core;

TEST_CASE("PhaseGameObserverBridge: Initial State") {
    PhaseGameObserverBridge obs;
    CHECK_FALSE(obs.IsObserverMode());
    CHECK(obs.GetCurrentState() == ObserverState::Inactive);
    
    // Check config defaults
    const auto& config = obs.GetConfig();
    CHECK(config.maxWanderDistance > 0.0f);
    CHECK(config.restrictToMapBounds == true);
}

TEST_CASE("PhaseGameObserverBridge: Enable / Disable Mode") {
    PhaseGameObserverBridge obs;
    
    auto enableRes = obs.EnableObserverMode();
    CHECK(enableRes.has_value());
    CHECK(obs.IsObserverMode());
    CHECK(obs.GetCurrentState() == ObserverState::FreeCamera);
    
    auto enableAgainRes = obs.EnableObserverMode();
    CHECK_FALSE(enableAgainRes.has_value());
    CHECK(enableAgainRes.error() == ObserverError::AlreadyInState);
    
    auto disableRes = obs.DisableObserverMode();
    CHECK(disableRes.has_value());
    CHECK_FALSE(obs.IsObserverMode());
    CHECK(obs.GetCurrentState() == ObserverState::Inactive);
    
    auto disableAgainRes = obs.DisableObserverMode();
    CHECK_FALSE(disableAgainRes.has_value());
    CHECK(disableAgainRes.error() == ObserverError::NotInObserverMode);
}

TEST_CASE("PhaseGameObserverBridge: State Transitions and Tracking") {
    PhaseGameObserverBridge obs;
    obs.EnableObserverMode();
    
    SUBCASE("Valid Target Tracking") {
        auto res = obs.SetTrackingTarget(EntityVid{1337});
        CHECK(res.has_value());
        CHECK(obs.GetCurrentState() == ObserverState::TargetTracking);
        CHECK(obs.GetTrackingTarget() == EntityVid{1337});
        
        auto clearRes = obs.ClearTrackingTarget();
        CHECK(clearRes.has_value());
        CHECK(obs.GetCurrentState() == ObserverState::FreeCamera);
        CHECK(obs.GetTrackingTarget() == EntityVid{0});
    }
    
    SUBCASE("Invalid Target Tracking") {
        auto res = obs.SetTrackingTarget(EntityVid{0});
        CHECK_FALSE(res.has_value());
        CHECK(res.error() == ObserverError::InvalidTargetVid);
        CHECK(obs.GetCurrentState() == ObserverState::FreeCamera);
    }
}

TEST_CASE("PhaseGameObserverBridge: Free Camera Movement and Bounds Validation") {
    PhaseGameObserverBridge obs;
    obs.EnableObserverMode();
    
    ObserverConfig cfg;
    cfg.maxWanderDistance = 5000.0f;
    cfg.minHeight = 0.0f;
    cfg.maxHeight = 2000.0f;
    cfg.restrictToMapBounds = true;
    obs.SetConfig(cfg);
    
    SUBCASE("Valid Position Set") {
        auto res = obs.SetFreeCameraPosition(MapCoords{100.0f, 200.0f, 50.0f});
        CHECK(res.has_value());
        auto pos = obs.GetCurrentPosition();
        CHECK(pos.x == doctest::Approx(100.0f));
        CHECK(pos.y == doctest::Approx(200.0f));
        CHECK(pos.z == doctest::Approx(50.0f));
    }
    
    SUBCASE("Valid Movement") {
        obs.SetFreeCameraPosition(MapCoords{100.0f, 200.0f, 50.0f});
        auto res = obs.MoveFreeCamera(MapCoords{50.0f, -50.0f, 10.0f});
        CHECK(res.has_value());
        auto pos = obs.GetCurrentPosition();
        CHECK(pos.x == doctest::Approx(150.0f));
        CHECK(pos.y == doctest::Approx(150.0f));
        CHECK(pos.z == doctest::Approx(60.0f));
    }
    
    SUBCASE("Out of Bounds X/Y") {
        auto res = obs.SetFreeCameraPosition(MapCoords{6000.0f, 0.0f, 0.0f});
        CHECK_FALSE(res.has_value());
        CHECK(res.error() == ObserverError::OutOfMapBounds);
    }
    
    SUBCASE("Out of Bounds Height") {
        auto res = obs.SetFreeCameraPosition(MapCoords{0.0f, 0.0f, -100.0f}); // below minHeight
        CHECK_FALSE(res.has_value());
        CHECK(res.error() == ObserverError::OutOfMapBounds);
        
        auto res2 = obs.SetFreeCameraPosition(MapCoords{0.0f, 0.0f, 3000.0f}); // above maxHeight
        CHECK_FALSE(res2.has_value());
        CHECK(res2.error() == ObserverError::OutOfMapBounds);
    }
    
    SUBCASE("Move into Bounds Error") {
        obs.SetFreeCameraPosition(MapCoords{4900.0f, 0.0f, 0.0f});
        auto res = obs.MoveFreeCamera(MapCoords{200.0f, 0.0f, 0.0f}); // pushes past 5000.0f
        CHECK_FALSE(res.has_value());
        CHECK(res.error() == ObserverError::OutOfMapBounds);
    }
}

TEST_CASE("PhaseGameObserverBridge: Telemetry and Update") {
    PhaseGameObserverBridge obs;
    
    SUBCASE("Telemetry when inactive") {
        auto res = obs.GetTelemetry();
        CHECK_FALSE(res.has_value());
        CHECK(res.error() == ObserverError::NotInObserverMode);
    }
    
    SUBCASE("Telemetry when active") {
        obs.EnableObserverMode();
        obs.SetFreeCameraPosition(MapCoords{10.0f, 20.0f, 30.0f});
        auto res = obs.GetTelemetry();
        CHECK(res.has_value());
        CHECK(res->state == ObserverState::FreeCamera);
        CHECK(res->position.x == doctest::Approx(10.0f));
    }
    
    SUBCASE("Update Loop Tracking validation") {
        obs.EnableObserverMode();
        obs.SetTrackingTarget(EntityVid{999});
        
        // Simulating target drop by accessing the context directly or relying on missing target logic
        // As a bridge it just keeps tracking. Let's make sure update doesn't crash
        auto res = obs.Update(1000);
        CHECK(res.has_value());
        CHECK(obs.GetCurrentState() == ObserverState::TargetTracking);
    }
}
