#pragma once

#include <cstdint>
#include <expected>
#include <optional>
#include <string_view>
#include <format>
#include "Client/Core/StrongTypes.h"

namespace Client::Actors {

enum class AcceError : uint8_t {
    None = 0,
    AlreadyEquipped,
    NotEquipped,
    InvalidAcce
};

[[nodiscard]] inline std::string_view to_string(AcceError error) noexcept {
    switch (error) {
        case AcceError::None: return "AcceError::None";
        case AcceError::AlreadyEquipped: return "AcceError::AlreadyEquipped";
        case AcceError::NotEquipped: return "AcceError::NotEquipped";
        case AcceError::InvalidAcce: return "AcceError::InvalidAcce";
        default: return "AcceError::Unknown";
    }
}

template <typename T, typename E = AcceError>
using AcceResult = std::expected<T, E>;

class ActorAcceComponent {
public:
    ActorAcceComponent() = default;
    ~ActorAcceComponent() = default;

    ActorAcceComponent(const ActorAcceComponent&) = delete;
    ActorAcceComponent& operator=(const ActorAcceComponent&) = delete;
    ActorAcceComponent(ActorAcceComponent&&) noexcept = default;
    ActorAcceComponent& operator=(ActorAcceComponent&&) noexcept = default;

    AcceResult<void> EquipAcce(Client::Core::ItemVnum acceId);
    AcceResult<void> UnequipAcce();

    [[nodiscard]] bool IsAcceEquipped() const noexcept;
    [[nodiscard]] std::optional<Client::Core::ItemVnum> GetEquippedAcce() const noexcept;

private:
    std::optional<Client::Core::ItemVnum> m_equippedAcce;
};

} // namespace Client::Actors

template <>
struct std::formatter<Client::Actors::AcceError> : std::formatter<std::string_view> {
    auto format(Client::Actors::AcceError err, std::format_context& ctx) const {
        return std::formatter<std::string_view>::format(Client::Actors::to_string(err), ctx);
    }
};
