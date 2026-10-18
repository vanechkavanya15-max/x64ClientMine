#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include "../src/Client/Network/InventoryCommandEncoder.h"
#include "../src/Client/Network/Protocol/Protocol.h"

TEST_CASE("InventoryCommandEncoder - EncodeUseItem")
{
    EterBase::ItemSlot slot(10);
    auto result = Client::Network::InventoryCommandEncoder::EncodeUseItem(slot);
    REQUIRE(result.has_value());

    const auto& buffer = result.value();
    REQUIRE(buffer.size() == sizeof(TPacketCGItemUse));

    const TPacketCGItemUse* packet = reinterpret_cast<const TPacketCGItemUse*>(buffer.data());
    CHECK(packet->header == CG::ITEM_USE);
    CHECK(packet->length == sizeof(TPacketCGItemUse));
    CHECK(packet->pos.window_type == INVENTORY);
    CHECK(packet->pos.cell == 10);
}

TEST_CASE("InventoryCommandEncoder - EncodeDropItem")
{
    EterBase::ItemSlot slot(15);
    uint32_t gold = 1000;
    uint8_t count = 5;

    auto result = Client::Network::InventoryCommandEncoder::EncodeDropItem(slot, gold, count);
    REQUIRE(result.has_value());

    const auto& buffer = result.value();
    REQUIRE(buffer.size() == sizeof(TPacketCGItemDrop2));

    const TPacketCGItemDrop2* packet = reinterpret_cast<const TPacketCGItemDrop2*>(buffer.data());
    CHECK(packet->header == CG::ITEM_DROP2);
    CHECK(packet->length == sizeof(TPacketCGItemDrop2));
    CHECK(packet->pos.window_type == INVENTORY);
    CHECK(packet->pos.cell == 15);
    CHECK(packet->gold == 1000);
    CHECK(packet->count == 5);
}

TEST_CASE("InventoryCommandEncoder - EncodePickupItem")
{
    EterBase::EntityId vid(404);
    auto result = Client::Network::InventoryCommandEncoder::EncodePickupItem(vid);
    REQUIRE(result.has_value());

    const auto& buffer = result.value();
    REQUIRE(buffer.size() == sizeof(TPacketCGItemPickUp));

    const TPacketCGItemPickUp* packet = reinterpret_cast<const TPacketCGItemPickUp*>(buffer.data());
    CHECK(packet->header == CG::ITEM_PICKUP);
    CHECK(packet->length == sizeof(TPacketCGItemPickUp));
    CHECK(packet->vid == 404);
}
