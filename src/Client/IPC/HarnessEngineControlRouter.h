#pragma once
#include <cstdint>
#include <string>

namespace Client::IPC {

class HarnessEngineControlRouter {
public:
    HarnessEngineControlRouter() = default;
    ~HarnessEngineControlRouter() = default;

    void SetWindowVisible(bool visible) noexcept { m_visible = visible; }
    [[nodiscard]] bool IsWindowVisible() const noexcept { return m_visible; }

    void SetGpuThrottle(bool throttle) noexcept { m_gpuThrottle = throttle; }
    [[nodiscard]] bool IsGpuThrottled() const noexcept { return m_gpuThrottle; }

    void SetProtocolDriver(const std::string& driver) { m_protocolDriver = driver; }
    [[nodiscard]] std::string GetProtocolDriver() const { return m_protocolDriver; }

    void StepFrame(float dt) noexcept { m_elapsedTime += dt; m_frameCount++; }
    [[nodiscard]] uint64_t GetFrameCount() const noexcept { return m_frameCount; }

private:
    bool m_visible{true};
    bool m_gpuThrottle{false};
    std::string m_protocolDriver{"standard_x64"};
    float m_elapsedTime{0.0f};
    uint64_t m_frameCount{0};
};

} // namespace Client::IPC
