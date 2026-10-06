#pragma once

#include <cstdint>
#include <string>

namespace Network
{
    /**
     * @brief Typ serwera docelowego w architekturze Multi-Server C++20.
     */
    enum class ServerType : uint8_t
    {
        Official = 0,    // Oryginalny serwer Metin2 x64 (libsodium, naglowki CG::/GC:: 16-bit)
        Beavium  = 1,    // Prywatny serwer Beavium x64 (Handshake TimeSync 0xFF, AES, opcody 8-bit)
        Auto     = 2     // Automatyczne wykrywanie typu serwera przy handshake
    };

    inline const char* ServerTypeToString(ServerType type)
    {
        switch (type)
        {
            case ServerType::Official: return "Official (Standard x64)";
            case ServerType::Beavium:  return "Beavium (x64)";
            case ServerType::Auto:     return "Auto-Detect";
            default:                   return "Unknown";
        }
    }
}
