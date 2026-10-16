#include "PlayerMainActorPositionTracker.h"
#include <cmath>
#include <algorithm>

namespace Client::Player {

void PlayerMainActorPositionTracker::Reconfigure(const TrackerConfig& config) noexcept {
    m_config = config;
}

Client::Core::Result<void, TrackerError> PlayerMainActorPositionTracker::ForcePosition(const Client::Core::MapCoords& position) noexcept {
    if (std::isnan(position.x) || std::isnan(position.y) || std::isnan(position.z) ||
        std::isinf(position.x) || std::isinf(position.y) || std::isinf(position.z)) {
        return std::unexpected(TrackerError::InvalidInput);
    }

    m_currentPos = position;
    m_state = InterpolationState::Idle;
    m_waypoints.clear();
    m_timeAccumulator = 0;
    
    m_history.clear();
    m_totalTimeAcc = 0;
    RecordHistory(m_currentPos, m_totalTimeAcc);

    return {};
}

Client::Core::Result<void, TrackerError> PlayerMainActorPositionTracker::PushNetworkWaypoint(const Client::Core::MapCoords& targetPos, uint32_t timestampMs) noexcept {
    if (std::isnan(targetPos.x) || std::isnan(targetPos.y) || std::isnan(targetPos.z) ||
        std::isinf(targetPos.x) || std::isinf(targetPos.y) || std::isinf(targetPos.z)) {
        return std::unexpected(TrackerError::InvalidInput);
    }

    if (m_waypoints.size() >= m_config.maxQueueSize) {
        return std::unexpected(TrackerError::QueueOverflow);
    }

    // Determine state
    if (m_state == InterpolationState::Idle) {
        m_state = InterpolationState::Linear;
    }

    m_waypoints.push_back({targetPos, timestampMs});

    if (m_waypoints.size() >= m_config.splineActivationThreshold) {
        m_state = InterpolationState::Spline;
    }

    return {};
}

void PlayerMainActorPositionTracker::Update(uint32_t deltaMs) noexcept {
    m_totalTimeAcc += deltaMs;

    if (m_waypoints.empty()) {
        m_state = InterpolationState::Idle;
        m_timeAccumulator = 0;
        RecordHistory(m_currentPos, m_totalTimeAcc);
        return;
    }

    if (m_state == InterpolationState::Linear) {
        AdvanceLinear(deltaMs);
    } else if (m_state == InterpolationState::Spline) {
        AdvanceSpline(deltaMs);
    }

    RecordHistory(m_currentPos, m_totalTimeAcc);
}

void PlayerMainActorPositionTracker::AdvanceLinear(uint32_t deltaMs) noexcept {
    if (m_waypoints.empty()) return;

    auto& target = m_waypoints.front();
    float dist = m_currentPos.Distance(target.position);
    
    // Very close? Snap and consume.
    if (dist < 1.0f) {
        m_currentPos = target.position;
        m_waypoints.pop_front();
        if (m_waypoints.empty()) {
            m_state = InterpolationState::Idle;
        }
        return;
    }

    // Calculate move
    float moveDist = (m_config.baseMoveSpeed / 1000.0f) * static_cast<float>(deltaMs);
    
    if (moveDist >= dist) {
        m_currentPos = target.position;
        m_waypoints.pop_front();
        if (m_waypoints.empty()) {
            m_state = InterpolationState::Idle;
        }
    } else {
        auto dir = (target.position - m_currentPos).Normalize();
        m_currentPos += dir * moveDist;
    }
}

void PlayerMainActorPositionTracker::AdvanceSpline(uint32_t deltaMs) noexcept {
    if (m_waypoints.size() < 2) {
        m_state = InterpolationState::Linear;
        AdvanceLinear(deltaMs);
        return;
    }

    auto& wp0 = m_waypoints[0];
    auto& wp1 = m_waypoints[1];

    uint32_t segmentTime = (wp1.timestampMs > wp0.timestampMs) ? (wp1.timestampMs - wp0.timestampMs) : 100;
    if (segmentTime == 0) segmentTime = 1;

    m_timeAccumulator += deltaMs;

    float t = std::clamp(static_cast<float>(m_timeAccumulator) / static_cast<float>(segmentTime), 0.0f, 1.0f);

    // Determine points for Catmull-Rom
    Client::Core::MapCoords p0 = m_currentPos; // Ideally we use previous point, but current works as an approximation if we just entered spline
    Client::Core::MapCoords p1 = wp0.position;
    Client::Core::MapCoords p2 = wp1.position;
    Client::Core::MapCoords p3 = m_waypoints.size() > 2 ? m_waypoints[2].position : p2 + (p2 - p1);

    m_currentPos = EvaluateCatmullRom(t, p0, p1, p2, p3);

    if (m_timeAccumulator >= segmentTime) {
        m_waypoints.pop_front();
        m_timeAccumulator -= segmentTime;
        if (m_waypoints.size() < m_config.splineActivationThreshold) {
            m_state = InterpolationState::Linear;
        }
    }
}

Client::Core::MapCoords PlayerMainActorPositionTracker::EvaluateCatmullRom(float t, const Client::Core::MapCoords& p0, const Client::Core::MapCoords& p1, const Client::Core::MapCoords& p2, const Client::Core::MapCoords& p3) const noexcept {
    float t2 = t * t;
    float t3 = t2 * t;

    float f0 = -0.5f * t3 + t2 - 0.5f * t;
    float f1 =  1.5f * t3 - 2.5f * t2 + 1.0f;
    float f2 = -1.5f * t3 + 2.0f * t2 + 0.5f * t;
    float f3 =  0.5f * t3 - 0.5f * t2;

    return p0 * f0 + p1 * f1 + p2 * f2 + p3 * f3;
}

void PlayerMainActorPositionTracker::RecordHistory(const Client::Core::MapCoords& pos, uint32_t timeAcc) noexcept {
    m_history.push_back({pos, timeAcc});
    
    // Cleanup old history that is older than lag config + some buffer (e.g., 200ms extra)
    while (m_history.size() > 2 && (timeAcc - m_history.front().timeAcc) > (m_config.cameraLagMs + 200)) {
        m_history.pop_front();
    }
}

Client::Core::MapCoords PlayerMainActorPositionTracker::GetCameraSyncPosition() const noexcept {
    if (m_history.empty()) {
        return m_currentPos;
    }

    if (m_config.cameraLagMs == 0) {
        return m_currentPos;
    }

    uint32_t targetTime = (m_totalTimeAcc > m_config.cameraLagMs) ? (m_totalTimeAcc - m_config.cameraLagMs) : 0;

    // Find the segment in history
    for (size_t i = 1; i < m_history.size(); ++i) {
        auto& p0 = m_history[i - 1];
        auto& p1 = m_history[i];

        if (targetTime >= p0.timeAcc && targetTime <= p1.timeAcc) {
            uint32_t dt = p1.timeAcc - p0.timeAcc;
            if (dt == 0) return p1.pos;

            float t = static_cast<float>(targetTime - p0.timeAcc) / static_cast<float>(dt);
            return p0.pos + (p1.pos - p0.pos) * t; // Linear interpolation for camera history
        }
    }

    // If we couldn't find an exact segment (e.g. all history is older than lag, or very new), return closest
    if (m_history.back().timeAcc <= targetTime) {
        return m_history.back().pos;
    }

    return m_history.front().pos;
}

} // namespace Client::Player
