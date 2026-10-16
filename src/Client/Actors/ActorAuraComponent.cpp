#include "ActorAuraComponent.h"

namespace Client::Actors {

AuraResult<void> ActorAuraComponent::ActivateAura(Client::Core::ItemVnum auraId) {
    if (m_activeAura.has_value()) {
        return std::unexpected(AuraError::AlreadyActive);
    }
    if (auraId.value() == 0) {
        return std::unexpected(AuraError::InvalidAura);
    }

    m_activeAura = auraId;
    return {};
}

AuraResult<void> ActorAuraComponent::DeactivateAura() {
    if (!m_activeAura.has_value()) {
        return std::unexpected(AuraError::NotActive);
    }

    m_activeAura.reset();
    return {};
}

bool ActorAuraComponent::IsAuraActive() const noexcept {
    return m_activeAura.has_value();
}

std::optional<Client::Core::ItemVnum> ActorAuraComponent::GetActiveAura() const noexcept {
    return m_activeAura;
}

} // namespace Client::Actors
