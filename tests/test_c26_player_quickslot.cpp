#define TEST_MODE_DISABLE_STDAFX 1
#include <cassert>
#include <iostream>
#include <memory>
#include "../src/Client/Player/PlayerQuickslotController.h"
#include "../src/Client/Player/IQuickslotNetwork.h"
#include "../src/Client/Gameplay/QuickslotDomain.h"

using namespace Client::Player;
using namespace Client::Gameplay;

class MockQuickslotNetwork : public IQuickslotNetwork {
public:
    int addPacketsSent = 0;
    int delPacketsSent = 0;
    int movePacketsSent = 0;

    uint8_t lastAddPos = 0;
    uint8_t lastAddType = 0;
    uint8_t lastAddItemPos = 0;

    uint8_t lastDelPos = 0;
    
    uint8_t lastMoveFrom = 0;
    uint8_t lastMoveTo = 0;

    bool SendQuickSlotAddPacket(uint8_t pos, uint8_t type, uint8_t itemPos) override {
        addPacketsSent++;
        lastAddPos = pos;
        lastAddType = type;
        lastAddItemPos = itemPos;
        return true;
    }

    bool SendQuickSlotDelPacket(uint8_t pos) override {
        delPacketsSent++;
        lastDelPos = pos;
        return true;
    }

    bool SendQuickSlotMovePacket(uint8_t pos, uint8_t change_pos) override {
        movePacketsSent++;
        lastMoveFrom = pos;
        lastMoveTo = change_pos;
        return true;
    }

    void Reset() {
        addPacketsSent = 0;
        delPacketsSent = 0;
        movePacketsSent = 0;
    }
};

int main()
{
    auto domain = std::make_shared<QuickslotDomain>();
    auto network = std::make_shared<MockQuickslotNetwork>();
    PlayerQuickslotController controller(domain, network);

    // Test 1: Set and Get Quickslot
    {
        network->Reset();
        QuickslotItem item{1, 5};
        auto setRes = controller.SetQuickslot(0, item);
        assert(setRes.has_value());

        auto getRes = controller.GetQuickslot(0);
        assert(getRes.has_value());
        assert(getRes->type == 1);
        assert(getRes->position == 5);
        assert(!getRes->IsEmpty());

        assert(network->addPacketsSent == 1);
        assert(network->lastAddPos == 0);
        assert(network->lastAddType == 1);
        assert(network->lastAddItemPos == 5);
    }

    // Test 2: Delete Quickslot
    {
        network->Reset();
        auto delRes = controller.DeleteQuickslot(0);
        assert(delRes.has_value());

        auto getRes = controller.GetQuickslot(0);
        assert(getRes.has_value());
        assert(getRes->IsEmpty());

        assert(network->delPacketsSent == 1);
        assert(network->lastDelPos == 0);
    }

    // Test 3: Swap Quickslots
    {
        QuickslotItem itemA{2, 10};
        QuickslotItem itemB{3, 20};
        assert(controller.SetQuickslot(1, itemA).has_value());
        assert(controller.SetQuickslot(2, itemB).has_value());

        network->Reset();
        auto swapRes = controller.SwapQuickslots(1, 2);
        assert(swapRes.has_value());

        auto getA = controller.GetQuickslot(1);
        auto getB = controller.GetQuickslot(2);
        assert(getA->type == 3 && getA->position == 20);
        assert(getB->type == 2 && getB->position == 10);

        assert(network->movePacketsSent == 1);
        assert(network->lastMoveFrom == 1);
        assert(network->lastMoveTo == 2);
    }

    // Test 4: Clear All
    {
        network->Reset();
        controller.ClearAll();
        auto getRes = controller.GetQuickslot(1);
        assert(getRes.has_value());
        assert(getRes->IsEmpty());

        // We expect delPackets to be sent for slots that were not empty (1 and 2)
        assert(network->delPacketsSent == 2); 
    }

    // Test 5: Error Handling
    {
        network->Reset();
        QuickslotItem item{1, 1};
        assert(!controller.SetQuickslot(100, item).has_value());
        assert(network->addPacketsSent == 0);

        assert(!controller.DeleteQuickslot(100).has_value());
        assert(network->delPacketsSent == 0);

        assert(!controller.SwapQuickslots(0, 100).has_value());
        assert(network->movePacketsSent == 0);

        assert(!controller.GetQuickslot(100).has_value());
    }

    std::cout << "test_c26_player_quickslot: ALL TESTS PASSED (100%)\n";
    return 0;
}
