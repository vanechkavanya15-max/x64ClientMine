#include "AreaPortalManager.h"

namespace Client::World {

void AreaPortalManager::EnablePortal(bool enable) noexcept {
    m_isPortalEnabled = enable;
}

bool AreaPortalManager::IsPortalEnabled() const noexcept {
    return m_isPortalEnabled;
}

void AreaPortalManager::ClearPortals() noexcept {
    m_showingPortals.clear();
}

std::expected<void, PortalError> AreaPortalManager::AddShowingPortalId(PortalId id) {
    if (id.value() < 0) {
        return std::unexpected(PortalError::InvalidId);
    }
    
    auto [it, inserted] = m_showingPortals.insert(id);
    if (!inserted) {
        return std::unexpected(PortalError::AlreadyVisible);
    }
    
    return {};
}

std::expected<void, PortalError> AreaPortalManager::RemoveShowingPortalId(PortalId id) {
    if (id.value() < 0) {
        return std::unexpected(PortalError::InvalidId);
    }
    
    auto erased = m_showingPortals.erase(id);
    if (erased == 0) {
        return std::unexpected(PortalError::NotVisible);
    }
    
    return {};
}

bool AreaPortalManager::IsPortalShowing(PortalId id) const noexcept {
    return m_showingPortals.contains(id);
}

const std::unordered_set<PortalId>& AreaPortalManager::GetShowingPortals() const noexcept {
    return m_showingPortals;
}

} // namespace Client::World
