#pragma once

#include <cstdint>
#include <vector>

#include "../Gameplay/SkillDomain.h"
#include "../Gameplay/IPlayerStatsService.h"
#include "../Network/SkillPacketCodec.h"
#include "../Core/Result.h"

namespace Client::Player {

    /**
     * @brief Kody bledow domenowych kontrolera skilli gracza.
     */
    enum class CommandError : uint8_t {
        None = 0,
        SkillNotFound,
        NotEnoughSP,
        OnCooldown,
        RequirementNotMet,
        NetworkError,
        TargetInvalid
    };

    /**
     * @class PlayerSkillController
     * @brief Kontroler odpowiedzialny za uzywanie umiejetnosci przez gracza.
     * 
     * Implementuje logike walidacji uzycia umiejetnosci z wykorzystaniem SkillDomain,
     * a nastepnie koduje pakiet sieciowy za pomoca SkillPacketCodec.
     */
    class PlayerSkillController {
    public:
        PlayerSkillController(Client::Gameplay::SkillDomain& skillDomain, Client::Gameplay::IPlayerStatsService& statsService);
        ~PlayerSkillController() = default;

        /**
         * @brief Proba uzycia umiejetnosci na wybranym celu.
         * 
         * @param skillVnum VNUM (Id) umiejetnosci.
         * @param targetVid VID celu.
         * @return Wynik operacji - payload pakietu sieciowego lub blad CommandError.
         */
        Client::Core::Result<std::vector<uint8_t>, CommandError> UseSkill(uint32_t skillVnum, uint32_t targetVid);

    private:
        Client::Gameplay::SkillDomain& m_skillDomain;
        Client::Gameplay::IPlayerStatsService& m_statsService;

        CommandError MapSkillError(Client::Core::SkillError err) const noexcept;
    };

} // namespace Client::Player
