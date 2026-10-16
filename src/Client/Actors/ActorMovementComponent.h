#pragma once

#include <expected>
#include <string_view>
#include <cstdint>
#include <cmath>
#include "Client/Core/StrongTypes.h"

namespace Client::Actor {

enum class MovementError : uint8_t {
    None = 0,
    InvalidDestination,
    AlreadyAtDestination,
    Unreachable,
    SpeedZero,
    ZeroDuration,
    NotMoving
};

[[nodiscard]] constexpr std::string_view ToString(MovementError err) noexcept {
    switch (err) {
        case MovementError::None: return "None";
        case MovementError::InvalidDestination: return "InvalidDestination";
        case MovementError::AlreadyAtDestination: return "AlreadyAtDestination";
        case MovementError::Unreachable: return "Unreachable";
        case MovementError::SpeedZero: return "SpeedZero";
        case MovementError::ZeroDuration: return "ZeroDuration";
        case MovementError::NotMoving: return "NotMoving";
    }
    return "UnknownMovementError";
}

template <typename T = void>
using MovementResult = std::expected<T, MovementError>;

class ActorMovementComponent {
public:
    constexpr ActorMovementComponent() noexcept = default;

    void SetPosition(const Core::MapCoords& position) noexcept;
    
    [[nodiscard]] const Core::MapCoords& GetPosition() const noexcept;
    [[nodiscard]] const Core::MapCoords& GetDestination() const noexcept;
    [[nodiscard]] bool IsMoving() const noexcept;
    
    MovementResult<void> MoveTo(const Core::MapCoords& destination, float speed) noexcept;
    MovementResult<void> Update(float deltaTime) noexcept;
    void Stop() noexcept;

private:
    Core::MapCoords m_currentPosition{};
    Core::MapCoords m_destination{};
    Core::MapCoords m_direction{};
    float m_speed = 0.0f;
    bool m_isMoving = false;
    float m_distanceToMove = 0.0f;
    float m_movedDistance = 0.0f;
};

} // namespace Client::Actor
