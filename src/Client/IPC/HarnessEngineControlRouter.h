#pragma once

#include <cstdint>
#include <string>
#include <vector>
#include <expected>
#include <optional>
#include <random>
#include <deque>
#include <format>
#include <string_view>

namespace Client::IPC {

enum class WindowMode : uint8_t {
    Windowed,
    Borderless,
    Fullscreen
};

enum class EngineState : uint8_t {
    Initializing,
    Running,
    Paused,
    Crashed
};

enum class CrashReason : uint8_t {
    None,
    MemoryCorruption,
    Timeout,
    InvalidStateTransition,
    FuzzerInduced
};

struct FuzzerConfig {
    bool enabled{false};
    float dropRate{0.0f};      // 0.0 to 1.0
    float duplicateRate{0.0f}; // 0.0 to 1.0
    float corruptRate{0.0f};   // 0.0 to 1.0
    float crashRate{0.0f};     // 0.0 to 1.0
    uint32_t seed{42};
};

struct Packet {
    uint32_t id{0};
    uint64_t timestamp{0};
    std::vector<uint8_t> payload;
    bool corrupted{false};
};

struct CrashReport {
    EngineState lastState;
    CrashReason reason;
    uint64_t frameCount;
    float upTime;
    uint32_t packetsProcessed;
    uint32_t packetsDropped;
    uint32_t packetsCorrupted;
    std::string contextDump;
};

class HarnessEngineControlRouter {
public:
    enum class ErrorCode {
        AlreadyCrashed,
        InvalidTransition,
        InvalidDriver,
        QueueFull
    };

    explicit HarnessEngineControlRouter(FuzzerConfig fuzzerCfg = {});
    ~HarnessEngineControlRouter() = default;

    // Window & Graphics (Logical)
    void SetWindowVisible(bool visible) noexcept { m_visible = visible; }
    [[nodiscard]] bool IsWindowVisible() const noexcept { return m_visible; }
    
    std::expected<void, ErrorCode> SetWindowMode(WindowMode mode) noexcept;
    [[nodiscard]] WindowMode GetWindowMode() const noexcept { return m_windowMode; }

    void SetGpuThrottle(bool throttle) noexcept { m_gpuThrottle = throttle; }
    [[nodiscard]] bool IsGpuThrottled() const noexcept { return m_gpuThrottle; }

    // Protocol & Engine State
    std::expected<void, ErrorCode> SetProtocolDriver(std::string_view driver);
    [[nodiscard]] std::string GetProtocolDriver() const { return m_protocolDriver; }

    std::expected<void, ErrorCode> TransitionState(EngineState newState);
    [[nodiscard]] EngineState GetState() const noexcept { return m_state; }

    // Frame & Execution
    void TickFrame(float dt);
    [[nodiscard]] uint64_t GetFrameCount() const noexcept { return m_frameCount; }
    [[nodiscard]] float GetUpTime() const noexcept { return m_elapsedTime; }

    // IPC / Networking
    std::expected<void, ErrorCode> InjectPacket(Packet packet);
    [[nodiscard]] size_t GetPendingPacketCount() const noexcept { return m_packetQueue.size(); }
    
    // Debug & Crash Reporting
    void ForceCrash(CrashReason reason, std::string_view context);
    [[nodiscard]] std::optional<CrashReport> GetCrashReport() const;
    void Reset(EngineState initialState = EngineState::Initializing);

    // Fuzzer control
    void UpdateFuzzerConfig(const FuzzerConfig& cfg);

private:
    void ProcessPackets();
    void ApplyFuzzerEffects(Packet& packet);
    bool RollChance(float probability);

private:
    // Core state
    EngineState m_state{EngineState::Initializing};
    CrashReason m_crashReason{CrashReason::None};
    std::string m_crashContext;
    
    // Window settings
    bool m_visible{true};
    bool m_gpuThrottle{false};
    WindowMode m_windowMode{WindowMode::Windowed};
    std::string m_protocolDriver{"standard_x64"};
    
    // Time & Metrics
    float m_elapsedTime{0.0f};
    uint64_t m_frameCount{0};
    uint32_t m_packetsProcessed{0};
    uint32_t m_packetsDropped{0};
    uint32_t m_packetsCorrupted{0};

    // Queues
    std::deque<Packet> m_packetQueue;
    static constexpr size_t MAX_QUEUE_SIZE = 10000;

    // Fuzzer
    FuzzerConfig m_fuzzerConfig;
    std::mt19937_64 m_rng;
};

} // namespace Client::IPC
