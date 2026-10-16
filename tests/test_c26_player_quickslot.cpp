#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#define TEST_MODE_DISABLE_STDAFX 1
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

TEST_CASE("PlayerQuickslotController functionality") {
    auto domain = std::make_shared<QuickslotDomain>();
    auto network = std::make_shared<MockQuickslotNetwork>();
    PlayerQuickslotController controller(domain, network);

    SUBCASE("Set and Get Quickslot") {
        network->Reset();
        QuickslotItem item{1, 5};
        auto setRes = controller.SetQuickslot(0, item);
        CHECK(setRes.has_value());

        auto getRes = controller.GetQuickslot(0);
        REQUIRE(getRes.has_value());
        CHECK(getRes->type == 1);
        CHECK(getRes->position == 5);
        CHECK_FALSE(getRes->IsEmpty());

        CHECK(network->addPacketsSent == 1);
        CHECK(network->lastAddPos == 0);
        CHECK(network->lastAddType == 1);
        CHECK(network->lastAddItemPos == 5);
    }

    SUBCASE("Delete Quickslot") {
        QuickslotItem item{1, 5};
        REQUIRE(controller.SetQuickslot(0, item).has_value());

        network->Reset();
        auto delRes = controller.DeleteQuickslot(0);
        CHECK(delRes.has_value());

        auto getRes = controller.GetQuickslot(0);
        REQUIRE(getRes.has_value());
        CHECK(getRes->IsEmpty());

        CHECK(network->delPacketsSent == 1);
        CHECK(network->lastDelPos == 0);
    }

    SUBCASE("Swap Quickslots") {
        QuickslotItem itemA{2, 10};
        QuickslotItem itemB{3, 20};
        REQUIRE(controller.SetQuickslot(1, itemA).has_value());
        REQUIRE(controller.SetQuickslot(2, itemB).has_value());

        network->Reset();
        auto swapRes = controller.SwapQuickslots(1, 2);
        CHECK(swapRes.has_value());

        auto getA = controller.GetQuickslot(1);
        auto getB = controller.GetQuickslot(2);
        REQUIRE(getA.has_value());
        REQUIRE(getB.has_value());
        CHECK(getA->type == 3);
        CHECK(getA->position == 20);
        CHECK(getB->type == 2);
        CHECK(getB->position == 10);

        CHECK(network->movePacketsSent == 1);
        CHECK(network->lastMoveFrom == 1);
        CHECK(network->lastMoveTo == 2);
    }

    SUBCASE("Clear All Quickslots") {
        QuickslotItem itemA{2, 10};
        QuickslotItem itemB{3, 20};
        REQUIRE(controller.SetQuickslot(1, itemA).has_value());
        REQUIRE(controller.SetQuickslot(2, itemB).has_value());

        network->Reset();
        controller.ClearAll();

        auto getRes1 = controller.GetQuickslot(1);
        REQUIRE(getRes1.has_value());
        CHECK(getRes1->IsEmpty());

        auto getRes2 = controller.GetQuickslot(2);
        REQUIRE(getRes2.has_value());
        CHECK(getRes2->IsEmpty());

        // We expect delPackets to be sent for slots that were not empty (1 and 2)
        CHECK(network->delPacketsSent == 2); 
    }

    SUBCASE("Error Handling") {
        network->Reset();
        QuickslotItem item{1, 1};
        CHECK_FALSE(controller.SetQuickslot(100, item).has_value());
        CHECK(network->addPacketsSent == 0);

        CHECK_FALSE(controller.DeleteQuickslot(100).has_value());
        CHECK(network->delPacketsSent == 0);

        CHECK_FALSE(controller.SwapQuickslots(0, 100).has_value());
        CHECK(network->movePacketsSent == 0);

        CHECK_FALSE(controller.GetQuickslot(100).has_value());
    }
}
