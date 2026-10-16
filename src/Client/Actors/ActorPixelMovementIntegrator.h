#pragma once
#include <cstdint>
#include <cmath>

namespace Client::Actors {

struct PixelPos {
    float x{0.0f};
    float y{0.0f};
    float z{0.0f};
};

struct MovementVector {
    float dirX{0.0f};
    float dirY{0.0f};
    float speed{0.0f};
};

class ActorPixelMovementIntegrator {
public:
    ActorPixelMovementIntegrator() = default;
    ~ActorPixelMovementIntegrator() = default;

    void SetPosition(float x, float y, float z = 0.0f) noexcept { m_pos = {x, y, z}; }
    [[nodiscard]] PixelPos GetPosition() const noexcept { return m_pos; }

    void SetMovement(float dirX, float dirY, float speed) noexcept {
        m_mov = {dirX, dirY, speed};
    }
    void StopMovement() noexcept { m_mov = {0.0f, 0.0f, 0.0f}; }
    [[nodiscard]] bool IsMoving() const noexcept { return m_mov.speed > 0.001f; }

    bool IntegrateStep(float deltaTime, bool blockedByWall = false);
    [[nodiscard]] float DistanceTo(const PixelPos& target) const noexcept;

private:
    PixelPos m_pos{};
    MovementVector m_mov{};
};

} // namespace Client::Actors
