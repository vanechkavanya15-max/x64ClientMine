#pragma once

#include <cstdint>
#include <vector>
#include <memory>

#include "Client/Core/Result.h"
#include "EterBase/StrongTypes.h"
#include "Client/Gameplay/SocialDomain.h"

namespace Client::Player {

/**
 * @brief Enum błędu operacji drużynowych.
 */
enum class PartyError : uint8_t {
    None = 0,
    NotInParty,
    AlreadyInParty,
    NotLeader,
    TargetInvalid,
    NetworkError
};

/**
 * @brief Kontroler gracza odpowiedzialny za zarządzanie drużyną (party).
 * 
 * Izolowany moduł C++23. Wykorzystuje domenę społecznościową (SocialManager)
 * oraz generuje pakiety sieciowe za pomocą PartyPacketDomainHandler.
 */
class PlayerPartyManager {
public:
    explicit PlayerPartyManager(std::shared_ptr<Client::Gameplay::SocialManager> socialManager);
    ~PlayerPartyManager() = default;

    Client::Core::Result<std::vector<uint8_t>, PartyError> InviteToParty(EterBase::EntityId targetVid);
    Client::Core::Result<std::vector<uint8_t>, PartyError> AcceptInvite(EterBase::EntityId leaderVid);
    Client::Core::Result<std::vector<uint8_t>, PartyError> DeclineInvite(EterBase::EntityId leaderVid);
    Client::Core::Result<std::vector<uint8_t>, PartyError> LeaveParty();
    Client::Core::Result<std::vector<uint8_t>, PartyError> RemoveMember(EterBase::EntityId pid);
    Client::Core::Result<std::vector<uint8_t>, PartyError> SetDistributeMode(uint8_t mode);

private:
    std::shared_ptr<Client::Gameplay::SocialManager> m_socialManager;
};

} // namespace Client::Player
