#include "ActorPixelMovementIntegrator.h"

namespace Client::Actors {

bool ActorPixelMovementIntegrator::IntegrateStep(float deltaTime, bool blockedByWall) {
    if (!IsMoving() || blockedByWall) {
        return false;
    }
    float len = std::sqrt(m_mov.dirX * m_mov.dirX + m_mov.dirY * m_mov.dirY);
    if (len > 0.0001f) {
        float normX = m_mov.dirX / len;
        float normY = m_mov.dirY / len;
        m_pos.x += normX * m_mov.speed * deltaTime;
        m_pos.y += normY * m_mov.speed * deltaTime;
        return true;
    }
    return false;
}

float ActorPixelMovementIntegrator::DistanceTo(const PixelPos& target) const noexcept {
    float dx = m_pos.x - target.x;
    float dy = m_pos.y - target.y;
    return std::sqrt(dx * dx + dy * dy);
}

} // namespace Client::Actors
