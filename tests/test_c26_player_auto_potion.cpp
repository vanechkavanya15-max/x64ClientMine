#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include "Client/Player/PlayerAutoPotionController.h"
#include "Client/Gameplay/AutoPotionManager.h"
#include "Client/Contracts/IPlayerDomainContract.h"
#include "Client/Core/INetworkPort.h"

// Stubs for network so we don't compile InventoryCommandEncoder.cpp on linux tests
#include "Client/Network/InventoryCommandEncoder.h"
namespace Client::Network {
    [[nodiscard]] EterBase::PacketResult<std::vector<uint8_t>> InventoryCommandEncoder::EncodeUseItem(EterBase::ItemSlot slot) {
        std::vector<uint8_t> buffer(sizeof(uint8_t) + sizeof(uint16_t)); // Mock packet size
        buffer[0] = 0x0C; // mock opcode
        return buffer;
    }
}

class MockPlayerDomainContract : public Client::Contracts::IPlayerDomainContract {
public:
    int64_t hp = 100;
    int64_t maxHp = 100;
    int64_t sp = 100;
    int64_t maxSp = 100;

    [[nodiscard]] uint32_t GetPlayerVID() const noexcept override { return 1; }
    [[nodiscard]] std::string_view GetPlayerName() const noexcept override { return "TestPlayer"; }
    [[nodiscard]] int GetPlayerLevel() const noexcept override { return 1; }

    [[nodiscard]] int64_t GetCurrentHP() const noexcept override { return hp; }
    [[nodiscard]] int64_t GetMaxHP() const noexcept override { return maxHp; }
    [[nodiscard]] int64_t GetCurrentSP() const noexcept override { return sp; }
    [[nodiscard]] int64_t GetMaxSP() const noexcept override { return maxSp; }
    [[nodiscard]] int64_t GetYang() const noexcept override { return 0; }
    [[nodiscard]] int64_t GetExp() const noexcept override { return 0; }

    [[nodiscard]] Client::Contracts::PlayerCoordinates GetCoordinates() const noexcept override { return {0.0f, 0.0f, 0.0f, 0.0f}; }
    [[nodiscard]] uint32_t GetTargetVID() const noexcept override { return 0; }
    [[nodiscard]] bool IsInCombat() const noexcept override { return false; }

    [[nodiscard]] Client::Contracts::PlayerStatusSnapshot CaptureSnapshot() const override {
        return Client::Contracts::PlayerStatusSnapshot{};
    }
};

class MockNetworkPort : public Client::Core::INetworkPort {
public:
    bool packetSent = false;
    uint8_t lastOpcode = 0;
    std::vector<uint8_t> lastPayload;

    [[nodiscard]] Client::Core::Result<void, Client::Core::PacketError> SendRaw(uint8_t opcode, std::span<const uint8_t> payload) override {
        packetSent = true;
        lastOpcode = opcode;
        lastPayload.assign(payload.begin(), payload.end());
        return {};
    }

    [[nodiscard]] bool IsConnected() const noexcept override { return true; }
    void Reset() {
        packetSent = false;
        lastOpcode = 0;
        lastPayload.clear();
    }
};

TEST_SUITE("PlayerAutoPotionController") {
    TEST_CASE("Healing triggers when HP falls below threshold") {
        auto manager = std::make_shared<Client::Gameplay::AutoPotionManager>();
        auto contract = std::make_shared<MockPlayerDomainContract>();
        auto network = std::make_shared<MockNetworkPort>();

        Client::Player::PlayerAutoPotionController controller(manager, contract, network);

        // Setup AutoPotion
        auto enableResult = manager->Enable(Client::Gameplay::AutoPotionType::Health, 50.0f, 1000); // Heal if < 50%
        REQUIRE(enableResult.has_value());

        EterBase::ItemSlot slot{10};
        controller.SetPotionSlot(Client::Gameplay::AutoPotionType::Health, slot);

        // HP is full, should not trigger
        contract->hp = 100;
        contract->maxHp = 100;
        auto result = controller.Update(1000);
        REQUIRE(result.has_value());
        CHECK_FALSE(network->packetSent);

        // HP drops below 50%
        contract->hp = 40;
        result = controller.Update(2000);
        REQUIRE(result.has_value());
        CHECK(network->packetSent); // Should send network packet for item use
    }

    TEST_CASE("No action on full HP") {
        auto manager = std::make_shared<Client::Gameplay::AutoPotionManager>();
        auto contract = std::make_shared<MockPlayerDomainContract>();
        auto network = std::make_shared<MockNetworkPort>();

        Client::Player::PlayerAutoPotionController controller(manager, contract, network);

        manager->Enable(Client::Gameplay::AutoPotionType::Health, 50.0f, 1000);
        controller.SetPotionSlot(Client::Gameplay::AutoPotionType::Health, EterBase::ItemSlot{10});

        contract->hp = 100;
        contract->maxHp = 100;
        auto result = controller.Update(1000);
        
        REQUIRE(result.has_value());
        CHECK_FALSE(network->packetSent);
    }

    TEST_CASE("Proper use-after-cooldown") {
        auto manager = std::make_shared<Client::Gameplay::AutoPotionManager>();
        auto contract = std::make_shared<MockPlayerDomainContract>();
        auto network = std::make_shared<MockNetworkPort>();

        Client::Player::PlayerAutoPotionController controller(manager, contract, network);

        manager->Enable(Client::Gameplay::AutoPotionType::Health, 50.0f, 1000);
        controller.SetPotionSlot(Client::Gameplay::AutoPotionType::Health, EterBase::ItemSlot{10});

        contract->hp = 40;
        contract->maxHp = 100;
        
        // First heal should work
        auto result = controller.Update(1000);
        REQUIRE(result.has_value());
        CHECK(network->packetSent);
        network->Reset();

        // Second heal immediately after should be blocked by cooldown
        result = controller.Update(1500); // Only 500ms passed, default cooldown is 1000ms
        REQUIRE(result.has_value()); // The update itself succeeds, but process skips healing
        CHECK_FALSE(network->packetSent);

        // Third heal after cooldown should work again
        result = controller.Update(2500);
        REQUIRE(result.has_value());
        CHECK(network->packetSent);
    }

    TEST_CASE("Checking error returns if not configured") {
        Client::Player::PlayerAutoPotionController controller(nullptr, nullptr, nullptr);
        auto result = controller.Update(1000);
        REQUIRE_FALSE(result.has_value());
        CHECK(result.error() == Client::Player::AutoPotionControllerError::ManagerNotSet);
    }
}
