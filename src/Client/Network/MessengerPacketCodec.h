#pragma once

#include <cstdint>
#include <vector>
#include <span>
#include <string>
#include <expected>

#include "EterBase/Result.h"
#include "Client/Network/Protocol/Protocol.h"
#include "Client/Network/Protocol/Protocol_GuildParty.h"

namespace Client::Network {

    struct MessengerFriendEvent {
        std::string name;
        bool isOnline;
    };

    struct MessengerDomainEvent {
        uint8_t subheader;
        std::vector<MessengerFriendEvent> friends;
        std::string targetName;
    };

    class MessengerPacketCodec {
    public:
        static EterBase::PacketResult<MessengerDomainEvent> DecodeMessenger(std::span<const uint8_t> buffer);
    };

} // namespace Client::Network
