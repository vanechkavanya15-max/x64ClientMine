#pragma once

#include <expected>
#include <optional>
#include "Client/Core/StrongTypes.h"

namespace Client::Actors {

enum class PetError {
    None,
    AlreadySummoned,
    NotSummoned
};

class ActorPetComponent {
public:
    ActorPetComponent() = default;
    ~ActorPetComponent() = default;

    ActorPetComponent(const ActorPetComponent&) = delete;
    ActorPetComponent& operator=(const ActorPetComponent&) = delete;

    ActorPetComponent(ActorPetComponent&&) noexcept = default;
    ActorPetComponent& operator=(ActorPetComponent&&) noexcept = default;

    std::expected<void, PetError> Summon(Core::EntityVid petVid, Core::ItemVnum petVnum);
    std::expected<void, PetError> Unsummon();

    std::optional<Core::EntityVid> GetPetVid() const noexcept;
    std::optional<Core::ItemVnum> GetPetVnum() const noexcept;
    bool IsSummoned() const noexcept;

private:
    std::optional<Core::EntityVid> m_petVid;
    std::optional<Core::ItemVnum> m_petVnum;
};

} // namespace Client::Actors
