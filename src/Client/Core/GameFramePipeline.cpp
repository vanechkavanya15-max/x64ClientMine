#include "GameFramePipeline.h"
#include <format>
#include <cstring>
#include <algorithm>

namespace Client::Core
{
    GameFramePipeline& GameFramePipeline::Instance() noexcept
    {
        static GameFramePipeline s_instance;
        return s_instance;
    }

    void GameFramePipeline::BeginPhase(FramePhase phase) noexcept
    {
        const auto idx = static_cast<size_t>(phase);
        if (idx < 6)
        {
            m_phaseStartTimes[idx] = std::chrono::steady_clock::now();
        }
    }

    void GameFramePipeline::EndPhase(FramePhase phase) noexcept
    {
        const auto idx = static_cast<size_t>(phase);
        if (idx < 6)
        {
            const auto endTime = std::chrono::steady_clock::now();
            const auto durationUs = std::chrono::duration_cast<std::chrono::microseconds>(endTime - m_phaseStartTimes[idx]).count();
            m_phaseDurationsUs[idx].store(static_cast<uint32_t>(durationUs), std::memory_order_relaxed);
        }
    }

    void GameFramePipeline::RecordFrameComplete(uint64_t frameIndex, uint64_t tick, float deltaSec) noexcept
    {
        m_frameCounter.store(frameIndex, std::memory_order_relaxed);
        m_tickCounter.store(tick, std::memory_order_relaxed);
        m_lastDeltaSec.store(deltaSec, std::memory_order_relaxed);
    }

    void GameFramePipeline::UpdateWorldMetrics(uint32_t aliveActors, std::string_view netPhase, std::string_view driver) noexcept
    {
        m_aliveActorsCount.store(aliveActors, std::memory_order_relaxed);

        if (!netPhase.empty())
        {
            size_t len = (std::min)(netPhase.size(), sizeof(m_networkPhaseName) - 1);
            std::memcpy(m_networkPhaseName, netPhase.data(), len);
            m_networkPhaseName[len] = '\0';
        }

        if (!driver.empty())
        {
            size_t len = (std::min)(driver.size(), sizeof(m_activeProtocolDriver) - 1);
            std::memcpy(m_activeProtocolDriver, driver.data(), len);
            m_activeProtocolDriver[len] = '\0';
        }
    }

    void GameFramePipeline::SetEngineModeFlags(bool testMode, bool frozen, bool headless) noexcept
    {
        m_isTestMode.store(testMode, std::memory_order_relaxed);
        m_isFrozen.store(frozen, std::memory_order_relaxed);
        m_isHeadless.store(headless, std::memory_order_relaxed);
    }

    FrameTelemetryData GameFramePipeline::GetTelemetry() const noexcept
    {
        FrameTelemetryData data;
        data.frameIndex = m_frameCounter.load(std::memory_order_relaxed);
        data.currentTick = m_tickCounter.load(std::memory_order_relaxed);
        data.deltaTimeSec = m_lastDeltaSec.load(std::memory_order_relaxed);

        data.ipcDurationUs = m_phaseDurationsUs[static_cast<size_t>(FramePhase::IpcIngest)].load(std::memory_order_relaxed);
        data.inputDurationUs = m_phaseDurationsUs[static_cast<size_t>(FramePhase::InputUpdate)].load(std::memory_order_relaxed);
        data.networkDurationUs = m_phaseDurationsUs[static_cast<size_t>(FramePhase::NetworkPump)].load(std::memory_order_relaxed);
        data.simulationDurationUs = m_phaseDurationsUs[static_cast<size_t>(FramePhase::WorldSimulation)].load(std::memory_order_relaxed);
        data.uiDurationUs = m_phaseDurationsUs[static_cast<size_t>(FramePhase::UiSynchronization)].load(std::memory_order_relaxed);
        data.totalFrameDurationUs = data.ipcDurationUs + data.inputDurationUs + data.networkDurationUs + data.simulationDurationUs + data.uiDurationUs;

        data.isTestMode = m_isTestMode.load(std::memory_order_relaxed);
        data.isFrozen = m_isFrozen.load(std::memory_order_relaxed);
        data.isHeadless = m_isHeadless.load(std::memory_order_relaxed);

        data.aliveActorsCount = m_aliveActorsCount.load(std::memory_order_relaxed);
        data.networkPhaseName = m_networkPhaseName;
        data.activeProtocolDriver = m_activeProtocolDriver;

        return data;
    }

    std::string GameFramePipeline::GetTelemetryJson() const
    {
        const auto t = GetTelemetry();

        return std::format(
            "{{\n"
            "  \"ok\": true,\n"
            "  \"frame_index\": {},\n"
            "  \"current_tick\": {},\n"
            "  \"delta_time_ms\": {:.2f},\n"
            "  \"timing_us\": {{\n"
            "    \"ipc\": {},\n"
            "    \"input\": {},\n"
            "    \"network\": {},\n"
            "    \"simulation\": {},\n"
            "    \"ui\": {},\n"
            "    \"total\": {}\n"
            "  }},\n"
            "  \"engine_flags\": {{\n"
            "    \"test_mode\": {},\n"
            "    \"frozen\": {},\n"
            "    \"headless\": {}\n"
            "  }},\n"
            "  \"world_summary\": {{\n"
            "    \"alive_actors\": {},\n"
            "    \"network_phase\": \"{}\",\n"
            "    \"protocol_driver\": \"{}\"\n"
            "  }}\n"
            "}}",
            t.frameIndex,
            t.currentTick,
            t.deltaTimeSec * 1000.0f,
            t.ipcDurationUs,
            t.inputDurationUs,
            t.networkDurationUs,
            t.simulationDurationUs,
            t.uiDurationUs,
            t.totalFrameDurationUs,
            t.isTestMode ? "true" : "false",
            t.isFrozen ? "true" : "false",
            t.isHeadless ? "true" : "false",
            t.aliveActorsCount,
            t.networkPhaseName,
            t.activeProtocolDriver
        );
    }

    void GameFramePipeline::Reset() noexcept
    {
        m_frameCounter.store(0, std::memory_order_relaxed);
        m_tickCounter.store(0, std::memory_order_relaxed);
        m_lastDeltaSec.store(0.0166f, std::memory_order_relaxed);

        for (auto& duration : m_phaseDurationsUs)
        {
            duration.store(0, std::memory_order_relaxed);
        }
    }

} // namespace Client::Core
