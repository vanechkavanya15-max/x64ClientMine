#include "ActorMovementComponent.h"
#include <cmath>

namespace Client::Actor {

ActorMovementComponent::ActorMovementComponent(const Client::Core::MapCoords& initialPos) noexcept
    : m_currentPosition(initialPos)
    , m_targetPosition(initialPos)
{
}

[[nodiscard]] EterBase::Result<void, EterBase::EntityError> ActorMovementComponent::MoveTo(const Client::Core::MapCoords& targetPos, float speed) noexcept {
    if (speed <= 0.0f) {
        return EterBase::MakeError(EterBase::EntityError::InvalidType); // Używam InvalidType jako błędu walidacji prędkości
    }

    if (m_currentPosition == targetPos) {
        Stop();
        return {}; // Już jesteśmy na miejscu
    }

    m_targetPosition = targetPos;
    m_speed = speed;
    m_isMoving = true;

    return {};
}

void ActorMovementComponent::Stop() noexcept {
    m_isMoving = false;
    m_speed = 0.0f;
    m_targetPosition = m_currentPosition;
}

std::optional<Client::Core::MapCoords> ActorMovementComponent::Update(float deltaTime) noexcept {
    if (!m_isMoving || deltaTime <= 0.0f) {
        return std::nullopt;
    }

    const float distanceToTarget = m_currentPosition.Distance(m_targetPosition);
    const float moveDistance = m_speed * deltaTime;

    if (distanceToTarget <= moveDistance) {
        // Dotarliśmy do celu
        m_currentPosition = m_targetPosition;
        Stop();
        return m_currentPosition;
    }

    // Interpolacja liniowa
    Client::Core::MapCoords direction = (m_targetPosition - m_currentPosition).Normalize();
    m_currentPosition += direction * moveDistance;

    return m_currentPosition;
}

[[nodiscard]] const Client::Core::MapCoords& ActorMovementComponent::GetCurrentPosition() const noexcept {
    return m_currentPosition;
}

[[nodiscard]] const Client::Core::MapCoords& ActorMovementComponent::GetTargetPosition() const noexcept {
    return m_targetPosition;
}

[[nodiscard]] bool ActorMovementComponent::IsMoving() const noexcept {
    return m_isMoving;
}

void ActorMovementComponent::SetPosition(const Client::Core::MapCoords& pos) noexcept {
    m_currentPosition = pos;
    Stop();
}

} // namespace Client::Actor
