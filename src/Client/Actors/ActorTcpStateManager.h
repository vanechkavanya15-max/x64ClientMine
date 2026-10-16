#pragma once
#include <cstdint>
#include <vector>
#include <optional>
#include <expected>
#include <deque>
#include <algorithm>

namespace Client::Actors {

enum ActorStateBits : uint32_t {
    STATE_NONE     = 0,
    STATE_STUN     = 1 << 0,
    STATE_DEAD     = 1 << 1,
    STATE_POISON   = 1 << 2,
    STATE_SLOW     = 1 << 3,
    STATE_BERSERK  = 1 << 4,
    STATE_BLEEDING = 1 << 5
};

enum class StateManagerError {
    None,
    InvalidSequence,
    DuplicateUpdate,
    StaleUpdate,
    PredictionNotFound,
    PredictionTimeout
};

struct StateUpdateCommand {
    uint32_t sequence;
    uint64_t serverTimestamp;
    uint32_t stateMask;
};

struct StateSnapshot {
    uint64_t timestamp;
    uint32_t stateMask;
};

struct LocalPrediction {
    uint32_t predictionId;
    uint64_t timestamp;
    uint32_t stateBitsToSet;
    uint32_t stateBitsToClear;
    bool isConfirmed;
};

class ActorTcpStateManager {
public:
    ActorTcpStateManager() = default;
    ~ActorTcpStateManager() = default;

    void SetState(uint32_t stateBits) noexcept { 
        m_confirmedStateMask |= stateBits; 
        UpdateEffectiveMask();
    }
    void ClearState(uint32_t stateBits) noexcept { 
        m_confirmedStateMask &= ~stateBits; 
        UpdateEffectiveMask();
    }
    void ResetAll() noexcept { 
        m_confirmedStateMask = STATE_NONE; 
        m_effectiveStateMask = STATE_NONE;
        m_jitterBuffer.clear();
        m_historyBuffer.clear();
        m_predictions.clear();
        m_lastProcessedSequence = 0;
        m_hasReceivedFirstUpdate = false;
    }

    [[nodiscard]] bool HasState(uint32_t stateBits) const noexcept {
        return (m_effectiveStateMask & stateBits) == stateBits;
    }
    [[nodiscard]] bool IsStunned() const noexcept { return HasState(STATE_STUN); }
    [[nodiscard]] bool IsDead() const noexcept { return HasState(STATE_DEAD); }
    [[nodiscard]] bool CanMove() const noexcept { return !IsStunned() && !IsDead(); }

    std::expected<void, StateManagerError> PushNetworkUpdate(const StateUpdateCommand& cmd);
    void ProcessPlayout(uint64_t clientPlayoutTime);

    uint32_t PredictStateLocal(uint64_t timestamp, uint32_t bitsToSet, uint32_t bitsToClear);
    std::expected<void, StateManagerError> ConfirmLocalPrediction(uint32_t predictionId);
    std::expected<void, StateManagerError> RollbackLocalPrediction(uint32_t predictionId);
    void CleanupStalePredictions(uint64_t currentTime, uint64_t timeoutMs);

    [[nodiscard]] std::optional<uint32_t> GetStateAtTimestamp(uint64_t timestamp) const;
    [[nodiscard]] bool HadStateInWindow(uint64_t startTime, uint64_t endTime, uint32_t stateBits) const;

    [[nodiscard]] uint32_t GetConfirmedMask() const noexcept { return m_confirmedStateMask; }
    [[nodiscard]] uint32_t GetEffectiveMask() const noexcept { return m_effectiveStateMask; }

private:
    void UpdateEffectiveMask() noexcept;

    uint32_t m_confirmedStateMask{STATE_NONE};
    uint32_t m_effectiveStateMask{STATE_NONE};

    uint32_t m_lastProcessedSequence{0};
    bool m_hasReceivedFirstUpdate{false};

    std::deque<StateUpdateCommand> m_jitterBuffer;
    std::deque<StateSnapshot> m_historyBuffer;
    std::vector<LocalPrediction> m_predictions;

    uint32_t m_nextPredictionId{1};
    static constexpr size_t MAX_HISTORY_SIZE = 128;
};

} // namespace Client::Actors
