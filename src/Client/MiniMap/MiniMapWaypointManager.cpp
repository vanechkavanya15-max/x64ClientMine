#include "MiniMapWaypointManager.h"

namespace Client::MiniMap {

Client::Core::Result<void, WaypointError> MiniMapWaypointManager::AddWaypoint(Waypoint waypoint) {
    auto it = std::ranges::lower_bound(m_waypoints, waypoint.id, {}, &Waypoint::id);
    if (it != m_waypoints.end() && it->id == waypoint.id) {
        return std::unexpected(WaypointError::AlreadyExists);
    }
    m_waypoints.insert(it, std::move(waypoint));
    return {};
}

Client::Core::Result<void, WaypointError> MiniMapWaypointManager::RemoveWaypoint(WaypointId id) {
    auto it = std::ranges::lower_bound(m_waypoints, id, {}, &Waypoint::id);
    if (it == m_waypoints.end() || it->id != id) {
        return std::unexpected(WaypointError::NotFound);
    }
    m_waypoints.erase(it);
    return {};
}

Client::Core::Result<Waypoint, WaypointError> MiniMapWaypointManager::GetWaypoint(WaypointId id) const {
    auto it = std::ranges::lower_bound(m_waypoints, id, {}, &Waypoint::id);
    if (it == m_waypoints.end() || it->id != id) {
        return std::unexpected(WaypointError::NotFound);
    }
    return *it;
}

Client::Core::Result<void, WaypointError> MiniMapWaypointManager::UpdateWaypoint(WaypointId id, float x, float y) {
    auto it = std::ranges::lower_bound(m_waypoints, id, {}, &Waypoint::id);
    if (it == m_waypoints.end() || it->id != id) {
        return std::unexpected(WaypointError::NotFound);
    }
    it->x = x;
    it->y = y;
    return {};
}

void MiniMapWaypointManager::ClearAllWaypoints() {
    m_waypoints.clear();
}

std::span<const Waypoint> MiniMapWaypointManager::GetAllWaypoints() const {
    return std::span<const Waypoint>(m_waypoints);
}

} // namespace Client::MiniMap
