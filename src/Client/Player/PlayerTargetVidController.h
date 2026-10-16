#pragma once
#include <cstdint>

namespace Client::Player {

class PlayerTargetVidController {
public:
    PlayerTargetVidController() = default;
    ~PlayerTargetVidController() = default;

    void SetTargetVid(uint32_t vid) noexcept { m_targetVid = vid; }
    void ClearTarget() noexcept { m_targetVid = 0; }
    [[nodiscard]] uint32_t GetTargetVid() const noexcept { return m_targetVid; }
    [[nodiscard]] bool HasTarget() const noexcept { return m_targetVid != 0; }

private:
    uint32_t m_targetVid{0};
};

} // namespace Client::Player
