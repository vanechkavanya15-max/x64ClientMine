#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include "Client/MiniMap/MiniMapWaypointManager.h"

using namespace Client::MiniMap;
using namespace Client::Core;

TEST_CASE("MiniMapWaypointManager - AddWaypoint") {
    MiniMapWaypointManager manager;

    SUBCASE("Sukces - dodanie waypointu") {
        Waypoint wp{
            .type = WaypointType::Waypoint,
            .id = WaypointId{1},
            .x = 100.0f,
            .y = 200.0f,
            .chrVid = EntityId{10},
            .text = "TestWaypoint"
        };

        auto result = manager.AddWaypoint(wp);
        CHECK(result.has_value() == true);

        auto waypoints = manager.GetAllWaypoints();
        CHECK(waypoints.size() == 1);
        CHECK(waypoints[0].id == WaypointId{1});
        CHECK(waypoints[0].text == "TestWaypoint");
    }

    SUBCASE("Blad - proba dodania duplikatu") {
        Waypoint wp1{ .type = WaypointType::Waypoint, .id = WaypointId{1}, .text = "First" };
        Waypoint wp2{ .type = WaypointType::Target, .id = WaypointId{1}, .text = "Second" };

        auto result1 = manager.AddWaypoint(wp1);
        CHECK(result1.has_value() == true);

        auto result2 = manager.AddWaypoint(wp2);
        CHECK(result2.has_value() == false);
        CHECK(result2.error() == WaypointError::AlreadyExists);

        auto waypoints = manager.GetAllWaypoints();
        CHECK(waypoints.size() == 1);
        CHECK(waypoints[0].text == "First");
    }
}

TEST_CASE("MiniMapWaypointManager - RemoveWaypoint") {
    MiniMapWaypointManager manager;

    Waypoint wp1{ .id = WaypointId{1} };
    Waypoint wp2{ .id = WaypointId{2} };
    manager.AddWaypoint(wp1);
    manager.AddWaypoint(wp2);

    SUBCASE("Sukces - usuniecie istniejacego waypointu") {
        auto result = manager.RemoveWaypoint(WaypointId{1});
        CHECK(result.has_value() == true);

        auto waypoints = manager.GetAllWaypoints();
        CHECK(waypoints.size() == 1);
        CHECK(waypoints[0].id == WaypointId{2});
    }

    SUBCASE("Blad - proba usuniecia nieistniejacego waypointu") {
        auto result = manager.RemoveWaypoint(WaypointId{999});
        CHECK(result.has_value() == false);
        CHECK(result.error() == WaypointError::NotFound);

        auto waypoints = manager.GetAllWaypoints();
        CHECK(waypoints.size() == 2);
    }
}

TEST_CASE("MiniMapWaypointManager - GetWaypoint") {
    MiniMapWaypointManager manager;

    Waypoint wp{ .id = WaypointId{42}, .text = "Answer" };
    manager.AddWaypoint(wp);

    SUBCASE("Sukces - pobranie istniejacego waypointu") {
        auto result = manager.GetWaypoint(WaypointId{42});
        CHECK(result.has_value() == true);
        CHECK(result.value().text == "Answer");
    }

    SUBCASE("Blad - pobranie nieistniejacego waypointu") {
        auto result = manager.GetWaypoint(WaypointId{99});
        CHECK(result.has_value() == false);
        CHECK(result.error() == WaypointError::NotFound);
    }
}

TEST_CASE("MiniMapWaypointManager - GetAllWaypoints posortowane") {
    MiniMapWaypointManager manager;

    manager.AddWaypoint({ .id = WaypointId{5} });
    manager.AddWaypoint({ .id = WaypointId{1} });
    manager.AddWaypoint({ .id = WaypointId{3} });

    auto waypoints = manager.GetAllWaypoints();
    CHECK(waypoints.size() == 3);
    CHECK(waypoints[0].id == WaypointId{1});
    CHECK(waypoints[1].id == WaypointId{3});
    CHECK(waypoints[2].id == WaypointId{5});
}

TEST_CASE("MiniMapWaypointManager - UpdateWaypoint") {
    MiniMapWaypointManager manager;

    manager.AddWaypoint({ .id = WaypointId{42}, .x = 10.0f, .y = 20.0f });

    SUBCASE("Sukces - aktualizacja waypointu") {
        auto result = manager.UpdateWaypoint(WaypointId{42}, 100.0f, 200.0f);
        CHECK(result.has_value() == true);
        
        auto wp = manager.GetWaypoint(WaypointId{42}).value();
        CHECK(wp.x == 100.0f);
        CHECK(wp.y == 200.0f);
    }

    SUBCASE("Blad - aktualizacja nieistniejacego") {
        auto result = manager.UpdateWaypoint(WaypointId{99}, 100.0f, 200.0f);
        CHECK(result.has_value() == false);
        CHECK(result.error() == WaypointError::NotFound);
    }
}

TEST_CASE("MiniMapWaypointManager - ClearAllWaypoints") {
    MiniMapWaypointManager manager;

    manager.AddWaypoint({ .id = WaypointId{1} });
    manager.AddWaypoint({ .id = WaypointId{2} });
    
    CHECK(manager.GetAllWaypoints().size() == 2);
    
    manager.ClearAllWaypoints();
    
    CHECK(manager.GetAllWaypoints().size() == 0);
}
