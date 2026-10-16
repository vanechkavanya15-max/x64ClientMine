#pragma once

#include <cstdint>
#include <unordered_map>
#include <unordered_set>
#include <functional>
#include <vector>

#include "../../EterBase/StrongTypes.h"
#include "../../Client/Core/Result.h"
#include "SpatialHashGrid.h"

namespace Client::World {

struct AreaObjectIdTag {};
using AreaObjectId = EterBase::StrongType<AreaObjectIdTag, uint32_t, 0>;

enum class AreaStreamerError : uint8_t {
    NotFound,
    AlreadyExists,
    InvalidRadius
};

struct AreaObjectRecord {
    AreaObjectId id;
    float x;
    float y;
    float z;
    uint32_t vnum; // e.g., tree model vnum, building vnum
};

class AreaObjectStreamer {
public:
    AreaObjectStreamer() = default;
    ~AreaObjectStreamer() = default;

    // Non-copyable, non-movable
    AreaObjectStreamer(const AreaObjectStreamer&) = delete;
    AreaObjectStreamer& operator=(const AreaObjectStreamer&) = delete;
    AreaObjectStreamer(AreaObjectStreamer&&) = delete;
    AreaObjectStreamer& operator=(AreaObjectStreamer&&) = delete;

    Client::Core::Result<void, AreaStreamerError> RegisterObject(const AreaObjectRecord& record);
    Client::Core::Result<void, AreaStreamerError> UnregisterObject(AreaObjectId id);

    void UpdateViewerPosition(float x, float y, float radius,
                              const std::function<void(const AreaObjectRecord&)>& onStreamIn,
                              const std::function<void(AreaObjectId)>& onStreamOut);

    [[nodiscard]] bool IsVisible(AreaObjectId id) const;
    [[nodiscard]] const std::unordered_set<AreaObjectId>& GetVisibleObjects() const;
    [[nodiscard]] size_t GetRegisteredCount() const;
    
    void Clear();

private:
    SpatialHashGrid m_grid;
    std::unordered_map<AreaObjectId, AreaObjectRecord> m_registeredObjects;
    std::unordered_set<AreaObjectId> m_visibleObjects;
};

} // namespace Client::World
