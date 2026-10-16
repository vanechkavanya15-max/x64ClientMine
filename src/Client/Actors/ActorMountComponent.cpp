#include "ActorMountComponent.h"

namespace Client::Actors {

std::expected<void, MountError> ActorMountComponent::Mount(Core::ItemVnum mountVnum) {
    if (m_mountVnum.has_value()) {
        return std::unexpected(MountError::AlreadyMounted);
    }
    
    if (mountVnum.get() == 0) {
        return std::unexpected(MountError::InvalidVnum);
    }

    m_mountVnum = mountVnum;
    return {};
}

std::expected<void, MountError> ActorMountComponent::Unmount() {
    if (!m_mountVnum.has_value()) {
        return std::unexpected(MountError::NotMounted);
    }

    m_mountVnum.reset();
    return {};
}

std::optional<Core::ItemVnum> ActorMountComponent::GetMountVnum() const noexcept {
    return m_mountVnum;
}

bool ActorMountComponent::IsMounted() const noexcept {
    return m_mountVnum.has_value();
}

} // namespace Client::Actors
