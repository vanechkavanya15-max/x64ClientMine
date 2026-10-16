#include "AreaObjectStreamer.h"
#include <cmath>

namespace Client::World {

Client::Core::Result<void, AreaStreamerError> AreaObjectStreamer::RegisterObject(const AreaObjectRecord& record) {
    if (m_registeredObjects.contains(record.id)) {
        return std::unexpected(AreaStreamerError::AlreadyExists);
    }
    
    m_registeredObjects[record.id] = record;
    m_grid.Insert(EterBase::EntityId{record.id.get()}, record.x, record.y);
    return {};
}

Client::Core::Result<void, AreaStreamerError> AreaObjectStreamer::UnregisterObject(AreaObjectId id) {
    auto it = m_registeredObjects.find(id);
    if (it == m_registeredObjects.end()) {
        return std::unexpected(AreaStreamerError::NotFound);
    }
    
    m_grid.Remove(EterBase::EntityId{id.get()});
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

    std::unordered_set<AreaObjectId> currentVisibleInRange;
    
    m_grid.QueryRadiusVisitor(x, y, radius, [&](EterBase::EntityId id, float /*px*/, float /*py*/) {
        currentVisibleInRange.insert(AreaObjectId{id.get()});
    });

    std::vector<AreaObjectId> newlyVisible;
    std::vector<AreaObjectId> newlyInvisible;

    for (AreaObjectId id : currentVisibleInRange) {
        if (!m_visibleObjects.contains(id)) {
            newlyVisible.push_back(id);
        }
    }

    for (AreaObjectId id : m_visibleObjects) {
        if (!currentVisibleInRange.contains(id)) {
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
    m_grid.Clear();
    m_registeredObjects.clear();
    m_visibleObjects.clear();
}

} // namespace Client::World
