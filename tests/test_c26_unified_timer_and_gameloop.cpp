#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include "EterBase/Timer.h"
#include "EterLib/FrameTimer.h"
#include <thread>
#include <chrono>

TEST_CASE("T179: Unified steady_clock Timer Monotonicity and Precision")
{
    ELTimer_Init();

    const DWORD msStart = ELTimer_GetMSec();
    const double secStart = ELTimer_GetHighResSec();

    CHECK(secStart >= 0.0);
    CHECK(msStart >= 0);

    // Short sleep to ensure advancement
    std::this_thread::sleep_for(std::chrono::milliseconds(5));

    const DWORD msAfter = ELTimer_GetMSec();
    const double secAfter = ELTimer_GetHighResSec();

    CHECK(msAfter >= msStart);
    CHECK(secAfter >= secStart);
    CHECK((secAfter - secStart) >= 0.003); // at least 3ms advanced
}

TEST_CASE("T179: CTimer Frame Latching Eliminates In-Frame Jitter")
{
    CTimer timer;
    timer.Advance();

    const float fTime1 = timer.GetCurrentSecond();
    const DWORD dwTime1 = timer.GetCurrentMillisecond();

    // In-frame sleep: time must NOT drift inside the frame
    std::this_thread::sleep_for(std::chrono::milliseconds(5));

    const float fTime2 = timer.GetCurrentSecond();
    const DWORD dwTime2 = timer.GetCurrentMillisecond();

    CHECK(fTime1 == fTime2);
    CHECK(dwTime1 == dwTime2);

    // Once Advance is called, frame time moves forward
    timer.Advance();
    const float fTime3 = timer.GetCurrentSecond();
    CHECK(fTime3 >= fTime1);
}

TEST_CASE("T179: CTimer AdvanceFixed Deterministic Stepping")
{
    CTimer timer;
    timer.SetBaseTime();

    constexpr double STEP_60HZ = 1.0 / 60.0;
    timer.AdvanceFixed(STEP_60HZ);
    CHECK(doctest::Approx(timer.GetCurrentSecond()).epsilon(0.001) == static_cast<float>(STEP_60HZ));
    CHECK(timer.GetElapsedMilliecond() == 16);

    timer.AdvanceFixed(STEP_60HZ);
    CHECK(doctest::Approx(timer.GetCurrentSecond()).epsilon(0.001) == static_cast<float>(STEP_60HZ * 2.0));
}

TEST_CASE("T177/T179: FrameTimer Clamping and Alpha Interpolation")
{
    FrameTimer frameTimer;
    frameTimer.Reset();

    uint32_t tickCount = 0;
    auto mockUpdate = [&](float fixedStep) {
        CHECK(fixedStep > 0.016f);
        CHECK(fixedStep < 0.017f);
        ++tickCount;
    };

    frameTimer.Tick(mockUpdate);
    const float alpha = frameTimer.GetInterpolationAlpha();
    CHECK(alpha >= 0.0f);
    CHECK(alpha <= 1.0f);

    // Delta time clamped to max accumulator (0.10s = max 6 ticks)
    CHECK(frameTimer.GetDeltaTime() <= 0.10f);
}
