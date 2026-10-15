#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"

#define TEST_MODE_DISABLE_STDAFX 1

#include "Client/Player/PlayerInventoryController.h"
#include "Client/Contracts/IInventoryDomainContract.h"

using namespace Client::Player;
using namespace Client::Contracts;
using namespace Client::Core;

class MockInventoryDomain : public IInventoryDomainContract {
public:
    std::optional<ItemDataSnapshot> itemToReturn;
    std::vector<ItemDataSnapshot> allItemsToReturn;
    bool isCellEmptyToReturn = true;
    size_t freeSlotCountToReturn = 10;
    bool actionSuccess = true;

    int useItemCalls = 0;
    int moveItemCalls = 0;
    int dropItemCalls = 0;

    std::optional<ItemDataSnapshot> GetItem(const ItemCellAddress& pos) const override {
        return itemToReturn;
    }

    std::vector<ItemDataSnapshot> GetAllItems(uint8_t windowType) const override {
        return allItemsToReturn;
    }

    bool IsCellEmpty(const ItemCellAddress& pos) const noexcept override {
        return isCellEmptyToReturn;
    }

    size_t GetFreeSlotCount(uint8_t windowType) const noexcept override {
        return freeSlotCountToReturn;
    }

    bool RequestUseItem(const ItemCellAddress& pos) override {
        useItemCalls++;
        return actionSuccess;
    }

    bool RequestMoveItem(const ItemCellAddress& src, const ItemCellAddress& dst, uint32_t count) override {
        moveItemCalls++;
        return actionSuccess;
    }

    bool RequestDropItem(const ItemCellAddress& pos, uint32_t count) override {
        dropItemCalls++;
        return actionSuccess;
    }
};

TEST_CASE("PlayerInventoryController - DomainContract Null") {
    PlayerInventoryController controller(nullptr);

    CHECK(!controller.GetItemInfo(1, 0).has_value());
    CHECK(controller.GetItemInfo(1, 0).error() == InventoryError::DomainContractNull);

    CHECK(!controller.GetAllItems(1).has_value());
    CHECK(controller.GetAllItems(1).error() == InventoryError::DomainContractNull);

    CHECK(!controller.IsCellEmpty(1, 0).has_value());
    CHECK(controller.IsCellEmpty(1, 0).error() == InventoryError::DomainContractNull);

    CHECK(!controller.GetFreeSlotCount(1).has_value());
    CHECK(controller.GetFreeSlotCount(1).error() == InventoryError::DomainContractNull);

    CHECK(!controller.RequestUseItem(1, 0).has_value());
    CHECK(controller.RequestUseItem(1, 0).error() == InventoryError::DomainContractNull);

    CHECK(!controller.RequestMoveItem(1, 0, 1, 1, 1).has_value());
    CHECK(controller.RequestMoveItem(1, 0, 1, 1, 1).error() == InventoryError::DomainContractNull);

    CHECK(!controller.RequestDropItem(1, 0, 1).has_value());
    CHECK(controller.RequestDropItem(1, 0, 1).error() == InventoryError::DomainContractNull);
}

TEST_CASE("PlayerInventoryController - Invalid Window Type") {
    auto mock = std::make_shared<MockInventoryDomain>();
    PlayerInventoryController controller(mock);

    uint8_t invalidWindow = 99;

    CHECK(!controller.GetItemInfo(invalidWindow, 0).has_value());
    CHECK(controller.GetItemInfo(invalidWindow, 0).error() == InventoryError::InvalidWindowType);

    CHECK(!controller.RequestUseItem(invalidWindow, 0).has_value());
    CHECK(controller.RequestUseItem(invalidWindow, 0).error() == InventoryError::InvalidWindowType);
}

TEST_CASE("PlayerInventoryController - GetItemInfo") {
    auto mock = std::make_shared<MockInventoryDomain>();
    PlayerInventoryController controller(mock);

    SUBCASE("Item found") {
        mock->itemToReturn = ItemDataSnapshot{ {1, 0}, 123, 5, {}, {}, {}, false, "Sword" };
        auto result = controller.GetItemInfo(1, 0);
        REQUIRE(result.has_value());
        CHECK(result.value().vnum == 123);
        CHECK(result.value().count == 5);
        CHECK(result.value().name == "Sword");
    }

    SUBCASE("Item not found") {
        mock->itemToReturn = std::nullopt;
        auto result = controller.GetItemInfo(1, 0);
        CHECK(!result.has_value());
        CHECK(result.error() == InventoryError::ItemNotFound);
    }
}

TEST_CASE("PlayerInventoryController - GetAllItems") {
    auto mock = std::make_shared<MockInventoryDomain>();
    PlayerInventoryController controller(mock);

    mock->allItemsToReturn = {
        ItemDataSnapshot{ {1, 0}, 123, 1, {}, {}, {}, false, "Sword1" },
        ItemDataSnapshot{ {1, 1}, 456, 2, {}, {}, {}, false, "Sword2" }
    };

    auto result = controller.GetAllItems(1);
    REQUIRE(result.has_value());
    CHECK(result.value().size() == 2);
    CHECK(result.value()[0].vnum == 123);
    CHECK(result.value()[1].vnum == 456);
}

TEST_CASE("PlayerInventoryController - Status Check") {
    auto mock = std::make_shared<MockInventoryDomain>();
    PlayerInventoryController controller(mock);

    mock->isCellEmptyToReturn = false;
    auto emptyRes = controller.IsCellEmpty(1, 0);
    REQUIRE(emptyRes.has_value());
    CHECK(emptyRes.value() == false);

    mock->freeSlotCountToReturn = 5;
    auto countRes = controller.GetFreeSlotCount(1);
    REQUIRE(countRes.has_value());
    CHECK(countRes.value() == 5);
}

TEST_CASE("PlayerInventoryController - Actions") {
    auto mock = std::make_shared<MockInventoryDomain>();
    PlayerInventoryController controller(mock);

    SUBCASE("Success") {
        mock->actionSuccess = true;
        
        auto res1 = controller.RequestUseItem(1, 0);
        CHECK(res1.has_value());
        CHECK(mock->useItemCalls == 1);

        auto res2 = controller.RequestMoveItem(1, 0, 1, 1, 1);
        CHECK(res2.has_value());
        CHECK(mock->moveItemCalls == 1);

        auto res3 = controller.RequestDropItem(1, 0, 1);
        CHECK(res3.has_value());
        CHECK(mock->dropItemCalls == 1);
    }

    SUBCASE("Failure") {
        mock->actionSuccess = false;

        auto res1 = controller.RequestUseItem(1, 0);
        CHECK(!res1.has_value());
        CHECK(res1.error() == InventoryError::ActionFailed);

        auto res2 = controller.RequestMoveItem(1, 0, 1, 1, 1);
        CHECK(!res2.has_value());
        CHECK(res2.error() == InventoryError::ActionFailed);

        auto res3 = controller.RequestDropItem(1, 0, 1);
        CHECK(!res3.has_value());
        CHECK(res3.error() == InventoryError::ActionFailed);
    }
}
