#pragma once

#include <cstdint>
#include <vector>
#include <expected>
#include <optional>
#include <string_view>
#include <format>
#include "EterBase/StrongTypes.h"
#include "Client/Core/Result.h"

namespace Client::Actors {

// Silny typ dla identyfikatora efektu i uchwytu instancji efektu
struct EffectIdTag {};
struct EffectHandleTag {};

using EffectId = EterBase::StrongType<EffectIdTag, uint32_t, 0>;
using EffectHandle = EterBase::StrongType<EffectHandleTag, uint32_t, 0>;

enum class EffectError : uint8_t {
    None = 0,
    LimitReached,
    InvalidEffect,
    EffectNotFound
};

[[nodiscard]] inline std::string_view to_string(EffectError error) noexcept {
    switch (error) {
        case EffectError::None: return "EffectError::None";
        case EffectError::LimitReached: return "EffectError::LimitReached";
        case EffectError::InvalidEffect: return "EffectError::InvalidEffect";
        case EffectError::EffectNotFound: return "EffectError::EffectNotFound";
        default: return "EffectError::Unknown";
    }
}

template <typename T, typename E = EffectError>
using EffectResult = std::expected<T, E>;

class ActorEffectComponent {
public:
    static constexpr size_t MAX_ACTIVE_EFFECTS = 255;

    ActorEffectComponent() = default;
    ~ActorEffectComponent() = default;

    ActorEffectComponent(const ActorEffectComponent&) = delete;
    ActorEffectComponent& operator=(const ActorEffectComponent&) = delete;
    ActorEffectComponent(ActorEffectComponent&&) noexcept = default;
    ActorEffectComponent& operator=(ActorEffectComponent&&) noexcept = default;

    EffectResult<EffectHandle> AttachEffect(EffectId effectId);
    EffectResult<void> DetachEffect(EffectHandle handle);
    EffectResult<void> DetachAllEffects();
    
    [[nodiscard]] size_t GetActiveEffectCount() const noexcept;
    [[nodiscard]] bool HasEffect(EffectHandle handle) const noexcept;
    
    // Test/Headless utils:
    [[nodiscard]] std::vector<EffectHandle> GetActiveEffects() const;

private:
    struct ActiveEffect {
        EffectId id;
        EffectHandle handle;
    };

    std::vector<ActiveEffect> m_activeEffects;
    uint32_t m_nextHandleId{1};
};

} // namespace Client::Actors

template <>
struct std::formatter<Client::Actors::EffectError> : std::formatter<std::string_view> {
    auto format(Client::Actors::EffectError err, std::format_context& ctx) const {
        return std::formatter<std::string_view>::format(Client::Actors::to_string(err), ctx);
    }
};
