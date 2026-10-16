#include "PhaseGameWhisperBridge.h"
#include <string>
#include <cstring>

namespace Client::Bridge {
    EterBase::PacketResult<void> PhaseGameWhisperBridge::Handle(std::span<const uint8_t> payload) {
        if (payload.size() < sizeof(PacketWhisperHeader)) {
            return EterBase::MakeError(EterBase::PacketError::BufferUnderflow);
        }

        const auto* packet = reinterpret_cast<const PacketWhisperHeader*>(payload.data());
        
        if (packet->length < sizeof(PacketWhisperHeader)) {
            return EterBase::MakeError(EterBase::PacketError::MalformedPayload);
        }
        
        if (payload.size() < packet->length) {
            return EterBase::MakeError(EterBase::PacketError::BufferUnderflow);
        }

        size_t nameLen = strnlen(packet->senderName, sizeof(packet->senderName));
        std::string senderName(packet->senderName, nameLen);

        size_t messageLen = packet->length - sizeof(PacketWhisperHeader);
        const char* textPtr = reinterpret_cast<const char*>(payload.data() + sizeof(PacketWhisperHeader));
        std::string message(textPtr, messageLen);

        if (packet->type == 0 /* WHISPER_TYPE_CHAT */ || packet->type == 5 /* WHISPER_TYPE_GM */) {
            std::string line = senderName + " : " + message;
            m_pythonRuntime.Invoke("game", "OnRecvWhisper", packet->type, senderName, line);
        } else if (packet->type == 0xFF /* WHISPER_TYPE_SYSTEM */ || packet->type == 4 /* WHISPER_TYPE_ERROR */) {
            m_pythonRuntime.Invoke("game", "OnRecvWhisperSystemMessage", packet->type, senderName, message);
        } else {
            m_pythonRuntime.Invoke("game", "OnRecvWhisperError", packet->type, senderName, message);
        }

        return {};
    }
}
