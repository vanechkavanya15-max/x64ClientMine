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
        explicit SkillVisualPipeline(ISkillVisualEffectSystem& effectSystem);

        // Inicjuje efekt dla konkretnego celu.
        Core::Result<void, Core::SkillError> TriggerSkillEffect(
            Core::EntityVid target, 
            Core::SkillIndex skillId, 
            float durationSec,
            std::optional<Core::MapCoords> coords = std::nullopt);

        // Anuluje efekt recznie przed jego wygasnieciem.
        Core::Result<void, Core::SkillError> CancelEffect(Core::EntityVid target, Core::SkillIndex skillId);

        // Aktualizuje wszystkie aktywne efekty i usuwa te, ktore wygasly.
        void Update(float deltaTime);

        // Zwraca liczbe aktywnych efektow (glownie do celow testowych).
        [[nodiscard]] size_t GetActiveEffectsCount() const noexcept;

    private:
        ISkillVisualEffectSystem& effectSystem_;
        std::vector<SkillVisualEffect> activeEffects_;
    };

} // namespace Client::Skills
