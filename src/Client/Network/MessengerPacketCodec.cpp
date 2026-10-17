#include "MessengerPacketCodec.h"
#include <cstring>

namespace Client::Network {

    EterBase::PacketResult<MessengerDomainEvent> MessengerPacketCodec::DecodeMessenger(std::span<const uint8_t> buffer) {
        if (buffer.size() < sizeof(TPacketGCMessenger)) {
            return std::unexpected(EterBase::PacketError::BufferUnderflow);
        }

        TPacketGCMessenger headerPacket;
        std::memcpy(&headerPacket, buffer.data(), sizeof(TPacketGCMessenger));

        if (headerPacket.length < sizeof(TPacketGCMessenger)) {
            return std::unexpected(EterBase::PacketError::BufferUnderflow);
        }

        if (buffer.size() < headerPacket.length) {
            return std::unexpected(EterBase::PacketError::BufferUnderflow);
        }

        MessengerDomainEvent event;
        event.subheader = headerPacket.subheader;

        size_t offset = sizeof(TPacketGCMessenger);
        size_t remainingLength = headerPacket.length - offset;

        switch (event.subheader) {
            case MessengerSub::GC::LIST: {
                while (remainingLength > 0) {
                    if (remainingLength < sizeof(TPacketGCMessengerListOffline)) {
                        return std::unexpected(EterBase::PacketError::BufferUnderflow);
                    }

                    TPacketGCMessengerListOffline entryHeader;
                    std::memcpy(&entryHeader, buffer.data() + offset, sizeof(TPacketGCMessengerListOffline));

                    offset += sizeof(TPacketGCMessengerListOffline);
                    remainingLength -= sizeof(TPacketGCMessengerListOffline);

                    if (remainingLength < entryHeader.length) {
                        return std::unexpected(EterBase::PacketError::BufferUnderflow);
                    }

                    std::string name(reinterpret_cast<const char*>(buffer.data() + offset), entryHeader.length);
                    offset += entryHeader.length;
                    remainingLength -= entryHeader.length;

                    bool isOnline = (entryHeader.connected & MESSENGER_CONNECTED_STATE_ONLINE) != 0;
                    event.friends.push_back({name, isOnline});
                }
                break;
            }

            case MessengerSub::GC::LOGIN: {
                if (remainingLength < sizeof(TPacketGCMessengerLogin)) {
                    return std::unexpected(EterBase::PacketError::BufferUnderflow);
                }

                TPacketGCMessengerLogin loginHeader;
                std::memcpy(&loginHeader, buffer.data() + offset, sizeof(TPacketGCMessengerLogin));

                offset += sizeof(TPacketGCMessengerLogin);
                remainingLength -= sizeof(TPacketGCMessengerLogin);

                if (remainingLength < loginHeader.length) {
                    return std::unexpected(EterBase::PacketError::BufferUnderflow);
                }

                std::string name(reinterpret_cast<const char*>(buffer.data() + offset), loginHeader.length);
                event.targetName = name;
                break;
            }

            case MessengerSub::GC::LOGOUT: {
                if (remainingLength < sizeof(TPacketGCMessengerLogout)) {
                    return std::unexpected(EterBase::PacketError::BufferUnderflow);
                }

                TPacketGCMessengerLogout logoutHeader;
                std::memcpy(&logoutHeader, buffer.data() + offset, sizeof(TPacketGCMessengerLogout));

                offset += sizeof(TPacketGCMessengerLogout);
                remainingLength -= sizeof(TPacketGCMessengerLogout);

                if (remainingLength < logoutHeader.length) {
                    return std::unexpected(EterBase::PacketError::BufferUnderflow);
                }

                std::string name(reinterpret_cast<const char*>(buffer.data() + offset), logoutHeader.length);
                event.targetName = name;
                break;
            }

            case MessengerSub::GC::REMOVE_FRIEND: {
                if (remainingLength < sizeof(uint8_t)) {
                    return std::unexpected(EterBase::PacketError::BufferUnderflow);
                }

                uint8_t nameLength = buffer[offset];
                offset += sizeof(uint8_t);
                remainingLength -= sizeof(uint8_t);

                if (remainingLength < nameLength) {
                    return std::unexpected(EterBase::PacketError::BufferUnderflow);
                }

                std::string name(reinterpret_cast<const char*>(buffer.data() + offset), nameLength);
                event.targetName = name;
                break;
            }

            default:
                break;
        }

        return event;
    }

} // namespace Client::Network
