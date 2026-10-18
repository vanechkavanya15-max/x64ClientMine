#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include <cmath>
#include <string>
#include <vector>
#include <algorithm>

struct Audio3DListenerModel {
    float posX{0.0f}, posY{0.0f}, posZ{0.0f};
    float forwardX{0.0f}, forwardY{1.0f}, forwardZ{0.0f};
    float upX{0.0f}, upY{0.0f}, upZ{1.0f};

    float masterVolume{1.0f};
    float musicVolume{1.0f};
    float sfxVolume{1.0f};

    void SetPosition(float x, float y, float z) noexcept {
        posX = x; posY = y; posZ = z;
    }

    void SetOrientation(float fx, float fy, float fz, float ux, float uy, float uz) noexcept {
        forwardX = fx; forwardY = fy; forwardZ = fz;
        upX = ux; upY = uy; upZ = uz;
    }

    /// Oblicza wspolczynnik glosnosci 3D w oparciu o odleglosc z odcieciem min/max (HRTF 3D attenuation)
    [[nodiscard]] float Calculate3DAttenuation(float srcX, float srcY, float srcZ, float minDistance, float maxDistance, float rolloff = 1.0f) const noexcept {
        float dx = srcX - posX;
        float dy = srcY - posY;
        float dz = srcZ - posZ;
        float dist = std::sqrt(dx * dx + dy * dy + dz * dz);

        if (dist <= minDistance)
            return 1.0f;
        if (dist >= maxDistance)
            return 0.0f;

        // Linear / logarithmic falloff
        float factor = (maxDistance - dist) / (maxDistance - minDistance);
        return std::clamp(std::pow(factor, rolloff), 0.0f, 1.0f);
    }

    /// Oblicza kat panoramy stereo (Azimuth Pan) od -1.0 (lewy kanal) do +1.0 (prawy kanal)
    [[nodiscard]] float CalculateStereoPan(float srcX, float srcY) const noexcept {
        float dx = srcX - posX;
        float dy = srcY - posY;
        float dist = std::sqrt(dx * dx + dy * dy);
        if (dist < 1e-4f) return 0.0f; // Srodek

        // Iloczyn wektorowy w plaszczyznie z wektorem forward
        // prawy wektor: right = [forwardY, -forwardX]
        float rightX = forwardY;
        float rightY = -forwardX;

        float pan = (dx * rightX + dy * rightY) / dist;
        return std::clamp(pan, -1.0f, 1.0f);
    }
};

struct MusicStreamFadeModel {
    float currentVolume{0.0f};
    float targetVolume{1.0f};
    float fadeDuration{1.5f}; // w sekundach

    void StepFade(float dt) noexcept {
        if (fadeDuration <= 0.0f) {
            currentVolume = targetVolume;
            return;
        }
        float step = (1.0f / fadeDuration) * dt;
        if (targetVolume > currentVolume) {
            currentVolume = std::min(targetVolume, currentVolume + step);
        } else if (targetVolume < currentVolume) {
            currentVolume = std::max(targetVolume, currentVolume - step);
        }
    }
};

TEST_CASE("OpenAL Sound Stream - 3D Positional Attenuation and Falloff")
{
    Audio3DListenerModel listener;
    listener.SetPosition(0.0f, 0.0f, 0.0f);
    listener.SetOrientation(0.0f, 1.0f, 0.0f, 0.0f, 0.0f, 1.0f);

    float minDistance = 200.0f;
    float maxDistance = 2000.0f;

    // 1. Zrodlo blisko sluchacza (wewnatrz minDistance) -> 100% glosnosci
    float nearAtt = listener.Calculate3DAttenuation(50.0f, 50.0f, 0.0f, minDistance, maxDistance);
    CHECK(nearAtt == doctest::Approx(1.0f));

    // 2. Zrodlo dokladnie w polowie odleglosci -> ~50%
    float midAtt = listener.Calculate3DAttenuation(0.0f, 1100.0f, 0.0f, minDistance, maxDistance);
    CHECK(midAtt == doctest::Approx(0.5f));

    // 3. Zrodlo poza maxDistance -> 0% (culling)
    float farAtt = listener.Calculate3DAttenuation(0.0f, 2500.0f, 0.0f, minDistance, maxDistance);
    CHECK(farAtt == doctest::Approx(0.0f));
}

TEST_CASE("OpenAL Sound Stream - Stereo Panning Calculation")
{
    Audio3DListenerModel listener;
    listener.SetPosition(0.0f, 0.0f, 0.0f);
    // Patrzy w gore w osi Y (North)
    listener.SetOrientation(0.0f, 1.0f, 0.0f, 0.0f, 0.0f, 1.0f);

    // Zrodlo na wprost (0, 100) -> pan 0.0 (centrum)
    float centerPan = listener.CalculateStereoPan(0.0f, 100.0f);
    CHECK(centerPan == doctest::Approx(0.0f));

    // Zrodlo po prawej stronie na osi X (100, 0) -> pan +1.0 (prawy kanal)
    float rightPan = listener.CalculateStereoPan(100.0f, 0.0f);
    CHECK(rightPan == doctest::Approx(1.0f));

    // Zrodlo po lewej stronie na osi X (-100, 0) -> pan -1.0 (lewy kanal)
    float leftPan = listener.CalculateStereoPan(-100.0f, 0.0f);
    CHECK(leftPan == doctest::Approx(-1.0f));
}

TEST_CASE("OpenAL Sound Stream - Background Music (BGM) Fade In and Out")
{
    MusicStreamFadeModel fade;
    fade.currentVolume = 0.0f;
    fade.targetVolume = 1.0f;
    fade.fadeDuration = 1.0f;

    // Krok 0.5s -> polowa glosnosci
    fade.StepFade(0.5f);
    CHECK(fade.currentVolume > 0.0f);
    CHECK(fade.currentVolume < 1.0f);

    // Krok do konca -> pelna glosnosc 1.0
    for (int i = 0; i < 10; ++i) {
        fade.StepFade(0.2f);
    }
    CHECK(fade.currentVolume == doctest::Approx(1.0f));

    // Fade out do 0.0
    fade.targetVolume = 0.0f;
    fade.fadeDuration = 0.5f;
    for (int i = 0; i < 10; ++i) {
        fade.StepFade(0.1f);
    }
    CHECK(fade.currentVolume == doctest::Approx(0.0f));
}
