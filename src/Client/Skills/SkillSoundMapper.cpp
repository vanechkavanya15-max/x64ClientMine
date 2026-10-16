#include "SkillSoundMapper.h"

namespace Client::Skills {

    Core::Result<void, Core::SkillError> SkillSoundMapper::RegisterSound(Core::SkillIndex skillId, std::string soundPath) {
        if (soundPath.empty()) {
            return std::unexpected(Core::SkillError::RequirementNotMet); // Wymagamy niepustej sciezki
        }

        try {
            soundMap_[skillId] = std::move(soundPath);
            return {};
        } catch (...) {
            // W przypadku problemow z alokacja pamieci, choc w nowoczesnym C++ i przy unordered_map nie musimy szczegolowo mapowac,
            // wedlug wytycznych mozemy rzucac domain error zamiast pozwalac na crash, lub zaufac ze catch(...) przechwyci std::bad_alloc
            // W typowym C++ mozna po prostu przepuscic bad_alloc, ale bezpieczniej jest zwrocic blad jesli zalezy nam na noexcept-podobnych 
            // zachowaniach w domenie (choc tutaj RegisterSound nie jest noexcept).
            return std::unexpected(Core::SkillError::RequirementNotMet);
        }
    }

    Core::Result<std::string_view, Core::SkillError> SkillSoundMapper::GetSoundPath(Core::SkillIndex skillId) const noexcept {
        auto it = soundMap_.find(skillId);
        if (it != soundMap_.end()) {
            return std::string_view(it->second);
        }
        return std::unexpected(Core::SkillError::SkillNotFound);
    }

    Core::Result<void, Core::SkillError> SkillSoundMapper::UnregisterSound(Core::SkillIndex skillId) {
        auto it = soundMap_.find(skillId);
        if (it != soundMap_.end()) {
            soundMap_.erase(it);
            return {};
        }
        return std::unexpected(Core::SkillError::SkillNotFound);
    }

    void SkillSoundMapper::Clear() noexcept {
        soundMap_.clear();
    }

    size_t SkillSoundMapper::GetMappedCount() const noexcept {
        return soundMap_.size();
    }

} // namespace Client::Skills
