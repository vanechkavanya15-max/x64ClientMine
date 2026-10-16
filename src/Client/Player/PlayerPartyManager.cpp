#include "PlayerPartyManager.h"
#include "Client/Network/Handlers/PartyPacketDomainHandler.h"

namespace Client::Player {

PlayerPartyManager::PlayerPartyManager(std::shared_ptr<Client::Gameplay::SocialManager> socialManager)
    : m_socialManager(std::move(socialManager)) {
}

Client::Core::Result<std::vector<uint8_t>, PartyError> PlayerPartyManager::InviteToParty(EterBase::EntityId targetVid) {
    if (auto party = m_socialManager->GetParty()) {
        // Jesli jest w party, moze zapraszac tylko wtedy, gdy jest liderem
    }
    
    return Client::Network::Handlers::PartyPacketDomainHandler::EncodePartyInvite(targetVid.Value());
}

Client::Core::Result<std::vector<uint8_t>, PartyError> PlayerPartyManager::AcceptInvite(EterBase::EntityId leaderVid) {
    if (m_socialManager->GetParty()) {
        return std::unexpected(PartyError::AlreadyInParty);
    }
    return Client::Network::Handlers::PartyPacketDomainHandler::EncodePartyInviteAnswer(leaderVid.Value(), 1);
}

Client::Core::Result<std::vector<uint8_t>, PartyError> PlayerPartyManager::DeclineInvite(EterBase::EntityId leaderVid) {
    if (m_socialManager->GetParty()) {
        return std::unexpected(PartyError::AlreadyInParty);
    }
    return Client::Network::Handlers::PartyPacketDomainHandler::EncodePartyInviteAnswer(leaderVid.Value(), 0);
}

Client::Core::Result<std::vector<uint8_t>, PartyError> PlayerPartyManager::LeaveParty() {
    auto party = m_socialManager->GetParty();
    if (!party) {
        return std::unexpected(PartyError::NotInParty);
    }

    m_socialManager->LeaveParty();
    
    return Client::Network::Handlers::PartyPacketDomainHandler::EncodePartyRemove(0);
}

Client::Core::Result<std::vector<uint8_t>, PartyError> PlayerPartyManager::RemoveMember(EterBase::EntityId pid) {
    auto party = m_socialManager->GetParty();
    if (!party) {
        return std::unexpected(PartyError::NotInParty);
    }

    return Client::Network::Handlers::PartyPacketDomainHandler::EncodePartyRemove(pid.Value());
}

Client::Core::Result<std::vector<uint8_t>, PartyError> PlayerPartyManager::SetDistributeMode(uint8_t mode) {
    auto party = m_socialManager->GetParty();
    if (!party) {
        return std::unexpected(PartyError::NotInParty);
    }
    
    return Client::Network::Handlers::PartyPacketDomainHandler::EncodePartyParameter(mode);
}

} // namespace Client::Player
