#include "AreaObjectStreamer.h"
#include <cmath>

namespace Client::World {

Client::Core::Result<void, AreaStreamerError> AreaObjectStreamer::RegisterObject(const AreaObjectRecord& record) {
    if (m_registeredObjects.contains(record.id)) {
        return std::unexpected(AreaStreamerError::AlreadyExists);
    }
    
    m_registeredObjects[record.id] = record;
    return {};
}

Client::Core::Result<void, AreaStreamerError> AreaObjectStreamer::UnregisterObject(AreaObjectId id) {
    auto it = m_registeredObjects.find(id);
    if (it == m_registeredObjects.end()) {
        return std::unexpected(AreaStreamerError::NotFound);
    }
    
    m_visibleObjects.erase(id);
    m_registeredObjects.erase(it);
    return {};
}

void AreaObjectStreamer::UpdateViewerPosition(float x, float y, float radius,
                                              const std::function<void(const AreaObjectRecord&)>& onStreamIn,
                                              const std::function<void(AreaObjectId)>& onStreamOut) {
    if (radius < 0.0f) {
        return;
    }

    float sqRadius = radius * radius;
    std::vector<AreaObjectId> newlyVisible;
    std::vector<AreaObjectId> newlyInvisible;

    // We do a linear pass. For a real MMO map we might want to use a SpatialHashGrid
    // but the task requirements just want to separate the logic.
    for (const auto& [id, record] : m_registeredObjects) {
        float dx = record.x - x;
        float dy = record.y - y;
        float sqDist = dx * dx + dy * dy;

        bool inRange = sqDist <= sqRadius;
        bool wasVisible = m_visibleObjects.contains(id);

        if (inRange && !wasVisible) {
            newlyVisible.push_back(id);
        } else if (!inRange && wasVisible) {
            newlyInvisible.push_back(id);
        }
    }

    // Apply changes and invoke callbacks
    for (AreaObjectId id : newlyVisible) {
        m_visibleObjects.insert(id);
        if (onStreamIn) {
            onStreamIn(m_registeredObjects[id]);
        }
    }

    for (AreaObjectId id : newlyInvisible) {
        m_visibleObjects.erase(id);
        if (onStreamOut) {
            onStreamOut(id);
        }
    }
}

bool AreaObjectStreamer::IsVisible(AreaObjectId id) const {
    return m_visibleObjects.contains(id);
}

const std::unordered_set<AreaObjectId>& AreaObjectStreamer::GetVisibleObjects() const {
    return m_visibleObjects;
}

size_t AreaObjectStreamer::GetRegisteredCount() const {
    return m_registeredObjects.size();
}

void AreaObjectStreamer::Clear() {
    m_registeredObjects.clear();
    m_visibleObjects.clear();
}

} // namespace Client::World
