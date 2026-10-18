#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include <cmath>
#include <algorithm>
#include <cstdint>

struct AttackEventSyncModel {
    float fMotionDuration{1.0f};      // Dlugosc animacji bazowej w sekundach
    float fHitTimeRatio{0.35f};       // Punkt uderzenia miecza (35% animacji)
    float fAttackSpeedRatio{1.0f};    // Mnoznik predkosci ataku gracza
    float fInvisibleTime{0.5f};       // Czas niewrazliwosci ofiary na powtorny cios

    [[nodiscard]] float GetScaledDuration() const noexcept {
        return fMotionDuration / std::max(0.1f, fAttackSpeedRatio);
    }

    [[nodiscard]] float CalculateHitTimestamp(float fStartTime) const noexcept {
        return fStartTime + (fMotionDuration * fHitTimeRatio) / std::max(0.1f, fAttackSpeedRatio);
    }

    [[nodiscard]] bool IsHitTimeReached(float fStartTime, float fCurrentTime) const noexcept {
        return fCurrentTime >= CalculateHitTimestamp(fStartTime);
    }

    [[nodiscard]] float CalculateShamanMountAdjustment(uint32_t race, uint32_t skillId) const noexcept {
        if ((race != 3 && race != 7) || skillId != 0 || fAttackSpeedRatio < 1.3f) {
            return 0.0f;
        }
        float scale = (fAttackSpeedRatio - 1.3f) / 1.3f;
        float inv = fInvisibleTime * 0.5f;
        return inv * scale;
    }
};

TEST_CASE("AttackHitSync - Scaled Hit Timestamp Calculation")
{
    AttackEventSyncModel model;
    model.fMotionDuration = 1.2f;
    model.fHitTimeRatio = 0.4f; // 40% wglab animacji nastepuje uderzenie
    model.fAttackSpeedRatio = 1.0f;

    float startTime = 100.0f;
    float hitTime = model.CalculateHitTimestamp(startTime);

    // 100.0 + 1.2 * 0.4 = 100.48s
    CHECK(hitTime == doctest::Approx(100.48f));
    CHECK_FALSE(model.IsHitTimeReached(startTime, 100.40f));
    CHECK(model.IsHitTimeReached(startTime, 100.48f));
    CHECK(model.IsHitTimeReached(startTime, 100.50f));
}

TEST_CASE("AttackHitSync - High Attack Speed Acceleration (No Frame Lag)")
{
    AttackEventSyncModel model;
    model.fMotionDuration = 1.0f;
    model.fHitTimeRatio = 0.5f;
    model.fAttackSpeedRatio = 2.0f; // 200% szybkosci ataku (Zielona potka + berek)

    float startTime = 10.0f;
    float hitTime = model.CalculateHitTimestamp(startTime);

    // 10.0 + (1.0 * 0.5) / 2.0 = 10.25s
    CHECK(hitTime == doctest::Approx(10.25f));
    CHECK(model.GetScaledDuration() == doctest::Approx(0.5f));
}

TEST_CASE("AttackHitSync - Shaman Mount Hitting Adjustment (MR-3)")
{
    AttackEventSyncModel model;
    model.fInvisibleTime = 0.6f;

    // Wojownik (race 0) - brak korekty
    model.fAttackSpeedRatio = 1.6f;
    CHECK(model.CalculateShamanMountAdjustment(0, 0) == doctest::Approx(0.0f));

    // Szamanka (race 3) ponizej progu 1.3 - brak korekty
    model.fAttackSpeedRatio = 1.2f;
    CHECK(model.CalculateShamanMountAdjustment(3, 0) == doctest::Approx(0.0f));

    // Szamanka (race 3) przy szybkosci 1.69 (scale = 0.39 / 1.3 = 0.3)
    model.fAttackSpeedRatio = 1.69f;
    float expectedAdj = (0.6f * 0.5f) * ((1.69f - 1.3f) / 1.3f);
    CHECK(model.CalculateShamanMountAdjustment(3, 0) == doctest::Approx(expectedAdj));

    // Szaman mezczyzna (race 7)
    CHECK(model.CalculateShamanMountAdjustment(7, 0) == doctest::Approx(expectedAdj));
}
