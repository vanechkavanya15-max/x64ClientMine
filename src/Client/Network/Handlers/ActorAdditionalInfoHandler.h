#pragma once

#include "ActorAdditionalInfoPacketHandler.h"

namespace Client::Network::Handlers
{
    // Alias zapewniajacy kompatybilnosc wsteczna dla alternatywnego nazewnictwa handlera
    using ActorAdditionalInfoHandler = ActorAdditionalInfoPacketHandler;

    inline EterBase::PacketResult<void> HandleActorAdditionalInfo(std::span<const uint8_t> payload, Client::Network::IPendingSpawnRegistry& registry)
    {
        return ActorAdditionalInfoPacketHandler::Handle(payload, registry);
    }
}
