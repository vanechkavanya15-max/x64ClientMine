#include "PhaseGameMessengerBridge.h"
#include "Client/Network/MessengerPacketCodec.h"
#include "Client/Network/Protocol/Protocol_GuildParty.h"

namespace Client::Bridge {

    bool PhaseGameMessengerBridge::HandleMessenger(std::span<const uint8_t> buffer) {
        auto result = Client::Network::MessengerPacketCodec::DecodeMessenger(buffer);
        if (!result) {
            return false;
        }

        const auto& event = result.value();

        switch (event.subheader) {
            case MessengerSub::GC::LIST: {
                for (const auto& friendEntry : event.friends) {
                    if (friendEntry.isOnline) {
                        m_pythonRuntime.Invoke("messenger", "OnFriendLogin", friendEntry.name);
                    } else {
                        m_pythonRuntime.Invoke("messenger", "OnFriendLogout", friendEntry.name);
                    }
                }
                break;
            }

            case MessengerSub::GC::LOGIN: {
                m_pythonRuntime.Invoke("messenger", "OnFriendLogin", event.targetName);
                m_pythonRuntime.Invoke("game", "RefreshTargetBoardByName", event.targetName);
                break;
            }

            case MessengerSub::GC::LOGOUT: {
                m_pythonRuntime.Invoke("messenger", "OnFriendLogout", event.targetName);
                break;
            }

            case MessengerSub::GC::REMOVE_FRIEND: {
                m_pythonRuntime.Invoke("messenger", "RemoveFriend", event.targetName);
                m_pythonRuntime.Invoke("game", "RefreshTargetBoardByName", event.targetName);
                break;
            }

            default:
                break;
        }

        return true;
    }

} // namespace Client::Bridge
