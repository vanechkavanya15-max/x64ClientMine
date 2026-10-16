#pragma once

#include <cstdint>
#include <unordered_set>
#include <vector>
#include <expected>
#include "../../EterBase/StrongTypes.h"
#include "../../Client/Core/Result.h"

namespace Client::World {

struct PortalIdTag {};
using PortalId = EterBase::StrongType<PortalIdTag, int32_t, -1>;

enum class PortalError : uint8_t {
    AlreadyVisible,
    NotVisible,
    InvalidId
};

class AreaPortalManager {
public:
    AreaPortalManager() = default;
    ~AreaPortalManager() = default;

    // Non-copyable, non-movable
    AreaPortalManager(const AreaPortalManager&) = delete;
    AreaPortalManager& operator=(const AreaPortalManager&) = delete;
    AreaPortalManager(AreaPortalManager&&) = delete;
    AreaPortalManager& operator=(AreaPortalManager&&) = delete;

    void EnablePortal(bool enable) noexcept;
    [[nodiscard]] bool IsPortalEnabled() const noexcept;

    void ClearPortals() noexcept;

    std::expected<void, PortalError> AddShowingPortalId(PortalId id);
    std::expected<void, PortalError> RemoveShowingPortalId(PortalId id);

    [[nodiscard]] bool IsPortalShowing(PortalId id) const noexcept;
    [[nodiscard]] const std::unordered_set<PortalId>& GetShowingPortals() const noexcept;

private:
    bool m_isPortalEnabled{false};
    std::unordered_set<PortalId> m_showingPortals;
};

} // namespace Client::World
