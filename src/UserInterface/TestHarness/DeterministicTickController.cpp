#include "DeterministicTickController.h"
#include <windows.h>
#include <cstring>
#include <algorithm>
#include <chrono>

namespace UserInterface::TestHarness
{
    DeterministicTickController& DeterministicTickController::Instance() noexcept
    {
        static DeterministicTickController s_instance;
        return s_instance;
    }

    DeterministicTickController::DeterministicTickController() noexcept
    {
        LARGE_INTEGER freq{};
        if (QueryPerformanceFrequency(&freq))
        {
            m_qpcFreq = freq.QuadPart;
        }

        m_hHighResTimer = CreateWaitableTimerExW(
            nullptr,
            nullptr,
            CREATE_WAITABLE_TIMER_HIGH_RESOLUTION,
            TIMER_ALL_ACCESS
        );
        if (!m_hHighResTimer)
        {
            m_hHighResTimer = CreateWaitableTimerW(nullptr, FALSE, nullptr);
        }
    }

    DeterministicTickController::~DeterministicTickController() noexcept
    {
        if (m_hHighResTimer)
        {
            CloseHandle(static_cast<HANDLE>(m_hHighResTimer));
            m_hHighResTimer = nullptr;
        }
    }

    void DeterministicTickController::InitFromCommandLine(const char* lpCmdLine) noexcept
    {
        if (!lpCmdLine)
            return;

        if (std::strstr(lpCmdLine, "--test-mode") != nullptr)
        {
            m_isTestMode.store(true, std::memory_order_release);
        }

        if (std::strstr(lpCmdLine, "--freeze-on-start") != nullptr)
        {
            m_isTestMode.store(true, std::memory_order_release);
            m_isFrozen.store(true, std::memory_order_release);
        }
    }

    void DeterministicTickController::SetFrozen(bool frozen) noexcept
    {
        if (frozen)
        {
            m_isTestMode.store(true, std::memory_order_release);
        }
        m_isFrozen.store(frozen, std::memory_order_release);
        WakeMainThread();
    }

    uint64_t DeterministicTickController::Step(uint32_t count, float deltaTime) noexcept
    {
        if (count == 0)
            return m_currentTick.load(std::memory_order_acquire);

        const float clampedDt = (std::max)(0.0001f, deltaTime);
        m_stepDeltaTime.store(clampedDt, std::memory_order_release);

        uint64_t targetTick = 0;
        {
            std::lock_guard<std::mutex> lock(m_syncMutex);
            const uint32_t prevPending = m_pendingSteps.fetch_add(count, std::memory_order_acq_rel);
            targetTick = m_currentTick.load(std::memory_order_acquire) + prevPending + count;
        }

        m_wakeMainCv.notify_all();
        return targetTick;
    }

    bool DeterministicTickController::WaitForTickReached(uint64_t targetTick, uint32_t timeoutMs) noexcept
    {
        if (m_currentTick.load(std::memory_order_acquire) >= targetTick)
            return true;

        std::unique_lock<std::mutex> lock(m_syncMutex);
        return m_tickDoneCv.wait_for(
            lock,
            std::chrono::milliseconds(timeoutMs),
            [this, targetTick]() noexcept {
                return m_currentTick.load(std::memory_order_acquire) >= targetTick;
            }
        );
    }

    uint32_t DeterministicTickController::ConsumePendingSteps() noexcept
    {
        return m_pendingSteps.exchange(0, std::memory_order_acq_rel);
    }

    void DeterministicTickController::NotifyTickCompleted(float stepDeltaSec) noexcept
    {
        const uint64_t deltaUs = static_cast<uint64_t>((std::max)(0.0f, stepDeltaSec) * 1'000'000.0f + 0.5f);
        m_logicalTimeUs.fetch_add(deltaUs, std::memory_order_acq_rel);
        m_currentTick.fetch_add(1, std::memory_order_acq_rel);

        std::lock_guard<std::mutex> lock(m_syncMutex);
        m_tickDoneCv.notify_all();
    }

    void DeterministicTickController::WakeMainThread() noexcept
    {
        std::lock_guard<std::mutex> lock(m_syncMutex);
        m_wakeMainCv.notify_all();
    }

    void DeterministicTickController::ThrottleFrameLoop(bool isHeadless, uint32_t targetFps) noexcept
    {
        if (m_isFrozen.load(std::memory_order_acquire))
        {
            if (m_pendingSteps.load(std::memory_order_acquire) > 0)
                return;

            std::unique_lock<std::mutex> lock(m_syncMutex);
            m_wakeMainCv.wait_for(lock, std::chrono::milliseconds(16), [this]() noexcept {
                return !m_isFrozen.load(std::memory_order_acquire) ||
                       m_pendingSteps.load(std::memory_order_acquire) > 0;
            });
            return;
        }

        if (!isHeadless || targetFps == 0 || m_qpcFreq <= 0)
            return;

        const int64_t frameIntervalQpc = m_qpcFreq / targetFps;
        LARGE_INTEGER now{};
        QueryPerformanceCounter(&now);

        if (m_nextFrameQpc == 0 || now.QuadPart > m_nextFrameQpc + frameIntervalQpc * 4)
        {
            m_nextFrameQpc = now.QuadPart + frameIntervalQpc;
            return;
        }

        const int64_t remainingQpc = m_nextFrameQpc - now.QuadPart;
        if (remainingQpc > 0)
        {
            const int64_t wait100ns = -((remainingQpc * 10'000'000LL) / m_qpcFreq);
            if (wait100ns < -2000LL && m_hHighResTimer)
            {
                LARGE_INTEGER dueTime{};
                dueTime.QuadPart = wait100ns;
                if (SetWaitableTimer(static_cast<HANDLE>(m_hHighResTimer), &dueTime, 0, nullptr, nullptr, FALSE))
                {
                    WaitForSingleObject(static_cast<HANDLE>(m_hHighResTimer), 20);
                }
            }
        }

        m_nextFrameQpc += frameIntervalQpc;
    }

    void DeterministicTickController::Reset() noexcept
    {
        {
            std::lock_guard<std::mutex> lock(m_syncMutex);
            m_isTestMode.store(false, std::memory_order_release);
            m_isFrozen.store(false, std::memory_order_release);
            m_pendingSteps.store(0, std::memory_order_release);
            m_stepDeltaTime.store(1.0f / 60.0f, std::memory_order_release);
            m_currentTick.store(0, std::memory_order_release);
            m_logicalTimeUs.store(0, std::memory_order_release);
            m_nextFrameQpc = 0;
        }
        m_wakeMainCv.notify_all();
        m_tickDoneCv.notify_all();
    }
}
