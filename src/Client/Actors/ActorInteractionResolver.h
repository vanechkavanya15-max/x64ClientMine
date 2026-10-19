#pragma once

#include <expected>
#include <cstdint>
#include <string_view>
#include <format>
#include "Client/Core/StrongTypes.h"
#include "Client/World/ActorRegistry.h"
#include "Client/Actors/ActorCollisionComponent.h"

namespace Client::Actors {

enum class InteractionError : uint8_t {
    None = 0,
    AttackerNotFound,
    TargetNotFound,
    TargetDead,
    OutOfRange,
    InvalidCollision
};

[[nodiscard]] constexpr std::string_view ToString(InteractionError err) noexcept {
    switch (err) {
        case InteractionError::None: return "None";
        case InteractionError::AttackerNotFound: return "AttackerNotFound";
        case InteractionError::TargetNotFound: return "TargetNotFound";
        case InteractionError::TargetDead: return "TargetDead";
        case InteractionError::OutOfRange: return "OutOfRange";
        case InteractionError::InvalidCollision: return "InvalidCollision";
    }
    return "UnknownError";
}

class ActorInteractionResolver {
public:
    explicit ActorInteractionResolver(const Client::World::ActorRegistry& registry) noexcept;
    ~ActorInteractionResolver() = default;

    // Zabrania kopiowania i przenoszenia (zgodnie z Clean Architecture dla systemow bezstanowych)
    ActorInteractionResolver(const ActorInteractionResolver&) = delete;
    ActorInteractionResolver& operator=(const ActorInteractionResolver&) = delete;
    ActorInteractionResolver(ActorInteractionResolver&&) = delete;
    ActorInteractionResolver& operator=(ActorInteractionResolver&&) = delete;

    [[nodiscard]] std::expected<bool, InteractionError> CheckAdvancingCollision(
        Client::World::EntityVid actorVid, 
        const Client::Actor::ActorCollisionComponent& actorCollision) const noexcept;

    [[nodiscard]] std::expected<Client::World::EntityVid, InteractionError> ProcessAttack(
        Client::World::EntityVid attackerVid, 
        const Client::Actor::ActorCollisionComponent& attackerWeaponCollision) const noexcept;

private:
    const Client::World::ActorRegistry& m_registry;
};

} // namespace Client::Actors

template <>
struct std::formatter<Client::Actors::InteractionError> : std::formatter<std::string_view> {
    auto format(Client::Actors::InteractionError err, format_context& ctx) const {
        return formatter<string_view>::format(Client::Actors::ToString(err), ctx);
    }
};
