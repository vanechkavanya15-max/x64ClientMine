#include "ActorAffectBridge.h"
#include "Client/Bridge/IPythonRuntime.h"

namespace Client::Actors {

    ActorAffectBridge::ActorAffectBridge(Client::Bridge::IPythonRuntime* pythonRuntime) noexcept
        : m_pythonRuntime(pythonRuntime)
    {
    }

    EterBase::VoidResult<ActorAffectError> ActorAffectBridge::SetAffect(uint32_t vid, uint32_t affectType, bool isVisible) const noexcept
    {
        if (!m_pythonRuntime) {
            return std::unexpected(ActorAffectError::PythonRuntimeNotAvailable);
        }

        // The python invocation maps to chrmgrSetAffect in UserInterface/PythonCharacterManagerModule.cpp
        // Note: the third argument is actually evaluated as integer 1/0 in python via PyTuple_GetInteger in chrmgrSetAffect
        auto result = m_pythonRuntime->Invoke("chrmgr", "SetAffect", vid, affectType, isVisible ? 1 : 0);
        if (!result.has_value()) {
            return std::unexpected(ActorAffectError::SetAffectFailed);
        }

        return {};
    }

} // namespace Client::Actors
