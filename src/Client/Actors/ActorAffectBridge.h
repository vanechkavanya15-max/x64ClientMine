#pragma once

#include <cstdint>
#include <string_view>
#include "EterBase/Result.h"

namespace Client::Bridge {
    class IPythonRuntime;
}

namespace Client::Actors {

    /**
     * @brief Domain errors for Actor Affect operations
     */
    enum class ActorAffectError : uint8_t {
        None = 0,
        PythonRuntimeNotAvailable,
        SetAffectFailed
    };

    /**
     * @brief Bridge to decouple actor affect (buff/debuff) logic from legacy UI/DirectX components.
     * Uses IPythonRuntime to invoke python callbacks headlessly.
     */
    class ActorAffectBridge {
    public:
        explicit ActorAffectBridge(Client::Bridge::IPythonRuntime* pythonRuntime) noexcept;

        /**
         * @brief Notifies python layer to set or unset an affect on an actor.
         */
        EterBase::VoidResult<ActorAffectError> SetAffect(uint32_t vid, uint32_t affectType, bool isVisible) const noexcept;

    private:
        Client::Bridge::IPythonRuntime* m_pythonRuntime;
    };

} // namespace Client::Actors
