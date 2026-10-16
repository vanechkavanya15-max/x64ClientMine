#include "SpeedTreeLodEvaluator.h"
#include <algorithm>

namespace Client::Vegetation {

SpeedTreeLodEvaluator::SpeedTreeLodEvaluator(LodThresholds thresholds)
    : m_thresholds(thresholds)
{
}

Core::Result<LodResult, LodError> SpeedTreeLodEvaluator::Evaluate(float distance) const noexcept {
    if (distance < 0.0f) {
        return std::unexpected(LodError::InvalidDistance);
    }

    if (m_thresholds.maxDistance <= m_thresholds.minDistance) {
        return std::unexpected(LodError::ThresholdsMisconfigured);
    }

    if (distance >= m_thresholds.maxDistance) {
        return LodResult{1.0f, false};
    }

    if (distance <= m_thresholds.minDistance) {
        return LodResult{0.0f, true};
    }

    float range = m_thresholds.maxDistance - m_thresholds.minDistance;
    float lodLevel = (distance - m_thresholds.minDistance) / range;

    return LodResult{std::clamp(lodLevel, 0.0f, 1.0f), true};
}

void SpeedTreeLodEvaluator::SetThresholds(LodThresholds thresholds) noexcept {
    m_thresholds = thresholds;
}

LodThresholds SpeedTreeLodEvaluator::GetThresholds() const noexcept {
    return m_thresholds;
}

} // namespace Client::Vegetation
