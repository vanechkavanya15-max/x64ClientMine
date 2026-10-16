#pragma once

#include <cstdint>
#include <expected>
#include <string_view>
#include <format>
#include <optional>
#include "Client/Core/StrongTypes.h"

namespace Client::Actors {

enum class AuraError : uint8_t {
    None = 0,
    AlreadyActive,
    NotActive,
    InvalidAura
};

[[nodiscard]] inline std::string_view to_string(AuraError error) noexcept {
    switch (error) {
        case AuraError::None: return "AuraError::None";
        case AuraError::AlreadyActive: return "AuraError::AlreadyActive";
        case AuraError::NotActive: return "AuraError::NotActive";
        case AuraError::InvalidAura: return "AuraError::InvalidAura";
        default: return "AuraError::Unknown";
    }
}

template <typename T, typename E = AuraError>
using AuraResult = std::expected<T, E>;

class ActorAuraComponent {
public:
    ActorAuraComponent() = default;
    ~ActorAuraComponent() = default;

    ActorAuraComponent(const ActorAuraComponent&) = delete;
    ActorAuraComponent& operator=(const ActorAuraComponent&) = delete;
    ActorAuraComponent(ActorAuraComponent&&) noexcept = default;
    ActorAuraComponent& operator=(ActorAuraComponent&&) noexcept = default;

    AuraResult<void> ActivateAura(Client::Core::ItemVnum auraId);
    AuraResult<void> DeactivateAura();
    
    [[nodiscard]] bool IsAuraActive() const noexcept;
    [[nodiscard]] std::optional<Client::Core::ItemVnum> GetActiveAura() const noexcept;

private:
    std::optional<Client::Core::ItemVnum> m_activeAura;
};

} // namespace Client::Actors

template <>
struct std::formatter<Client::Actors::AuraError> : std::formatter<std::string_view> {
    auto format(Client::Actors::AuraError err, std::format_context& ctx) const {
        return std::formatter<std::string_view>::format(Client::Actors::to_string(err), ctx);
    }
};
