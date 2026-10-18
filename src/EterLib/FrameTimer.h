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

class FrameTimer
{
public:
    static FrameTimer& Instance() noexcept
    {
        static FrameTimer s_instance;
        return s_instance;
    }

    FrameTimer() noexcept
        : m_fixedStep(1.0 / 60.0)
        , m_accumulator(0.0)
        , m_interpolationAlpha(0.0f)
        , m_deltaTime(0.0f)
        , m_fps(60.0f)
        , m_frameCount(0)
        , m_fpsTimer(0.0)
    {
        m_lastTime = std::chrono::steady_clock::now();
    }

    void Reset() noexcept
    {
        m_lastTime = std::chrono::steady_clock::now();
        m_accumulator = 0.0;
        m_interpolationAlpha = 0.0f;
        m_deltaTime = 0.0f;
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
        const std::chrono::duration<double> elapsed = now - m_lastTime;
        m_lastTime = now;

        // Clamping to 0.1s (max 6 logic ticks) prevents spiral of death & micro-teleportation
        constexpr double MAX_ACCUMULATOR = 0.10;
        const double rawDelta = (std::max)(0.0, elapsed.count());
        const double clampedDelta = (std::min)(rawDelta, MAX_ACCUMULATOR);
        m_deltaTime = static_cast<float>(clampedDelta);

        m_accumulator += clampedDelta;

        // Sztywny krok logiki gry (60Hz)
        const float fixedStepF = static_cast<float>(m_fixedStep);
        while (m_accumulator >= m_fixedStep)
        {
            fixedUpdateFunc(fixedStepF);
            m_accumulator -= m_fixedStep;
        }

        // Wspolczynnik wyrownania renderera pomiedzy klatkami logiki [0.0f, 1.0f]
        m_interpolationAlpha = static_cast<float>(m_accumulator / m_fixedStep);

        // Obliczanie FPS
        ++m_frameCount;
        m_fpsTimer += clampedDelta;
        if (m_fpsTimer >= 1.0)
        {
            m_fps = static_cast<float>(static_cast<double>(m_frameCount) / m_fpsTimer);
            m_frameCount = 0;
            m_fpsTimer = 0.0;
        }
    }

    [[nodiscard]] float GetInterpolationAlpha() const noexcept { return m_interpolationAlpha; }
    [[nodiscard]] float GetFixedStep() const noexcept { return static_cast<float>(m_fixedStep); }
    [[nodiscard]] float GetDeltaTime() const noexcept { return m_deltaTime; }
    [[nodiscard]] float GetFPS() const noexcept { return m_fps; }

    void SetFixedStep(double step) noexcept { m_fixedStep = step; }

private:
    std::chrono::steady_clock::time_point m_lastTime;
    double m_fixedStep;
    double m_accumulator;
    float m_interpolationAlpha;
    float m_deltaTime;
    float m_fps;
    uint32_t m_frameCount;
    double m_fpsTimer;
};
