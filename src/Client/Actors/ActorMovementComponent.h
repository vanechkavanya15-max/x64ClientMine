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

    // Set initial position without moving
    constexpr void SetPosition(const Core::MapCoords& position) noexcept {
        m_currentPosition = position;
        m_destination = position;
        m_isMoving = false;
        m_distanceToMove = 0.0f;
        m_movedDistance = 0.0f;
    }

    [[nodiscard]] constexpr const Core::MapCoords& GetPosition() const noexcept {
        return m_currentPosition;
    }

    [[nodiscard]] constexpr const Core::MapCoords& GetDestination() const noexcept {
        return m_destination;
    }

    [[nodiscard]] constexpr bool IsMoving() const noexcept {
        return m_isMoving;
    }

    constexpr MovementResult<void> MoveTo(const Core::MapCoords& destination, float speed) noexcept {
        if (speed <= 0.0f) {
            return std::unexpected(MovementError::SpeedZero);
        }

        if (m_currentPosition.x == destination.x && m_currentPosition.y == destination.y && m_currentPosition.z == destination.z) {
            return std::unexpected(MovementError::AlreadyAtDestination);
        }

        m_destination = destination;
        m_speed = speed;
        m_isMoving = true;
        
        m_direction = destination - m_currentPosition;
        m_distanceToMove = std::sqrt(m_direction.x * m_direction.x + m_direction.y * m_direction.y + m_direction.z * m_direction.z);
        m_movedDistance = 0.0f;

        if (m_distanceToMove > 0.0f) {
            m_direction /= m_distanceToMove;
        }

        return {};
    }

    constexpr MovementResult<void> Update(float deltaTime) noexcept {
        if (!m_isMoving) {
            return std::unexpected(MovementError::NotMoving);
        }

        if (deltaTime <= 0.0f) {
            return std::unexpected(MovementError::ZeroDuration);
        }

        float moveAmount = m_speed * deltaTime;
        m_movedDistance += moveAmount;

        if (m_movedDistance >= m_distanceToMove) {
            m_currentPosition = m_destination;
            m_isMoving = false;
            m_distanceToMove = 0.0f;
            m_movedDistance = 0.0f;
        } else {
            m_currentPosition += m_direction * moveAmount;
        }

        return {};
    }

    constexpr void Stop() noexcept {
        m_isMoving = false;
        m_destination = m_currentPosition;
        m_distanceToMove = 0.0f;
        m_movedDistance = 0.0f;
    }

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
