#pragma once

#include <atomic>
#include <cstdint>
#include <mutex>
#include <condition_variable>
#include <string_view>

namespace UserInterface::TestHarness
{
    /**
     * @brief Deterministyczny kontroler klatek i taktu logiki gry pod MCP i testy automatyczne.
     */
    class DeterministicTickController
    {
    public:
        static DeterministicTickController& Instance() noexcept;

        DeterministicTickController(const DeterministicTickController&) = delete;
        DeterministicTickController& operator=(const DeterministicTickController&) = delete;

        void InitFromCommandLine(const char* lpCmdLine) noexcept;
        void Reset() noexcept;

        // Konfiguracja trybow
        void SetTestMode(bool enabled) noexcept { m_isTestMode.store(enabled, std::memory_order_release); }
        [[nodiscard]] bool IsTestMode() const noexcept { return m_isTestMode.load(std::memory_order_acquire); }

        void SetFrozen(bool frozen) noexcept;
        [[nodiscard]] bool IsFrozen() const noexcept { return m_isFrozen.load(std::memory_order_acquire); }

        // Zlecanie krokow symulacji (zwraca docelowy numer ticku po wykonaniu)
        uint64_t Step(uint32_t count = 1, float deltaTime = 1.0f / 60.0f) noexcept;

        // Synchroniczne oczekiwanie watku IPC na fizyczne wykonanie zleconych krokow przez watek glowny
        bool WaitForTickReached(uint64_t targetTick, uint32_t timeoutMs) noexcept;

        // Wywolywane w glownym watku gry (CPythonApplication::Process)
        [[nodiscard]] uint32_t ConsumePendingSteps() noexcept;
        void NotifyTickCompleted(float stepDeltaSec) noexcept;
        void IncrementTickCount() noexcept { m_currentTick.fetch_add(1, std::memory_order_acq_rel); }

        // Wybudzenie watku glownego (np. gdy przyszlo zadanie IPC w trybie Frozen)
        void WakeMainThread() noexcept;

        // Hybrydowy dlawik dla trybu Frozen (0.0% CPU) oraz Headless 60Hz (<0.2% CPU)
        void ThrottleFrameLoop(bool isHeadless, uint32_t targetFps = 60) noexcept;

        // Deterministyczny zegar wirtualny (zastepuje GetTickCount64 / ELTimer_GetMSec w testach)
        [[nodiscard]] float GetStepDeltaTime() const noexcept { return m_stepDeltaTime.load(std::memory_order_acquire); }
        [[nodiscard]] uint64_t GetCurrentTick() const noexcept { return m_currentTick.load(std::memory_order_acquire); }
        [[nodiscard]] uint64_t GetLogicalTimeMs() const noexcept { return m_logicalTimeUs.load(std::memory_order_acquire) / 1000ULL; }
        [[nodiscard]] uint64_t GetLogicalTimeUs() const noexcept { return m_logicalTimeUs.load(std::memory_order_acquire); }
        [[nodiscard]] double GetLogicalTimeSec() const noexcept { return static_cast<double>(m_logicalTimeUs.load(std::memory_order_acquire)) * 1e-6; }

    private:
        DeterministicTickController() noexcept;
        ~DeterministicTickController() noexcept;

        std::atomic<bool> m_isTestMode{false};
        std::atomic<bool> m_isFrozen{false};
        std::atomic<uint32_t> m_pendingSteps{0};
        std::atomic<float> m_stepDeltaTime{1.0f / 60.0f};
        std::atomic<uint64_t> m_currentTick{0};
        std::atomic<uint64_t> m_logicalTimeUs{0};

        mutable std::mutex m_syncMutex;
        std::condition_variable m_wakeMainCv;
        std::condition_variable m_tickDoneCv;

        void* m_hHighResTimer{nullptr};
        int64_t m_nextFrameQpc{0};
        int64_t m_qpcFreq{0};
    };
}
