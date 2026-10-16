#pragma once

#include <array>
#include "Core/MathSIMD.h"

namespace Client::Vegetation {

class SpeedTreeWindAnimationModel {
public:
    static constexpr size_t NUM_WIND_MATRICES = 4;

    SpeedTreeWindAnimationModel() = default;

    // Calculates and updates internal state for the wind matrices.
    // Replaces legacy CSpeedTreeForest::SetupWindMatrices global state mutation.
    std::array<Client::Core::Mat4, NUM_WIND_MATRICES> UpdateAndGetMatrices(float timeInSecs, float windStrength);

private:
    float lastTimeInSecs_ = 0.0f;
    float oldWindStrength_ = 0.0f;
    std::array<float, NUM_WIND_MATRICES> matrixTimes_ = {0.0f, 0.0f, 0.0f, 0.0f};

    static constexpr std::array<std::array<float, 2>, NUM_WIND_MATRICES> FREQUENCIES = {{
        {0.15f, 0.17f},
        {0.25f, 0.15f},
        {0.19f, 0.05f},
        {0.15f, 0.22f}
    }};
};

} // namespace Client::Vegetation
