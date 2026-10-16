#include "ActorPixelMovementIntegrator.h"
#include <algorithm>

namespace Client::Actors {

std::expected<PixelPos, MovementError> ActorPixelMovementIntegrator::IntegrateStep(float deltaTime) {
    if (!IsMoving()) {
        return std::unexpected(MovementError::NotMoving);
    }
    if (!m_env) {
        return std::unexpected(MovementError::EnvironmentNotSet);
    }
    if (deltaTime <= 0.0f) {
        return std::unexpected(MovementError::InvalidParameters);
    }

    float len = std::sqrt(m_mov.dirX * m_mov.dirX + m_mov.dirY * m_mov.dirY);
    if (len < 0.0001f) {
        return std::unexpected(MovementError::NotMoving);
    }

    float normX = m_mov.dirX / len;
    float normY = m_mov.dirY / len;
    
    float totalDistance = m_mov.speed * deltaTime;
    if (totalDistance < 0.0001f) {
        return m_pos; // practically not moving
    }

    float targetX = m_pos.x + normX * totalDistance;
    float targetY = m_pos.y + normY * totalDistance;

    // Sub-stepping for precise collision
    int numSteps = std::max(1, static_cast<int>(std::ceil(totalDistance / m_microStepSize)));
    float stepX = (targetX - m_pos.x) / numSteps;
    float stepY = (targetY - m_pos.y) / numSteps;

    float currentX = m_pos.x;
    float currentY = m_pos.y;
    
    std::expected<PixelPos, MovementError> lastResult = m_pos;

    for (int i = 0; i < numSteps; ++i) {
        float nextX = currentX + stepX;
        float nextY = currentY + stepY;
        
        auto stepResult = ProcessMicroStep(currentX, currentY, nextX, nextY);
        if (!stepResult.has_value()) {
            // we hit a hard block
            // Even if blocked, update the position to the last valid one
            m_pos.x = currentX;
            m_pos.y = currentY;
            m_pos.z = m_env->GetHeight(currentX, currentY);
            return std::unexpected(stepResult.error());
        }
        
        currentX = stepResult->x;
        currentY = stepResult->y;
        
        // Update velocity if sliding modified the path
        if (std::abs(currentX - nextX) > 0.001f || std::abs(currentY - nextY) > 0.001f) {
            // path was modified (sliding), recalculate step size for remaining steps
            float remainingDistance = static_cast<float>(numSteps - i - 1) * m_microStepSize;
            if (remainingDistance > 0.0f) {
                // If sliding caused us to stop moving in target direction, we should just break
                float newDx = currentX - m_pos.x;
                float newDy = currentY - m_pos.y;
                if (std::abs(newDx) < 0.001f && std::abs(newDy) < 0.001f) {
                    break;
                }
            }
        }
    }

    m_pos.x = currentX;
    m_pos.y = currentY;
    m_pos.z = m_env->GetHeight(currentX, currentY);

    return m_pos;
}

std::expected<PixelPos, MovementError> ActorPixelMovementIntegrator::ProcessMicroStep(float startX, float startY, float targetX, float targetY) {
    float checkX = targetX;
    float checkY = targetY;

    // 1. Water collision
    if (m_env->IsWater(checkX, checkY)) {
        return std::unexpected(MovementError::BlockedByWater);
    }

    // 2. Wall collision
    if (m_env->IsWall(checkX, checkY)) {
        // Simple sliding logic for walls if avoid object is not strictly wall avoiding
        // In this implementation, AvoidObject handles sliding. Let's ask AvoidObject first.
        float avoidX = checkX;
        float avoidY = checkY;
        if (m_env->AvoidObject(startX, startY, avoidX, avoidY)) {
            checkX = avoidX;
            checkY = avoidY;
            // Re-verify wall and water
            if (m_env->IsWall(checkX, checkY) || m_env->IsWater(checkX, checkY)) {
                return std::unexpected(MovementError::BlockedByWall);
            }
        } else {
            return std::unexpected(MovementError::BlockedByWall);
        }
    }

    // 3. Object block collision
    if (m_env->IsBlockObject(checkX, checkY)) {
        float avoidX = checkX;
        float avoidY = checkY;
        if (m_env->AvoidObject(startX, startY, avoidX, avoidY)) {
            checkX = avoidX;
            checkY = avoidY;
            // Re-verify after avoiding
            if (m_env->IsBlockObject(checkX, checkY) || m_env->IsWall(checkX, checkY) || m_env->IsWater(checkX, checkY)) {
                return std::unexpected(MovementError::BlockedByObject);
            }
        } else {
            return std::unexpected(MovementError::BlockedByObject);
        }
    }

    // 4. Vertical step limitation
    float startZ = m_env->GetHeight(startX, startY);
    float targetZ = m_env->GetHeight(checkX, checkY);
    if (std::abs(targetZ - startZ) > m_maxVerticalStep) {
        return std::unexpected(MovementError::VerticalStepTooHigh);
    }

    PixelPos newPos;
    newPos.x = checkX;
    newPos.y = checkY;
    newPos.z = targetZ;
    return newPos;
}

float ActorPixelMovementIntegrator::DistanceTo(const PixelPos& target) const noexcept {
    float dx = m_pos.x - target.x;
    float dy = m_pos.y - target.y;
    return std::sqrt(dx * dx + dy * dy);
}

} // namespace Client::Actors
