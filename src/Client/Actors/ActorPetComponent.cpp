#include "ActorPetComponent.h"

namespace Client::Actors {

std::expected<void, PetError> ActorPetComponent::Summon(Core::EntityVid petVid, Core::ItemVnum petVnum) {
    if (IsSummoned()) {
        return std::unexpected(PetError::AlreadySummoned);
    }

    m_petVid = petVid;
    m_petVnum = petVnum;
    return {};
}

std::expected<void, PetError> ActorPetComponent::Unsummon() {
    if (!IsSummoned()) {
        return std::unexpected(PetError::NotSummoned);
    }

    m_petVid.reset();
    m_petVnum.reset();
    return {};
}

std::optional<Core::EntityVid> ActorPetComponent::GetPetVid() const noexcept {
    return m_petVid;
}

std::optional<Core::ItemVnum> ActorPetComponent::GetPetVnum() const noexcept {
    return m_petVnum;
}

bool ActorPetComponent::IsSummoned() const noexcept {
    return m_petVid.has_value() && m_petVnum.has_value();
}

} // namespace Client::Actors
