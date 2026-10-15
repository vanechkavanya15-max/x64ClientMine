#pragma once

#include <cstdint>

namespace Client::Player {

class IQuickslotNetwork {
public:
    virtual ~IQuickslotNetwork() = default;

    virtual bool SendQuickSlotAddPacket(uint8_t pos, uint8_t type, uint8_t itemPos) = 0;
    virtual bool SendQuickSlotDelPacket(uint8_t pos) = 0;
    virtual bool SendQuickSlotMovePacket(uint8_t pos, uint8_t change_pos) = 0;
};

} // namespace Client::Player
