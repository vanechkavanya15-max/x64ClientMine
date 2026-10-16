#pragma once
#include <cstdint>
#include <cmath>
#include <expected>

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

enum class MovementError {
    None,
    BlockedByWall,
    BlockedByObject,
    BlockedByWater,
    VerticalStepTooHigh,
    EnvironmentNotSet,
    InvalidParameters,
    NotMoving
};

class ITerrainEnvironment {
public:
    virtual ~ITerrainEnvironment() = default;

    virtual bool IsWall(float x, float y) const noexcept = 0;
    virtual bool IsBlockObject(float x, float y) const noexcept = 0;
    virtual bool IsWater(float x, float y) const noexcept = 0;
    
    // returns modified target position if blocked but slidable
    virtual bool AvoidObject(float startX, float startY, float& targetX, float& targetY) const noexcept = 0;
    
    virtual float GetHeight(float x, float y) const noexcept = 0;
};

class ActorPixelMovementIntegrator {
public:
    ActorPixelMovementIntegrator() = default;
    ~ActorPixelMovementIntegrator() = default;

    void SetEnvironment(ITerrainEnvironment* env) noexcept { m_env = env; }
    
    void SetPosition(float x, float y, float z = 0.0f) noexcept { m_pos = {x, y, z}; }
    [[nodiscard]] PixelPos GetPosition() const noexcept { return m_pos; }

    void SetMovement(float dirX, float dirY, float speed) noexcept {
        m_mov = {dirX, dirY, speed};
    }
    void StopMovement() noexcept { m_mov = {0.0f, 0.0f, 0.0f}; }
    [[nodiscard]] bool IsMoving() const noexcept { return m_mov.speed > 0.001f; }

    // Obsolete signature kept for backward compatibility (or modified to return expected, 
    // let's change it or provide overload. Wait, test is checking `CHECK_FALSE(mover.IntegrateStep(0.5f, true));`
    // We should keep the old signature or update the tests. The plan said "expand methods to return std::expected".
    // I will provide a new IntegrateStep returning std::expected and modify the old test or just break compatibility 
    // and fix the test. The plan said "expand IntegrateStep to return std::expected".
    [[nodiscard]] std::expected<PixelPos, MovementError> IntegrateStep(float deltaTime);

    [[nodiscard]] float DistanceTo(const PixelPos& target) const noexcept;

    void SetMaxVerticalStep(float maxStep) noexcept { m_maxVerticalStep = maxStep; }
    [[nodiscard]] float GetMaxVerticalStep() const noexcept { return m_maxVerticalStep; }

    void SetMicroStepSize(float stepSize) noexcept { m_microStepSize = stepSize; }

private:
    PixelPos m_pos{};
    MovementVector m_mov{};
    ITerrainEnvironment* m_env{nullptr};
    float m_maxVerticalStep{50.0f};
    float m_microStepSize{10.0f}; // Size of a single micro-step for accurate collision
    
    std::expected<PixelPos, MovementError> ProcessMicroStep(float startX, float startY, float targetX, float targetY);
};

} // namespace Client::Actors
