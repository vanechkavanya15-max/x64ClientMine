#include "StdAfx.h"
#include "../../PythonNetworkStream.h"
#include "../../PythonSafeBox.h"
#include "../../Packet.h"

/**
 * @brief Handles the reception of the safe box size update packet.
 * 
 * This handler is responsible for processing the SAFEBOX_SIZE network packet.
 * It reads the packet size and updates the safe box capacity state in CPythonSafeBox.
 * Complies with C++20 standard, strict single responsibility, and decoupled from GUI calls.
 * 
 * @return true if the packet was successfully received and processed, false otherwise.
 */
bool CPythonNetworkStream::RecvSafeBoxSizePacket()
{
    TPacketGCSafeboxSize safeBoxSizePacket;
    if (!Recv(sizeof(safeBoxSizePacket), &safeBoxSizePacket))
    {
        return false;
    }

    // Update the internal safe box data structure
    CPythonSafeBox::Instance().OpenSafeBox(safeBoxSizePacket.bSize);

    // Note: Python UI notifications (e.g. OpenSafeboxWindow) are deliberately removed
    // to decouple the network handler from the GUI layer as per architecture rules.

    return true;
}
