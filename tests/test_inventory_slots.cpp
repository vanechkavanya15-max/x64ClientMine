#include <doctest.h>
#include "Client/Gameplay/InventorySlotOwner.hpp"

using namespace Client::Gameplay;
using namespace Client::Contracts;
using namespace Client::Core;

TEST_CASE("InventorySlotOwner assigns and releases item correctly") {
    InventorySlotOwner owner;

    REQUIRE_FALSE(owner.HasItem());
    REQUIRE(owner.GetItem() == nullptr);

    ItemSlotDTO item;
    item.vnum = 123;
    item.count = 5;

    auto result = owner.Assign(item);
    REQUIRE(result.has_value());

    REQUIRE(owner.HasItem());
    REQUIRE(owner.GetItem() != nullptr);
    REQUIRE(owner.GetItem()->vnum == 123);
    REQUIRE(owner.GetItem()->count == 5);

    auto releaseResult = owner.Release();
    REQUIRE(releaseResult.has_value());
    REQUIRE(releaseResult->vnum == 123);
    REQUIRE(releaseResult->count == 5);

    REQUIRE_FALSE(owner.HasItem());
    REQUIRE(owner.GetItem() == nullptr);
}

TEST_CASE("InventorySlotOwner prevents double assignment") {
    InventorySlotOwner owner;
    ItemSlotDTO item;
    item.vnum = 123;
    item.count = 5;
    owner.Assign(item);

    ItemSlotDTO item2;
    item2.vnum = 456;
    item2.count = 1;

    auto result = owner.Assign(item2);

    REQUIRE_FALSE(result.has_value());
    REQUIRE(result.error() == InventorySlotError::AlreadyOwned);
}

TEST_CASE("InventorySlotOwner prevents release when empty") {
    InventorySlotOwner owner;

    auto result = owner.Release();
    REQUIRE_FALSE(result.has_value());
    REQUIRE(result.error() == InventorySlotError::NotOwned);
}

TEST_CASE("InventorySlotOwner respects single ownership via move semantics") {
    InventorySlotOwner owner1;
    ItemSlotDTO item;
    item.vnum = 123;
    item.count = 5;
    owner1.Assign(item);

    InventorySlotOwner owner2 = std::move(owner1);

    REQUIRE_FALSE(owner1.HasItem());
    REQUIRE(owner2.HasItem());
    REQUIRE(owner2.GetItem()->vnum == 123);
}
