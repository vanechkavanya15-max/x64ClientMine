#include "SkillAffectBridge.h"
#include "ISkillPlayerAdapter.h"
#include "Client/Bridge/IPythonRuntime.h"

namespace Client::Skills {

    SkillAffectBridge::SkillAffectBridge(Client::Bridge::IPythonRuntime* pythonRuntime, ISkillPlayerAdapter* playerAdapter)
        : m_pythonRuntime(pythonRuntime), m_playerAdapter(playerAdapter)
    {
    }

    EterBase::VoidResult<SkillAffectError> SkillAffectBridge::AddAffect(uint32_t type, uint8_t applyOn, int32_t applyValue, uint32_t duration, uint32_t flag) const noexcept
    {
        if (!m_pythonRuntime) {
            return std::unexpected(SkillAffectError::PythonRuntimeNotAvailable);
        }

        auto result = m_pythonRuntime->Invoke("game", "BINARY_NEW_AddAffect", type, applyOn, applyValue, duration, flag);
        if (!result.has_value()) {
            return std::unexpected(SkillAffectError::AddAffectFailed);
        }

        return {};
    }

    EterBase::VoidResult<SkillAffectError> SkillAffectBridge::RemoveAffect(uint32_t type, uint8_t applyOn) const noexcept
    {
        if (!m_pythonRuntime) {
            return std::unexpected(SkillAffectError::PythonRuntimeNotAvailable);
        }
        
        if (!m_playerAdapter) {
            return std::unexpected(SkillAffectError::AdapterNotAvailable);
        }

        auto result = m_pythonRuntime->Invoke("game", "BINARY_NEW_RemoveAffect", type, applyOn);
        if (!result.has_value()) {
            return std::unexpected(SkillAffectError::RemoveAffectFailed);
        }

        m_playerAdapter->ResetAffectState();

        return {};
    }

} // namespace Client::Skills
