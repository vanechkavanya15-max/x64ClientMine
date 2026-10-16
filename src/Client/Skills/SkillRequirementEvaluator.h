#pragma once
#include <cstdint>

namespace Client::Skills {

struct SkillUsageContext {
    uint32_t currentSP{0};
    uint32_t requiredSP{0};
    float currentDistance{0.0f};
    float maxRange{0.0f};
    bool weaponAllowed{true};
    bool isCooling{false};
};

class SkillRequirementEvaluator {
public:
    SkillRequirementEvaluator() = default;
    ~SkillRequirementEvaluator() = default;

    [[nodiscard]] static bool CanCast(const SkillUsageContext& ctx) noexcept {
        if (ctx.isCooling) return false;
        if (!ctx.weaponAllowed) return false;
        if (ctx.currentSP < ctx.requiredSP) return false;
        if (ctx.maxRange > 0.0f && ctx.currentDistance > ctx.maxRange) return false;
        return true;
    }
};

} // namespace Client::Skills
