#pragma once
#include <cstdint>

namespace Client::Player {

struct Coord3D { float x{0.0f}; float y{0.0f}; float z{0.0f}; };

class PlayerMainActorPositionTracker {
public:
    PlayerMainActorPositionTracker() = default;
    ~PlayerMainActorPositionTracker() = default;

    void UpdatePosition(float x, float y, float z) noexcept { m_pos = {x, y, z}; }
    [[nodiscard]] Coord3D GetPosition() const noexcept { return m_pos; }

private:
    Coord3D m_pos{};
};

} // namespace Client::Player
