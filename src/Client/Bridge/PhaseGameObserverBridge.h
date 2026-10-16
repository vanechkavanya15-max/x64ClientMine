#pragma once
#include <cstdint>

namespace Client::Bridge {

class PhaseGameObserverBridge {
public:
    PhaseGameObserverBridge() = default;
    ~PhaseGameObserverBridge() = default;

    void SetObserverMode(bool enable) noexcept { m_isObserver = enable; }
    [[nodiscard]] bool IsObserverMode() const noexcept { return m_isObserver; }

    void SetTargetVid(uint32_t vid) noexcept { m_targetVid = vid; }
    [[nodiscard]] uint32_t GetTargetVid() const noexcept { return m_targetVid; }

private:
    bool m_isObserver{false};
    uint32_t m_targetVid{0};
};

} // namespace Client::Bridge
