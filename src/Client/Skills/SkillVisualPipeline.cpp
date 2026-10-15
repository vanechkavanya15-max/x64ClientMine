#include "SkillVisualPipeline.h"
#include <algorithm>

namespace Client::Skills {

    SkillVisualPipeline::SkillVisualPipeline(ISkillVisualEffectSystem& effectSystem) 
        : effectSystem_(effectSystem) {}

    Core::Result<void, Core::SkillError> SkillVisualPipeline::TriggerSkillEffect(
        Core::EntityVid target, 
        Core::SkillIndex skillId, 
        float durationSec,
        std::optional<Core::MapCoords> coords) 
    {
        if (!target) {
            return std::unexpected(Core::SkillError::RequirementNotMet); // Cel jest wymagany
        }

        if (durationSec <= 0.0f) {
            return std::unexpected(Core::SkillError::RequirementNotMet); // Czas trwania musi byc dodatni
        }

        auto result = effectSystem_.PlayEffect(target, skillId, coords);
        if (!result) {
            return result;
        }

        activeEffects_.push_back(SkillVisualEffect{
            .target = target,
            .skillId = skillId,
            .coords = coords,
            .remainingDuration = durationSec
        });

        return {};
    }

    Core::Result<void, Core::SkillError> SkillVisualPipeline::CancelEffect(Core::EntityVid target, Core::SkillIndex skillId) {
        auto it = std::find_if(activeEffects_.begin(), activeEffects_.end(),
            [target, skillId](const SkillVisualEffect& effect) {
                return effect.target == target && effect.skillId == skillId;
            });

        if (it == activeEffects_.end()) {
            return std::unexpected(Core::SkillError::SkillNotFound);
        }

        // Zatrzymanie w systemie zewnetrznym
        auto result = effectSystem_.StopEffect(target, skillId);
        if (result) {
            activeEffects_.erase(it);
        }
        return result;
    }

    void SkillVisualPipeline::Update(float deltaTime) {
        std::erase_if(activeEffects_, [this, deltaTime](SkillVisualEffect& effect) {
            effect.remainingDuration -= deltaTime;
            if (effect.remainingDuration <= 0.0f) {
                // Wywolujemy StopEffect, ale ignorujemy bledy podczas wygasania
                (void)effectSystem_.StopEffect(effect.target, effect.skillId);
                return true;
            }
            return false;
        });
    }

    size_t SkillVisualPipeline::GetActiveEffectsCount() const noexcept {
        return activeEffects_.size();
    }

} // namespace Client::Skills
