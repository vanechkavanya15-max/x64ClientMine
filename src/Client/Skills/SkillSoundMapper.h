#pragma once

#include "Client/Core/StrongTypes.h"
#include "Client/Core/Result.h"
#include "Client/Core/DomainErrors.h"
#include <unordered_map>
#include <string>
#include <string_view>

namespace Client::Skills {

    class SkillSoundMapper {
    public:
        SkillSoundMapper() = default;
        ~SkillSoundMapper() = default;

        // Powiazuje dzwiek z konkretna umiejetnoscia.
        Core::Result<void, Core::SkillError> RegisterSound(Core::SkillIndex skillId, std::string soundPath);

        // Zwraca sciezke do pliku dzwiekowego dla danej umiejetnosci.
        [[nodiscard]] Core::Result<std::string_view, Core::SkillError> GetSoundPath(Core::SkillIndex skillId) const noexcept;

        // Usuwa powiazanie dzwieku dla danej umiejetnosci.
        Core::Result<void, Core::SkillError> UnregisterSound(Core::SkillIndex skillId);

        // Czysci wszystkie powiazania.
        void Clear() noexcept;

        // Zwraca liczbe powiazan.
        [[nodiscard]] size_t GetMappedCount() const noexcept;

    private:
        std::unordered_map<Core::SkillIndex, std::string> soundMap_;
    };

} // namespace Client::Skills
