#pragma once

#include <vector>
#include <string>
#include <span>
#include <expected>
#include <algorithm>
#include "Client/Core/StrongTypes.h"
#include "Client/Core/Result.h"

namespace Client::MiniMap {

enum class WaypointError : uint8_t {
    None,
    NotFound,
    AlreadyExists
};

using WaypointId = Client::Core::StrongType<struct WaypointIdTag, uint32_t, 0>;

enum class WaypointType : uint8_t {
    Opc = 0,
    OpcPvp,
    OpcPvpSelf,
    Npc,
    Monster,
    Warp,
    Waypoint,
    Party,
    Empire,
    Target,
    Count
};

struct Waypoint {
    WaypointType type{WaypointType::Waypoint};
    WaypointId id;
    float x{0.0f};
    float y{0.0f};
    Client::Core::EntityId chrVid;
    std::string text;
};

class MiniMapWaypointManager {
public:
    MiniMapWaypointManager() = default;
    ~MiniMapWaypointManager() = default;

    Client::Core::Result<void, WaypointError> AddWaypoint(Waypoint waypoint);
    Client::Core::Result<void, WaypointError> RemoveWaypoint(WaypointId id);
    Client::Core::Result<Waypoint, WaypointError> GetWaypoint(WaypointId id) const;
    Client::Core::Result<void, WaypointError> UpdateWaypoint(WaypointId id, float x, float y);
    void ClearAllWaypoints();
    std::span<const Waypoint> GetAllWaypoints() const;

private:
    std::vector<Waypoint> m_waypoints;
};

} // namespace Client::MiniMap
