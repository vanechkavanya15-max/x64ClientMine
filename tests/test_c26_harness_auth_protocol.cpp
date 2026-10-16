#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest/doctest.h"
#include "Client/IPC/HarnessAuthProtocolRouter.h"

using namespace Client::IPC;

class MockAuthNetworkProvider : public IAuthNetworkProvider {
public:
    std::expected<void, AuthError> loginResult = {};
    std::expected<std::vector<CharacterSlotInfo>, AuthError> slotsResult = std::vector<CharacterSlotInfo>{};
    std::expected<void, AuthError> selectResult = {};
    std::expected<void, AuthError> createResult = {};
    
    int loginCalls = 0;

    std::expected<void, AuthError> SendLogin(std::string_view user, std::string_view pass) override {
        loginCalls++;
        return loginResult;
    }

    std::expected<std::vector<CharacterSlotInfo>, AuthError> RequestCharacterSlots() override {
        return slotsResult;
    }

    std::expected<void, AuthError> SendSelectCharacter(uint8_t slotIndex) override {
        return selectResult;
    }

    std::expected<void, AuthError> SendCreateCharacter(uint8_t slotIndex, std::string_view name, uint8_t job, uint8_t shape) override {
        return createResult;
    }
};

TEST_CASE("HarnessAuthProtocolRouter: Credentials validation") {
    MockAuthNetworkProvider mockProvider;
    HarnessAuthProtocolRouter auth(mockProvider);
    
    CHECK_FALSE(auth.HasCredentials());
    
    // Invalid length
    auto res1 = auth.SetCredentials("usr", "pass123");
    CHECK_FALSE(res1.has_value());
    CHECK(res1.error() == AuthError::InvalidCredentials);
    
    auto res2 = auth.SetCredentials("user", "123");
    CHECK_FALSE(res2.has_value());
    CHECK(res2.error() == AuthError::InvalidCredentials);
    
    // Valid
    auto res3 = auth.SetCredentials("player1", "pass123");
    CHECK(res3.has_value());
    CHECK(auth.HasCredentials());
}

TEST_CASE("HarnessAuthProtocolRouter: Full state machine flow") {
    MockAuthNetworkProvider mockProvider;
    HarnessAuthProtocolRouter auth(mockProvider);
    
    auth.SetCredentials("player1", "pass123");
    
    // Login -> Authenticated
    mockProvider.loginResult = {};
    auto loginRes = auth.Login();
    CHECK(loginRes.has_value());
    CHECK(auth.GetState() == AuthState::Authenticated);
    
    // Fetch Slots -> SlotsReady
    mockProvider.slotsResult = std::vector<CharacterSlotInfo>{{0, "Hero", 10, 1, 3600}};
    auto slotsRes = auth.GetCharacterSlots();
    CHECK(slotsRes.has_value());
    CHECK(slotsRes.value().size() == 1);
    CHECK(auth.GetState() == AuthState::SlotsReady);
    
    // Create Character -> SlotsReady
    mockProvider.createResult = {};
    auto createRes = auth.CreateCharacter(1, "NewHero", 2, 0);
    CHECK(createRes.has_value());
    CHECK(auth.GetState() == AuthState::SlotsReady);
    
    // Select Character -> EnteringGame
    mockProvider.selectResult = {};
    auto selectRes = auth.SelectCharacter(0);
    CHECK(selectRes.has_value());
    CHECK(auth.GetState() == AuthState::EnteringGame);
}

TEST_CASE("HarnessAuthProtocolRouter: Negative state transitions and failures") {
    MockAuthNetworkProvider mockProvider;
    HarnessAuthProtocolRouter auth(mockProvider);
    
    auth.SetCredentials("player1", "pass123");
    
    // Network error on login
    mockProvider.loginResult = std::unexpected(AuthError::NetworkError);
    auto loginRes = auth.Login();
    CHECK_FALSE(loginRes.has_value());
    CHECK(loginRes.error() == AuthError::NetworkError);
    CHECK(auth.GetState() == AuthState::Disconnected);
    
    // Try to get slots while disconnected
    auto slotsRes = auth.GetCharacterSlots();
    CHECK_FALSE(slotsRes.has_value());
    CHECK(slotsRes.error() == AuthError::InvalidStateTransition);
    
    // Login successfully
    mockProvider.loginResult = {};
    auth.Login();
    CHECK(auth.GetState() == AuthState::Authenticated);
    
    // Try to select character before fetching slots
    auto selectRes = auth.SelectCharacter(0);
    CHECK_FALSE(selectRes.has_value());
    CHECK(selectRes.error() == AuthError::InvalidStateTransition);
    
    // Try invalid slot
    mockProvider.slotsResult = std::vector<CharacterSlotInfo>{};
    auth.GetCharacterSlots();
    CHECK(auth.GetState() == AuthState::SlotsReady);
    
    auto invalidSelectRes = auth.SelectCharacter(4);
    CHECK_FALSE(invalidSelectRes.has_value());
    CHECK(invalidSelectRes.error() == AuthError::InvalidSlot);
    CHECK(auth.GetState() == AuthState::SlotsReady);
    
    // Try invalid character name
    auto invalidNameRes = auth.CreateCharacter(1, "Bad Name!", 1, 1);
    CHECK_FALSE(invalidNameRes.has_value());
    CHECK(invalidNameRes.error() == AuthError::InvalidCharacterName);
}

TEST_CASE("HarnessAuthProtocolRouter: Auto-reconnect backoff") {
    MockAuthNetworkProvider mockProvider;
    HarnessAuthProtocolRouter auth(mockProvider);
    
    auth.SetCredentials("player1", "pass123");
    auth.SetAutoReconnect(true);
    
    // First reconnect, backoff 2s
    mockProvider.loginResult = std::unexpected(AuthError::NetworkError);
    
    auth.TickAutoReconnect(std::chrono::milliseconds(1000));
    CHECK(mockProvider.loginCalls == 0); // Not enough time
    
    auth.TickAutoReconnect(std::chrono::milliseconds(1000));
    CHECK(mockProvider.loginCalls == 1); // 2s elapsed, attempt 1
    
    // Second reconnect, backoff 4s
    auth.TickAutoReconnect(std::chrono::milliseconds(3000));
    CHECK(mockProvider.loginCalls == 1); // Not enough time
    
    auth.TickAutoReconnect(std::chrono::milliseconds(1000));
    CHECK(mockProvider.loginCalls == 2); // 4s elapsed, attempt 2
    
    // Third reconnect, backoff 8s
    auth.TickAutoReconnect(std::chrono::milliseconds(8000));
    CHECK(mockProvider.loginCalls == 3); // 8s elapsed, attempt 3
}
