#pragma once

#include "Client/Core/StrongTypes.h"
#include "Client/Core/Result.h"
#include "Client/Core/DomainErrors.h"
#include <vector>
#include <memory>
#include <optional>
#include <algorithm>

namespace Client::Skills {

    // Reprezentacja efektu wizualnego umiejetnosci.
    struct SkillVisualEffect {
        Core::EntityVid target;
        Core::SkillIndex skillId;
        std::optional<Core::MapCoords> coords;
        float remainingDuration; // czas trwania w sekundach

        bool operator==(const SkillVisualEffect& other) const = default;
    };

    // Interfejs do systemu efektow wizualnych (oddzielenie od silnika renderujacego).
    class ISkillVisualEffectSystem {
    public:
        virtual ~ISkillVisualEffectSystem() = default;

        // Uruchamia efekt wizualny na danym celu
        virtual Core::Result<void, Core::SkillError> PlayEffect(Core::EntityVid target, Core::SkillIndex skillId, const std::optional<Core::MapCoords>& coords) = 0;
        
        // Zatrzymuje wczesniej uruchomiony efekt
        virtual Core::Result<void, Core::SkillError> StopEffect(Core::EntityVid target, Core::SkillIndex skillId) = 0;
    };

    // Glowny potok wizualny dla umiejetnosci, zarzadza czasem zycia efektow.
    class SkillVisualPipeline {
    public:
        explicit SkillVisualPipeline(ISkillVisualEffectSystem& effectSystem) 
            : effectSystem_(effectSystem) {}

        // Inicjuje efekt dla konkretnego celu.
        Core::Result<void, Core::SkillError> TriggerSkillEffect(
            Core::EntityVid target, 
            Core::SkillIndex skillId, 
            float durationSec,
            std::optional<Core::MapCoords> coords = std::nullopt) 
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

        // Anuluje efekt recznie przed jego wygasnieciem.
        Core::Result<void, Core::SkillError> CancelEffect(Core::EntityVid target, Core::SkillIndex skillId) {
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

        // Aktualizuje wszystkie aktywne efekty i usuwa te, ktore wygasly.
        void Update(float deltaTime) {
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

        // Zwraca liczbe aktywnych efektow (glownie do celow testowych).
        [[nodiscard]] size_t GetActiveEffectsCount() const noexcept {
            return activeEffects_.size();
        }

    private:
        ISkillVisualEffectSystem& effectSystem_;
        std::vector<SkillVisualEffect> activeEffects_;
    };

} // namespace Client::Skills
