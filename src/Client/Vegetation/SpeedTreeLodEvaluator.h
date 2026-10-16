#pragma once

#include <cstdint>
#include <string_view>
#include <format>
#include "Core/Result.h"

namespace Client::Vegetation {

enum class LodError : uint8_t {
    InvalidDistance,
    ThresholdsMisconfigured,
    Unknown
};

[[nodiscard]] constexpr std::string_view to_string(LodError error) noexcept {
    switch (error) {
        case LodError::InvalidDistance: return "LodError::InvalidDistance - Evaluated distance is negative or invalid";
        case LodError::ThresholdsMisconfigured: return "LodError::ThresholdsMisconfigured - Max distance is less than or equal to min distance";
        default: return "LodError::Unknown";
    }
}

struct LodResult {
    float lodLevel;       // Continuous LOD value between 0.0 (closest, highest detail) and 1.0 (furthest, lowest detail)
    bool isVisible;       // True if within max distance
};

struct LodThresholds {
    float minDistance;
    float maxDistance;
};

class SpeedTreeLodEvaluator {
public:
    SpeedTreeLodEvaluator(LodThresholds thresholds);

    [[nodiscard]] Core::Result<LodResult, LodError> Evaluate(float distance) const noexcept;

    void SetThresholds(LodThresholds thresholds) noexcept;
    [[nodiscard]] LodThresholds GetThresholds() const noexcept;

private:
    LodThresholds m_thresholds;
};

} // namespace Client::Vegetation

template <>
struct std::formatter<Client::Vegetation::LodError> : std::formatter<std::string_view> {
    auto format(Client::Vegetation::LodError err, std::format_context& ctx) const {
        return std::formatter<std::string_view>::format(Client::Vegetation::to_string(err), ctx);
    }
};
