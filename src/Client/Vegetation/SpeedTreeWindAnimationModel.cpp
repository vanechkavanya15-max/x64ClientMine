#include "SpeedTreeWindAnimationModel.h"
#include <cmath>

namespace Client::Vegetation {

std::array<Client::Core::Mat4, SpeedTreeWindAnimationModel::NUM_WIND_MATRICES> 
SpeedTreeWindAnimationModel::UpdateAndGetMatrices(float timeInSecs, float windStrength) {
    float timeSinceLastCall = timeInSecs - lastTimeInSecs_;
    lastTimeInSecs_ = timeInSecs;

    for (size_t i = 0; i < NUM_WIND_MATRICES; ++i) {
        matrixTimes_[i] += timeSinceLastCall;
    }

    float baseAngle = windStrength * 35.0f;
    std::array<Client::Core::Mat4, NUM_WIND_MATRICES> result;

    for (size_t j = 0; j < NUM_WIND_MATRICES; ++j) {
        if (windStrength != 0.0f) {
            matrixTimes_[j] = (matrixTimes_[j] * oldWindStrength_) / windStrength;
        }

        float baseFreq = windStrength * 20.0f;
        float xPercent = std::sin(baseFreq * FREQUENCIES[j][0] * matrixTimes_[j]);
        float yPercent = std::cos(baseFreq * FREQUENCIES[j][1] * matrixTimes_[j]);

        const float DEG2RAD = 57.2957795f;
        float sinX = std::sin(baseAngle * xPercent / DEG2RAD);
        float sinY = std::sin(baseAngle * yPercent / DEG2RAD);
        float cosX = std::cos(baseAngle * xPercent / DEG2RAD);
        float cosY = std::cos(baseAngle * yPercent / DEG2RAD);

        float m[16] = { 0.0f };
        m[0] = cosY;
        m[2] = -sinY;
        m[4] = sinX * sinY;
        m[5] = cosX;
        m[6] = sinX * cosY;
        m[8] = sinY * cosX;
        m[9] = -sinX;
        m[10] = cosX * cosY;
        m[15] = 1.0f;

        result[j] = Client::Core::Mat4(m);
    }

    oldWindStrength_ = windStrength;

    return result;
}

} // namespace Client::Vegetation
