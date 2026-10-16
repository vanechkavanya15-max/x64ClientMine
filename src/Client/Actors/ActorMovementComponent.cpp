#include "ActorMovementComponent.h"
#include <cmath>

namespace Client::Actor {

void ActorMovementComponent::SetPosition(const Core::MapCoords& position) noexcept {
    m_currentPosition = position;
    m_destination = position;
    m_isMoving = false;
    m_distanceToMove = 0.0f;
    m_movedDistance = 0.0f;
}

[[nodiscard]] const Core::MapCoords& ActorMovementComponent::GetPosition() const noexcept {
    return m_currentPosition;
}

[[nodiscard]] const Core::MapCoords& ActorMovementComponent::GetDestination() const noexcept {
    return m_destination;
}

[[nodiscard]] bool ActorMovementComponent::IsMoving() const noexcept {
    return m_isMoving;
}

MovementResult<void> ActorMovementComponent::MoveTo(const Core::MapCoords& destination, float speed) noexcept {
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

MovementResult<void> ActorMovementComponent::Update(float deltaTime) noexcept {
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

void ActorMovementComponent::Stop() noexcept {
    m_isMoving = false;
    m_destination = m_currentPosition;
    m_distanceToMove = 0.0f;
    m_movedDistance = 0.0f;
}

} // namespace Client::Actor
