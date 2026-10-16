#pragma once

#include <cstdint>
#include <string_view>
#include "EterBase/Result.h"

namespace Client::Bridge {
    class IPythonRuntime;
}

namespace Client::Skills {

    class ISkillPlayerAdapter;

    /**
     * @brief Domain errors for Skill Affect operations
     */
    enum class SkillAffectError : uint8_t {
        None = 0,
        PythonRuntimeNotAvailable,
        AdapterNotAvailable,
        AddAffectFailed,
        RemoveAffectFailed
    };

    /**
     * @brief Bridge to decouple affect (buff/debuff) network events from legacy UI.
     * Uses IPythonRuntime to invoke python callbacks and ISkillPlayerAdapter for state reset.
     */
    class SkillAffectBridge {
    public:
        SkillAffectBridge(Client::Bridge::IPythonRuntime* pythonRuntime, ISkillPlayerAdapter* playerAdapter);

        EterBase::VoidResult<SkillAffectError> AddAffect(uint32_t type, uint8_t applyOn, int32_t applyValue, uint32_t duration, uint32_t flag) const noexcept;
        EterBase::VoidResult<SkillAffectError> RemoveAffect(uint32_t type, uint8_t applyOn) const noexcept;

    private:
        Client::Bridge::IPythonRuntime* m_pythonRuntime;
        ISkillPlayerAdapter* m_playerAdapter;
    };

} // namespace Client::Skills
