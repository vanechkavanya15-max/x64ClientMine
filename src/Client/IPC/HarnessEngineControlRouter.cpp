#include "HarnessEngineControlRouter.h"

namespace Client::IPC {

HarnessEngineControlRouter::HarnessEngineControlRouter(FuzzerConfig fuzzerCfg)
    : m_fuzzerConfig(fuzzerCfg),
      m_rng(fuzzerCfg.seed)
{
}

std::expected<void, HarnessEngineControlRouter::ErrorCode> HarnessEngineControlRouter::SetWindowMode(WindowMode mode) noexcept {
    if (m_state == EngineState::Crashed) {
        return std::unexpected(ErrorCode::AlreadyCrashed);
    }
    m_windowMode = mode;
    return {};
}

std::expected<void, HarnessEngineControlRouter::ErrorCode> HarnessEngineControlRouter::SetProtocolDriver(std::string_view driver) {
    if (m_state == EngineState::Crashed) {
        return std::unexpected(ErrorCode::AlreadyCrashed);
    }
    if (driver.empty()) {
        return std::unexpected(ErrorCode::InvalidDriver);
    }
    m_protocolDriver = std::string(driver);
    return {};
}

std::expected<void, HarnessEngineControlRouter::ErrorCode> HarnessEngineControlRouter::TransitionState(EngineState newState) {
    if (m_state == EngineState::Crashed && newState != EngineState::Initializing) {
        // Only allow transition from Crashed to Initializing (reset)
        return std::unexpected(ErrorCode::InvalidTransition);
    }

    if (m_state == EngineState::Initializing && newState == EngineState::Paused) {
        // Disallow direct jump from Initializing to Paused
        return std::unexpected(ErrorCode::InvalidTransition);
    }

    m_state = newState;
    return {};
}

void HarnessEngineControlRouter::TickFrame(float dt) {
    if (m_state == EngineState::Crashed || m_state == EngineState::Paused) {
        return;
    }

    m_elapsedTime += dt;
    m_frameCount++;

    if (m_fuzzerConfig.enabled) {
        if (RollChance(m_fuzzerConfig.crashRate)) {
            ForceCrash(CrashReason::FuzzerInduced, "Fuzzer injected random crash during TickFrame");
            return;
        }
    }

    ProcessPackets();
}

std::expected<void, HarnessEngineControlRouter::ErrorCode> HarnessEngineControlRouter::InjectPacket(Packet packet) {
    if (m_state == EngineState::Crashed) {
        return std::unexpected(ErrorCode::AlreadyCrashed);
    }
    
    if (m_packetQueue.size() >= MAX_QUEUE_SIZE) {
        return std::unexpected(ErrorCode::QueueFull);
    }

    if (m_fuzzerConfig.enabled) {
        if (RollChance(m_fuzzerConfig.dropRate)) {
            m_packetsDropped++;
            return {}; // Simulate success from sender's perspective, but packet is silently dropped
        }

        bool shouldDuplicate = RollChance(m_fuzzerConfig.duplicateRate);
        
        ApplyFuzzerEffects(packet);
        m_packetQueue.push_back(packet);

        if (shouldDuplicate && m_packetQueue.size() < MAX_QUEUE_SIZE) {
            m_packetQueue.push_back(packet);
        }
    } else {
        m_packetQueue.push_back(std::move(packet));
    }

    return {};
}

void HarnessEngineControlRouter::ProcessPackets() {
    // Process all pending packets in the queue for this frame
    while (!m_packetQueue.empty()) {
        if (m_state == EngineState::Crashed) {
            break;
        }
        
        const Packet& packet = m_packetQueue.front();
        
        // Mock processing logic
        if (packet.corrupted) {
            // For safety critical packets, this might trigger a crash
            // but for typical IPC we might just increment the counter.
            m_packetsCorrupted++;
        }
        
        m_packetsProcessed++;
        m_packetQueue.pop_front();
    }
}

void HarnessEngineControlRouter::ForceCrash(CrashReason reason, std::string_view context) {
    if (m_state == EngineState::Crashed) {
        return;
    }
    m_state = EngineState::Crashed;
    m_crashReason = reason;
    m_crashContext = std::string(context);
}

std::optional<CrashReport> HarnessEngineControlRouter::GetCrashReport() const {
    if (m_state != EngineState::Crashed) {
        return std::nullopt;
    }

    CrashReport report{
        .lastState = m_state,
        .reason = m_crashReason,
        .frameCount = m_frameCount,
        .upTime = m_elapsedTime,
        .packetsProcessed = m_packetsProcessed,
        .packetsDropped = m_packetsDropped,
        .packetsCorrupted = m_packetsCorrupted,
        .contextDump = std::format("Crash occurred at frame {}. Context: {}", m_frameCount, m_crashContext)
    };

    return report;
}

void HarnessEngineControlRouter::Reset(EngineState initialState) {
    m_state = initialState;
    m_crashReason = CrashReason::None;
    m_crashContext.clear();
    m_elapsedTime = 0.0f;
    m_frameCount = 0;
    m_packetsProcessed = 0;
    m_packetsDropped = 0;
    m_packetsCorrupted = 0;
    m_packetQueue.clear();
}

void HarnessEngineControlRouter::UpdateFuzzerConfig(const FuzzerConfig& cfg) {
    m_fuzzerConfig = cfg;
    m_rng.seed(m_fuzzerConfig.seed); // Deterministic reset when config is updated
}

void HarnessEngineControlRouter::ApplyFuzzerEffects(Packet& packet) {
    if (RollChance(m_fuzzerConfig.corruptRate) && !packet.payload.empty()) {
        packet.corrupted = true;
        // Flip a random bit in a random byte
        std::uniform_int_distribution<size_t> byteDist(0, packet.payload.size() - 1);
        std::uniform_int_distribution<int> bitDist(0, 7);
        
        size_t byteIdx = byteDist(m_rng);
        int bitIdx = bitDist(m_rng);
        
        packet.payload[byteIdx] ^= (1 << bitIdx);
    }
}

bool HarnessEngineControlRouter::RollChance(float probability) {
    if (probability <= 0.0f) return false;
    if (probability >= 1.0f) return true;
    
    std::uniform_real_distribution<float> dist(0.0f, 1.0f);
    return dist(m_rng) < probability;
}

} // namespace Client::IPC
