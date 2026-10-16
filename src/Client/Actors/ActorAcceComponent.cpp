#include "ActorAcceComponent.h"

namespace Client::Actors {

AcceResult<void> ActorAcceComponent::EquipAcce(Client::Core::ItemVnum acceId) {
    if (!acceId) {
        return std::unexpected(AcceError::InvalidAcce);
    }
    
    if (m_equippedAcce.has_value()) {
        return std::unexpected(AcceError::AlreadyEquipped);
    }

    m_equippedAcce = acceId;
    return {};
}

AcceResult<void> ActorAcceComponent::UnequipAcce() {
    if (!m_equippedAcce.has_value()) {
        return std::unexpected(AcceError::NotEquipped);
    }

    m_equippedAcce.reset();
    return {};
}

bool ActorAcceComponent::IsAcceEquipped() const noexcept {
    return m_equippedAcce.has_value();
}

std::optional<Client::Core::ItemVnum> ActorAcceComponent::GetEquippedAcce() const noexcept {
    return m_equippedAcce;
}

} // namespace Client::Actors
