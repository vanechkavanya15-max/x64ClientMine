#pragma once

#include <cstdint>
#include <deque>
#include <vector>
#include <expected>
#include <format>
#include <string_view>
#include "Client/Core/StrongTypes.h"
#include "Client/Core/Result.h"

namespace Client::Player {

enum class TrackerError : uint8_t {
    None = 0,
    InvalidInput,
    QueueOverflow,
    NotInitialized
};

[[nodiscard]] constexpr std::string_view to_string(TrackerError error) noexcept {
    switch (error) {
        case TrackerError::None: return "TrackerError::None";
        case TrackerError::InvalidInput: return "TrackerError::InvalidInput";
        case TrackerError::QueueOverflow: return "TrackerError::QueueOverflow";
        case TrackerError::NotInitialized: return "TrackerError::NotInitialized";
        default: return "TrackerError::Unknown";
    }
}

struct Waypoint {
    Client::Core::MapCoords position;
    uint32_t timestampMs;
};

struct TrackerConfig {
    uint32_t maxQueueSize{32};
    uint32_t splineActivationThreshold{3}; // Use spline if we have at least 3 points
    uint32_t cameraLagMs{100}; // Camera trails behind by this many ms
    float baseMoveSpeed{400.0f}; // Units per second
};

enum class InterpolationState : uint8_t {
    Idle,
    Linear,
    Spline
};

class PlayerMainActorPositionTracker {
public:
    PlayerMainActorPositionTracker() = default;
    explicit PlayerMainActorPositionTracker(const TrackerConfig& config) noexcept : m_config(config) {}
    ~PlayerMainActorPositionTracker() = default;

    void Reconfigure(const TrackerConfig& config) noexcept;

    Client::Core::Result<void, TrackerError> ForcePosition(const Client::Core::MapCoords& position) noexcept;
    Client::Core::Result<void, TrackerError> PushNetworkWaypoint(const Client::Core::MapCoords& targetPos, uint32_t timestampMs) noexcept;

    void Update(uint32_t deltaMs) noexcept;

    [[nodiscard]] Client::Core::MapCoords NEW_GetMainActorPosition() const noexcept { return m_currentPos; }
    [[nodiscard]] Client::Core::MapCoords GetCameraSyncPosition() const noexcept;
    [[nodiscard]] InterpolationState GetState() const noexcept { return m_state; }

private:
    void AdvanceLinear(uint32_t deltaMs) noexcept;
    void AdvanceSpline(uint32_t deltaMs) noexcept;
    [[nodiscard]] Client::Core::MapCoords EvaluateCatmullRom(float t, const Client::Core::MapCoords& p0, const Client::Core::MapCoords& p1, const Client::Core::MapCoords& p2, const Client::Core::MapCoords& p3) const noexcept;
    
    void RecordHistory(const Client::Core::MapCoords& pos, uint32_t timeAcc) noexcept;

    TrackerConfig m_config{};
    InterpolationState m_state{InterpolationState::Idle};
    
    Client::Core::MapCoords m_currentPos{};
    
    std::deque<Waypoint> m_waypoints;
    uint32_t m_timeAccumulator{0};
    
    struct HistoryPoint {
        Client::Core::MapCoords pos;
        uint32_t timeAcc;
    };
    std::deque<HistoryPoint> m_history;
    uint32_t m_totalTimeAcc{0};
};

} // namespace Client::Player

template <>
struct std::formatter<Client::Player::TrackerError> : std::formatter<std::string_view> {
    auto format(Client::Player::TrackerError err, std::format_context& ctx) const {
        return std::formatter<std::string_view>::format(Client::Player::to_string(err), ctx);
    }
};
