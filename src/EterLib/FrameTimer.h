#pragma once

/**
 * @file FrameTimer.h
 * @brief Nowoczesny akumulator klatek C++23 (Fixed Timestep 60Hz + Interpolacja 144Hz/240Hz).
 * 
 * Zapewnia:
 * 1. Sztywny krok czasowy logiki gry (60 tickow/s) - pelna synchronizacja z serwerem i fizyka.
 * 2. Nielimitowany klatkaz renderera (144Hz, 240Hz, 360Hz) dzieki wspolczynnikowi interpolacji alpha.
 * 3. Ochrona przed "spirala smierci" (Accumulator Clamping).
 */

#include <chrono>
#include <cstdint>
#include <algorithm>
#include <cmath>
#include "../EterBase/Timer.h"

class FrameTimer
{
public:
    static FrameTimer& Instance() noexcept
    {
        static FrameTimer s_instance;
        return s_instance;
    }

    explicit FrameTimer(double fixedStep = 1.0 / 60.0, double maxAccumulator = 0.10) noexcept
        : m_fixedStep(fixedStep)
        , m_maxAccumulator(maxAccumulator)
        , m_accumulator(0.0)
        , m_interpolationAlpha(0.0f)
        , m_deltaTime(0.0)
        , m_rawDeltaTime(0.0)
        , m_fps(60.0f)
        , m_frameCount(0)
        , m_fpsTimer(0.0)
        , m_isFirstTick(true)
    {
        m_lastTime = std::chrono::steady_clock::now();
    }

    void Reset() noexcept
    {
        m_lastTime = std::chrono::steady_clock::now();
        m_accumulator = 0.0;
        m_interpolationAlpha = 0.0f;
        m_deltaTime = 0.0;
        m_rawDeltaTime = 0.0;
        m_isFirstTick = true;
    }

    /**
     * @brief Aktualizacja timera.
     * @tparam FixedUpdateFn Wolalna funkcja void(float fixedDeltaTime)
     * @param fixedUpdateFunc Callback wywolywany sztywno w 60Hz
     */
    template <typename FixedUpdateFn>
    void Tick(FixedUpdateFn&& fixedUpdateFunc)
    {
        const auto now = std::chrono::steady_clock::now();
        if (m_isFirstTick)
        {
            m_lastTime = now;
            m_isFirstTick = false;
            m_deltaTime = 0.0;
            m_rawDeltaTime = 0.0;
            return;
        }

        const std::chrono::duration<double> elapsed = now - m_lastTime;
        m_lastTime = now;

        m_rawDeltaTime = elapsed.count();
        m_deltaTime = std::clamp(m_rawDeltaTime, 0.0, m_maxAccumulator);
        m_accumulator = std::min(m_accumulator + m_deltaTime, m_maxAccumulator);

        // Sztywny krok logiki gry (60Hz)
        const float fixedStepF = static_cast<float>(m_fixedStep);
        while (m_accumulator >= m_fixedStep)
        {
            CTimer::Instance().AdvanceFixed(m_fixedStep);
            fixedUpdateFunc(fixedStepF);
            m_accumulator -= m_fixedStep;
        }

        // Wspolczynnik wyrownania renderera pomiedzy klatkami logiki [0.0f, 1.0f]
        m_interpolationAlpha = static_cast<float>(
            std::clamp(m_accumulator / m_fixedStep, 0.0, 1.0)
        );

        // Obliczanie FPS
        ++m_frameCount;
        m_fpsTimer += m_rawDeltaTime;
        if (m_fpsTimer >= 1.0)
        {
            m_fps = static_cast<float>(static_cast<double>(m_frameCount) / m_fpsTimer);
            m_frameCount = 0;
            m_fpsTimer = std::fmod(m_fpsTimer, 1.0);
        }
    }

    void TickManual() noexcept
    {
        const auto now = std::chrono::steady_clock::now();
        if (m_isFirstTick)
        {
            m_lastTime = now;
            m_isFirstTick = false;
            return;
        }
        m_rawDeltaTime = std::chrono::duration<double>(now - m_lastTime).count();
        m_lastTime = now;
        m_deltaTime = std::clamp(m_rawDeltaTime, 0.0, m_maxAccumulator);
        m_accumulator = std::min(m_accumulator + m_deltaTime, m_maxAccumulator);
    }

    [[nodiscard]] bool ConsumeFixedStep() noexcept
    {
        if (m_accumulator >= m_fixedStep)
        {
            CTimer::Instance().AdvanceFixed(m_fixedStep);
            m_accumulator -= m_fixedStep;
            m_interpolationAlpha = static_cast<float>(std::clamp(m_accumulator / m_fixedStep, 0.0, 1.0));
            return true;
        }
        m_interpolationAlpha = static_cast<float>(std::clamp(m_accumulator / m_fixedStep, 0.0, 1.0));
        return false;
    }

    [[nodiscard]] float GetInterpolationAlpha() const noexcept { return m_interpolationAlpha; }
    [[nodiscard]] float GetFixedStep() const noexcept { return static_cast<float>(m_fixedStep); }
    [[nodiscard]] float GetDeltaTime() const noexcept { return static_cast<float>(m_deltaTime); }
    [[nodiscard]] double GetDeltaTimeSec() const noexcept { return m_deltaTime; }
    [[nodiscard]] double GetRawDeltaTimeSec() const noexcept { return m_rawDeltaTime; }
    [[nodiscard]] float GetFPS() const noexcept { return m_fps; }

    void SetFixedStep(double step) noexcept { m_fixedStep = step; }
    void SetMaxAccumulator(double maxAcc) noexcept { m_maxAccumulator = maxAcc; }

private:
    std::chrono::steady_clock::time_point m_lastTime;
    double m_fixedStep;
    double m_maxAccumulator;
    double m_accumulator;
    float m_interpolationAlpha;
    double m_deltaTime;
    double m_rawDeltaTime;
    float m_fps;
    uint32_t m_frameCount;
    double m_fpsTimer;
    bool m_isFirstTick;
};
