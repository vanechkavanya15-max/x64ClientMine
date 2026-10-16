#include "ActorTcpStateManager.h"

namespace Client::Actors {

void ActorTcpStateManager::UpdateEffectiveMask() noexcept {
    m_effectiveStateMask = m_confirmedStateMask;
    for (const auto& prediction : m_predictions) {
        if (!prediction.isConfirmed) {
            m_effectiveStateMask |= prediction.stateBitsToSet;
            m_effectiveStateMask &= ~prediction.stateBitsToClear;
        }
    }
}

std::expected<void, StateManagerError> ActorTcpStateManager::PushNetworkUpdate(const StateUpdateCommand& cmd) {
    if (m_hasReceivedFirstUpdate) {
        if (cmd.sequence <= m_lastProcessedSequence) {
            return std::unexpected(StateManagerError::StaleUpdate);
        }
        
        // Check duplicates in jitter buffer
        auto it = std::find_if(m_jitterBuffer.begin(), m_jitterBuffer.end(), 
            [&cmd](const StateUpdateCommand& elem) {
                return elem.sequence == cmd.sequence;
            });
        
        if (it != m_jitterBuffer.end()) {
            return std::unexpected(StateManagerError::DuplicateUpdate);
        }
    } else {
        m_hasReceivedFirstUpdate = true;
        m_lastProcessedSequence = cmd.sequence - 1; // Accept this sequence
    }

    m_jitterBuffer.push_back(cmd);
    
    // Sort by sequence for ordered playout
    std::sort(m_jitterBuffer.begin(), m_jitterBuffer.end(),
        [](const StateUpdateCommand& a, const StateUpdateCommand& b) {
            return a.sequence < b.sequence;
        });

    return {};
}

void ActorTcpStateManager::ProcessPlayout(uint64_t clientPlayoutTime) {
    if (m_jitterBuffer.empty()) {
        return;
    }

    bool stateChanged = false;
    auto it = m_jitterBuffer.begin();
    while (it != m_jitterBuffer.end()) {
        if (it->serverTimestamp <= clientPlayoutTime) {
            // Check for sequence gap
            if (it->sequence > m_lastProcessedSequence + 1) {
                // We have a gap, we must wait or extrapolate. 
                // For this implementation, we apply what we have but it could mean missing states.
                // Assuming we just apply and catch up.
            }
            
            m_confirmedStateMask = it->stateMask;
            m_lastProcessedSequence = it->sequence;
            
            StateSnapshot snap;
            snap.timestamp = it->serverTimestamp;
            snap.stateMask = m_confirmedStateMask;
            
            m_historyBuffer.push_back(snap);
            if (m_historyBuffer.size() > MAX_HISTORY_SIZE) {
                m_historyBuffer.pop_front();
            }
            
            stateChanged = true;
            m_jitterBuffer.pop_front();
            it = m_jitterBuffer.begin();
        } else {
            break;
        }
    }

    if (stateChanged) {
        UpdateEffectiveMask();
    }
}

uint32_t ActorTcpStateManager::PredictStateLocal(uint64_t timestamp, uint32_t bitsToSet, uint32_t bitsToClear) {
    uint32_t id = m_nextPredictionId++;
    LocalPrediction pred;
    pred.predictionId = id;
    pred.timestamp = timestamp;
    pred.stateBitsToSet = bitsToSet;
    pred.stateBitsToClear = bitsToClear;
    pred.isConfirmed = false;
    
    m_predictions.push_back(pred);
    UpdateEffectiveMask();
    return id;
}

std::expected<void, StateManagerError> ActorTcpStateManager::ConfirmLocalPrediction(uint32_t predictionId) {
    auto it = std::find_if(m_predictions.begin(), m_predictions.end(),
        [predictionId](const LocalPrediction& p) {
            return p.predictionId == predictionId;
        });
        
    if (it == m_predictions.end()) {
        return std::unexpected(StateManagerError::PredictionNotFound);
    }
    
    m_confirmedStateMask |= it->stateBitsToSet;
    m_confirmedStateMask &= ~it->stateBitsToClear;
    it->isConfirmed = true;
    m_predictions.erase(it);
    UpdateEffectiveMask();
    return {};
}

std::expected<void, StateManagerError> ActorTcpStateManager::RollbackLocalPrediction(uint32_t predictionId) {
    auto it = std::find_if(m_predictions.begin(), m_predictions.end(),
        [predictionId](const LocalPrediction& p) {
            return p.predictionId == predictionId;
        });
        
    if (it == m_predictions.end()) {
        return std::unexpected(StateManagerError::PredictionNotFound);
    }
    
    m_predictions.erase(it);
    UpdateEffectiveMask();
    return {};
}

void ActorTcpStateManager::CleanupStalePredictions(uint64_t currentTime, uint64_t timeoutMs) {
    bool changed = false;
    auto it = m_predictions.begin();
    while (it != m_predictions.end()) {
        if (currentTime >= it->timestamp + timeoutMs) {
            it = m_predictions.erase(it);
            changed = true;
        } else {
            ++it;
        }
    }
    
    if (changed) {
        UpdateEffectiveMask();
    }
}

std::optional<uint32_t> ActorTcpStateManager::GetStateAtTimestamp(uint64_t timestamp) const {
    if (m_historyBuffer.empty()) {
        return std::nullopt;
    }
    
    if (timestamp >= m_historyBuffer.back().timestamp) {
        return m_historyBuffer.back().stateMask;
    }
    
    if (timestamp < m_historyBuffer.front().timestamp) {
        return m_historyBuffer.front().stateMask;
    }
    
    auto it = std::upper_bound(m_historyBuffer.begin(), m_historyBuffer.end(), timestamp,
        [](uint64_t ts, const StateSnapshot& snap) {
            return ts < snap.timestamp;
        });
        
    if (it != m_historyBuffer.begin()) {
        --it;
    }
    
    return it->stateMask;
}

bool ActorTcpStateManager::HadStateInWindow(uint64_t startTime, uint64_t endTime, uint32_t stateBits) const {
    if (m_historyBuffer.empty()) {
        return false;
    }

    if (endTime < m_historyBuffer.front().timestamp) {
        return (m_historyBuffer.front().stateMask & stateBits) == stateBits;
    }

    if (startTime > m_historyBuffer.back().timestamp) {
        return (m_historyBuffer.back().stateMask & stateBits) == stateBits;
    }

    auto startIt = std::lower_bound(m_historyBuffer.begin(), m_historyBuffer.end(), startTime,
        [](const StateSnapshot& snap, uint64_t ts) {
            return snap.timestamp < ts;
        });

    if (startIt != m_historyBuffer.begin()) {
        if (startIt == m_historyBuffer.end() || startIt->timestamp > startTime) {
            --startIt;
        }
    }

    for (auto it = startIt; it != m_historyBuffer.end() && it->timestamp <= endTime; ++it) {
        if ((it->stateMask & stateBits) == stateBits) {
            return true;
        }
    }
    
    return false;
}

} // namespace Client::Actors
