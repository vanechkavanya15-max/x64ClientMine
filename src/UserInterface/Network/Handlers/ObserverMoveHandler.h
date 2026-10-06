#pragma once

#include <cstdint>
#include <span>

class CPythonNetworkStream;

#pragma pack(push, 1)
/**
 * @brief Strictly aligned packet structure for OBSERVER_MOVE network packet
 */
struct TPacketGCObserverMove
{
    uint16_t header;
    uint16_t length;
    uint32_t id;
    uint16_t x;
    uint16_t y;
};
#pragma pack(pop)

/**
 * @brief Handler for network packets related to observer movement
 */
class ObserverMoveHandler
{
public:
    /**
     * @brief Processes the OBSERVER_MOVE network packet
     * @param networkStream Pointer to the active network stream
     * @param packetData The binary span containing the packet payload
     * @return true if successfully processed, false if payload was invalid
     */
    static bool HandleObserverMove(CPythonNetworkStream* networkStream, std::span<const uint8_t> packetData);
};
