#pragma once

#include <cstdint>

namespace Client::Skills {

    /**
     * @brief Adapter interface to decouple affect (buff/debuff) network events
     * from the legacy CPythonPlayer singleton.
     */
    class ISkillPlayerAdapter {
    public:
        virtual ~ISkillPlayerAdapter() = default;

        /**
         * @brief Resets the affect state in the player instance.
         */
        virtual void ResetAffectState() noexcept = 0;
    };

} // namespace Client::Skills
